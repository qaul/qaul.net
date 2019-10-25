//! Central qaul state holder module

use crate::{
    auth::AuthStore,
    discover::Discovery,
    users::{ContactStore, UserProfile, UserStore},
    Identity,
};
use ratman::{Router, RouterInit};

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

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

impl Default for Qaul {
    fn default() -> Self {
        Self {
            users: UserStore::new(),
            auth: AuthStore::new(),
            contacts: ContactStore::default(),
            discovery: Discovery::missing(),
            router: Arc::new(unimplemented!()),
        }
    }
}

impl Qaul {
    #[deprecated]
    pub fn start() -> Self {
        Default::default()
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

    pub fn send_test_message(&self, sender: Identity, recipient: Identity) {
        self.router.send(ratman::Message::build_signed(
            sender,
            ratman::netmod::Recipient::User(recipient),
            "testing",
            vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']
                .into_iter()
                .map(|x| x as u8)
                .collect(),
        ));
    }
}
