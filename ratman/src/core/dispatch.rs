//! Asynchronous Ratman routing core

use crate::core::{DriverMap, RouteTable};
use async_std::sync::Arc;

pub(crate) struct Dispatch {
    routes: Arc<RouteTable>,
    drivers: Arc<DriverMap>,
}

impl Dispatch {
    pub(crate) fn new(routes: Arc<RouteTable>, drivers: Arc<DriverMap>) -> Arc<Self> {
        Arc::new(Self { routes, drivers })
    }

    /// Dispatch a single frame across the network
    pub(crate) async fn send() {}
}
