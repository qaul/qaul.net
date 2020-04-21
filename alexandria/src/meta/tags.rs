use crate::{
    crypto::{asym::KeyPair, DetachedKey, Encrypted, EncryptedMap},
    error::Result,
    utils::{Id, Path, Tag, TagSet},
};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
            .fold(BTreeSet::new(), |mut set, paths| {
                paths.into_iter().for_each(|p| {
                    set.insert(p);
                });
                set
            })
            .into_iter()
            .collect()
    }

    /// Only return paths where tags are an exact match
    pub(crate) fn paths_matching(&self, tags: &TagSet) -> Vec<Path> {
        tags.iter()
            .map(|t| self.single_tag(t))
            .fold(BTreeSet::new(), |mut set, paths| {
                paths.into_iter().for_each(|p| {
                    let other = self.p2t.get(&p).unwrap();
                    if tags.exactly(&other) {
                        set.insert(p);
                    }
                });

                set
            })
            .into_iter()
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
        if let Ok(entry) = self.map.get_mut(id) {
            entry.clear(&path);
        }
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
}

#[test]
fn data_simple() {
    let path = Path::from("/msg:bob");
    let tag = Tag::new("msg-id", vec![1, 3, 1, 2]);
    let mut ut = UserTags::new();
    ut.insert(tag.clone(), &path);

    assert_eq!(ut.paths(&TagSet::from(vec![tag])), vec![path]);
}

#[test]
fn data_non_matching() {
    let path = Path::from("/msg:bob");
    let t1 = Tag::new("msg-id", vec![1, 3, 1, 2]);
    let t2 = Tag::new("room-id", vec![1, 3, 3, 7]);

    let mut ut = UserTags::new();
    ut.insert(t1.clone(), &path);
    ut.insert(t2.clone(), &path);

    assert_eq!(ut.paths(&TagSet::from(vec![t1])), vec![path]);
}

#[test]
fn data_two_non_matching() {
    let alice = Path::from("/msg:alice");
    let bob = Path::from("/msg:bob");
    let ta = Tag::new("msg-id", vec![0, 0, 0, 0]);
    let tb = Tag::new("msg-id", vec![1, 1, 1, 1]);
    let ts = Tag::new("shared", vec![1, 3, 1, 2]);

    let mut ut = UserTags::new();
    ut.insert(ta.clone(), &alice);
    ut.insert(ts.clone(), &alice);

    ut.insert(tb.clone(), &bob);
    ut.insert(ts.clone(), &bob);

    assert_eq!(ut.paths(&TagSet::from(vec![ts])), vec![alice, bob]);
}

#[test]
fn data_two_matching() {
    let alice = Path::from("/msg:alice");
    let bob = Path::from("/msg:bob");

    let ta = Tag::new("msg-id", vec![0, 0, 0, 0]);
    let tb = Tag::new("msg-id", vec![1, 1, 1, 1]);
    let ts = Tag::new("shared", vec![1, 3, 1, 2]);

    let mut ut = UserTags::new();
    ut.insert(ta.clone(), &alice);
    ut.insert(ts.clone(), &alice);

    ut.insert(tb.clone(), &bob);
    ut.insert(ts.clone(), &bob);

    assert_eq!(ut.paths_matching(&TagSet::from(vec![ta, ts])), vec![alice]);
}

#[test]
fn data_no_match() {
    let alice = Path::from("/msg:alice");
    let bob = Path::from("/msg:bob");

    let ta = Tag::new("msg-id", vec![1, 3, 1, 2]);
    let tb = Tag::new("msg-id", vec![1, 3, 3, 7]);
    let none = Tag::new("none", vec![0, 0, 0, 0]);

    let mut ut = UserTags::new();
    ut.insert(ta.clone(), &alice);
    ut.insert(tb.clone(), &bob);

    assert_eq!(ut.paths_matching(&TagSet::from(vec![none])), vec![]);
}
