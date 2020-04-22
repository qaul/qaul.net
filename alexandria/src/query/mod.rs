//! Database query builder type utilities
//!
//! Querying the database happens via a query object that is
//! constrained in one parameter of metadata.  What this means is that
//! you can either query for a direct or recursive path, a tag set, or
//! the exact Id of a record.
//!
//! This module contains types and helpers to build these queries, and
//! deal with return values that are yielded by [`query()`][query] and the
//! [`QueryIterator`][query_iter].
//!
//! [query]: ../struct.Library.html#method.query
//! [query_iter]: struct.QueryIterator.html

mod iter;
pub use iter::QueryIterator;

mod sub;
pub(crate) use sub::SubHub;
pub use sub::Subscription;

use crate::{
    record::RecordRef,
    utils::{Id, Path, TagSet},
};

/// A one-dimentional database query
///
/// It's recomended to use the builder-style constructor API, instead
/// of building a query by hand, because the internals may change at a
/// faster pace than the functions.
#[derive(Clone, Debug)]
pub enum Query {
    /// Return a record by exact Id
    Id(Id),
    /// Get a record by it's path
    Path(Path),
    /// Make a query for the tag set
    Tag(SetQuery<TagSet>),
    /// A fake query for tests
    #[cfg(test)]
    #[doc(hidden)]
    Fake,
}

impl Query {
    /// Create a direct record Id query
    pub fn id(id: Id) -> Self {
        Self::Id(id)
    }

    /// Create a path query
    pub fn path<P: Into<Path>>(p: P) -> Self {
        Self::Path(p.into())
    }

    /// Create a tag query
    pub fn tags() -> TagQuery {
        TagQuery
    }
}

/// An API type to build tag queries
pub struct TagQuery;

impl TagQuery {
    /// Build a partial (subset) tag query
    pub fn partial<T: Into<TagSet>>(t: T) -> Query {
        Query::Tag(SetQuery::Partial(t.into()))
    }

    /// Build a matching (equality) tag query
    pub fn matching<T: Into<TagSet>>(t: T) -> Query {
        Query::Tag(SetQuery::Matching(t.into()))
    }

    /// Build a negation (not) tag query
    pub fn not<T: Into<TagSet>>(t: T) -> Query {
        Query::Tag(SetQuery::Not(t.into()))
    }
}

/// The result of a query to the database
#[derive(Clone, Debug)]
pub enum QueryResult {
    /// There was a single matching item
    Single(RecordRef),
    /// There were many matching items
    Many(Vec<RecordRef>),
}

/// A special type of query on a set
///
/// The query can either be run for a partial, or complete match.  The
/// complete match is synonymous with equality (`A = B`), whereas the
/// partial match does not have to be a true subset (`A ⊆ B`)
#[derive(Clone, Debug)]
pub enum SetQuery<T> {
    /// A partial match (subset)
    Partial(T),
    /// An  equality match
    Matching(T),
    /// A negation match
    Not(T),
}
