//! Asynchronous Ratman routing core

use crate::core::{DriverMap, RouteTable};
use async_std::sync::Arc;
use netmod::{Frame, Target};

pub(crate) struct Dispatch {
    routes: Arc<RouteTable>,
    drivers: Arc<DriverMap>,
}

impl Dispatch {
    pub(crate) fn new(routes: Arc<RouteTable>, drivers: Arc<DriverMap>) -> Arc<Self> {
        Arc::new(Self { routes, drivers })
    }

    /// Dispatch a single frame across the network
    pub(crate) async fn send(&self, frame: Frame, target: Target) {}

    /// Reflood a message to the network, except the previous interface
    pub(crate) async fn reflood(&self, frame: Frame, target: Target) {}
}
