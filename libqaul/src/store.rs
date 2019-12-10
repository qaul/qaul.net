//! General database storage abstraction

use alexandria::{Address, Data, Delta, KeyAttr, Library, ScopeAttr};
use std::sync::Arc;

/// Describes that some internal state can be made persistent
///
/// This is done by implementing a simple `store` and `load` function
/// which is called by the `DataStore` to serialise data for the
/// Alexandria storage backend.
pub(crate) trait Persist {
    /// Provide storage backend with storable data
    fn store(&self) -> Vec<Data>;

    /// Load an instance from deserialised data
    fn load(&mut self, d: Vec<Data>);
}

/// Provides a persistent, namespaced key-value store
///
/// Handles the context to `alexandria` Library object and let's state
/// providers (i.e. stateful systems inside `libqaul`) register
/// themselves to be synced to disk when appropriate.
///
/// Every user has it's own namespace. Enforcement at an Alexandria
/// level doesn't yet happen (as of 0.1.0), which means that only
/// valid Auth tokens can be accepted!
///
/// ## Typed data
///
/// The storage module in libqaul is being used for several types of
/// data, not all of which can be shared equally liberally. Some data
/// stores are accessible via the Service API, others are purely
/// internal. This section will give a brief overview.
///
/// - Password hash store (internal)
/// - Shared keystore with user-overlays (semi-internal)
/// - User-specific file store (public)
/// - User-specific message store (public)
///
/// ## About Alexandria's usage
///
/// The way that this module uses Alexandria is to the fullest of it's
/// capabilities. An Alexandria `Library` object is created for every
/// instance of `libqaul`. It is given a path to store shared data,
/// such as shared files or transparent secrets like the
/// password-hash-stores.
///
/// When a user is created, the API endpoint exposes if the user has a
/// home directory on the system, which means that files addressed to
/// that user can automatically be stored in their file-tree (with
/// permissions!). In the easy case (or Android), file storage is
/// namespaced within the Alexandria HOME dir:
/// `$ALEX_HOME/<username>/...`
pub(crate) struct DataStore {
    inner: Library,
    homedir: String,
    states: Vec<Arc<dyn Persist>>,
}

impl DataStore {
    pub(crate) fn new(homedir: String) -> Self {
        let inner = Library::new();

        Self {
            inner,
            homedir,
            states: Vec::new(),
        }
    }

    /// Register a state component with the persistence backend
    pub(crate) fn register<S>(&mut self, name: S, offset: S, modl: Arc<impl Persist>) -> usize
    where
        S: Into<String>,
    {
        let id = self.states.len();
        self.inner.modify_path(
            Address::scope(None, &name.into()),
            Delta::Insert(ScopeAttr {
                ns_auth: false,
                encryption: KeyAttr::Off,
                offset: self.homedir.clone() + "/" + &offset.into(),
            }),
        );
        self.inner.sync();
        id
    }
}
