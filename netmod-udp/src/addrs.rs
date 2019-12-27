//! Address resolution table module

use std::{
    collections::BTreeMap,
    net::IpAddr,
    sync::{Arc, RwLock},
};

/// A small utility that creates sequential IDs
struct IdMaker {
    last: Arc<RwLock<u16>>,
}

impl IdMaker {
    fn curr(&self) -> u16 {
        *self.last.read().expect("IdMaker was poisoned!")
    }

    fn incr(&self) -> &Self {
        *self.last.write().expect("IdMaker was poisoned!") += 1;
        self
    }
}

pub(crate) struct AddrTable {
    factory: IdMaker,
    ips: Arc<RwLock<BTreeMap<u16, IpAddr>>>,
    ids: Arc<RwLock<BTreeMap<IpAddr, u16>>>,
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
    pub(crate) fn set(&self, ip: IpAddr) -> u16 {
        let id = self.factory.incr().curr();
        self.ips.write().expect("").insert(id, ip).unwrap();
        self.ids.write().expect("").insert(ip, id).unwrap();
        id
    }

    /// Get the ID for a given IP address
    pub(crate) fn id(&self, ip: &IpAddr) -> Option<u16> {
        self.ids.read().expect("AddrTable poisoned").get(ip).cloned()
    }

    /// Get the IP for a given internal ID
    pub(crate) fn ip(&self, id: u16) -> Option<IpAddr> {
        self.ips.read().expect("AddrTable poisoned").get(&id).cloned()
    }
}
