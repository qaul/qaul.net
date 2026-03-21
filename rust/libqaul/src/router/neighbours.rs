// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Table of all direct neighbour nodes
//!
//! There is a table per connection module.

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled::Tree;
use std::{collections::HashMap, sync::RwLock};

use super::info::RouterInfo;
use super::proto;
use crate::connections::ConnectionModule;
use crate::rpc::Rpc;
use crate::storage::database::DataBase;
use crate::utilities::{qaul_id::QaulId, timestamp::Timestamp};

/// Node entry in the data base
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// node id
    pub id: Vec<u8>,
    /// last connect
    pub connected_at: u64,
}

/// Instance-based neighbours state owning all per-module tables.
/// Replaces the global INTERNET/LAN/BLE/NODES statics for multi-instance use.
pub struct NeighboursState {
    pub internet: RwLock<Neighbours>,
    pub lan: RwLock<Neighbours>,
    pub ble: RwLock<Neighbours>,
    /// Optional persistent node db tree. None in simulation mode.
    /// Uses RwLock to allow interior mutability for late initialization.
    pub nodes_db: RwLock<Option<Tree>>,
}

impl NeighboursState {
    /// Create a new empty NeighboursState without a database backing.
    pub fn new() -> Self {
        Self {
            internet: RwLock::new(Neighbours {
                nodes: HashMap::new(),
            }),
            lan: RwLock::new(Neighbours {
                nodes: HashMap::new(),
            }),
            ble: RwLock::new(Neighbours {
                nodes: HashMap::new(),
            }),
            nodes_db: RwLock::new(None),
        }
    }

    /// Set the database tree for persistent neighbour storage.
    pub fn set_nodes_db(&self, tree: Tree) {
        let mut db = self.nodes_db.write().unwrap();
        *db = Some(tree);
    }

    /// Get the neighbours table for a given module.
    fn get_table(&self, module: &ConnectionModule) -> Option<&RwLock<Neighbours>> {
        match module {
            ConnectionModule::Internet => Some(&self.internet),
            ConnectionModule::Lan => Some(&self.lan),
            ConnectionModule::Ble => Some(&self.ble),
            _ => None,
        }
    }

    /// Update a node in the appropriate module table.
    /// Does NOT persist to DB or trigger RouterInfo (use for simulation).
    pub fn update_node(&self, module: ConnectionModule, node_id: PeerId, rtt: u32) {
        let table_lock = match self.get_table(&module) {
            Some(t) => t,
            None => return,
        };

        let mut neighbours = table_lock.write().unwrap();
        let node_option = neighbours.nodes.get_mut(&node_id);
        if let Some(node) = node_option {
            node.rtt = Neighbours::calculate_rtt(node.rtt, rtt);
            node.updated_at = Timestamp::get_timestamp();
        } else {
            neighbours.nodes.insert(
                node_id,
                Neighbour {
                    rtt,
                    updated_at: Timestamp::get_timestamp(),
                },
            );
        }
    }

    /// Delete a neighbour from the given module table.
    pub fn delete(&self, module: ConnectionModule, node_id: PeerId) {
        if let Some(table_lock) = self.get_table(&module) {
            let mut neighbours = table_lock.write().unwrap();
            neighbours.nodes.remove(&node_id);
        }
    }

    /// Get RTT for a neighbour in a specific module.
    pub fn get_rtt(&self, neighbour_id: &PeerId, module: &ConnectionModule) -> Option<u32> {
        match module {
            ConnectionModule::Local => return Some(0),
            ConnectionModule::None => return None,
            _ => {}
        }

        if let Some(table_lock) = self.get_table(module) {
            let neighbours = table_lock.read().unwrap();
            return neighbours.nodes.get(neighbour_id).map(|n| n.rtt);
        }
        None
    }

    /// Check if this node is a neighbour in any module.
    pub fn is_neighbour(&self, node_id: &PeerId) -> ConnectionModule {
        {
            let lan = self.lan.read().unwrap();
            if lan.nodes.contains_key(node_id) {
                return ConnectionModule::Lan;
            }
        }
        {
            let internet = self.internet.read().unwrap();
            if internet.nodes.contains_key(node_id) {
                return ConnectionModule::Internet;
            }
        }
        {
            let ble = self.ble.read().unwrap();
            if ble.nodes.contains_key(node_id) {
                return ConnectionModule::Ble;
            }
        }
        ConnectionModule::None
    }

    /// Get all BLE-only nodes (nodes reachable only via BLE).
    pub fn get_ble_only_nodes(&self) -> Vec<PeerId> {
        let ble = self.ble.read().unwrap();
        let mut nodes = Vec::with_capacity(ble.nodes.len());

        if !ble.nodes.is_empty() {
            let lan = self.lan.read().unwrap();
            let internet = self.internet.read().unwrap();

            for (id, _val) in ble.nodes.iter() {
                if lan.nodes.contains_key(id) {
                    continue;
                }
                if internet.nodes.contains_key(id) {
                    continue;
                }
                nodes.push(*id);
            }
        }

        nodes
    }
}

/// Neighbours Module State
///
/// Contains all connected neighbour nodes of a connection module
/// It represents the per connection module, module state.
pub struct Neighbours {
    nodes: HashMap<PeerId, Neighbour>,
}

/// Neighbour node connectivity entry
pub struct Neighbour {
    /// round trip time in micro seconds
    rtt: u32,
    /// when was this node last seen
    updated_at: u64,
}

impl Neighbours {
    /// Initialize neighbours module with an explicit router state reference.
    /// Opens the database tree and stores it in the RouterState.
    pub fn init_with_state(state: &crate::QaulState, router: &super::RouterState) {
        let db = DataBase::get_node_db(state);
        let tree = db.open_tree("nodes").unwrap();
        router.neighbours.set_nodes_db(tree);
    }

    /// update table with a new value
    ///
    /// If the node already exists, it updates it's rtt value.
    /// If the node does not yet exist, it creates it.
    pub fn update_node(router: &super::RouterState, module: ConnectionModule, node_id: PeerId, rtt: u32) {
        log::trace!("update_node node {:?}", node_id);
        let ns = &router.neighbours;
        let table_lock = match ns.get_table(&module) {
            Some(t) => t,
            None => return,
        };

        let mut neighbours = table_lock.write().unwrap();
        let node_option = neighbours.nodes.get_mut(&node_id);
        if let Some(node) = node_option {
            node.rtt = Self::calculate_rtt(node.rtt, rtt);
            node.updated_at = Timestamp::get_timestamp();
        } else {
            if neighbours.nodes.len() >= 100_000 {
                log::warn!(
                    "neighbours table has reached {} entries; possible resource exhaustion",
                    neighbours.nodes.len()
                );
            }
            log::trace!("add node {:?} to neighbours table", node_id);
            neighbours.nodes.insert(
                node_id,
                Neighbour {
                    rtt,
                    updated_at: Timestamp::get_timestamp(),
                },
            );

            // add neighbour in RouterInfo neighbours table
            RouterInfo::add_neighbour(router, node_id);

            // add node to nodes database
            let db = ns.nodes_db.read().unwrap();
            if let Some(tree) = db.as_ref() {
                let id = node_id.to_bytes();
                let node = Node {
                    id: id.clone(),
                    connected_at: Timestamp::get_timestamp(),
                };
                let node_bytes = bincode::serialize(&node).unwrap();
                if let Err(e) = tree.insert(id.as_slice(), node_bytes) {
                    log::error!("Error saving node to data base: {}", e);
                } else if let Err(e) = tree.flush() {
                    log::error!("Error when flushing data base to disk: {}", e);
                }
            }
        }
    }

    /// Delete Neighbour
    /// Delegates to the provided RouterState instance.
    pub fn delete(router: &super::RouterState, module: ConnectionModule, node_id: PeerId) {
        router.neighbours.delete(module, node_id);
    }

    /// Calculate average rtt using Exponentially Weighted Moving Average (EWMA)
    /// α = 1/8 (standard TCP and libp2p Kademlia smoothing factor)
    pub(crate) fn calculate_rtt(old_rtt: u32, new_rtt: u32) -> u32 {
        // If this is the first ping (old_rtt is 0), we just take the new value.
        if old_rtt == 0 {
            return new_rtt;
        }

        // We use an alpha of 1/8 to smooth out fluctuations:
        // Estimated RTT = (7/8) * old_rtt + (1/8) * new_rtt

        // Prevent overflow by doing the math in u64, then casting back
        let smoothed_rtt = (old_rtt as u64 * 7 + new_rtt as u64) / 8;

        smoothed_rtt as u32
    }

    /// Search for a neighbour by it's small qaul ID
    #[allow(dead_code)]
    #[deprecated(since = "2.0.0-rc.4", note = "Use `node_from_q8id` instead.")]
    pub fn node_from_small_id(router: &super::RouterState, small_id: Vec<u8>) -> Option<Node> {
        #[allow(deprecated)]
        let prefix = QaulId::small_to_search_prefix(small_id);
        let db = router.neighbours.nodes_db.read().unwrap();
        let tree = db.as_ref()?;
        let mut result = tree.scan_prefix(prefix);
        if let Some(Ok((_key, node_bytes))) = result.next() {
            let node: Node = bincode::deserialize(&node_bytes).unwrap();
            return Some(node);
        }
        None
    }

    /// Search for a neighbour by it's q8id (8 Byte qaul ID)
    pub fn node_from_q8id(router: &super::RouterState, q8id: Vec<u8>) -> Option<Node> {
        let prefix = QaulId::q8id_to_search_prefix(q8id);
        let db = router.neighbours.nodes_db.read().unwrap();
        let tree = db.as_ref()?;
        let mut result = tree.scan_prefix(prefix);
        if let Some(Ok((_key, node_bytes))) = result.next() {
            let node: Node = bincode::deserialize(&node_bytes).unwrap();
            return Some(node);
        }
        None
    }

    /// send protobuf RPC neighbours list
    pub fn rpc_send_neighbours_list(state: &crate::QaulState, router: &super::RouterState, request_id: String) {
        let ns = &router.neighbours;

        let lan_neighbours = {
            let lan = ns.lan.read().unwrap();
            lan.nodes
                .iter()
                .map(|(id, value)| proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                })
                .collect::<Vec<_>>()
        };

        let internet_neighbours = {
            let internet = ns.internet.read().unwrap();
            internet
                .nodes
                .iter()
                .map(|(id, value)| proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                })
                .collect::<Vec<_>>()
        };

        let ble_neighbours = {
            let ble = ns.ble.read().unwrap();
            ble.nodes
                .iter()
                .map(|(id, value)| proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                })
                .collect::<Vec<_>>()
        };

        let proto_message = proto::Router {
            message: Some(proto::router::Message::NeighboursList(
                proto::NeighboursList {
                    lan: lan_neighbours,
                    internet: internet_neighbours,
                    ble: ble_neighbours,
                },
            )),
        };

        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        Rpc::send_message(
            state,
            buf,
            crate::rpc::proto::Modules::Router.into(),
            request_id,
            Vec::new(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rtt() {
        // First ping: old_rtt is 0 -> should take the full new_rtt
        assert_eq!(Neighbours::calculate_rtt(0, 100), 100);

        // Smoothing typical scenario
        // old: 100, new: 180 -> (700 + 180) / 8 = 110
        assert_eq!(Neighbours::calculate_rtt(100, 180), 110);

        // Slow latency drop
        // old: 110, new: 30 -> (770 + 30) / 8 = 100
        assert_eq!(Neighbours::calculate_rtt(110, 30), 100);

        // Extreme spike resilience
        // old: 100, new: 1000 -> (700 + 1000) / 8 = 212
        assert_eq!(Neighbours::calculate_rtt(100, 1000), 212);

        // Steady state
        assert_eq!(Neighbours::calculate_rtt(100, 100), 100);
    }
}
