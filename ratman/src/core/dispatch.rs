//! Asynchronous Ratman routing core

use crate::{
    core::{DriverMap, EpTargetPair, RouteTable},
    Message, Result, Slicer,
};
use async_std::{sync::Arc, task};
use netmod::{Frame, Recipient, Target};

pub(crate) struct Dispatch {
    routes: Arc<RouteTable>,
    drivers: Arc<DriverMap>,
}

impl Dispatch {
    /// Create a new frame dispatcher
    pub(crate) fn new(routes: Arc<RouteTable>, drivers: Arc<DriverMap>) -> Arc<Self> {
        Arc::new(Self { routes, drivers })
    }

    pub(crate) async fn send_msg(&self, msg: Message) -> Result<()> {
        let r = msg.recipient;
        trace!("dispatching message to recpient: {:?}", r);

        // This is a hardcoded MTU for now.  We need to adapt the MTU
        // to the interface we're broadcasting on and we potentially
        // need a way to re-slice, or combine frames that we encounter
        // for better transmission metrics
        let frames = Slicer::slice(1312, msg);

        frames.into_iter().fold(Ok(()), |res, f| match (res, r) {
            (Ok(()), Recipient::User(_)) => task::block_on(async move { self.send_one(f).await }),
            (Ok(()), Recipient::Flood) => task::block_on(async move { self.flood(f).await }),
            (res, _) => res,
        })
    }

    /// Dispatch a single frame across the network
    pub(crate) async fn send_one(&self, frame: Frame) -> Result<()> {
        let EpTargetPair(epid, trgt) = self
            .routes
            .resolve(match frame.recipient {
                Recipient::User(id) => id,
                Recipient::Flood => unreachable!(),
            })
            .await
            .unwrap();

        let ep = self.drivers.get(epid as usize).await;
        Ok(ep.send(frame, trgt).await?)
    }

    pub(crate) async fn flood(&self, frame: Frame) -> Result<()> {
        for ep in self.drivers.get_all().await.into_iter() {
            let f = frame.clone();
            ep.send(f, Target::Flood).await.unwrap();
        }

        Ok(())
    }

    /// Reflood a message to the network, except the previous interface
    pub(crate) async fn reflood(&self, frame: Frame, ep: usize) {
        for ep in self.drivers.get_without(ep).await.into_iter() {
            let f = frame.clone();
            task::spawn(async move { ep.send(f, Target::Flood).await.unwrap() });
        }
    }
}
