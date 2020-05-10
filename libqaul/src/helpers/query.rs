//! A utility module to query messages

use crate::error::{Error, Result};
use alexandria::{query::QueryIterator, record::RecordRef};
use std::marker::PhantomData;

/// The resulting set of a query operation
pub struct QueryResult<T>
where
    T: From<RecordRef>,
{
    inner: QueryIterator,
    _type: PhantomData<T>,
}

impl<T> QueryResult<T>
where
    T: From<RecordRef>,
{
    pub(crate) fn new(inner: QueryIterator) -> Self {
        Self {
            inner,
            _type: PhantomData,
        }
    }

    /// Lock the garbage collection on iteratable items
    ///
    /// By default items that are contained in this result set can be
    /// deleted by other tasks, resulting in errors when accessing
    /// data via this type.  To hold the GC from running for deleted
    /// types, you can lock the set, meaning that deleted items will
    /// only be deleted when the last locked iterator goes out of
    /// scope.
    ///
    /// This may introduce race coditions for other queries, so be
    /// aware of that!
    pub async fn lock(&self) {
        self.inner.lock().await;
    }

    /// Skip a certain number of items in the result set
    pub fn skip(&self, num: usize) {
        self.inner.skip(num);
    }

    /// Take a number of items from the iterator to advance
    ///
    /// If no more items are present, this function will return
    /// `Err()`.  When less than the requested number of items are
    /// present, it will return `Ok()` but with all remaining items.
    pub async fn take(&self, num: usize) -> Result<Vec<T>> {
        if self.inner.remaining() == 0 {
            return Err(Error::NoData);
        }

        let mut vec = Vec::with_capacity(num);
        let mut ctr = 0;
        while let Ok(Some(rec)) = self.inner.next().await {
            vec.push(rec.into());

            ctr += 1;
            if ctr > num && break {}
        }

        Ok(vec)
    }

    /// Take all elements from this result set at once
    pub async fn all(&self) -> Result<Vec<T>> {
        let mut vec = vec![];
        while let Ok(Some(rec)) = self.inner.next().await {
            vec.push(rec.into());
        }
        Ok(vec)
    }
}
