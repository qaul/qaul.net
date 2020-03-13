//! Internal transaction handler 

use crate::Id;
use std::collections::BTreeSet;

/// A transaction to the active dataset of a library
///
/// A delta is atomic, touches one record, and can reside in the hot
/// cache before being fully commited.
pub(crate) struct Delta {
    user: Id,
    zone: String,
    tags: BTreeSet<Vec<u8>>,
    inner: DeltaType,
}

/// Encode a change made to a record
pub enum DeltaType {
    Insert,
    Update,
    Delete,
}
