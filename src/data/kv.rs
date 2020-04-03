use crate::{
    diff::{Diff, DiffExt, DiffResult, DiffSeg},
    Error, Result,
};
use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};

/// The key-value map used in a Kv record
pub type Map = BTreeMap<String, Value>;

/// A strongly typed alexandria data value
///
/// Because alexandria is written in Rust, all data is strongly typed,
/// and can be checked for being a certain type via dynamic dispatch
/// (unfortunately monomorphic generics kinda end at the disk sync
/// level).
///
/// Value types can be nested with `Value::Map`, which contains a
/// `BTreeMap<_, _>`.  Nestings can be arbitrarily deep.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    /// Some UTF-8 valid text, with no length limit
    String(String),

    /// Simple boolean values (`true` or `false`)
    Bool(bool),

    /// A nested tree object
    Child(Map),

    /// Signed 128bit integers
    I128(i128),
    /// Unsigned 128bit integers
    U128(u128),

    /// Signed 64bit integers
    I64(i64),
    /// Unsigned 64bit integers
    U64(u64),

    // Medium numbers
    /// Signed 32bit integers
    I32(i32),
    /// Unsigned 32bit integers
    U32(u32),

    /// Signed 16bit integers
    I16(i16),
    /// Unsigned 16bit integers
    U16(u16),

    /// Signed 8bit integers
    I8(i8),
    /// Unsigned 8bit integers
    U8(u8),

    /// Double precision floating point numbers
    F64(f64),
    /// Single precision floating point numbers
    F32(f32),

    /// Arbitrary embedded binary data
    Vec(Vec<u8>),
}

/// A key-value store record
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Kv {
    map: Map,
}

impl DiffExt for Kv {

    /// Apply a key-value diff to this Kv map instance
    fn apply(&mut self, diff: Diff) -> Result<()> {
        match diff {
            Diff::Map(map) => map
                .into_iter()
                .fold(Ok(()), |res: DiffResult<()>, (k, v)| {
                    match (
                        res,
                        match v {
                            DiffSeg::Insert(val) => self.insert(k, val),
                            DiffSeg::Update(val) => self.update(k, val),
                            DiffSeg::Delete => self.delete(&k),
                        },
                    ) {
                        (Ok(_), Ok(_)) => Ok(()),
                        (Ok(_), e @ Err(_)) => e,
                        (Err(o), Err(n)) => Err(o.add(n)),
                        (e @ Err(_), Ok(_)) => e,
                    }
                })
                .map_err(|e| e.into()),
            Diff::Binary(_) => Err(Error::BadDiffType),
        }
    }
}

impl Kv {
    pub fn new() -> Self {
        Self::default()
    }

    /// A helper function to insert and error if the key existed
    fn insert(&mut self, key: String, val: Value) -> DiffResult<()> {
        if self.map.contains_key(&key) {
            Err((0, format!("key `{}` already exists!", key)).into())
        } else {
            self.map.insert(key, val);
            Ok(())
        }
    }

    /// A helper function to insert and error if the key didn't exists
    fn update(&mut self, key: String, val: Value) -> DiffResult<()> {
        if self.map.contains_key(&key) {
            self.map.insert(key, val);
            Ok(())
        } else {
            Err((0, format!("key `{}` doesn't exist!", key)).into())
        }
    }

    /// A helper function to insert and error if the key didn't exists
    fn delete(&mut self, key: &String) -> DiffResult<()> {
        if self.map.contains_key(key) {
            self.map.remove(key);
            Ok(())
        } else {
            Err((0, format!("key `{}` doesn't exist!", key)).into())
        }
    }
}

impl Deref for Kv {
    type Target = Map;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

#[test]
fn apply_single_diff() {
    let mut kv = Kv::new();
    let diff = Diff::from((
        "hello".into(),
        DiffSeg::Insert(Value::String("world".into())),
    ));

    kv.apply(diff).unwrap();
    assert_eq!(kv.len(), 1);
}

#[test]
fn apply_multi_diff() {
    let mut kv = Kv::new();
    let diff = Diff::from(
        vec![
            ("hello".into(), DiffSeg::Insert(Value::String("workers".into()))),
            ("of".into(), DiffSeg::Insert(Value::String("the".into()))),
            ("world".into(), DiffSeg::Insert(Value::String("you".into()))),
            ("have".into(), DiffSeg::Insert(Value::String("nothing".into()))),
            ("to".into(), DiffSeg::Insert(Value::String("lose".into()))),
            ("but".into(), DiffSeg::Insert(Value::String("your".into()))),
            ("chains".into(), DiffSeg::Insert(Value::String("!".into()))),
   ]);

    kv.apply(diff).unwrap();
    assert_eq!(kv.len(), 7);
}
