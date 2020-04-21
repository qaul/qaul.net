//! User management API scope

use crate::{core::Library, error::Result, utils::Id};
use async_std::sync::Arc;

/// API scope handler for a single user in a library
///
pub struct Users {
    pub(crate) inner: Arc<Library>,
    pub(crate) id: Id,
}

impl Users {
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
    pub async fn close(&self) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.close(self.id)
    }

    /// Create a new user with a unique encryption key
    pub async fn create(&self, pw: &str) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.insert(self.id, pw).map(|_| ())
    }

    /// Remove a user Id and corresponding data from the library
    pub async fn remove(&self) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.delete(self.id)
    }
}
