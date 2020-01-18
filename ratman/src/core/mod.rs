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

use crate::Message;
use async_std::sync::Arc;
use netmod::Endpoint;

/// The Ratman routing core interface
///
/// The core handles sending, receiving and storing frames that can't
/// be delivered at that time (delay-tolerance).
pub(crate) struct Core {
    collector: Arc<Collector>,
    _dispatch: Arc<Dispatch>,
    journal: Arc<Journal>,
    _routes: Arc<RouteTable>,
    switch: Arc<Switch>,
    drivers: Arc<DriverMap>,
}

impl Core {
    /// Initialises, but doesn't run the routing core
    pub(crate) fn init() -> Self {
        let drivers = DriverMap::new();
        let routes = RouteTable::new();
        let journal = Journal::new();

        let dispatch = Dispatch::new(Arc::clone(&routes), Arc::clone(&drivers));
        let collector = Collector::new();

        let switch = Switch::new(
            Arc::clone(&routes),
            Arc::clone(&journal),
            Arc::clone(&dispatch),
            Arc::clone(&collector),
            Arc::clone(&drivers),
        );

        Self {
            _dispatch: dispatch,
            _routes: routes,
            collector,
            journal,
            switch,
            drivers,
        }
    }

    /// Asynchronously runs all routing core subroutines
    ///
    /// **Note**: currently it's not possible to gracefully shut down
    /// the core subsystems!
    pub(crate) fn run(&self) {
        Arc::clone(&self.switch).run();
        Arc::clone(&self.journal).run();
        Arc::clone(&self.collector).run();
    }

    /// Asynchronously send a Message
    pub(crate) async fn send(&self, msg: Message) {}

    /// Poll for the incoming Message
    pub(crate) async fn next(&self) -> Message {
        self.collector.completed().await
    }

    /// Insert a new endpoint
    pub(crate) fn add_ep(&self, ep: impl Endpoint + 'static + Send + Sync) {
        unsafe { self.drivers.add(ep) };
    }
}
