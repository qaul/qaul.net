//! Users API scope

use crate::{error::Result, Id, Library};

/// API scope handler for a single user in a library
///
/// The API is sharded into different scopes, each of which can be
/// nested to call more specific functions.
///
/// In Alexandria, a library has users that store data in zones and
/// records.  To modify a user's zone list, you first have to
/// initialise the API scope for this user Id.  This way all further
/// API calls will be properly namespaced, without having to replicate
/// the same arguments to all function calls again, thus reducing
/// errors in the usage.
///
/// ## Commiting data
///
/// All changes made are cached before being written to the
/// active data set.  You need to call `commit()` to queue them from
/// the cache to the active set.  Alternatively you can call
/// `discard()` to clear the entries from the cache.  Dropping the API
/// scope handler is synonymous to calling `commit()`.
///
/// ## Hot cache & cold cache
///
/// When you queue an action via the API it will be immediatiely
/// validated and entered into a hot cache.  This ensures that other
/// comsumers get immediate access to the data.  Altenatively you can
/// queue your changes into the cald cache, by first calling `cold()`
/// on the API state handler.
///
/// Then either `commit()` your changes or release the cold-lock by
/// calling `cache()`.
///
/// The hot cache will be periodically synced to disk to provide crash
/// resistence; **the cold cache will not!**
pub struct Users<'a> {
    pub(crate) inner: &'a Library,
    pub(crate) id: Id,
    pub(crate) hot: bool,
}

impl<'a> Users<'a> {
    /// Commit all changes up to this point and release the API scope
    pub fn commit(&'a self) -> &'a Library {
        self.inner
    }

    pub async fn open(&self, pw: &str) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.open(self.id, pw)
    }

    /// Close and sync a user's session
    pub async fn clone(&self) -> Result<()> {
        let ref mut u = self.inner.users.write().await;
        u.close(self.id)
    }

    /// Create a new user
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
