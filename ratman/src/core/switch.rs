use async_std::{sync::Arc, task};
use netmod::{Frame, Target};

use crate::core::{Collector, Dispatch, DriverMap, Journal, RouteTable};

/// A frame switch inside Ratman to route packets and signals
///
/// The switch is given the job to poll endpoints in a loop and then
/// send the incoming frames to various points.
///
/// - Journal: the ID is not reachable
/// - Dispatch: the ID _is_ reachable
/// - Collector: the ID is local
pub(crate) struct Switch {
    /// Used only to check if the route is deemed reachable
    routes: Arc<RouteTable>,
    journal: Arc<Journal>,
    dispatch: Arc<Dispatch>,
    collector: Arc<Collector>,
    drivers: Arc<DriverMap>,
}

impl Switch {
    /// Create a new switch for the various routing components
    pub(crate) fn new(
        routes: Arc<RouteTable>,
        journal: Arc<Journal>,
        dispatch: Arc<Dispatch>,
        collector: Arc<Collector>,
        drivers: Arc<DriverMap>,
    ) -> Arc<Self> {
        Arc::new(Self {
            routes,
            journal,
            dispatch,
            collector,
            drivers,
        })
    }

    /// Dispatches a long-running task to run the switching logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move {
            loop {
                Self::run_inner(&self);
            }
        });
    }

    async fn run_inner(s: &Arc<Self>) {
        let mut g = s.drivers.inner().await;
        g.iter_mut().for_each(|(_, ep)| {
            task::spawn(async {
                let (f, t) = ep.next().await.unwrap();
            });
        })
    }
}
