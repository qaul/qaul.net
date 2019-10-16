
//! The routing core module of `RATMAN`

use identity::Identity;
use netmod::{Endpoint, Frame, Payload};
use std::{collections::BTreeMap, sync::Arc};

/// The routing core powering `RATMAN`
///
/// Keeps track of available interfaces, the routing table (mapping `Identity` -> IF), as well as handling routing workers
pub(crate) struct Core {
    cnt: u8,
    /// A list of available interfaces, assigned sequentials IDs
    pub(crate) ifs: BTreeMap<u8, Box<dyn Endpoint>>,
    /// Mapping network IDs to interface IDs
    pub(crate) routes: BTreeMap<Identity, u8>,
}

impl Core {
    /// Create a new routing core
    pub(crate) fn new() -> Self {
        Core {
            cnt: 0,
            ifs: BTreeMap::new(),
            routes: BTreeMap::new(),
        }
    }

    /// Add an interface, assigning it a unique ID
    pub(crate) fn add_if(&mut self, ep: impl Endpoint + 'static) {
        let id = self.cnt;
        self.cnt += 1;
        self.ifs.insert(id, Box::new(ep));
    }

    /// Remove an interface with unique ID
    pub(crate) fn del_if(&mut self, id: u8) {
        self.ifs.remove(&id);
    }

    /// Remove a list of interface names and their unique IDs
    pub(crate) fn get_ifs(&self) -> Vec<(u8, String)> {
        vec![]
    }
}

pub(crate) struct Node {
    id: Identity,
    rxtx: Box<Endpoint>,
}

