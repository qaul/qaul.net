//! Users API scope

use crate::{error::Result, Id, Library};

/// API scope handler for a single user in a library
///
pub struct Users<'a> {
    pub(crate) inner: &'a Library,
    pub(crate) id: Id,
}

impl<'a> Users<'a> {
    /// Commit all changes up to this point and release the API scope
    pub fn commit(&'a self) -> &'a Library {
        self.inner
    }

    /// Open a user for transactions
    ///
    /// This means that future transactions for this user ID will be
    /// applied, instead of being queued to the inbox.
    pub async fn open(&self, pw: &str) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.open(self.id, pw)
    }

    /// Close an active user session
    ///
    /// This means that all future transactions will be queued to the
    /// inbox, until another session is created with `open()`
    pub async fn clone(&self) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.close(self.id)
    }

    /// Create a new user with a unique encryption key
    pub async fn create(&self, pw: &str) -> Result<Id> {
        let id = Id::random();
        let ref mut u = self.inner.users.write().await;
        u.insert(id, pw).map(|_| id)
    }

    /// Remove a user Id and corresponding data from the library
    pub async fn remove(&self, id: Id) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.delete(id)
    }

    pub fn update(&'a self, _: Id) -> Update<'a> {
        Update { inner: self }
    }
}

pub struct Update<'a> {
    pub(crate) inner: &'a Users<'a>,
}

impl<'a> Update<'a> {
    /// Drop this scope to return the inner library borrow;
    pub fn drop(&'a self) -> &'a Users {
        self.inner
    }
}
