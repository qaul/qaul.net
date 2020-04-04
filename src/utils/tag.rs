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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TagSet(BTreeSet<Tag>);

impl TagSet {
    pub fn empty() -> Self {
        Self(Default::default())
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
}
