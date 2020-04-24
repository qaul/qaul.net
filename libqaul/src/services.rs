use crate::{
    error::{Error, Result},
    messages::MsgRef,
    utils::IterUtils,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

/// Represents a service using libqaul
///
/// Via this type it's possible to either perform actions as a
/// particular survice, or none, which means that all service's events
/// become available.  While this is probably not desirable (and
/// should be turned off) in most situations, this way a user-level
/// service can do very powerful things with the "raw" netork traffic
/// of a qaul network.
pub enum Service {
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
}

impl ServiceRegistry {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BTreeMap::new())),
        }
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
