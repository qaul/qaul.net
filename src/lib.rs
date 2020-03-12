//! Alexandria storage library

pub(crate) use identity::Identity as Id;

pub(crate) mod crypto;
pub(crate) mod dir;
pub(crate) mod meta;
pub(crate) mod wire;

mod error;
pub use error::{Error, Result};

mod data;
pub use data::*;

mod builder;
pub use builder::Builder;

pub mod api;

use crate::{api::users::Users as UsersApi, meta::users::UserTable};
use std::path::PathBuf;

/// In-memory alexandria library
pub struct Library {
    /// The main management path
    pub(crate) root: PathBuf,
    /// Table with encrypted user metadata
    pub(crate) users: UserTable,
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
