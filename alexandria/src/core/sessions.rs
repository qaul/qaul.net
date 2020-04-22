//! User management API scope

use crate::{core::Library, error::Result, utils::Id};
use serde::{Deserialize, Serialize};

/// Represents a database session
///
/// A session is either bound to the global scope (meaning the
/// lifetime of the database in memory), or a specific id, which you
/// can yield via `id()`.  To learn more about sessions, have a look
/// at the [`SessionsApi`][api]!
///
/// [api]: struct.SessionsApi.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Session {
    /// The global session, accessed on `load()`
    Global,
    /// A user-specific session with separate key tree
    Id(Id),
}

impl Session {
    /// Get the inner Id, if applicable
    pub(crate) fn id(&self) -> Option<Id> {
        match self {
            Self::Id(id) => Some(*id),
            Self::Global => None,
        }
    }
}

/// Convenience type to represent the global namespace
pub const GLOBAL: Session = Session::Global;

/// Api scope wrapper for database sessions
///
/// A session is some random Id which is used as a namespace
/// identifier.  Each session namespace is encrypted independently,
/// with a unique key, and record tree.
pub struct SessionsApi<'alex> {
    pub(crate) inner: &'alex Library,
}

impl<'alex> SessionsApi<'alex> {
    /// List available sessions in this database
    pub async fn list(&self) -> Vec<Id> {
        vec![]
    }

    /// Open a previously created session
    pub async fn open(&self, id: Id, pw: &str) -> Result<Session> {
        let ref mut u = self.inner.users.write().await;
        u.open(id, pw).map(|_| Session::Id(id))
    }

    /// Close an active session
    pub async fn close(&self, id: Session) -> Result<()> {
        if let Some(id) = id.id() {
            let ref mut u = self.inner.users.write().await;
            u.close(id)
        } else {
            Ok(())
        }
    }

    /// Create a new user with a unique encryption key
    pub async fn create(&self, id: Id, pw: &str) -> Result<Session> {
        let ref mut u = self.inner.users.write().await;
        u.insert(id, pw).map(|_| Session::Id(id))
    }

    /// Remove a user Id and corresponding data from the library
    pub async fn destroy(&self, id: Session) -> Result<()> {
        if let Some(id) = id.id() {
            let ref mut u = self.inner.users.write().await;
            u.delete(id)
        } else {
            Ok(())
        }
    }
}
