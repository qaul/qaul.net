use crate::{
    error::{Error, Result},
    messages::MsgRef,
    utils::IterUtils,
};
use alexandria::Library;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

mod store;
pub use self::store::MetadataMap;
pub(crate) use self::store::MetadataStore;

/// Represents a service using libqaul
///
/// Via this type it's possible to either perform actions as a
/// particular survice, or none, which means that all service's events
/// become available.  While this is probably not desirable (and
/// should be turned off) in most situations, this way a user-level
/// service can do very powerful things with the "raw" netork traffic
/// of a qaul network.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Service {
    /// Get access to all service's events
    // One of the three most common passwords, you know?
    God,
    /// Service by domain qualified name (e.g. `net.qaul.chat`)
    Name(String),
}

impl<T> From<T> for Service
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self::Name(t.into())
    }
}

pub(crate) type Listener = Arc<dyn Fn(MsgRef) -> Result<()> + Send + Sync>;

/// A registered service, with a pre-made poll setup and listeners
pub(crate) struct IntService {
    callbacks: Arc<RwLock<Vec<Listener>>>,
}

impl IntService {
    fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(vec![])),
        }
    }
}

/// Keeps track of registered services and their callbacks
#[derive(Clone)]
pub(crate) struct ServiceRegistry {
    inner: Arc<RwLock<BTreeMap<String, IntService>>>,
    store: MetadataStore,
}

impl ServiceRegistry {
    pub(crate) fn new(library: Arc<Library>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(BTreeMap::new())),
            store: MetadataStore::new(library),
        }
    }

    /// Get access to the inner service store
    pub(crate) fn store(&self) -> &MetadataStore {
        &self.store
    }

    pub(crate) fn register(&self, name: String) -> Result<()> {
        let mut inner = self.inner.write().expect("ServiceRegistry was poisoned");
        if inner.contains_key(&name) {
            Err(Error::ServiceExists)
        } else {
            inner.insert(name, IntService::new());
            Ok(())
        }
    }

    /// Check if a service was registered before
    pub(crate) fn check(&self, name: &String) -> Result<()> {
        self.inner
            .read()
            .unwrap()
            .get(name)
            .map_or(Err(Error::NoService), |_| Ok(()))
    }

    pub(crate) fn unregister(&self, name: String) -> Result<()> {
        let mut inner = self.inner.write().expect("ServiceRegistry was poisoned");
        inner
            .remove(&name)
            .map_or(Err(Error::NoService), |_| Ok(()))
    }

    /// Push a Message out to all listener endpoints and pollers
    pub(crate) fn push_for(&self, service: String, msg: MsgRef) -> Result<()> {
        self.inner
            .read()
            .expect("ServiceRegistry was poisoned")
            .get(&service)
            .map_or(Err(Error::NoService), |srv| {
                srv.callbacks
                    .read()
                    .expect("Service callbacks were poisoned")
                    .iter()
                    .map(|cb| cb(msg.clone()))
                    .fold_errs(Error::CommFault)
            })
    }
}
