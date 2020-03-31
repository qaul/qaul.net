use async_std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};

/// The key-value map used in a Kv record
pub type Map = BTreeMap<String, Value>;

/// A key-value record reference
pub type KvRef = Arc<Kv>;

/// A strongly-typed value stored in an `alexandria` scope
#[derive(Debug, Serialize, Deserialize)]
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
