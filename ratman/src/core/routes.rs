//! Routing table module

use crate::{Error, Result};
use async_std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use {identity::Identity, netmod::Target};

/// A netmod endpoint ID and an endpoint target ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EpTargetPair(pub(crate) u8, pub(crate) Target);

/// Describes the reachability of a route
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteType {
    Remote(EpTargetPair),
    Local,
}

/// An ephemeral routing table
///
/// It only captures the current state of best routes and has no
/// persistence relationships.  It can update entries for topology
/// changes, but these are not carried between sessions.
#[derive(Default)]
pub(crate) struct RouteTable {
    routes: Arc<Mutex<BTreeMap<Identity, RouteType>>>,
}

impl RouteTable {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Update or add an IDs entry in the routing table
    pub(crate) async fn update(&self, ep: EpTargetPair, id: Identity) {
        let mut tbl = self.routes.lock().await;
        tbl.remove(&id);
        tbl.insert(id, RouteType::Remote(ep)).unwrap();
    }

    /// Track a local ID in the routes table
    pub(crate) async fn local(&self, id: Identity) -> Result<()> {
        match self.routes.lock().await.insert(id, RouteType::Local) {
            Some(_) => Err(Error::DuplicateUser),
            None => Ok(()),
        }
    }

    /// Delete an entry from the routing table
    pub(crate) async fn delete(&self, id: Identity) {
        self.routes.lock().await.remove(&id);
    }

    /// Get the endpoint and target ID for a user Identity
    ///
    /// **Note**: this function panics if the requested ID is not
    /// currently reachable, or if it is local to the device.  Maybe
    /// use the much safer function `reachable` instead.
    pub(crate) async fn resolve(&self, id: Identity) -> EpTargetPair {
        match self.routes.lock().await.get(&id).cloned().unwrap() {
            RouteType::Remote(ep) => ep,
            _ => unreachable!(),
        }
    }

    /// Check if an ID is reachable via currently known routes
    pub(crate) async fn reachable(&self, id: Identity) -> Option<RouteType> {
        self.routes.lock().await.get(&id).cloned()
    }
}
