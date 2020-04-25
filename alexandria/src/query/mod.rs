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

use crate::{utils::{Id, Path, TagSet}, record::RecordRef};

/// A one-dimentional database query
///
/// It's recomended to use the builder-style constructor API, instead
/// of building a query by hand, because the internals may change at a
/// faster pace than the functions.
#[derive(Clone, Debug, PartialEq, Eq)]
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
///
/// Following is an overview of the type constraints available.
///
/// | SetQuery type        | Constraints model |
/// |----------------------|-------------------|
/// | SetQuery::Intersect  | A ∩ B             |
/// | SetQuery::Subset     | A ⊆ B             |
/// | SetQuery::Equals     | A = B             |
/// | SetQuery::Not        | ¬(A ∩ B)          |
pub struct TagQuery;

impl TagQuery {
    /// Build a simple intersect query
    ///
    /// An [intersection] is defined by set theory as any overlap
    /// between sets A and B, meaning that neither A nor B needs to be
    /// contained in the other (A ∩ B).  This is the weakest tag
    /// constraint, as it doesn't filter sub-, or equality sets.
    ///
    /// [intersection]: https://en.wikipedia.org/wiki/Intersection_(set_theory)
    pub fn intersect<T: Into<TagSet>>(&self, t: T) -> Query {
        Query::Tag(SetQuery::Intersect(t.into()))
    }

    /// Build a subset query
    ///
    /// A [subset] is defined by set theory as a set A, which is
    /// contained in it's entirety by a set B.  The two sets may be
    /// equals: A ⊆ B.  This is the most common set query to use
    /// because it allows constrained tags, without disallowing
    /// additional tags that are irrelevant to the query program.
    ///
    /// [subset]: https://en.wikipedia.org/wiki/Set_(mathematics)#Subsets
    pub fn subset<T: Into<TagSet>>(&self, t: T) -> Query {
        Query::Tag(SetQuery::Subset(t.into()))
    }

    /// Build an equality set query
    ///
    /// An equality set checks for an exact match of tags in the
    /// query, meaning that additional tags will make the comparison
    /// fail.
    ///
    /// This query type is most likely a lot less useful than
    /// `subset()`, but can still be used as part of a processing
    /// pipeline that adds tags to records over time.
    pub fn equals<T: Into<TagSet>>(&self, t: T) -> Query {
        Query::Tag(SetQuery::Equals(t.into()))
    }

    /// Build an exclusion set query
    ///
    /// This query type can be considered the opposite of
    /// `intersect()`, because it will fail the comparison of sets
    /// if even a single tag is shared between them.
    pub fn not<T: Into<TagSet>>(&self, t: T) -> Query {
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

impl QueryResult {
    pub fn merge(self, o: QueryResult) -> Self {
        use self::QueryResult::*;

        match (self, o) {
            (Single(r1), Single(r2)) => Self::Many(vec![r1, r2]),
            (Single(r), Many(mut vec)) => {
                vec.push(r);
                Self::Many(vec)
            }
            (Many(mut vec), Single(r)) => {
                vec.push(r);
                Self::Many(vec)
            }
            (Many(mut vec1), Many(mut vec2)) => {
                vec1.append(&mut vec2);
                Self::Many(vec1)
            }
        }
    }
}

/// a special type of query on a set
///
/// It's highly recomended to read the function descriptions on
/// [`TagQuery`][tagquery] for an explanation of the differences between these
/// set query constraints.
///
/// [tagquery]: struct.TagQuery.html
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetQuery<T> {
    Intersect(T),
    Subset(T),
    Equals(T),
    Not(T),
}
