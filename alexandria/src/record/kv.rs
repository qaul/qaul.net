use crate::{
    error::{Error, Result},
    utils::{Diff, DiffExt, DiffResult, DiffSeg},
};
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// Some UTF-8 valid text, with no length limit
    String(String),

    /// Simple boolean values (`true` or `false`)
    Bool(bool),

    /// A nested tree node object
    Map(Map),

    /// A nested list node object
    List(Vec<Value>),

    /// Signed 128bit integers
    I128(i128),
    /// Unsigned 128bit integers
    U128(u128),

    /// Signed 64bit integers
    I64(i64),
    /// Unsigned 64bit integers
    U64(u64),

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

// Strings

impl From<String> for Value {
    fn from(o: String) -> Self {
        Self::String(o)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(o: &'a str) -> Self {
        Self::String(String::from(o))
    }
}

// Boolean logic

impl From<bool> for Value {
    fn from(o: bool) -> Self {
        Self::Bool(o)
    }
}

// Nested types (map/list)

impl From<Map> for Value {
    fn from(o: Map) -> Self {
        Self::Map(o)
    }
}

impl From<Vec<Value>> for Value {
    fn from(o: Vec<Value>) -> Self {
        Self::List(o)
    }
}

// 128 bit numbers

impl From<i128> for Value {
    fn from(o: i128) -> Self {
        Self::I128(o)
    }
}

impl From<u128> for Value {
    fn from(o: u128) -> Self {
        Self::U128(o)
    }
}

// 64 bit numbers

impl From<i64> for Value {
    fn from(o: i64) -> Self {
        Self::I64(o)
    }
}
impl From<u64> for Value {
    fn from(o: u64) -> Self {
        Self::U64(o)
    }
}

// 32 bit numbers

impl From<i32> for Value {
    fn from(o: i32) -> Self {
        Self::I32(o)
    }
}
impl From<u32> for Value {
    fn from(o: u32) -> Self {
        Self::U32(o)
    }
}

// 16 bit numbers

impl From<i16> for Value {
    fn from(o: i16) -> Self {
        Self::I16(o)
    }
}
impl From<u16> for Value {
    fn from(o: u16) -> Self {
        Self::U16(o)
    }
}

// 8 bit numbers

impl From<i8> for Value {
    fn from(o: i8) -> Self {
        Self::I8(o)
    }
}
impl From<u8> for Value {
    fn from(o: u8) -> Self {
        Self::U8(o)
    }
}

// Non-integer numbers

impl From<f64> for Value {
    fn from(o: f64) -> Self {
        Self::F64(o)
    }
}
impl From<f32> for Value {
    fn from(o: f32) -> Self {
        Self::F32(o)
    }
}

// Binary payloads

impl From<Vec<u8>> for Value {
    fn from(o: Vec<u8>) -> Self {
        Self::Vec(o)
    }
}



/// A key-value store record
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Kv {
    map: Map,
}

trait ApplyExt {
    /// A helper function to insert and error if the key existed
    fn insert(&mut self, key: String, val: Value) -> DiffResult<()>;

    /// A helper function to insert and error if the key didn't exists
    fn update(&mut self, key: String, val: Value) -> DiffResult<()>;

    /// A helper function to insert and error if the key didn't exists
    fn delete(&mut self, key: &String) -> DiffResult<()>;

    /// A helper function to handle nested diffs for Maps and Lists
    fn nested(&mut self, k1: String, k2: String, diff: DiffSeg) -> DiffResult<()>;
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
                            DiffSeg::Nested(k2, boxed) => self.nested(k, k2, *boxed),
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

impl ApplyExt for Map {
    fn insert(&mut self, key: String, val: Value) -> DiffResult<()> {
        if self.contains_key(&key) {
            Err((0, format!("key `{}` already exists!", key)).into())
        } else {
            self.insert(key, val);
            Ok(())
        }
    }

    fn update(&mut self, key: String, val: Value) -> DiffResult<()> {
        if self.contains_key(&key) {
            self.insert(key, val);
            Ok(())
        } else {
            Err((0, format!("key `{}` doesn't exist!", key)).into())
        }
    }

    fn delete(&mut self, key: &String) -> DiffResult<()> {
        if self.contains_key(key) {
            self.remove(key);
            Ok(())
        } else {
            Err((0, format!("key `{}` doesn't exist!", key)).into())
        }
    }

    fn nested(&mut self, key: String, k2: String, diff: DiffSeg) -> DiffResult<()> {
        match self.get_mut(&key) {
            Some(Value::Map(ref mut map)) => match diff {
                DiffSeg::Insert(val) => ApplyExt::insert(map, k2, val),
                DiffSeg::Update(val) => ApplyExt::update(map, k2, val),
                DiffSeg::Delete => ApplyExt::delete(map, &k2),
                DiffSeg::Nested(k3, boxed) => ApplyExt::nested(map, k2, k3, *boxed),
            }
            .map_err(|e| e.replace_text("key", "nested key")),
            Some(Value::List(_)) => unimplemented!(),
            Some(_) => Err((0, format!("key `{}` is not of format Map or List", key)).into()),
            None => Err((0, format!("key `{}` doesn't exist!", key)).into()),
        }
    }
}

impl Kv {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, key: String, val: Value) -> DiffResult<()> {
        ApplyExt::insert(&mut self.map, key, val)
    }

    fn update(&mut self, key: String, val: Value) -> DiffResult<()> {
        ApplyExt::update(&mut self.map, key, val)
    }

    fn delete(&mut self, key: &String) -> DiffResult<()> {
        ApplyExt::delete(&mut self.map, key)
    }

    fn nested(&mut self, k1: String, k2: String, diff: DiffSeg) -> DiffResult<()> {
        ApplyExt::nested(&mut self.map, k1, k2, diff)
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
    let diff: Diff = Diff::from(vec![
        ("hello", DiffSeg::Insert(Value::String("workers".into()))),
        ("of", DiffSeg::Insert(Value::String("the".into()))),
        ("world", DiffSeg::Insert(Value::String("you".into()))),
        ("have", DiffSeg::Insert(Value::String("nothing".into()))),
        ("to", DiffSeg::Insert(Value::String("lose".into()))),
        ("but", DiffSeg::Insert(Value::String("your".into()))),
        ("chains", DiffSeg::Insert(Value::String("!".into()))),
    ]);

    kv.apply(diff).unwrap();
    assert_eq!(kv.len(), 7);
}

#[test]
fn apply_nested_insert() {
    let mut kv = Kv::new();

    let d1 = Diff::from(vec![("map", DiffSeg::Insert(Value::Map(BTreeMap::new())))]);
    kv.apply(d1).unwrap();

    let d2 = Diff::from(vec![(
        "map",
        DiffSeg::Nested(
            "map_key".to_owned(),
            Box::new(DiffSeg::Insert(Value::String("map_val".into()))),
        ),
    )]);
    kv.apply(d2).unwrap();

    assert_eq!(
        match kv.map.get("map").unwrap() {
            Value::Map(map) => match map.get("map_key").unwrap() {
                Value::String(s) => s.as_str(),
                _ => panic!("Nested type is not Value::String!"),
            },
            _ => panic!("Type is not Value::Map!"),
        },
        "map_val"
    );
}

#[test]
fn apply_nested_nested_insert() {
    let mut kv = Kv::new();

    let d1 = Diff::from(vec![("map", DiffSeg::Insert(Value::Map(BTreeMap::new())))]);
    kv.apply(d1).unwrap();

    let d2 = Diff::from(vec![(
        "map",
        DiffSeg::Nested(
            "map_key".to_owned(),
            Box::new(DiffSeg::Insert(Value::Map(BTreeMap::new()))),
        ),
    )]);
    kv.apply(d2).unwrap();

    let d3 = Diff::from(vec![(
        "map",
        DiffSeg::Nested(
            "map_key".to_owned(),
            Box::new(DiffSeg::Nested(
                "nested_map_key".to_owned(),
                Box::new(DiffSeg::Insert(Value::String("nested_map_val".into()))),
            )),
        ),
    )]);
    kv.apply(d3).unwrap();

    assert_eq!(
        match kv.map.get("map").unwrap() {
            Value::Map(map) => match map.get("map_key").unwrap() {
                Value::Map(nested_map) => match nested_map.get("nested_map_key").unwrap() {
                    Value::String(s) => s.as_str(),
                    _ => panic!("Nested nested type is not Value::String!"),
                },
                _ => panic!("Nested type is not Value::Map!"),
            },
            _ => panic!("Type is not Value::Map!"),
        },
        "nested_map_val"
    );
}
