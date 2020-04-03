use crate::{data::Value, Id, Path, data::Tag};

/// A transaction to the active dataset of a library
///
/// A delta is atomic, touches one field of one record, and can reside in the hot
/// cache before being fully commited.  It is authenticated against an
/// active user before being cached.
///
/// The `path` segment is constructed via the
pub struct Delta {
    user: Id,
    path: Path,
    data: DeltaTarget<Value>,
}

/// What element of the record is getting touched by the delta
pub enum DeltaTarget<T> {
    /// Modify the header tags
    Tags(DeltaData<Tag>),
    /// Modify the actual data
    Data(DeltaData<T>),
}

/// Encode a single change made to a set of data
pub enum DeltaData<T> {
    /// Creating a new record
    Insert(T),
    /// Updating an existing record in place
    Update(T),
    /// Deleting a record
    Delete,
}
