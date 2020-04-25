use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::Result,
    utils::{Id, Path, Tag, TagSet},
    Session,
};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tracing::debug;

/// Per-user encrypted tag storage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    /// Remove a path from all tag models
    pub(crate) fn clear(&mut self, path: &Path) {
        self.p2t.remove(path);
        self.t2p.iter_mut().for_each(|(_, set)| {
            set.remove(path);
        });
    }

    /// Paths with an associated tagset that passed a check
    pub(crate) fn paths<F>(&self, cond: F) -> Vec<Path>
    where
        F: Fn(&TagSet) -> bool,
    {
        self.p2t
            .iter()
            .fold(BTreeSet::new(), |mut set, (path, tagset)| {
                if cond(tagset) {
                    set.insert(path);
                }

                set
            })
            .into_iter()
            .cloned()
            .collect()
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
    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) fn insert(&mut self, id: Session, path: Path, tag: Tag) -> Result<()> {
        let id = id.id().unwrap_or(self.id);
        debug!("Inserting tag for session ID: `{}`", id);

        self.map
            .entry(id)
            .or_insert({
                debug!("Creating new tag-cache page");
                Encrypted::new(UserTags::new())
            })
            .deref_mut()?
            .insert(tag, &path);

        Ok(())
    }

    /// Delete a path from all tag mappings
    #[tracing::instrument(skip(self), level = "debug")]
    pub(crate) fn delete_path(&mut self, id: Session, path: Path) -> Result<()> {
        let id = id.id().unwrap_or(self.id);
        debug!("Deleting path `{}` for session ID: `{}`", path, id);

        if let Ok(entry) = self.map.get_mut(id) {
            entry.clear(&path);
        }
        Ok(())
    }

    /// Get all paths associated with a tag
    #[tracing::instrument(skip(self, cond), level = "debug")]
    pub(crate) fn get_paths<'tags, F>(&self, id: Session, cond: F) -> Vec<Path>
    where
        F: Fn(&TagSet) -> bool,
    {
        let id = id.id().unwrap_or(self.id);
        debug!("Querying pathsfor session ID: `{}`", id);

        match self.map.get(id) {
            Ok(map) => map.paths(cond),
            Err(_) => {
                debug!("Empty cache page: assuming empty query!");
                vec![]
            }
        }
    }
}

// #[test]
// fn data_simple() {
//     let path = Path::from("/msg:bob");
//     let tag = Tag::new("msg-id", vec![1, 3, 1, 2]);
//     let mut ut = UserTags::new();
//     ut.insert(tag.clone(), &path);

//     assert_eq!(ut.paths(&TagSet::from(vec![tag])), vec![path]);
// }

// #[test]
// fn data_non_matching() {
//     let path = Path::from("/msg:bob");
//     let t1 = Tag::new("msg-id", vec![1, 3, 1, 2]);
//     let t2 = Tag::new("room-id", vec![1, 3, 3, 7]);

//     let mut ut = UserTags::new();
//     ut.insert(t1.clone(), &path);
//     ut.insert(t2.clone(), &path);

//     assert_eq!(ut.paths(&TagSet::from(vec![t1])), vec![path]);
// }

// #[test]
// fn data_two_non_matching() {
//     let alice = Path::from("/msg:alice");
//     let bob = Path::from("/msg:bob");
//     let ta = Tag::new("msg-id", vec![0, 0, 0, 0]);
//     let tb = Tag::new("msg-id", vec![1, 1, 1, 1]);
//     let ts = Tag::new("shared", vec![1, 3, 1, 2]);

//     let mut ut = UserTags::new();
//     ut.insert(ta.clone(), &alice);
//     ut.insert(ts.clone(), &alice);

//     ut.insert(tb.clone(), &bob);
//     ut.insert(ts.clone(), &bob);

//     assert_eq!(ut.paths(&TagSet::from(vec![ts])), vec![alice, bob]);
// }

// #[test]
// fn data_two_matching() {
//     let alice = Path::from("/msg:alice");
//     let bob = Path::from("/msg:bob");

//     let ta = Tag::new("msg-id", vec![0, 0, 0, 0]);
//     let tb = Tag::new("msg-id", vec![1, 1, 1, 1]);
//     let ts = Tag::new("shared", vec![1, 3, 1, 2]);

//     let mut ut = UserTags::new();
//     ut.insert(ta.clone(), &alice);
//     ut.insert(ts.clone(), &alice);

//     ut.insert(tb.clone(), &bob);
//     ut.insert(ts.clone(), &bob);

//     assert_eq!(ut.paths_matching(&TagSet::from(vec![ta, ts])), vec![alice]);
// }

// #[test]
// fn data_no_match() {
//     let alice = Path::from("/msg:alice");
//     let bob = Path::from("/msg:bob");

//     let ta = Tag::new("msg-id", vec![1, 3, 1, 2]);
//     let tb = Tag::new("msg-id", vec![1, 3, 3, 7]);
//     let none = Tag::new("none", vec![0, 0, 0, 0]);

//     let mut ut = UserTags::new();
//     ut.insert(ta.clone(), &alice);
//     ut.insert(tb.clone(), &bob);

//     assert_eq!(ut.paths_matching(&TagSet::from(vec![none])), vec![]);
// }
