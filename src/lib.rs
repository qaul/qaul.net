//! Alexandria storage library

pub(crate) mod cache;

pub(crate) mod crypto;
pub(crate) mod delta;
pub(crate) mod dir;
pub(crate) mod meta;
pub(crate) mod notify;
pub(crate) mod wire;

pub mod api;
mod builder;
mod data;
mod error;

pub use crate::{
    builder::Builder,
    data::*,
    delta::DeltaType as Delta,
    error::{Error, Result},
};
pub use identity::Identity as Id;

use crate::{
    api::users::Users as UsersApi,
    cache::{CacheRef, CombKey},
    dir::Dirs,
    meta::users::UserTable,
};
use async_std::sync::{Arc, RwLock};

/// In-memory alexandria library
pub struct Library {
    /// The main management path
    pub(crate) root: Dirs,
    /// Table with encrypted user metadata
    pub(crate) users: RwLock<UserTable>,
    /// Primary active/hot cache
    pub(crate) cache: CacheRef<CombKey, Id>,
}

impl Library {
    /// Internally called setup function
    pub(crate) fn init(self) -> Result<Self> {
        self.root.scaffold()?;
        Arc::clone(&self.cache).hot();
        Ok(self)
    }

    /// Load the user API scope
    pub fn user<'a>(&'a self, id: Id) -> UsersApi<'a> {
        UsersApi {
            inner: self,
            hot: true,
            id,
        }
    }
}
