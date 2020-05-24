// Re-export Identity to the rest of the crate
pub use ratman::Identity;

use crate::{
    api::{Contacts, Messages, Services, Users},
    auth::AuthStore,
    contacts::ContactStore,
    discover::Discovery,
    messages::MsgStore,
    security::Sec,
    services::ServiceRegistry,
    users::UserStore,
};

use alexandria::{Builder, Library};
use ratman::Router;
use std::{path::Path, sync::Arc};
use tracing::{error, info};

/// An atomic reference counted pointer to a running libqaul instance
pub type QaulRef = Arc<Qaul>;

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
/// 2. `libqaul` startup (this struct, call `new(...)`)
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

    /// Provide a persistent interface to query messages
    pub(crate) messages: MsgStore,

    /// An ephemeral (non persistent) store for external services
    pub(crate) services: ServiceRegistry,

    /// A reference to the underlying routing code
    pub(crate) router: Arc<Router>,

    /// A security subsystem
    pub(crate) sec: Arc<Sec>,

    /// Main library handle for storage
    pub(crate) store: Arc<Library>,
}

impl Qaul {
    /// This function exists entirely for doc tests
    #[doc(hidden)]
    #[allow(warnings)]
    #[cfg(feature = "testing")]
    pub fn dummy() -> QaulRef {
        use tempfile;
        let router = Router::new();
        let temp = tempfile::tempdir().unwrap();
        let store = Builder::new().build().unwrap();

        Arc::new(Self {
            router,
            users: UserStore::new(Arc::clone(&store)),
            auth: AuthStore::new(),
            contacts: ContactStore::new(),
            messages: MsgStore::new(Arc::clone(&store)),
            services: ServiceRegistry::new(Arc::clone(&store)),
            sec: Arc::new(Sec::new()),
            store,
        })
    }

    /// Get access to the inner Router
    #[deprecated]
    pub fn router(&self) -> &Router {
        &self.router
    }

    /// Create new qaul context, with pre-initialised `Router`
    ///
    /// This function sets up discovery and API handler threads, as
    /// well as local storage. Stopping a qaul instance is currently
    /// not possible (woops). This call is non-blocking and assumes
    /// that the main thread will take over execution of some other
    /// application loop so to enable further API abstractions to hook
    /// into the service API.
    #[tracing::instrument(skip(router), level = "info")]
    pub fn new(router: Arc<Router>) -> QaulRef {
        // let store = Builder::inspect_path(store_path.into(), "").map_or_else(
        //     |b| match b.build() {
        //         Ok(s) => {
        //             info!("Creating new backing store");
        //             s
        //         },
        //         Err(e) => {
        //             error!("Failed to create backing store: {}", e.to_string());
        //             std::process::exit(2);
        //         }
        //     },
        //     |s| {
        //         info!("Loading existing store from disk");
        //         s
        //     },
        // );

        let store = Builder::new().build().unwrap();

        let q = Arc::new(Self {
            router: Arc::clone(&router),
            users: UserStore::new(Arc::clone(&store)),
            auth: AuthStore::new(),
            contacts: ContactStore::new(),
            messages: MsgStore::new(Arc::clone(&store)),
            services: ServiceRegistry::new(Arc::clone(&store)),
            sec: Arc::new(Sec::new()),
            store,
        });

        // TODO: Where to store this?!
        Discovery::start(Arc::clone(&q), router);
        q
    }

    /// Get messages function scope
    pub fn messages(&self) -> Messages {
        Messages { q: self }
    }

    /// Get users function scope
    pub fn users(&self) -> Users {
        Users { q: self }
    }

    /// Get contact book function scope
    pub fn contacts(&self) -> Contacts {
        Contacts { q: self }
    }

    /// Get service management function scope
    pub fn services(&self) -> Services {
        Services { q: self }
    }
}
