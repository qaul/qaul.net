//! Fundamental API types

pub(crate) mod data;
pub use data::Data as DataApi;

mod users;
pub use users::Users as UsersApi;

mod builder;
pub use builder::Builder;

use crate::{
    dir::Dirs,
    error::Result,
    meta::users::UserTable,
    store::Store,
    utils::{Id, SubHub},
};
use async_std::sync::{Arc, RwLock};

/// In-memory representation  of an alexandria database
///
/// Refer to `Builder` on how to most easily construct an Alexandria
/// instance.  All actions (both actual and cached) are by default
/// mirrored to disk.  You may notice performance improvements by
/// turning off caches.
///
/// Alexandria addresses all data via `Path`, which is a nested
/// segment set of namespaces, zones, and subzones.
pub struct Library {
    /// The main management path
    pub(crate) root: Dirs,
    /// Table with encrypted user metadata
    pub(crate) users: RwLock<UserTable>,
    /// The main data store
    pub(crate) store: RwLock<Store>,
    /// The state handler for subscriptions
    pub(crate) subs: Arc<SubHub>,
}

impl Library {
    /// Internally called setup function
    pub(crate) fn init(self) -> Result<Self> {
        self.root.scaffold()?;
        Ok(self)
    }

    /// Load the user API scope
    pub fn user<'a>(&'a self, id: Id) -> UsersApi<'a> {
        UsersApi { inner: self, id }
    }

    /// Load the user API scope
    pub async fn data<'a, I: Into<Option<Id>>>(&'a self, id: I) -> Result<DataApi<'a>> {
        let id = id.into();

        if let Some(id) = id {
            self.users.read().await.is_open(id)?;
        }

        Ok(DataApi {
            inner: self,
            id: id,
        })
    }
}
