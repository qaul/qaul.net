use async_std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

/// A store for active user's keys in memory
#[derive(Default)]
pub(crate) struct KeyStore {
    map: RwLock<BTreeMap<String, ()>>,
}

impl KeyStore {
    pub(crate) fn new() -> Arc<Self> {
        Default::default()
    }

    ///
    pub(crate) fn get(self: Arc<Self>) -> () {

    }
}
