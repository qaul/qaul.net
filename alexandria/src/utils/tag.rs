use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A set of tags where every tag is unique
///
/// Simply construct a set via one of the `From` implementations of a
/// containing type.
///
/// ```norun
/// # use alexandria::data::TagSet;
/// # use std::collections::BTreeSet;
/// let _: TagSet = vec![].into();
/// let _: TagSet = BTreeSet::default().into();
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TagSet(BTreeSet<Tag>);

impl TagSet {
    pub fn empty() -> Self {
        Self(Default::default())
    }

    pub fn insert(&mut self, t: Tag) {
        self.0.insert(t);
    }

    pub fn remove(&mut self, t: &Tag) {
        self.0.remove(t);
    }

    pub fn contains(&self, t: &Tag) -> bool {
        self.0.contains(t)
    }

    #[cfg(test)]
    #[allow(unused)]
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn exactly(&self, o: &TagSet) -> bool {
        self.0 == o.0
    }

    /// Every tag from o needs to be in self
    pub(crate) fn subset(&self, o: &TagSet) -> bool {
        o.iter().fold(true, |acc, t| acc && self.0.contains(t))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Tag> {
        self.0.iter()
    }
}

impl From<Vec<Tag>> for TagSet {
    fn from(vec: Vec<Tag>) -> Self {
        Self(vec.into_iter().fold(BTreeSet::new(), |mut set, tag| {
            set.insert(tag);
            set
        }))
    }
}

impl From<&Vec<Tag>> for TagSet {
    fn from(vec: &Vec<Tag>) -> Self {
        Self(vec.iter().fold(BTreeSet::new(), |mut set, tag| {
            set.insert(tag.clone());
            set
        }))
    }
}

impl From<Vec<&Tag>> for TagSet {
    fn from(vec: Vec<&Tag>) -> Self {
        Self(vec.into_iter().fold(BTreeSet::new(), |mut set, tag| {
            set.insert(tag.clone());
            set
        }))
    }
}

impl From<BTreeSet<Tag>> for TagSet {
    fn from(set: BTreeSet<Tag>) -> Self {
        Self(set)
    }
}

impl From<TagSet> for BTreeSet<Tag> {
    fn from(ts: TagSet) -> Self {
        ts.0
    }
}

/// A generic metadata tag
///
/// Because searching through message or file payloads might be slow,
/// and I/O intensive (especially within thi secret storage module),
/// all public types have a tag metadata interface.  These are
/// included in the wire-format, meaning that they will get
/// transferred across to another node.
///
/// This can be used to implement things like conversation ID's,
/// In-Reply-To, and more.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Tag {
    /// A string key for a tag
    pub key: String,
    /// Some binary data that is up to a service to interpret
    pub val: Vec<u8>,
}

impl Tag {
    /// Create a new MsgTag with key and value
    pub fn new<K, I>(key: K, val: I) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = u8>,
    {
        Self {
            key: key.into(),
            val: val.into_iter().collect(),
        }
    }

    /// Create a tag that consists of only a key, with no value
    pub fn empty<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self::new(key, vec![])
    }
}

#[test]
fn subset_1() {
    let whole = TagSet::from(vec![Tag::empty("a"), Tag::empty("b")]);
    let sub = TagSet::from(vec![Tag::empty("a")]);
    assert!(whole.subset(&sub));
}

#[test]
fn subset_2() {
    let whole = TagSet::from(vec![Tag::empty("a"), Tag::empty("b")]);
    let sub = TagSet::from(vec![Tag::empty("a")]);
    assert!(!sub.subset(&whole));
}

#[test]
fn subset_3() {
    let whole = TagSet::from(vec![Tag::empty("a"), Tag::empty("b"), Tag::empty("c")]);
    let sub = TagSet::from(vec![Tag::empty("a"), Tag::empty("b")]);
    assert!(whole.subset(&sub));
}

#[test]
fn subset_4() {
    let whole = TagSet::from(vec![Tag::empty("a"), Tag::empty("b"), Tag::empty("c")]);
    let sub = TagSet::from(vec![Tag::empty("a"), Tag::empty("b")]);
    assert!(!sub.subset(&whole));
}
