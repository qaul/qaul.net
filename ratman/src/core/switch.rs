use async_std::{sync::Arc, task};
use netmod::Recipient;

use crate::core::{Dispatch, DriverMap, Journal, RouteTable, RouteType};

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
    // collector: Arc<Collector>,
    drivers: Arc<DriverMap>,
}

impl Switch {
    /// Create a new switch for the various routing components
    pub(crate) fn new(
        routes: Arc<RouteTable>,
        journal: Arc<Journal>,
        dispatch: Arc<Dispatch>,
        // collector: Arc<Collector>,
        drivers: Arc<DriverMap>,
    ) -> Arc<Self> {
        Arc::new(Self {
            routes,
            journal,
            dispatch,
            // collector,
            drivers,
        })
    }

    /// Dispatches a long-running task to run the switching logic
    pub(crate) fn run(self: Arc<Self>) {
        let _: Vec<_> = (0..self.drivers.len())
            .into_iter()
            .map(|i| {
                let switch = Arc::clone(&self);
                task::spawn(switch.run_inner(i))
            })
            .collect();
    }

    async fn run_inner(self: Arc<Self>, id: usize) {
        let ep = self.drivers.get_arc(id).await;
        loop {
            let (f, _) = match ep.next().await {
                Ok(f) => f,
                _ => continue,
            };

            // Switch the traffic to the appropriate place
            use {Recipient::*, RouteType::*};
            match f.recipient {
                Flood => {
                    let seqid = f.seq.seqid; // great names there kookie
                    if self.journal.known(&seqid).await {
                        self.journal.save(&seqid).await;
                        self.dispatch.reflood(f, id).await
                    }
                }
                User(id) => match self.routes.reachable(id).await {
                    Some(Local) => unimplemented!(), // self.collector.queue(f).await,
                    Some(Remote(_)) => self.dispatch.send(f).await,
                    None => self.journal.queue(f).await,
                },
            }
        }
    }
}
