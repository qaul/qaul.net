use crate::{error::DiffErrors, error::Result, record::kv::Value};
use std::collections::BTreeMap;

pub(crate) type DiffResult<T> = std::result::Result<T, DiffErrors>;

pub(crate) trait DiffExt {
    fn apply(&mut self, diff: Diff) -> Result<()>;
}

/// Encode a single change made to a set of data
#[derive(Clone, Debug, PartialEq)]
pub enum DiffSeg {
    /// Creating a new record
    Insert(Value),
    /// Updating an existing record in place
    Update(Value),
    /// Deleting a record
    Delete,
    /// A nested diff segment used for maps and lists
    Nested(String, Box<DiffSeg>),
}

/// An atomic set of changes applied to a record
pub enum Diff {
    /// Maps a key to a value
    Map(BTreeMap<String, DiffSeg>),
    /// Binary data
    Binary(DiffSeg),
}

impl Diff {
    /// Start building a map diff
    pub fn map() -> Self {
        Diff::Map(Default::default())
    }

    /// Add an insert line to the diff
    pub fn insert<S: Into<String>, V: Into<Value>>(mut self, key: S, val: V) -> Self {
        match self {
            Self::Map(ref mut map) => {
                map.insert(key.into(), DiffSeg::Insert(val.into()));
                self
            }
            _ => panic!("Bad builder payload: Binary type, can't have keyed inserts!"),
        }
    }

    /// Add an update line to the diff
    pub fn update<S: Into<String>, V: Into<Value>>(mut self, key: S, val: V) -> Self {
        match self {
            Self::Map(ref mut map) => {
                map.insert(key.into(), DiffSeg::Update(val.into()));
                self
            }
            _ => panic!("Bad builder payload: Binary type, can't have keyed inserts!"),
        }
    }

    /// Add an delete line to the diff
    pub fn delete<S: Into<String>>(mut self, key: S) -> Self {
        match self {
            Self::Map(ref mut map) => {
                map.insert(key.into(), DiffSeg::Delete);
                self
            }
            _ => panic!("Bad builder payload: Binary type, can't have keyed inserts!"),
        }
    }

    /// Add an insert line to the diff
    pub fn nested<S: Into<String>>(mut self, key: S, val: Diff) -> Self {
        match (&mut self, val) {
            (Self::Map(ref mut map), Self::Map(ref other)) => {
                let key = key.into();
                other.into_iter().for_each(|(ikey, ival)| {
                    map.insert(
                        key.clone(),
                        DiffSeg::Nested(ikey.clone(), Box::new(ival.clone())),
                    );
                });
            }
            _ => panic!("Bad builder payload: Binary type, can't have keyed inserts!"),
        }

        self
    }
}

impl From<Vec<u8>> for Diff {
    fn from(vec: Vec<u8>) -> Self {
        Self::Binary(DiffSeg::Insert(Value::Vec(vec)))
    }
}

impl<'a> From<&'a [u8]> for Diff {
    fn from(buf: &'a [u8]) -> Self {
        Self::Binary(DiffSeg::Insert(Value::Vec(buf.iter().cloned().collect())))
    }
}

impl From<BTreeMap<String, DiffSeg>> for Diff {
    fn from(map: BTreeMap<String, DiffSeg>) -> Self {
        Self::Map(map)
    }
}

impl From<(String, DiffSeg)> for Diff {
    fn from(tup: (String, DiffSeg)) -> Self {
        Self::Map({
            let mut map = BTreeMap::new();
            map.insert(tup.0, tup.1);
            map
        })
    }
}

impl From<Vec<(String, DiffSeg)>> for Diff {
    fn from(vec: Vec<(String, DiffSeg)>) -> Self {
        Self::Map(vec.into_iter().collect())
    }
}

impl<'s> From<Vec<(&'s str, DiffSeg)>> for Diff {
    fn from(vec: Vec<(&'s str, DiffSeg)>) -> Self {
        let v: Vec<(String, DiffSeg)> = vec
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        v.into()
    }
}

#[test]
fn usability() {
    let d = Diff::map().insert("my_key", "my_val");
    assert_eq!(
        match d {
            Diff::Map(mut map) => map.remove("my_key").unwrap(),
            _ => panic!(),
        },
        DiffSeg::Insert(Value::String("my_val".into()))
    );
}

#[test]
fn usability_nested() {
    let d = Diff::map().nested("my_key", Diff::map().insert("my_sub_key", "my_actual_data"));

    assert_eq!(
        match d {
            Diff::Map(mut map) => match map.remove("my_key").unwrap() {
                DiffSeg::Nested(key, boxed) => (key, *boxed),
                _ => panic!(),
            },
            _ => panic!(),
        },
        (
            "my_sub_key".into(),
            DiffSeg::Insert(Value::String("my_actual_data".into()))
        )
    );
}
