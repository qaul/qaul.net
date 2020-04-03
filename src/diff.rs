//! A public API facing diff system by which users can apply a diff
//! with a lot of changes to a record in one atomic write.
//!
//! This file contains some convenient from implementations to make
//! the public API easier to handle without having to always create a
//! whole tree of Diff objects

pub use crate::delta::DeltaData as DiffSeg;

use crate::{data::Value, error::DiffErrs, Error, Result};
use std::collections::BTreeMap;

pub(crate) type DiffResult<T> = std::result::Result<T, DiffErrs>;

pub(crate) trait DiffExt {
    fn apply(&mut self, diff: Diff) -> Result<()>;
}

/// An atomic set of changes applied to a record
pub enum Diff {
    /// Maps a key to a value
    Map(BTreeMap<String, DiffSeg<Value>>),
    /// Binary data
    Binary(DiffSeg<Value>),
}

impl From<Vec<u8>> for Diff {
    fn from(vec: Vec<u8>) -> Self {
        Self::Binary(DiffSeg::Insert(Value::Vec(vec)))
    }
}

impl From<BTreeMap<String, DiffSeg<Value>>> for Diff {
    fn from(map: BTreeMap<String, DiffSeg<Value>>) -> Self {
        Self::Map(map)
    }
}

impl From<(String, DiffSeg<Value>)> for Diff {
    fn from(tup: (String, DiffSeg<Value>)) -> Self {
        Self::Map({
            let mut map = BTreeMap::new();
            map.insert(tup.0, tup.1);
            map
        })
    }
}
