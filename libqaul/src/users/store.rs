//! Store for user profiles

use crate::{
    error::{Error, Result},
    qaul::Identity,
    security::KeyId,
    store::KeyWrap,
    users::{UserProfile, UserUpdate},
};
use alexandria::{
    utils::{Id, Path, Query, QueryResult, SetQuery, Tag, TagSet},
    Library,
};
use async_std::prelude::FutureExt;
use ed25519_dalek::Keypair;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

const KEY_PATH: &'static str = "/meta:keys";

fn profile_path(id: Id) -> Path {
    Path::from(format!("/users:{}", id))
}

/// A type wrapper around the alexandria storage library
#[derive(Clone)]
pub(crate) struct UserStore {
    inner: Arc<Library>,
}

impl UserStore {
    /// Create a new type abstraction over an existing Alexandria lib
    pub(crate) fn new(inner: Arc<Library>) -> Self {
        Self { inner }
    }

    /// Create a new local user
    pub(crate) async fn create_local(&self, keyid: KeyId, pw: &str) {
        let KeyId { id, keypair } = keyid;
        self.inner.user(id).create(pw).await.unwrap();
        let wrapped = KeyWrap(keypair);

        // Store the key
        self.inner
            .data(id)
            .await
            .unwrap()
            .insert(Path::from(KEY_PATH), TagSet::empty(), wrapped.make_diff())
            .await
            .unwrap();

        self.insert_profile(id, vec![Tag::empty("profile"), Tag::empty("local")])
            .await;
    }

    /// Add an empty user profile for an id
    pub(crate) async fn insert_profile<T: Into<TagSet>>(&self, id: Identity, tags: T) {
        let profile = UserProfile::new(id);
        self.inner
            .data(None)
            .await
            .unwrap()
            .batch(profile_path(id), tags, profile.init_diff())
            .await
            .unwrap();
    }

    /// Delete the key and profile for a local user
    pub(crate) async fn delete_local(&self, id: Identity) {
        let (r1, r2) = self
            .inner
            .data(id)
            .await
            .unwrap()
            .delete(Path::from(KEY_PATH))
            .join(
                self.inner
                    .data(None)
                    .await
                    .unwrap()
                    .delete(profile_path(id)),
            )
            .await;
        r1.and(r2).unwrap();
    }

    /// Modify a single user inside the store in-place
    pub(crate) async fn modify(&self, id: Identity, modifier: UserUpdate) -> Result<()> {
        let mut curr = self.get(id).await?;
        let diff = curr.gen_diff(modifier);
        self.inner
            .data(None)
            .await
            .unwrap()
            .update(profile_path(id), diff)
            .await
            .unwrap();

        Ok(())
    }

    /// Don't call this on non-local users please
    pub(crate) async fn get_key(&self, id: Identity) -> Keypair {
        match self
            .inner
            .data(id)
            .await
            .unwrap()
            .query(Query::Path(Path::from(KEY_PATH)))
            .await
        {
            Ok(QueryResult::Single(rec)) => KeyWrap::from(&*rec).0,
            _ => panic!("Key not properly stored in the database"),
        }
    }

    pub(crate) async fn get(&self, id: Identity) -> Result<UserProfile> {
        match self
            .inner
            .data(None)
            .await
            .unwrap()
            .query(Query::Path(profile_path(id)))
            .await
        {
            Ok(QueryResult::Single(rec)) => Ok(UserProfile::from(&*rec)),
            _ => panic!(),
        }
    }

    /// Get all locally available users
    pub(crate) async fn all_local(&self) -> Vec<UserProfile> {
        match self
            .inner
            .data(None)
            .await
            .unwrap()
            .query(Query::Tag(SetQuery::Partial(
                vec![Tag::empty("profile"), Tag::empty("local")].into(),
            )))
            .await
            .unwrap()
        {
            QueryResult::Many(vec) => dbg!(vec)
                .into_iter()
                .map(|rec| UserProfile::from(&*rec))
                .collect(),
            _ => unreachable!(),
        }
    }

    /// Get all remote users this device knows about
    #[allow(unused)]
    pub(crate) async fn all_remote(&self) -> Vec<UserProfile> {
        match self
            .inner
            .data(None)
            .await
            .unwrap()
            .query(Query::Tag(SetQuery::Matching(
                vec![Tag::empty("profile")].into(),
            )))
            .await
            .unwrap()
        {
            QueryResult::Many(vec) => vec
                .into_iter()
                .map(|rec| UserProfile::from(&*rec))
                .collect(),
            _ => unreachable!(),
        }
    }

    /// Get *all* users this device knows about
    #[allow(unused)]
    pub(crate) async fn all(&self) -> Vec<UserProfile> {
        match self
            .inner
            .data(None)
            .await
            .unwrap()
            .query(Query::Tag(SetQuery::Partial(
                vec![Tag::empty("profile")].into(),
            )))
            .await
            .unwrap()
        {
            QueryResult::Many(vec) => vec
                .into_iter()
                .map(|rec| UserProfile::from(&*rec))
                .collect(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod harness {
    use crate::{users::UserStore, Identity};
    use alexandria::utils::Tag;
    use async_std::sync::Arc;

    pub(super) fn setup() -> UserStore {
        use alexandria::Builder;
        let dir = tempfile::tempdir().unwrap();
        let lib = Builder::new()
            .offset(dir.path())
            .build()
            .map(|l| Arc::new(l))
            .unwrap();

        UserStore::new(lib)
    }

    /// Insert a random user into the store and return the Id
    pub(super) fn insert_random(store: &UserStore) -> Identity {
        async_std::task::block_on(async {
            use crate::security::Sec;
            let keyid = Sec::new().generate().await;
            let id = keyid.id;
            store.create_local(keyid, "car horse battery staple").await;
            id
        })
    }

    /// Insert a random user into the store and return the Id
    pub(super) fn insert_random_remote(store: &UserStore) -> Identity {
        async_std::task::block_on(async {
            let id = Identity::random();
            store.insert_profile(id, vec![Tag::empty("profile")]).await;
            id
        })
    }
}

#[test]
fn create_user() {
    let store = harness::setup();
    harness::insert_random(&store);
}

#[async_std::test]
async fn create_and_get_key() {
    let store = harness::setup();
    let id = harness::insert_random(&store);
    store.get_key(id).await;
}

#[async_std::test]
async fn create_and_get_user() {
    let store = harness::setup();
    let id = harness::insert_random(&store);
    let user = store.get(id).await.unwrap();
    assert_eq!(user.id, id);
}

#[async_std::test]
async fn update_user() {
    let store = harness::setup();
    let id = harness::insert_random(&store);

    let update = UserUpdate::DisplayName(Some("spacekookie".into()));
    store.modify(id, update).await.unwrap();

    let after = store.get(id).await.unwrap();
    assert_eq!(after.display_name, Some("spacekookie".into()));
}

#[async_std::test]
async fn delete_user() {
    let store = harness::setup();
    let id = harness::insert_random(&store);

    store.delete_local(id).await;
}

#[async_std::test]
async fn create_local_and_query() {
    let store = harness::setup();
    let id = harness::insert_random(&store);
    assert_eq!(dbg!(store.all_local().await).len(), 1);
}
