// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Table of all direct neighbour nodes
//!
//! There is a table per connection module.

use libp2p::PeerId;
use prost::Message;
use serde::{Deserialize, Serialize};
use sled_extensions::{bincode::Tree, DbExt};
use state::Storage;
use std::{collections::HashMap, sync::RwLock, time::SystemTime};

use super::info::RouterInfo;
use super::proto;
use crate::connections::ConnectionModule;
use crate::rpc::Rpc;
use crate::storage::database::DataBase;
use crate::utilities::{qaul_id::QaulId, timestamp::Timestamp};

/// mutable state of Internet neighbour node table
static INTERNET: Storage<RwLock<Neighbours>> = Storage::new();
/// mutable state of LAN neighbour node table
static LAN: Storage<RwLock<Neighbours>> = Storage::new();
/// mutable state of BLE neighbour node table
static BLE: Storage<RwLock<Neighbours>> = Storage::new();

/// Data base table of all ever discovered neighbour nodes
///
/// This table is used to find the node id from the small id
/// used by the BLE module.
static NODES: Storage<Tree<Node>> = Storage::new();

/// Node entry in the data base
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// node id
    pub id: Vec<u8>,
    /// last connect
    pub connected_at: u64,
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
    updated_at: SystemTime,
}

impl Neighbours {
    /// Initialize neighbours module
    pub fn init() {
        // neighbours table for internet connection module
        let internet = Neighbours {
            nodes: HashMap::new(),
        };
        INTERNET.set(RwLock::new(internet));

        // neighbours table for lan connection module
        let lan = Neighbours {
            nodes: HashMap::new(),
        };
        LAN.set(RwLock::new(lan));

        // neighbours table for ble connection module
        let ble = Neighbours {
            nodes: HashMap::new(),
        };
        BLE.set(RwLock::new(ble));

        // get nodes tree from data base and set it to state
        let db = DataBase::get_node_db();
        let tree = db.open_bincode_tree("nodes").unwrap();
        NODES.set(tree);
    }

    /// update table with a new value
    ///
    /// If the node already exists, it updates it's rtt value.
    /// If the node does not yet exist, it creates it.
    pub fn update_node(module: ConnectionModule, node_id: PeerId, rtt: u32) {
        log::info!("update_node node {:?}", node_id);
        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::Ble => neighbours = BLE.get().write().unwrap(),
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }

        // get node from table
        let node_option = neighbours.nodes.get_mut(&node_id);
        if let Some(node) = node_option {
            node.rtt = Self::calculate_rtt(node.rtt, rtt);
            node.updated_at = SystemTime::now();
        } else {
            log::info!("add node {:?} to neighbours table", node_id);
            neighbours.nodes.insert(
                node_id,
                Neighbour {
                    rtt,
                    updated_at: SystemTime::now(),
                },
            );

            // add neighbour in RouterInfo neighbours table
            RouterInfo::add_neighbour(node_id);

            // add node to nodes table
            {
                let tree = NODES.get();
                let id = node_id.to_bytes();

                // create data base entry
                let node = Node {
                    id: id.clone(),
                    connected_at: Timestamp::get_timestamp(),
                };

                // save user
                if let Err(e) = tree.insert(id.as_slice(), node) {
                    log::error!("Error saving node to data base: {}", e);
                } else {
                    if let Err(e) = tree.flush() {
                        log::error!("Error when flushing data base to disk: {}", e);
                    }
                }
            }
        }
    }

    /// Delete Neighbour
    pub fn delete(module: ConnectionModule, node_id: PeerId) {
        // get table
        let mut neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().write().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().write().unwrap(),
            ConnectionModule::Ble => neighbours = BLE.get().write().unwrap(),
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }

        // delete entry
        neighbours.nodes.remove(&node_id);
    }

    /// Calculate average rtt
    fn calculate_rtt(_old_rtt: u32, new_rtt: u32) -> u32 {
        // DISCUSSION: how to value history and flatten the curve
        //             between different results.
        //             Other possibilities: create small ring buffer with last results
        //(old_rtt * 3 + new_rtt) / 4
        new_rtt
    }

    /// get rtt for a neighbour
    /// returns the round trip time for the neighbour in the
    /// connection module.
    /// If the neighbour does not exist, it returns None.
    pub fn get_rtt(neighbour_id: &PeerId, module: &ConnectionModule) -> Option<u32> {
        // get table
        let neighbours;
        match module {
            ConnectionModule::Lan => neighbours = LAN.get().read().unwrap(),
            ConnectionModule::Internet => neighbours = INTERNET.get().read().unwrap(),
            ConnectionModule::Ble => neighbours = BLE.get().read().unwrap(),
            ConnectionModule::Local => return Some(0),
            ConnectionModule::None => return None,
        }

        // search for neighbour
        if let Some(neighbour) = neighbours.nodes.get(neighbour_id) {
            return Some(neighbour.rtt);
        } else {
            return None;
        }
    }

    /// Is this node ID a neighbour in any module?
    /// returns the first found module or `None`
    pub fn is_neighbour(node_id: &PeerId) -> ConnectionModule {
        // check if neighbour is in Lan table
        {
            let lan = LAN.get().read().unwrap();
            if lan.nodes.contains_key(node_id) {
                return ConnectionModule::Lan;
            }
        }
        // check if neighbour exists in Internet table
        {
            let internet = INTERNET.get().read().unwrap();
            if internet.nodes.contains_key(node_id) {
                return ConnectionModule::Internet;
            }
        }
        // check if neighbour exists in BLE table
        {
            let ble = BLE.get().read().unwrap();
            if ble.nodes.contains_key(node_id) {
                return ConnectionModule::Ble;
            }
        }

        ConnectionModule::None
    }

    /// Search for a neighbour by it's small qaul ID
    ///
    /// Returns node if it exists in the data base,
    /// otherwise it returns None.
    pub fn node_from_small_id(small_id: Vec<u8>) -> Option<Node> {
        // create search key
        let prefix = QaulId::small_to_search_prefix(small_id);

        // get data base tree
        let tree = NODES.get();

        // search for key
        let mut result = tree.scan_prefix(prefix);

        if let Some(Ok((_key, node))) = result.next() {
            return Some(node);
        }

        None
    }

    /// Get a list of all neighbours that are only connected via BLE module
    ///
    /// This function is used to decide to which nodes we need to send the
    /// flooding information.
    pub fn get_ble_only_nodes() -> Vec<PeerId> {
        let mut nodes: Vec<PeerId> = Vec::new();

        // get state
        let ble = BLE.get().read().unwrap();

        // check if we have nodes listed
        if ble.nodes.len() > 0 {
            let lan = LAN.get().read().unwrap();
            let internet = INTERNET.get().read().unwrap();

            // search for all nodes that are only reachable via BLE
            for (id, _val) in ble.nodes.iter() {
                // check if it exists in LAN
                if lan.nodes.contains_key(id) {
                    continue;
                }

                // check if it exists in Internet
                if internet.nodes.contains_key(id) {
                    continue;
                }

                // if not found, add it to the nodes list
                nodes.push(id.to_owned());
            }
        }

        // return list of neighbour nodes
        nodes
    }

    /// send protobuf RPC neighbours list
    pub fn rpc_send_neighbours_list() {
        // create lists per module
        let mut lan_neighbours: Vec<proto::NeighboursEntry> = Vec::new();
        let mut internet_neighbours: Vec<proto::NeighboursEntry> = Vec::new();
        let mut ble_neighbours: Vec<proto::NeighboursEntry> = Vec::new();

        // fill lan connection module neighbours
        {
            let lan = LAN.get().read().unwrap();

            for (id, value) in &lan.nodes {
                lan_neighbours.push(proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                });
            }
        }

        // fill internet connection module neighbours
        {
            let internet = INTERNET.get().write().unwrap();

            for (id, value) in &internet.nodes {
                internet_neighbours.push(proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                });
            }
        }

        // fill ble connection module neighbours
        {
            let ble = BLE.get().write().unwrap();

            for (id, value) in &ble.nodes {
                ble_neighbours.push(proto::NeighboursEntry {
                    node_id: id.to_bytes(),
                    rtt: value.rtt,
                });
            }
        }

        // create neighbours list message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::NeighboursList(
                proto::NeighboursList {
                    lan: lan_neighbours,
                    internet: internet_neighbours,
                    ble: ble_neighbours,
                },
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(
            buf,
            crate::rpc::proto::Modules::Router.into(),
            "".to_string(),
            Vec::new(),
        );
    }
}
