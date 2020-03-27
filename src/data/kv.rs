
use serde::{Serialize, Deserialize};
use std::{collections::BTreeMap, ops::Deref};
use async_std::sync::Arc;

/// The key-value map used in a Kv record
pub type Map = BTreeMap<String, Value>;

/// A key-value record reference
pub type KvRef = Arc<Kv>;

/// A strongly-typed value stored in an `alexandria` scope
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    // Text
    String(String),

    // True and false
    Bool(bool),

    // A tree object
    Child(Map),

    // Big numbers
    I128(i128),
    U128(u128),

    // Less big numbers
    I64(i64),
    U64(u64),

    // Medium numbers
    I32(i32),
    U32(u32),

    // Smallish numbers
    I16(i16),
    U16(u16),

    // Tiny numbers
    I8(i8),
    U8(u8),

    // Floaty numbers
    F64(f64),
    F32(f32),
}

/// A key-value store record
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Kv {
    map: Map,
}

impl Kv {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for Kv {
    type Target = Map;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
