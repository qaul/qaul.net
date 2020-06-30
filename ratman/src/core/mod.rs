//! Routing core components
//!
//! In previous designs (both code and docs) this was a single
//! component. This has proven to be a hard to maintain approach, so
//! instead the core has been split into several parts.

mod collector;
mod dispatch;
mod drivers;
mod journal;
mod routes;
mod switch;

pub(self) use collector::Collector;
pub(self) use dispatch::Dispatch;
pub(self) use drivers::DriverMap;
pub(self) use journal::Journal;
pub(self) use routes::{EpTargetPair, RouteTable, RouteType};
pub(self) use switch::Switch;

use crate::{Endpoint, Error, Identity, Message, Result};
use async_std::sync::Arc;
use netmod::Frame;

/// The Ratman routing core interface
///
/// The core handles sending, receiving and storing frames that can't
/// be delivered at that time (delay-tolerance).
pub(crate) struct Core {
    collector: Arc<Collector>,
    dispatch: Arc<Dispatch>,
    _journal: Arc<Journal>,
    routes: Arc<RouteTable>,
    switch: Arc<Switch>,
    drivers: Arc<DriverMap>,
}

impl Core {
    /// Initialises, but doesn't run the routing core
    pub(crate) fn init() -> Self {
        let drivers = DriverMap::new();
        let routes = RouteTable::new();
        let _journal = Journal::new();

        let dispatch = Dispatch::new(Arc::clone(&routes), Arc::clone(&drivers));
        let collector = Collector::new();

        let switch = Switch::new(
            Arc::clone(&routes),
            Arc::clone(&_journal),
            Arc::clone(&dispatch),
            Arc::clone(&collector),
            Arc::clone(&drivers),
        );

        // Dispatch the runners
        Arc::clone(&switch).run();
        Arc::clone(&_journal).run();

        Self {
            dispatch,
            routes,
            collector,
            _journal,
            switch,
            drivers,
        }
    }

    /// Asynchronously send a Message
    pub(crate) async fn send(&self, msg: Message) -> Result<()> {
        self.dispatch.send_msg(msg).await
    }

    /// Send a frame directly, without message slicing
    ///
    /// Some components in Ratman, outside of the routing core, need
    /// access to direct frame intercepts, because protocol logic
    /// depends on unmodified frames.
    pub(crate) async fn raw_flood(&self, f: Frame) -> Result<()> {
        self.dispatch.flood(f).await
    }

    /// Poll for the incoming Message
    pub(crate) async fn next(&self) -> Message {
        self.collector.completed().await
    }

    /// Check if an Id is present in the routing table
    pub(crate) async fn known(&self, id: Identity, local: bool) -> Result<()> {
        if local {
            self.routes.local(id).await
        } else {
            self.routes
                .resolve(id)
                .await
                .map_or(Err(Error::NoUser), |_| Ok(()))
        }
    }

    /// Returns users that were newly discovered in the network
    pub(crate) async fn discover(&self) -> Identity {
        self.routes.discover().await
    }

    /// Insert a new endpoint
    pub(crate) async fn add_ep(&self, ep: Arc<impl Endpoint + 'static + Send + Sync>) -> usize {
        let id = self.drivers.add(ep).await;
        self.switch.add(id).await;
        id
    }

    /// Get an endpoint back from the driver set via it's ID
    pub(crate) async fn get_ep(&self, id: usize) -> Arc<dyn Endpoint + 'static + Send + Sync> {
        self.drivers.get(id).await
    }

    /// Remove an endpoint
    pub(crate) async fn rm_ep(&self, id: usize) {
        self.drivers.remove(id).await;
    }

    /// Add a local user endpoint
    pub(crate) async fn add_local(&self, id: Identity) -> Result<()> {
        self.routes.add_local(id).await
    }

    /// Remove a local user endpoint
    pub(crate) async fn rm_local(&self, id: Identity) -> Result<()> {
        self.routes.delete(id).await
    }

    #[cfg(test)]
    pub(crate) async fn get_users(&self) -> Vec<Identity> {
        self.routes.all().await
    }
}
