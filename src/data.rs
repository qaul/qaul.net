//! Data descriptor module

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use crate::store::Storable;

/// Discriminant data container
#[derive(Debug)]
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
    pub fn to_stream(&self) -> Cow<[u8]> {
        match self {
            Data::Blob(vec) => vec.into(),
            Data::KV(tree) => serde_json::to_vec(&tree).unwrap().into(),
        }
    }
}

// The `offset` given to the data is the scope offset of it's parent
impl Storable for Data {
    fn read(offset: &str, name: &str) -> Self {
        let mut path = PathBuf::new();
        path.push(offset);
        path.push(name);

        let mut data = Vec::new();
        OpenOptions::new()
            .read(true)
            .create(false)
            .open(&path)
            .unwrap()
            .read_to_end(&mut data)
            .unwrap();

        // Is it a KV or Blob? Parse it and find out! :)
        match serde_json::from_slice(&data) {
            Ok(tree) => Data::KV(tree),
            Err(_) => Data::Blob(data),
        }
    }

    fn write(&self, offset: &str, name: &str) {
        let mut path = PathBuf::new();
        path.push(offset);
        path.push(name);

        let mut f = OpenOptions::new().write(true).open(&path).unwrap();
        f.write_all(&self.to_stream()).unwrap();
    }
}

/// A strongly-typed value stored in an `alexandria` scope
#[derive(Serialize, Deserialize, Debug)]
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
