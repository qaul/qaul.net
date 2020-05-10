use crate::{error::Result, Identity};
use alexandria::{
    query::{Query, QueryResult},
    utils::{Path, Tag, TagSet},
    Library, Session,
};
use async_std::sync::Arc;
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use tracing::warn;

/// An arbitrary map of metadata that can be stored by a service
///
/// Data is stored per service/per user and is tagged with search
/// tags.  This structure (and API) can be used to store service
/// related data on a device that will be encrypted and can be loaded
/// on reboot, meaning that your service doesn't have to worry about
/// storing things securely on different platforms.
///
/// `MetadataMap` has a builder API that makes constructing initial
/// maps easier than just providing an already initialised BTreeMap.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MetadataMap {
    name: String,
    map: BTreeMap<String, Vec<u8>>,
}

impl MetadataMap {
    /// Creates a new, empty metadata map
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            map: Default::default(),
        }
    }

    /// Create a metadata map from a name and initialised map construct
    ///
    /// ```
    /// # use libqaul::services::MetadataMap;
    /// MetadataMap::from("numbers", vec![("fav", vec![1, 2, 3, 4])]);
    /// ```
    ///
    /// Because from takes `IntoIterator`, you can also initialise
    /// your map in-place:
    ///
    /// ```
    /// # use libqaul::services::MetadataMap;
    /// MetadataMap::from("numbers", vec![
    ///     ("fav", vec![1, 2, 3, 4]),
    ///     ("prime", vec![1, 3, 5, 7, 11]),
    ///     ("acab", vec![13, 12]),
    /// ]);
    /// ```
    pub fn from<S, K, M, V>(name: S, map: M) -> Self
    where
        S: Into<String>,
        K: Into<String>,
        M: IntoIterator<Item = (K, V)>,
        V: IntoIterator<Item = u8>,
    {
        let name = name.into();
        let map = map
            .into_iter()
            .map(|(k, v)| (k.into(), v.into_iter().collect()))
            .collect();
        Self { name, map }
    }

    /// Return this entries name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Add (and override) a key-value map and return the modified map
    pub fn add<K, V>(mut self, k: K, v: V) -> Self
    where
        K: Into<String>,
        V: Into<Vec<u8>>,
    {
        self.map.insert(k.into(), v.into());
        self
    }
}

impl Deref for MetadataMap {
    type Target = BTreeMap<String, Vec<u8>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for MetadataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

/// Generate the service metedata entry path
fn gen_path(serv: &String, name: &String) -> Path {
    Path::from(format!("/service/{}:{}", serv, name))
}

fn tag_service(serv: &String) -> Tag {
    Tag::new("libqaul._int.service", serv.as_bytes().to_vec())
}

const TAG_METADATA: &'static str = "libqaul._int.metadata";

/// Internal metadata store wrapper for Alexandria
#[derive(Clone)]
pub(crate) struct MetadataStore {
    inner: Arc<Library>,
}

impl MetadataStore {
    pub(crate) fn new(inner: Arc<Library>) -> Self {
        Self { inner }
    }

    pub(crate) async fn save(
        &self,
        user: Identity,
        serv: String,
        data: MetadataMap,
        mut tags: TagSet,
    ) -> Result<()> {
        let k = data.name().clone();
        let sess = Session::Id(user);

        // Generate diffs based on previous value
        let diffs = match self
            .inner
            .query(sess, Query::path(gen_path(&serv, &k)))
            .await
        {
            Ok(QueryResult::Single(rec)) => data.gen_diffset(&rec.into()),
            Err(_) => data.init_diff(),
            _ => unreachable!(),
        };

        // Add libqaul internal search tags
        tags.insert(tag_service(&serv));
        tags.insert(Tag::empty(TAG_METADATA));

        // Try to insert, otherwise update
        if let Err(_) = self
            .inner
            .batch(Session::Id(user), gen_path(&serv, &k), tags, diffs.clone())
            .await
        {
            for diff in diffs {
                self.inner
                    .update(Session::Id(user), gen_path(&serv, &k), diff)
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) async fn delete(&self, user: Identity, service: String, key: String) {
        if let Err(e) = self
            .inner
            .delete(Session::Id(user), gen_path(&service, &key))
            .await
        {
            warn!("An error occured while deleting metadata: {}", e);
        }
    }

    pub(crate) async fn query(
        &self,
        user: Identity,
        serv: String,
        mut tags: TagSet,
    ) -> Vec<MetadataMap> {
        let sess = Session::Id(user);

        // Add libqaul internal search tags
        tags.insert(tag_service(&serv));
        tags.insert(Tag::empty(TAG_METADATA));

        match self.inner.query(sess, Query::tags().subset(tags)).await {
            Ok(QueryResult::Single(rec)) => vec![rec.into()],
            Ok(QueryResult::Many(vec)) => vec.into_iter().map(|rec| rec.into()).collect(),
            Err(_) => vec![],
        }
    }
}
