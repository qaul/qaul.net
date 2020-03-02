//! On-file data formats
//!
//! Defines the key-value store data formats

use serde::{Serialize, Deserialize};

/// A strongly-typed value stored in an `alexandria` scope
#[derive(Serialize, Deserialize, Debug)]
pub enum Value {

    // Text
    String(String),

    // True and false
    Bool(bool),

    // A tree object
    Child(BTreeMap<String, Value>),

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
