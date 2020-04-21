//! A set of utilities to work with an alexandria library
//!
//! Most of these are rather fundamental, and are clustered here to
//! simplify imports.

/// Primary identifier type for records and users
///
/// Internally an Id is represented as the size of a word on the
/// platform that alexandria was compiled for.  Each Id is unique and
/// random (via `Id::random()`).
pub use id::Identity as Id;

pub use crate::core::data::{Query, QueryResult, SetQuery};

mod diff;
pub use diff::{Diff, DiffSeg};
pub(crate) use diff::{DiffExt, DiffResult};

mod iter;
pub use iter::QueryIterator;

mod tag;
pub use tag::{Tag, TagSet};

mod path;
pub use path::Path;

mod sub;
pub(crate) use sub::SubHub;
pub use sub::Subscription;
