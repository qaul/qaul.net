//! General database storage abstraction

use crate::{QaulResult, User};
use alexandria::{Address, Delta, Library};

/// Provides a persistent, namespaced key-value store
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
///

pub(crate) struct DataStore {
    inner: Library,
}

trait StoreZone {
    fn init() -> QaulResult<()>;
}
