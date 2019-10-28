pub use crate::api::services::Listener;

use crate::messages::Message;
use std::{
    collections::BTreeMap,
    sync::{
        mpsc::{channel, Sender},
        {Arc, Mutex},
    },
};

pub(crate) struct Service {
    name: String,
    poll_send: Sender<Message>,
    listeners: Vec<Box<Listener>>,
}

/// Keeps track of registered services and their callbacks
#[derive(Clone)]
pub(crate) struct ServiceRegistry {
    inner: Arc<Mutex<BTreeMap<String, Service>>>,
}

impl ServiceRegistry {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}
