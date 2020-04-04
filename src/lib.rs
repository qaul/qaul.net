//! Encrypted record-oriented database
//!
//! **Experimental:** please note that this database was writted for
//! [qaul.net], which itself is a very experimental platform.  There
//! will be data retention bugs, and you shouldn't use Alexandria
//! unless you're okay with losing the data you're storing!
//!
//! A multi-payload, zone-encrypting, journaled persistence module,
//! built with low-overhead applications in mind.
//!
//! - Stores data in namespaces and scopes
//! - Key-value stores and lazy blobs
//! - Supports per-scope asymetric encryption key
//! - Uses transaction Deltas for journal and concurrency safety
//! - Integrates into OS persistence layers (storing things on spinning
//!   rust or zappy quantum tunnels)
//!
//! `alexandria` provides an easy to use database interface with
//! transactions, merges and dynamic queries, ensuring that your
//! in-memory representation of data never get's out-of-sync with your
//! on-disk representation. Don't burn your data.
#![allow(dead_code, unused_imports, unused_variables)]

// pub(crate) mod cache;
pub(crate) mod crypto;
pub(crate) mod delta;
pub(crate) mod diff;
pub(crate) mod dir;
pub(crate) mod meta;
pub(crate) mod notify;
pub(crate) mod path;
pub(crate) mod store;
pub(crate) mod wire;

pub mod api;
mod builder;
pub mod data;
mod error;
mod sub;

pub use crate::{
    builder::Builder,
    diff::{Diff, DiffSeg},
    error::{Error, Result},
    path::Path,
    sub::Subscription,
};

/// Primary identifier type for records and users
pub use identity::Identity as Id;

use crate::{
    api::{Data as DataApi, Users as UsersApi},
    dir::Dirs,
    meta::users::UserTable,
    store::Store,
    sub::SubHub,
};
use async_std::sync::{Arc, RwLock};

/// The in-memory representation of an alexandria storage library
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
