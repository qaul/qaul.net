use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};

/// The key-value map used in a Kv record
pub type Map = BTreeMap<String, Value>;

/// A key-value record reference
pub type KvRef = Arc<Kv>;

/// A strongly typed alexandria data value
///
/// Because alexandria is written in Rust, all data is strongly typed,
/// and can be checked for being a certain type via dynamic dispatch
/// (unfortunately monomorphic generics kinda end at the disk sync
/// level).
///
/// Value types can be nested with `Value::Map`, which contains a
/// `BTreeMap<_, _>`.  Nestings can be arbitrarily deep.
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Kv {
    map: Map,
}

impl Kv {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl Deref for Kv {
    type Target = Map;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
