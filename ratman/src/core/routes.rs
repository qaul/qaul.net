//! Routing table module

use crate::{Error, IoPair, Result};
use async_std::{
    sync::{channel, Arc, Mutex},
    task,
};
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
pub(crate) struct RouteTable {
    routes: Arc<Mutex<BTreeMap<Identity, RouteType>>>,
    new: IoPair<Identity>,
}

impl RouteTable {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            routes: Default::default(),
            new: channel(1),
        })
    }

    /// Update or add an IDs entry in the routing table
    ///
    /// If the Id was not previously known to the router, it is queued
    /// to the `new` set which can be polled by calling `discovered().await`.
    pub(crate) async fn update(self: &Arc<Self>, if_: u8, t: Target, id: Identity) {
        let mut tbl = self.routes.lock().await;
        let route = RouteType::Remote(EpTargetPair(if_, t));

        // Only "announce" a new user if it was not known before
        if tbl.insert(id, route).is_none() {
            let s = Arc::clone(&self);
            task::spawn(async move { s.new.0.send(id).await });
        }
    }

    /// Poll the set of newly discovered users
    pub(crate) async fn discover(&self) -> Identity {
        self.new.1.recv().await.unwrap()
    }

    /// Track a local ID in the routes table
    pub(crate) async fn add_local(&self, id: Identity) -> Result<()> {
        match self.routes.lock().await.insert(id, RouteType::Local) {
            Some(_) => Err(Error::DuplicateUser),
            None => Ok(()),
        }
    }

    /// Check if a user is locally known
    pub(crate) async fn local(&self, id: Identity) -> Result<()> {
        match self.reachable(id).await {
            Some(RouteType::Local) => Ok(()),
            _ => Err(Error::NoUser),
        }
    }

    /// Delete an entry from the routing table
    pub(crate) async fn delete(&self, id: Identity) -> Result<()> {
        match self.routes.lock().await.remove(&id) {
            Some(_) => Ok(()),
            None => Err(Error::NoUser),
        }
    }

    /// Get all users in the routing table
    #[cfg(test)]
    pub(crate) async fn all(&self) -> Vec<Identity> {
        self.routes.lock().await.iter().map(|(i, _)| *i).collect()
    }

    /// Get the endpoint and target ID for a user Identity
    ///
    /// **Note**: this function may panic if no entry was found, and
    /// returns `None` if the specified ID isn't remote.  To get more
    /// control over how the table is queried, use `reachable` instead
    pub(crate) async fn resolve(&self, id: Identity) -> Option<EpTargetPair> {
        match self.routes.lock().await.get(&id).cloned().unwrap() {
            RouteType::Remote(ep) => Some(ep),
            RouteType::Local => None,
        }
    }

    /// Check if an ID is reachable via currently known routes
    pub(crate) async fn reachable(&self, id: Identity) -> Option<RouteType> {
        self.routes.lock().await.get(&id).cloned()
    }
}
