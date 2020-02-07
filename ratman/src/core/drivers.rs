use async_std::sync::Arc;
use netmod::Endpoint;
use std::sync::atomic::{AtomicUsize, Ordering};

type Ep = dyn Endpoint + 'static + Send + Sync;
type EpVec = Vec<Box<Ep>>;

/// A map of available endpoint drivers
///
/// Currently the removing of drivers isn't supported, but it's
/// possible to have the same endpoint in the map multiple times, with
/// unique IDs.
#[derive(Default)]
pub(crate) struct DriverMap {
    curr: AtomicUsize,
    map: EpVec,
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
    ///
    /// This function comes with some caveats.  To avoid the overhead
    /// of blocking on a mutex (both code and speed overhead), we
    /// store each endpoint in a vec that is sequentially addressed.
    /// An endpoint can't be removed from the list.
    ///
    /// The reason we do this is that we can spawn a future for each
    /// endpoint that will keep polling it's respective socket for
    /// packets, without having to share the entire structure as
    /// mutable or locking an endpoint when we know that we're the
    /// only ones with access.
    ///
    /// Now: this does mean that a send and receive call can be run at
    /// the same time, meanining that the endpoint needs to implement
    /// Sync, which we are enforcing with the trait bounds.
    ///
    /// Some thoughts about this: maybe there's a way to use
    /// UnsafeCell, which is slightly less gross than doing a mut
    /// transmute?  It's not sync though, so there's a bunch of
    /// overhead there which really defeats the point.  On the other
    /// hand, maybe we don't even need this collection.  We could have
    /// a handler here that can spawn tasks for an endpoint, meaning
    /// we never have to yield references to poll.. food for thought!
    #[allow(mutable_transmutes)]
    pub(crate) unsafe fn add<E>(&self, ep: E)
    where
        E: Endpoint + 'static + Send + Sync,
    {
        let map: &mut EpVec = std::mem::transmute(&self.map);
        let curr = self.curr.fetch_add(1, Ordering::Relaxed);
        map.push(Box::new(ep));
    }

    /// Get raw mutable access to an endpoint (see `add` for more)
    #[allow(mutable_transmutes)]
    pub(crate) unsafe fn get_mut(&self, id: usize) -> &mut Ep {
        let map: &mut EpVec = std::mem::transmute(&self.map);
        &mut *map[id]
    }

    // Add a new interface with a guaranteed unique ID to the map
    // pub(crate) async fn add(&self, ep: impl Endpoint + 'static + Send) {
    //     let (mut map, mut id) = self.map.lock().join(self.curr.lock()).await;
    //     map.insert(*id, Box::new(ep));
    //     *id += 1;
    // }

    // Returns access to the unlocked endpoint collection
    //     pub(crate) async fn inner<'this>(&'this self) -> MutexGuard<'_, EndpointMap> {
    //         self.map.lock().await
    //     }
}
