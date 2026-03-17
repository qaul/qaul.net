// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Global Routing Table
//!
//! This file contains the global routing table
//!
//! * contains all currently reachable users.
//! * There is an entry for each user over which connection modules
//!   it can be reached. Each connection module only contains
//!   information of the best node.

use libp2p::PeerId;
use prost::Message;
use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;

use super::proto;
use crate::connections::ConnectionModule;
use crate::router::router_net_proto;
use crate::rpc::Rpc;
use crate::utilities::qaul_id::QaulId;

/// table entry per user
#[derive(Debug, Clone)]
pub struct RoutingUserEntry {
    /// user q8id, 8 Byte qaul user id
    #[allow(dead_code)]
    pub id: Vec<u8>,
    /// propagation id
    pub pgid: u32,
    /// propagation id update time
    pub pgid_update: u64,
    /// shortest hop count for user within this propagation id
    #[allow(dead_code)]
    pub pgid_update_hc: u8,
    //online time
    #[allow(dead_code)]
    pub online_time: u64,
    /// best routing entry per connection module
    pub connections: Vec<RoutingConnectionEntry>,
}

/// connection entry per connection module
#[derive(Debug, Clone)]
pub struct RoutingConnectionEntry {
    /// connections module
    pub module: ConnectionModule,
    /// node id
    /// via which the user can be reached
    pub node: PeerId,
    /// round trip time
    /// addition of all round trip times for all hops
    pub rtt: u32,
    /// hop count
    /// how many hops has the connection
    pub hc: u8,
    /// link quality
    pub lq: u32,
    /// last_update
    pub last_update: u64,
}

/// Global Routing Table Implementation
///
/// This is the table to turn to when checking where to send
/// a package.
pub struct RoutingTable {
    /// routing table key is a users q8id
    pub table: HashMap<Vec<u8>, RoutingUserEntry>,
}

/// Instance-based routing table state.
/// Replaces the global ROUTINGTABLE static for multi-instance use.
pub struct RoutingTableState {
    pub inner: RwLock<RoutingTable>,
}

impl RoutingTableState {
    /// Create a new empty routing table state.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(RoutingTable {
                table: HashMap::new(),
            }),
        }
    }

    /// Replace the routing table with a new one.
    pub fn set(&self, new_table: RoutingTable) {
        let mut table = self.inner.write().unwrap();
        table.table = new_table.table;
    }

    /// Create routing information for a specific neighbour node.
    pub fn create_routing_info(
        &self,
        neighbour: PeerId,
        last_sent: u64,
    ) -> router_net_proto::RoutingInfoTable {
        let routing_table = self.inner.read().unwrap();
        let mut table = router_net_proto::RoutingInfoTable {
            entry: Vec::with_capacity(routing_table.table.len()),
        };

        for (user_id, user) in routing_table.table.iter() {
            if user.connections.is_empty() {
                continue;
            }

            let min_conn = user
                .connections
                .iter()
                .min_by_key(|connection| connection.lq)
                .unwrap();

            if neighbour != min_conn.node && (min_conn.last_update >= last_sent || min_conn.hc == 0)
            {
                let table_entry = router_net_proto::RoutingInfoEntry {
                    user: user_id.clone(),
                    rtt: min_conn.rtt,
                    hc: vec![min_conn.hc],
                    pgid: user.pgid,
                };
                table.entry.push(table_entry);
            }
        }

        table
    }

    /// Get online users and hop count.
    pub fn get_online_users(&self) -> BTreeMap<Vec<u8>, u8> {
        let mut user_ids: BTreeMap<Vec<u8>, u8> = BTreeMap::new();
        let routing_table = self.inner.read().unwrap();

        for (user_id, user) in routing_table.table.iter() {
            if !user.connections.is_empty() {
                user_ids.insert(user_id.clone(), user.connections[0].hc);
            }
        }
        user_ids
    }

    /// Get the routing connection entry for a specific user.
    pub fn get_route_to_user(&self, user_id: PeerId) -> Option<RoutingConnectionEntry> {
        let routing_table = self.inner.read().unwrap();
        let user_q8id = QaulId::to_q8id(user_id);

        if let Some(user_entry) = routing_table.table.get(&user_q8id) {
            let mut compare: Option<&RoutingConnectionEntry> = None;

            for connection in &user_entry.connections {
                match compare {
                    Some(current) => {
                        if RoutingTable::compare_connections(current, connection) {
                            compare = Some(connection);
                        }
                    }
                    None => compare = Some(connection),
                }
            }

            match compare {
                None => return None,
                Some(connection) => return Some(connection.to_owned()),
            }
        }
        None
    }
}

impl RoutingTable {
    /// set and replace routing table with a new table
    /// Delegates to the provided RouterState instance.
    pub fn set(router: &super::RouterState, new_table: RoutingTable) {
        router.routing_table.set(new_table);
    }

    /// Create routing information for a specific neighbour node.
    /// Delegates to the provided RouterState instance.
    pub fn create_routing_info(
        router: &super::RouterState,
        neighbour: PeerId,
        last_sent: u64,
    ) -> router_net_proto::RoutingInfoTable {
        router
            .routing_table
            .create_routing_info(neighbour, last_sent)
    }

    /// get online users and hop count
    /// Delegates to the provided RouterState instance.
    pub fn get_online_users(router: &super::RouterState) -> BTreeMap<Vec<u8>, u8> {
        router.routing_table.get_online_users()
    }

    /// get online users info
    pub fn get_online_users_info(router: &super::RouterState) -> BTreeMap<Vec<u8>, Vec<RoutingConnectionEntry>> {
        let routing_table = router
            .routing_table
            .inner
            .read()
            .unwrap();
        let mut users: BTreeMap<Vec<u8>, Vec<RoutingConnectionEntry>> = BTreeMap::new();
        for (user_id, user) in routing_table.table.iter() {
            if !user.connections.is_empty() {
                users.insert(user_id.clone(), user.connections.clone());
            }
        }
        users
    }

    /// send protobuf RPC routing table list
    pub fn rpc_send_routing_table(state: &crate::QaulState, router: &super::RouterState, request_id: String) {
        let routing_table = router
            .routing_table
            .inner
            .read()
            .unwrap();
        let mut table_list = Vec::with_capacity(routing_table.table.len());

        // loop through all user table entries
        for (id, entry) in &routing_table.table {
            let mut table_entry = proto::RoutingTableEntry {
                user_id: id.clone(),
                connections: Vec::with_capacity(entry.connections.len()),
            };

            // loop through all connection entries in a user entry
            for connection in &entry.connections {
                // check module
                let module: i32;
                match connection.module {
                    ConnectionModule::Lan => module = proto::ConnectionModule::Lan as i32,
                    ConnectionModule::Internet => module = proto::ConnectionModule::Internet as i32,
                    ConnectionModule::Ble => module = proto::ConnectionModule::Ble as i32,
                    ConnectionModule::Local => module = proto::ConnectionModule::Local as i32,
                    _ => module = proto::ConnectionModule::None as i32,
                }

                // create entry
                table_entry.connections.push(proto::RoutingTableConnection {
                    module,
                    rtt: connection.rtt,
                    hop_count: connection.hc as u32,
                    via: connection.node.to_bytes(),
                });
            }

            // add user entry to table list
            table_list.push(table_entry);
        }

        // create table list message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::RoutingTable(
                proto::RoutingTableList {
                    routing_table: table_list,
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
            state,
            buf,
            crate::rpc::proto::Modules::Router.into(),
            request_id,
            Vec::new(),
        );
    }

    /// Get the routing connection entry for a specific user
    ///
    /// The connection entry for the provided user_id contains
    /// the neighbour id as well as the connection module via
    /// which to send the packages.
    ///
    /// It selects the best route according to the rank_routing_connection function.
    ///
    pub fn get_route_to_user(router: &super::RouterState, user_id: PeerId) -> Option<RoutingConnectionEntry> {
        let routing_table = router
            .routing_table
            .inner
            .read()
            .unwrap();

        // get q8id for qaul user
        let user_q8id = QaulId::to_q8id(user_id);

        // find user
        if let Some(user_entry) = routing_table.table.get(&user_q8id) {
            let mut compare: Option<&RoutingConnectionEntry> = None;

            // find best route
            for connection in &user_entry.connections {
                match compare {
                    Some(current) => {
                        if Self::compare_connections(current, connection) {
                            compare = Some(connection);
                        }
                    }
                    None => compare = Some(connection),
                }
            }

            // return route
            match compare {
                None => return None,
                Some(connection) => return Some(connection.to_owned()),
            }
        }
        None
    }

    /// Compare two routing connections and decides which one is better
    ///
    /// This function decides which connection to favour based on the
    /// rank_routing_connection function
    ///
    /// Return values:
    ///
    /// * returns true, when the new connection is better
    /// * returns false, when the current connection is better
    ///
    pub(crate) fn compare_connections(
        current: &RoutingConnectionEntry,
        new: &RoutingConnectionEntry,
    ) -> bool {
        let current_value = Self::rank_routing_connection(current);
        let new_value = Self::rank_routing_connection(new);

        if current_value < new_value {
            return true;
        }

        false
    }

    /// give a ranking to the routing connection
    ///
    /// This function decides which connection to favour based on the following qualities:
    ///
    /// * Hierarchy of connection modules in the following order:
    ///   Local, LAN, Internet, BLE, None
    ///
    fn rank_routing_connection(connection: &RoutingConnectionEntry) -> u8 {
        match connection.module {
            ConnectionModule::None => return 0,
            ConnectionModule::Ble => return 1,
            ConnectionModule::Internet => return 2,
            ConnectionModule::Lan => return 3,
            ConnectionModule::Local => return 4,
        }
    }
}
