use async_std::{sync::{Arc, channel}, task};
use netmod::Recipient;

use crate::{
    core::{Collector, Dispatch, DriverMap, Journal, RouteTable, RouteType},
    IoPair, Protocol,
};

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

    /// Control channel to start new endpoints
    ctrl: IoPair<usize>,
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
            ctrl: channel(1),
        })
    }

    /// Add a new interface to the run switch
    pub(crate) async fn add(&self, id: usize) {
        self.ctrl.0.send(id).await;
    }
    
    /// Dispatches a long-running task to run the switching logic
    pub(crate) fn run(self: Arc<Self>) {
        task::spawn(async move {
            while let Some(i) = self.ctrl.1.recv().await {
                let switch = Arc::clone(&self);
                task::spawn(switch.run_inner(i));
            }
        });
    }

    async fn run_inner(self: Arc<Self>, id: usize) {
        let ep = self.drivers.get(id).await;
        loop {
            let (f, t) = match ep.next().await {
                Ok(f) => f,
                _ => continue,
            };

            // Switch the traffic to the appropriate place
            use {Recipient::*, RouteType::*};
            match f.recipient {
                Flood => {
                    let seqid = f.seq.seqid;
                    if self.journal.unknown(&seqid).await {
                        if let Some(sender) = Protocol::is_announce(&f) {
                            self.routes.update(id as u8, t, sender).await;
                        }

                        self.journal.save(&seqid).await;
                        self.dispatch.reflood(f, id).await;
                    }
                }
                User(id) => match self.routes.reachable(id).await {
                    Some(Local) => self.collector.queue_and_spawn(f.seqid(), f).await,
                    Some(Remote(_)) => self.dispatch.send_one(f).await.unwrap(),
                    None => self.journal.queue(f).await,
                },
            }
        }
    }
}
