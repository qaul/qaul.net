use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::{Error, Result},
    meta::users::User,
    utils::{Id, Path, Tag, TagSet},
};

use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Per-user encrypted tag storage
#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct UserTags {
    t2p: BTreeMap<Tag, BTreeSet<Path>>,
    p2t: BTreeMap<Path, TagSet>,
}

impl UserTags {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /// Insert a new tag-path relationship into the store
    ///
    /// If the tag already exists, the path will be appended.  If it
    /// doesn't a new dataset will be created.
    pub(crate) fn insert(&mut self, tag: Tag, path: &Path) {
        self.t2p
            .entry(tag.clone())
            .or_default()
            .insert(path.clone());
        self.p2t.entry(path.clone()).or_default().insert(tag);
    }

    /// Remove a tag-path relationship from the store
    ///
    /// If it the last association for a tag, it will be removed
    /// entirely from the table, meaning that the tag is no longer
    /// present anywhere in the database.
    pub(crate) fn delete(&mut self, tag: &Tag, path: &Path) {
        self.t2p.get_mut(tag).unwrap().remove(path);
        self.p2t.get_mut(path).unwrap().remove(tag);

        if self.t2p.get(tag).unwrap().len() == 0 {
            self.t2p.remove(tag);
        }

        if self.p2t.get(path).unwrap().len() == 0 {
            self.p2t.remove(path);
        }
    }

    /// Remove a path from all tag models
    pub(crate) fn clear(&mut self, path: &Path) {
        self.t2p.iter_mut().for_each(|(_, set)| {
            set.remove(path);
        });
    }

    /// Return a set of unique paths associated to a tag
    fn single_tag(&self, tag: &Tag) -> Vec<Path> {
        self.t2p
            .get(tag)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or(vec![])
    }

    /// Only return paths where tags are an exact match
    pub(crate) fn paths(&self, tags: &TagSet) -> Vec<Path> {
        tags.iter()
            .map(|t| self.single_tag(t))
            .fold(vec![], |mut vec, paths| {
                paths.into_iter().for_each(|p| vec.push(p));
                vec
            })
    }

    /// Only return paths where tags are an exact match
    pub(crate) fn paths_matching(&self, tags: &TagSet) -> Vec<Path> {
        tags.iter()
            .map(|t| self.single_tag(t))
            .fold(vec![], |mut vec, paths| {
                paths.into_iter().for_each(|p| {
                    let other = self.p2t.get(&p).unwrap();
                    if tags.exactly(&other) {
                        vec.push(p);
                    }
                });

                vec
            })
    }
}

impl DetachedKey<KeyPair> for UserTags {}

/// A per-user tag cache.
pub(crate) struct TagCache {
    map: EncryptedMap<Id, UserTags, KeyPair>,
    id: Id,
}

impl TagCache {
    pub(crate) fn new() -> Self {
        let id = Id::truncate(&(0..).take(64).collect::<Vec<u8>>());

        Self {
            map: EncryptedMap::new(),
            id,
        }
    }

    /// Insert a new path-tag mapping for a user
    pub(crate) fn insert<I: Into<Option<Id>>>(
        &mut self,
        id: I,
        path: Path,
        tag: Tag,
    ) -> Result<()> {
        let id = id.into().unwrap_or(self.id);

        self.map
            .entry(id)
            .or_insert(Encrypted::new(UserTags::new()))
            .deref_mut()?
            .insert(tag, &path);

        Ok(())
    }

    /// Delete a path from all tag mappings
    pub(crate) fn delete_path<I: Into<Option<Id>>>(&mut self, id: I, path: Path) -> Result<()> {
        let id = id.into().unwrap_or(self.id);
        self.map.get_mut(id)?.clear(&path);
        Ok(())
    }

    /// Get all paths associated with a tag
    pub(crate) fn get_paths<I: Into<Option<Id>>>(&self, id: I, tags: &TagSet) -> Result<Vec<Path>> {
        let id = id.into().unwrap_or(self.id);
        Ok(self.map.get(id)?.paths(tags))
    }

    /// Get all paths associated with a tag
    pub(crate) fn get_paths_matching<I: Into<Option<Id>>>(
        &self,
        id: I,
        tags: &TagSet,
    ) -> Result<Vec<Path>> {
        let id = id.into().unwrap_or(self.id);
        Ok(self.map.get(id)?.paths_matching(tags))
    }

    pub(crate) fn open(&mut self, user: &User) -> Result<()> {
        self.map.open(user.id, &*user.key)
    }

    /// Re-seal the user metadata structure in place
    pub(crate) fn close(&mut self, user: &User) -> Result<()> {
        self.map.close(user.id, Arc::clone(&user.key))
    }
}
