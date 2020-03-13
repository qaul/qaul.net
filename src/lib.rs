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

use crate::{api::users::Users as UsersApi, meta::users::UserTable};
use async_std::sync::RwLock;
use std::path::PathBuf;

/// In-memory alexandria library
pub struct Library {
    /// The main management path
    pub(crate) root: PathBuf,
    /// Table with encrypted user metadata
    pub(crate) users: RwLock<UserTable>,
}

impl Library {
    /// Internally called setup function
    pub(crate) fn init(self) -> Result<Self> {
        dir::scaffold(&self.root)?;
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
