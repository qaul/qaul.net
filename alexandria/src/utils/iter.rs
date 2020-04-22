//! QueryIterators

use crate::{
    error::Result,
    record::RecordRef,
    utils::{Id, Path, Query, QueryResult},
    Library,
};
use async_std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A dynamically stepped iterator for query results
///
/// See `query_iter()` to construct an iterator, and for more detailed
/// behaviour.  Be sure to drop the iterator when done to allow
/// garbage collection of deleted paths.
pub struct QueryIterator {
    pos: AtomicUsize,
    paths: Vec<Path>,
    inner: Arc<Library>,
    query: Query,
    id: Option<Id>,
}

impl QueryIterator {
    pub(crate) fn new(id: Option<Id>, paths: Vec<Path>, inner: Arc<Library>, query: Query) -> Self {
        Self {
            pos: 0.into(),
            paths,
            inner,
            query,
            id,
        }
    }

    /// Skip ahead to a certain position in the iterator
    ///
    /// If the provided position is larger than the iterator set, all
    /// future `yield`s will simply return `None`.
    pub fn skip(&self, pos: usize) {
        self.pos.fetch_add(pos, Ordering::Relaxed);
    }

    /// Return a reference to the original query of the iterator
    pub fn query(&self) -> &Query {
        &self.query
    }

    /// Lock the GC for the iterator scope
    ///
    /// Normally, when an iterator wants to access records that were
    /// deleted by other transactions, it will return an error.  To
    /// avoid this possible race condition, you can lock the garbage
    /// collector for the set of paths the iterator can touch, meaning
    /// they will remain accessible until the iterator goes out of
    /// scope.
    ///
    /// This can have unwanted side-effects, such as having records
    /// still accessible by other tasks after they were deleted, when
    /// accessed by path directly (but not via tags), or not actually
    /// deleting records if the program aborts before the iterator can
    /// restart the garbage collector again.
    pub async fn lock(&self) {
        let mut s = self.inner.store.write().await;
        s.gc_lock(self.id, &self.paths);
    }

    /// Get the next item in the iterator
    ///
    /// When the iterator has reached it's end, it will start
    /// returning `None`, at which point this instance should be
    /// dropped to allow freeing records that were held for this
    /// iterator.
    ///
    /// If any other errors occur during access, this function will
    /// return an Error, which doesn't neccessarily mean no more
    /// records can be fetched in the future.
    pub async fn next(&self) -> Result<Option<RecordRef>> {
        if self.pos.load(Ordering::Relaxed) >= self.paths.len() {
            return Ok(None);
        }

        let pos = self.pos.fetch_add(1, Ordering::Relaxed);
        let path = self.paths.get(pos).unwrap().clone();

        self.inner
            .data(self.id)
            .await?
            .query(Query::Path(path))
            .await
            .map(|r| match r {
                QueryResult::Single(rec) => Some(rec),
                QueryResult::Many(_) => unreachable!(),
            })
    }
}

impl Drop for QueryIterator {
    fn drop(&mut self) {
        async_std::task::block_on(async {
            let mut s = self.inner.store.write().await;
            s.gc_release(self.id, &self.paths)
                .expect("Failed to release deleted records!");
        });
    }
}

/// This test is dependent on the external API for queries because
/// that's how the iterator accesses records after being created.  It
/// still lives in this module, instead of the public API module
/// because it's mainly about testing this modules code, not the query code
#[cfg(test)]
mod harness {
    use crate::{
        utils::{Diff, Path, TagSet},
        Builder, Library,
    };
    use async_std::sync::Arc;
    use hex;
    use rand::{rngs::OsRng, RngCore};
    use tempfile::tempdir;

    pub struct TestData {
        lib: Arc<Library>,
        rng: OsRng,
    }

    impl TestData {
        /// Create a new test data setup
        pub fn setup() -> Self {
            let dir = tempdir().unwrap();
            let lib = Builder::new().offset(dir.path()).build().unwrap();
            let rng = OsRng {};

            Self { lib, rng }
        }

        /// Clone the library Arc
        pub fn lib(&self) -> Arc<Library> {
            Arc::clone(&self.lib)
        }

        /// Generate a random path and random payload
        pub async fn insert_random(&mut self) -> Path {
            let mut seed = [0 as u8; 8];
            self.rng.fill_bytes(&mut seed);
            let name = hex::encode_upper(&seed);
            let path = Path::from(format!("/test:{}", name));

            self.rng.fill_bytes(&mut seed);
            let key = hex::encode_upper(&seed);

            self.rng.fill_bytes(&mut seed);
            let value = hex::encode_upper(&seed);

            self.lib
                .data(None)
                .await
                .unwrap()
                .insert(
                    path.clone(),
                    TagSet::empty(),
                    Diff::map().insert(key, value),
                )
                .await
                .unwrap();

            path
        }
    }
}

#[cfg(test)]
use harness::TestData;

/// A basic iterator with three steps
#[async_std::test]
async fn basic_iterator() -> Result<()> {
    let mut t = TestData::setup();
    let paths = vec![
        t.insert_random().await,
        t.insert_random().await,
        t.insert_random().await,
    ];

    let iter = QueryIterator::new(None, paths, t.lib(), Query::Fake);
    assert!(iter.next().await?.is_some());
    assert!(iter.next().await?.is_some());
    assert!(iter.next().await?.is_some());

    // Then it ends
    assert!(iter.next().await?.is_none());
    Ok(())
}

/// Iterator test with three items, two of which are skipped
#[async_std::test]
async fn skip_iterator() -> Result<()> {
    let mut t = TestData::setup();
    let paths = vec![
        t.insert_random().await,
        t.insert_random().await,
        t.insert_random().await,
    ];

    let iter = QueryIterator::new(None, paths, t.lib(), Query::Fake);
    iter.skip(2);

    assert!(iter.next().await?.is_some());
    assert!(iter.next().await?.is_none());
    Ok(())
}

/// Iterator test with three items, two of which are skipped
#[async_std::test]
async fn gc_iterator_fail() -> Result<()> {
    let mut t = TestData::setup();
    let paths = vec![
        t.insert_random().await,
        t.insert_random().await,
        t.insert_random().await,
    ];

    let iter = QueryIterator::new(None, paths.clone(), t.lib(), Query::Fake);

    // Delete the first item
    t.lib()
        .data(None)
        .await?
        .delete(paths[0].clone())
        .await
        .unwrap();

    // The iterator will error!
    assert!(iter.next().await.is_err());

    Ok(())
}

/// Iterator test with three items, two of which are skipped
#[async_std::test]
async fn lock_gc_iterator() -> Result<()> {
    let mut t = TestData::setup();
    let paths = vec![
        t.insert_random().await,
        t.insert_random().await,
        t.insert_random().await,
    ];

    let iter = QueryIterator::new(None, paths.clone(), t.lib(), Query::Fake);
    iter.lock().await;

    // Delete the first item
    t.lib()
        .data(None)
        .await?
        .delete(paths[0].clone())
        .await
        .unwrap();

    // The path should still be there!
    assert!(iter.next().await?.is_some());

    Ok(())
}
