// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
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
use state::Storage;
use prost::Message;
use std::sync::RwLock;
use std::collections::HashMap;

use crate::connections::ConnectionModule;
use super::proto;
use crate::rpc::Rpc;
use crate::router::router_net_proto;

/// mutable state of table
static ROUTINGTABLE: Storage<RwLock<RoutingTable>> = Storage::new();

/// table entry per user
#[derive(Debug, Clone)]
pub struct RoutingUserEntry {
    /// user id
    pub id: PeerId,
    /// propagation id
    pub pgid: u32,
    /// propagation id update time
    pub pgid_update: u64,
    /// shortest hop count for user within this propagation id
    pub pgid_update_hc: u8,
    //online time    
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
    pub table: HashMap<PeerId, RoutingUserEntry>
}

impl RoutingTable {
    /// Initialize routing table
    /// Creates global routing table and saves it to state.
    pub fn init() {
        // create global routing table and save it to state
        let table = RoutingTable { table: HashMap::new() };
        ROUTINGTABLE.set(RwLock::new(table));
    }

    /// set and replace routing table with a new table
    pub fn set(new_table: RoutingTable) {
        let mut table = ROUTINGTABLE.get().write().unwrap();
        table.table = new_table.table;
    }

    /// Create routing information for a specific neighbour node,
    /// to be sent to this neighbour node.
    pub fn create_routing_info( neighbour: PeerId, last_sent: u64 ) -> router_net_proto::RoutingInfoTable {
        let mut table = router_net_proto::RoutingInfoTable {
            entry: Vec::new()
        };        

        // get access to routing table
        let routing_table = ROUTINGTABLE.get().read().unwrap();

        // loop through routing table
        for (user_id, user) in routing_table.table.iter() {
            if user.connections.len() == 0 {
                continue;
            }

            // choose best link quality
            let mut min_conn = user.connections[0].clone();
            for i in 0..user.connections.len(){
                if user.connections[i].lq < min_conn.lq{
                    min_conn = user.connections[i].clone();
                }
            }

            if neighbour != min_conn.node && min_conn.last_update >= last_sent{
                let mut hc = Vec::new();
                hc.push(min_conn.hc);

                let table_entry = router_net_proto::RoutingInfoEntry {
                    user: user_id.to_bytes(),
                    rtt: min_conn.rtt,
                    hc,
                    pgid: user.pgid,
                };
                table.entry.push(table_entry);
            }
        }

        table
    }

    /// Create routing information for a specific neighbour node,
    /// to be sent to this neighbour node.
    pub fn get_online_user_ids(last_sent: u64 ) -> Vec<PeerId> {

        let mut user_ids: Vec<PeerId> = vec![];

        // get access to routing table
        let routing_table = ROUTINGTABLE.get().read().unwrap();

        // loop through routing table
        for (user_id, user) in routing_table.table.iter() {
            if user.online_time >= last_sent {
                user_ids.push(user_id.clone());
            }
        }
        user_ids
    }

    /// send protobuf RPC neighbours list
    pub fn rpc_send_routing_table() {
        // create list
        let mut table_list: Vec<proto::RoutingTableEntry> = Vec::new();

        // get routing table state
        let routing_table = ROUTINGTABLE.get().read().unwrap();

        // loop through all user table entries
        for (id, entry) in &routing_table.table {
            let mut table_entry = proto::RoutingTableEntry {
                user_id: id.to_bytes(),
                connections: Vec::new(),
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
                table_entry.connections.push(
                    proto::RoutingTableConnection {
                        module,
                        rtt: connection.rtt,
                        hop_count: connection.hc as u32,
                        via: connection.node.to_bytes(),
                    }
                );
            }

            // add user entry to table list
            table_list.push(table_entry);
        }

        // create table list message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::RoutingTable(
                proto::RoutingTableList {
                    routing_table: table_list,
                }
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, crate::rpc::proto::Modules::Router.into(), "".to_string(), Vec::new());
    }

    /// Get the routing connection entry for a specific user
    /// 
    /// The connection entry for the provided user_id contains
    /// the neighbour id as well as the connection module via
    /// which to send the packages.
    /// 
    /// It selects the best route according to the rank_routing_connection function.
    /// 
    pub fn get_route_to_user(user_id: PeerId) -> Option<RoutingConnectionEntry> {
        // get routing table state
        let routing_table = ROUTINGTABLE.get().read().unwrap();

        // find user
        if let Some(user_entry) = routing_table.table.get(&user_id) {
            let mut compare: Option<&RoutingConnectionEntry> = None;

            //log::error!("found user entry connections = {}", user_entry.connections.len());

            // find best route
            for connection in &user_entry.connections {
                match compare {
                    Some(current) => {
                        if Self::compare_connections(current, connection) {
                            compare = Some(connection);
                        }
                    },
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
    fn compare_connections(current: &RoutingConnectionEntry, new: &RoutingConnectionEntry) -> bool {
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
