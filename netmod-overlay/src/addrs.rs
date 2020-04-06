use async_std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, RwLock},
};
use std::collections::BTreeMap;

/// A peer with IP and port
///
/// The netmod-overlay layer can't make assumptions about what port it
/// will be run on, so we need to keep track of that in this table.
/// The information can be pulled from the peer info returned by the
/// Udp abstraction.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Peer {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
}

/// A small utility that creates sequential IDs
struct IdMaker {
    last: Arc<RwLock<u16>>,
}

impl IdMaker {
    async fn curr(&self) -> u16 {
        *self.last.read().await
    }

    async fn incr(&self) -> &Self {
        *self.last.write().await += 1;
        self
    }
}

pub(crate) struct AddrTable {
    factory: IdMaker,
    ips: Arc<RwLock<BTreeMap<u16, Peer>>>,
    ids: Arc<RwLock<BTreeMap<Peer, u16>>>,
}

impl AddrTable {
    /// Create a new address lookup table
    pub(crate) fn new() -> Self {
        Self {
            factory: IdMaker {
                last: Default::default(),
            },
            ips: Default::default(),
            ids: Default::default(),
        }
    }

    /// Insert a given IP into the table, returning it's ID
    ///
    /// Topology changes are handled additively, because it's not
    /// possible to find out what previous IP a node had, without
    /// performing deep packet inspection and looking at certain
    /// Identity information.  As such, this table can only grow.
    pub(crate) async fn set<I: Into<Peer>>(&self, i: I) -> u16 {
        let id = self.factory.incr().await.curr().await;
        let peer = i.into();
        self.ips.write().await.insert(id, peer);
        self.ids.write().await.insert(peer, id);
        id
    }

    /// Get the ID for a given Peer address
    pub(crate) async fn id(&self, peer: Peer) -> Option<u16> {
        self.ids.read().await.get(&peer).cloned()
    }

    /// Get the Peer for a given internal ID
    pub(crate) async fn ip(&self, id: u16) -> Option<Peer> {
        self.ips.read().await.get(&id).cloned()
    }

    pub(crate) async fn all(&self) -> Vec<Peer> {
        self.ips.read().await.values().cloned().collect()
    }
}
