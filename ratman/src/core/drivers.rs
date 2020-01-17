use async_std::{
    pin::Pin,
    prelude::*,
    sync::{Arc, Mutex, MutexGuard},
};

use netmod::Endpoint;
use std::collections::BTreeMap;

type EndpointMap = BTreeMap<u8, Box<dyn Endpoint + 'static + Send>>;

/// A map of available endpoint drivers
///
/// Currently the removing of drivers isn't supported, but it's
/// possible to have the same endpoint in the map multiple times, with
/// unique IDs.
#[derive(Default)]
pub(crate) struct DriverMap {
    /// Used to create EP IDs
    curr: Mutex<u8>,
    map: Mutex<EndpointMap>,
}

impl DriverMap {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Add a new interface with a guaranteed unique ID to the map
    pub(crate) async fn add(&self, ep: impl Endpoint + 'static + Send) {
        let (mut map, mut id) = self.map.lock().join(self.curr.lock()).await;
        map.insert(*id, Box::new(ep));
        *id += 1;
    }

    /// Returns access to the unlocked endpoint collection
    pub(crate) async fn inner<'this>(&'this self) -> MutexGuard<'_, EndpointMap> {
        self.map.lock().await
    }
}
