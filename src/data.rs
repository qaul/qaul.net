//! Data descriptor module

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Discriminant data container
pub enum Data {
    /// A key-value map, encoded in json under the hood
    KV(BTreeMap<String, Value>),
    /// A binary large object
    Blob(Vec<u8>),
}

impl Data {

    /// Turn this data node into a raw data vec
    ///
    /// TODO: hint, this should stream at some point :)
    pub fn to_stream(self) -> Vec<u8> {
        match self {
            Data::Blob(vec) => vec,
            Data::KV(tree) => serde_json::to_vec(&tree).unwrap(),
        }
    }
}

/// A strongly-typed value stored in an `alexandria` scope
#[derive(Serialize, Deserialize)]
pub enum Value {
    String(String),
    Bool(bool),
    Child(BTreeMap<String, Value>),

    I128(i128),
    U128(u128),

    I64(i64),
    U64(u64),

    I32(i32),
    U32(u32),

    I16(i16),
    U16(u16),

    I8(i8),
    U8(u8),

    F64(f64),
    F32(f32),
}
