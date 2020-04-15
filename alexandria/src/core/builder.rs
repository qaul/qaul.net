use crate::{
    dir::Dirs,
    error::Result,
    meta::{tags::TagCache, users::UserTable},
    store::Store,
    utils::SubHub,
    Library,
};
use async_std::sync::RwLock;
use std::path::Path;

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
    pub fn offset<'tmp, P: Into<&'tmp Path>>(self, offset: P) -> Self {
        let p: &Path = offset.into();
        let offset = p.to_str().map(|s| s.to_string());
        Self { offset, ..self }
    }

    /// Consume the builder and create a Library
    pub fn build(self) -> Result<Library> {
        let root = Dirs::new(
            self.offset
                .expect("Builder without `offset` cannot be built"),
        );
        let users = RwLock::new(UserTable::new());
        let tag_cache = RwLock::new(TagCache::new());

        let store = RwLock::new(Store::new());
        let subs = SubHub::new();
        Library {
            root,
            users,
            tag_cache,
            store,
            subs,
        }
        .init()
    }
}
