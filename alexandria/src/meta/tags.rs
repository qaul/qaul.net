use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::{Error, Result},
    meta::users::User,
    utils::{Id, Path, Tag},
};

use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Per-user encrypted tag storage
#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct UserTags {
    map: BTreeMap<Tag, BTreeSet<Path>>,
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
        self.map.entry(tag).or_default().insert(path.clone());
    }

    /// Remove a tag-path relationship from the store
    ///
    /// If it the last association for a tag, it will be removed
    /// entirely from the table, meaning that the tag is no longer
    /// present anywhere in the database.
    pub(crate) fn delete(&mut self, tag: &Tag, path: &Path) {
        self.map.get_mut(tag).unwrap().remove(path);
        if self.map.get(tag).unwrap().len() == 0 {
            self.map.remove(tag);
        }
    }

    /// Remove a path from all tag models
    pub(crate) fn clear(&mut self, path: &Path) {
        self.map.iter_mut().for_each(|(_, set)| {
            set.remove(path);
        });
    }

    /// Return a set of unique paths associated to a tag
    pub(crate) fn paths(&self, tag: &Tag) -> Vec<Path> {
        self.map
            .get(tag)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or(vec![])
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
    pub(crate) fn get_paths<I: Into<Option<Id>>>(&self, id: I, tag: &Tag) -> Result<Vec<Path>> {
        let id = id.into().unwrap_or(self.id);
        Ok(self.map.get(id)?.paths(tag))
    }

    pub(crate) fn open(&mut self, user: &User) -> Result<()> {
        self.map.open(user.id, &*user.key)
    }

    /// Re-seal the user metadata structure in place
    pub(crate) fn close(&mut self, user: &User) -> Result<()> {
        self.map.close(user.id, Arc::clone(&user.key))
    }
}
