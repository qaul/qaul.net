//! Data handling

use crate::{
    core::Library,
    delta::{DeltaBuilder, DeltaType},
    error::Result,
    record::Record,
    utils::{Diff, Id, Path, Subscription, TagSet},
};
use async_std::sync::Arc;

pub struct Data<'a> {
    pub(crate) inner: &'a Library,
    pub(crate) id: Option<Id>,
}

/// A data query type
#[derive(Clone, Debug)]
pub enum Query {
    /// Return a record by exact Id
    Id(Id),
    /// Get a record by it's path
    Path(Path),
    /// Make a query for the tag set
    Tag(SetQuery<TagSet>),
}

/// The result of a query to the database
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

    /// Similar to `insert`, but instead operating on a batch of Diffs
    pub async fn batch<T, D>(&self, path: Path, tags: T, data: Vec<D>) -> Result<Id>
    where
        T: Into<TagSet>,
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Insert);

        let mut store = self.inner.store.write().await;
        let id = store.batch(
            &mut db,
            self.id,
            &path,
            tags.into(),
            data.into_iter().map(|d| d.into()).collect(),
        )?;

        Ok(id)
    }

    /// Insert a new record into the library and return it's ID
    ///
    /// You need to have a valid and active user session to do so, and
    /// the `path` must be unique.
    pub async fn insert<T, D>(&self, path: Path, tags: T, data: D) -> Result<Id>
    where
        T: Into<TagSet>,
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Insert);

        let mut store = self.inner.store.write().await;
        let id = store.insert(&mut db, self.id, &path, tags.into(), data.into())?;

        self.inner.subs.queue(db.make()).await;
        Ok(id)
    }

    pub async fn delete(&self, path: Path) -> Result<()> {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Delete);

        let mut store = self.inner.store.write().await;
        store.destroy(&mut db, self.id, &path)?;
        self.inner.subs.queue(db.make()).await;
        Ok(())
    }

    /// Update a record in-place
    pub async fn update<D>(&self, path: Path, diff: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Update);

        let mut store = self.inner.store.write().await;
        store.update(&mut db, self.id, &path, diff.into())?;
        self.inner.subs.queue(db.make()).await;
        Ok(())
    }

    /// Query the database with a specific query object
    ///
    /// Request data from alexandria via a `Query` object.  A query
    /// can only touch a single parameter, such as the Record Id, the
    /// path or a set query via tags.  The data returned are snapshots
    /// or records that are immutable.  If you want to make changes to
    /// them, use `update()` with a Diff instead.
    ///
    /// Also: future writes will not propagate to the copy of the
    /// Record returned from this function, because alexandria is
    /// Copy-on-Write.  You will need to query the database again in
    /// the future.
    ///
    /// Following are some query examples
    ///
    /// ```
    /// # use alexandria::{Builder, Library, error::Result, utils::{Tag, TagSet, Path, Query, SetQuery}};
    /// # async fn foo() -> Result<()> {
    /// # let tmp = tempfile::tempdir().unwrap();
    /// # let lib = Builder::new().offset(tmp.path()).build().unwrap();
    /// # let tag1 = Tag::new("tag1", vec![1, 3, 1, 2]);
    /// # let tag2 = Tag::new("tag2", vec![13, 12]);
    ///
    /// // Returns QueryResult::Single(_) with a single record
    /// let path = Path::from("/msg:alice");
    /// lib.data(None).await?.query(Query::Path(path)).await;
    ///
    /// // Returns QueryResult::Many(_) for every path that contains
    /// // the set of tags, or more!
    /// let tags = TagSet::from(vec![tag1, tag2]);
    /// lib.data(None).await?.query(Query::Tag(SetQuery::Partial(tags))).await;
    ///
    /// // Returns QueryResult::Many(_) for every path that contains
    /// // the exact set of tags (no more, no less)
    /// let tags = TagSet::from(vec![tag1, tag2]);
    /// lib.data(None).await?.query(Query::Tag(SetQuery::Matching(tags))).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query(&self, q: Query) -> Result<QueryResult> {
        let store = self.inner.store.read().await;
        match q {
            Query::Path(ref path) => store
                .get_path(self.id, path)
                .map(|rec| QueryResult::Single(rec)),
            Query::Tag(query) => {
                let tc = self.inner.tag_cache.read().await;

                match query {
                    SetQuery::Matching(ref tags) => tc.get_paths_matching(self.id, tags),
                    SetQuery::Partial(ref tags) => tc.get_paths(self.id, tags),
                }
                .map(|paths| {
                    paths
                        .iter()
                        .map(|p| store.get_path(self.id, p))
                        .collect::<Result<Vec<_>>>()
                        .map(|vec| QueryResult::Many(vec))
                })?
            }
            _ => unimplemented!(),
        }
    }

    pub async fn subscribe(&self, q: Query) -> Subscription {
        self.inner.subs.add_sub(q).await
    }
}
