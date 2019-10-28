pub use crate::api::services::Listener;

use crate::{
    error::{Error, Result},
    messages::Message,
    utils::IterUtils,
};
use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        {Arc, RwLock},
    },
};

/// A small wrapper around a pair of channel ends used to poll Messages
pub(crate) struct IoPair {
    rx: Receiver<Message>,
    tx: Sender<Message>,
}

/// A registered service, with a pre-made poll setup and listeners
pub(crate) struct Service {
    io: Arc<IoPair>,
    callbacks: Arc<RwLock<Vec<Box<Listener>>>>,
}

impl Service {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            io: Arc::new(IoPair { rx, tx }),
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

    /// Poll for a new Message from a service queue
    pub(crate) fn poll_for(&self, service: String) -> Result<Message> {
        self.inner
            .read()
            .expect("ServiceRegistry was poisoned")
            .get(&service)
            .map_or(Err(Error::NoService), |srv| {
                srv.io.rx.try_recv().map_err(|_| Error::CommFault)
            })
    }

    /// Push a Message out to all listener endpoints
    // TODO: Replace this with an Arc<T> ?
    pub(crate) fn push_for(&self, service: String, msg: Message) -> Result<()> {
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
