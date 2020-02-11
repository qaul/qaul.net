use async_std::sync::{Arc, RwLock};
use netmod::Endpoint;
use std::sync::atomic::{AtomicUsize, Ordering};

type Ep = dyn Endpoint + 'static + Send + Sync;
type EpVec = Vec<EpWrap>;

/// Wrap around endpoints that can be removed
///
/// This way, when remove an interface, the ID's of other interfaces
/// don't have have to be updated or mapped, because their place in the list doesn't change.
enum EpWrap {
    Used(Arc<Ep>),
    Void,
}

/// A map of available endpoint drivers
///
/// Currently the removing of drivers isn't supported, but it's
/// possible to have the same endpoint in the map multiple times, with
/// unique IDs.
#[derive(Default)]
pub(crate) struct DriverMap {
    curr: AtomicUsize,
    map: RwLock<EpVec>,
}

impl DriverMap {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Get the length of the driver set
    pub(crate) fn len(&self) -> usize {
        self.curr.load(Ordering::Relaxed)
    }

    /// Insert a new endpoint to the set of known endpoints
    pub(crate) async fn add<E>(&self, ep: E) -> usize
    where
        E: Endpoint + 'static + Send + Sync,
    {
        let mut map = self.map.write().await;
        let curr = self.curr.fetch_add(1, Ordering::Relaxed);
        map.push(EpWrap::Used(Arc::new(ep)));
        curr
    }

    /// Remove an endpoint from the list
    pub(crate) async fn remove(&self, id: usize) {
        let mut map = self.map.write().await;
        std::mem::swap(&mut map[id], &mut EpWrap::Void);
    }

    /// Get access to an endpoint via an Arc wrapper
    pub(crate) async fn get_arc(&self, id: usize) -> Arc<Ep> {
        let map = self.map.read().await;
        Arc::clone(match map[id] {
            EpWrap::Used(ref ep) => ep,
            EpWrap::Void => panic!("Trying to use a removed endpoint!"),
        })
    }

    /// Get all endpoints, except for the one provided via the ID
    pub(crate) async fn get_without(&self, not: usize) -> Vec<Arc<Ep>> {
        let map = self.map.read().await;
        map.iter()
            .enumerate()
            .filter_map(|(i, ep)| match ep {
                EpWrap::Used(ref ep) if i != not => Some(Arc::clone(ep)),
                _ => None,
            })
            .collect()
    }
}
