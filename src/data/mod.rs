//! On-file data formats

mod inbox;
mod tag;

use crate::error::Result;
use async_std::{fs::File, io::ReadExt, sync::Arc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};

/// The key-value map used in a Kv record
pub type Map = BTreeMap<String, Value>;

/// A key-value record reference
pub type KvRef = Arc<Kv>;

/// A root record reference
pub type RecRef = Arc<Record>;

/// A blob record reference
pub type BlobRef = Arc<Blob>;

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

/// A data record inside an alexandria library
pub enum Record {
    /// A large binary object record
    Blob(Blob),
    /// A key-value store record
    Kv(Kv),
}

/// A large binary object that is streamed from disk
///
/// By itself this object contains nothing but a file descriptor.  You
/// need to call `load()` on it to resolve the future and load the
/// data from disk into memory.
pub struct Blob {
    /// The filedescriptor of the object
    fd: File,
}

impl Blob {
    /// Resolve the file descriptor into
    pub async fn load(&mut self) -> Result<Vec<u8>> {
        let mut vec = vec![];
        self.fd.read_to_end(&mut vec).await?;
        Ok(vec)
    }
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
