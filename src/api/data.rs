//! Data handling

use crate::{
    data::{Record, Tag, TagSet, Type},
    Diff, Id, Library, Path, Result,
};
use async_std::sync::Arc;

pub struct Data<'a> {
    pub(crate) inner: &'a Library,
    pub(crate) id: Option<Id>,
}

/// A data query type
pub enum Query {
    /// Return a record by exact Id
    Id(Id),
    /// Get a record by it's path
    Path(Path),
    /// Make a query for the tag set
    Tag(SetQuery<TagSet>),
}

/// The result of a query to the database
pub enum QueryResult {
    /// There was a single matching item
    Single(Arc<Record>),
    /// There were many matching items
    Many(Vec<Arc<Record>>),
}

/// A special type of query on a set
///
/// The query can either be run for a partial, or complete match.  The
/// complete match is synonymous with equality (`A = B`), whereas the
/// partial match does not have to be a true subset (`A ⊆ B`)
pub enum SetQuery<T> {
    /// A partial match (subset)
    Partial(T),
    /// An  equality match
    Matching(T),
}

impl<'a> Data<'a> {
    pub fn drop(&'a self) -> &'a Library {
        self.inner
    }

    /// Insert a new record into the library
    ///
    /// You need to have a valid and active user session to do so, and
    /// the `path` must be unique.
    pub fn insert<D>(&self, path: Path, tags: Vec<Tag>, data: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        Ok(())
    }

    pub fn delete(&self, path: Path) -> Result<()> {
        Ok(())
    }

    /// Update a record in-place
    pub fn update<D>(&self, path: Path, diff: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        let diff: Diff = diff.into();

        Ok(())
    }

    /// Query the database with a specific query object
    pub fn query(&self, _: Query) -> Result<QueryResult> {
        unimplemented!()
    }
}
