use crate::{
    dir::Dirs,
    error::Result,
    meta::{tags::TagCache, users::UserTable},
    query::SubHub,
    store::Store,
    Library,
};
use async_std::sync::{Arc, RwLock};
use std::path::Path;

/// A utility to configure and initialise an alexandria database
///
/// To load an existing database from disk, look at
/// [`Library::load()`][load]!
///
/// [load]: struct.Library.html#load
///
/// ```
/// # use alexandria::{Builder, Library, error::Result};
/// # use tempfile::tempdir;
/// # fn test() -> Result<()> {
/// let dir = tempdir().unwrap();
/// let lib = Builder::new()
///               .offset(dir.path())
///               .root_sec("car horse battery staple")
///               .build()?;
/// # drop(lib);
/// # Ok(()) }
/// ```
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

    /// Some secret that will be used for the root namespace
    ///
    /// When loading a library from disk in a future session, this
    /// secret will have to be provided to [`Library::load()`][load]
    ///
    /// [load]: struct.Library.html#load
    pub fn root_sec<S: Into<String>>(self, _: S) -> Self {
        self
    }

    /// Consume the builder and create a Library
    pub fn build(self) -> Result<Arc<Library>> {
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
        .map(|l| Arc::new(l))
    }
}
