//! Central qaul state holder module

// Export the identity into root scope
pub use identity::Identity;

use crate::{
    api::{Messages, Users, Contacts},
    auth::AuthStore,
    contacts::ContactStore,
    discover::Discovery,
    users::UserStore,
};
use ratman::{Router, RouterInit};
use std::sync::Arc;

/// Primary context structure for `libqaul`
///
/// Handles user state, secret storage, network state,
/// I/O and services. Check `api` for the extended
/// service API
///
/// ## Bootstrapping
///
/// Starting an instance of `libqaul` requires several steps.
/// For one, it needs to be initialised with a valid config
/// for the routing-layer (`RATMAN`). This requires choosing
/// of network backends and client configuration.
///
/// Secondly, `libqaul` by itself does very little, except handle
/// service requests. The service API exposes various workloads
/// available, but the consuming services also need to be configured,
/// externally to `libqaul` and this instance.
///
/// A bootstrapping procedure should thus look as follows:
///
/// 1. RATMAN + netmod initialisation
/// 2. `libqaul` startup (this struct, call `init()`)
/// 3. Initialise services with a `libqaul` instance reference
/// 4. Your application is now ready for use
#[derive(Clone)]
pub struct Qaul {
    /// Store available user profile data
    pub(crate) users: UserStore,

    /// Handles user tokens and pw hashes
    pub(crate) auth: AuthStore,

    /// Handles user-local contact books
    pub(crate) contacts: ContactStore,

    /// A service which reacts to router messages
    pub(crate) discovery: Discovery,

    /// A reference to the underlying routing code
    pub(crate) router: Arc<Router>,
}

impl Qaul {
    /// This function exists entirely for doc tests
    #[doc(hidden)]
    #[allow(warnings)]
    pub fn dummy() -> Self {
        let RouterInit { router, channel } = Router::new();
        let router = Arc::new(router);
        let discovery = Discovery::new(Arc::clone(&router), channel);
        Self {
            router,
            discovery,
            users: UserStore::new(),
            auth: AuthStore::new(),
            contacts: ContactStore::default(),
        }
    }

    /// Get access to the inner Router
    #[deprecated]
    pub fn router(&self) -> &Router {
        &self.router
    }

    /// Create new `libqaul` context, with initialised `Router`
    pub fn new(r: RouterInit) -> Self {
        let RouterInit { router, channel } = r;
        let router = Arc::new(router);
        let discovery = Discovery::new(Arc::clone(&router), channel);

        Self {
            router,
            discovery,
            users: UserStore::new(),
            auth: AuthStore::new(),
            contacts: ContactStore::default(),
        }
    }

    /// Load the `messages` API scope for qaul
    pub fn messages(&self) -> Messages {
        Messages { q: self }
    }

    /// Load the `users` API scope for qaul
    pub fn users(&self) -> Users {
        Users { q: self }
    }

    pub fn contacts(&self) -> Contacts {
        Contacts { q: self }
    }
}
