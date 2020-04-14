use crate::utils::{Path, Tag};
use std::collections::{BTreeMap, BTreeSet};

/// Per-user encrypted tag storage
#[derive(Default)]
pub(crate) struct TagTable {
    map: BTreeMap<Tag, BTreeSet<Path>>,
}

impl TagTable {
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

    /// Return a set of unique paths associated to a tag
    pub(crate) fn paths(&self, tag: &Tag) -> Vec<Path> {
        self.map
            .get(tag)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or(vec![])
    }
}
