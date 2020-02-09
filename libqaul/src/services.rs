use crate::{
    error::{Error, Result},
    messages::MsgRef,
    utils::IterUtils,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

pub(crate) type Listener = Arc<dyn Fn(MsgRef) -> Result<()> + Send + Sync>;

/// A registered service, with a pre-made poll setup and listeners
pub(crate) struct Service {
    callbacks: Arc<RwLock<Vec<Listener>>>,
}

impl Service {
    fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(vec![])),
        }
    }
}

/// Keeps track of registered services and their callbacks
#[derive(Clone)]
pub(crate) struct ServiceRegistry {
    inner: Arc<RwLock<BTreeMap<String, Service>>>,
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
            inner.insert(name, Service::new());
            Ok(())
        }
    }

    pub(crate) fn unregister(&self, name: String) -> Result<()> {
        let mut inner = self.inner.write().expect("ServiceRegistry was poisoned");
        inner
            .remove(&name)
            .map_or(Err(Error::NoService), |_| Ok(()))
    }

    pub(crate) fn add_listener<F: 'static + Send + Sync>(
        &self,
        service: String,
        listener: F,
    ) -> Result<()>
    where
        F: Fn(MsgRef) -> Result<()>,
    {
        self.inner
            .write()
            .expect("ServiceRegistry was poisoned")
            .get(&service)
            .map_or(Err(Error::NoService), |srv| {
                Ok(srv
                    .callbacks
                    .write()
                    .expect("Service callbacks were poisoned")
                    .push(Arc::new(listener)))
            })
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
