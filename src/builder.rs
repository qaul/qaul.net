use crate::{dir::Dirs, error::Result, meta::users::UserTable, Library};
use async_std::sync::RwLock;
// use std::path::PathBuf;

/// A utility builder to construct the Alexandria library
#[derive(Default)]
pub struct Builder {
    /// The main offset path
    offset: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a normal path offset
    ///
    /// This will act as the root metadata store.  On multi-user
    /// devices it needs to be a directory that's accessibly from the
    /// daemon that owns the alexandria scope.
    pub fn offset(self, o: String) -> Self {
        Self {
            offset: o.into(),
            ..self
        }
    }

    /// Consume the builder and create a Library
    pub fn build(self) -> Result<Library> {
        let root = Dirs::new(
            self.offset
                .expect("Builder without `offset` cannot be built"),
        );
        let users = RwLock::new(UserTable::new());
        Library { root, users }.init()
    }
}
