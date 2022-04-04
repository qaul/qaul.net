// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Connection Module Connectivity Collection Table
//! 
//! This file contains the collected connectivity information
//! per connection module
//! 
//! * each connection module has it's own routing table containing
//!   all currently reachable users via this module.
//! * Each user has an entry for each node over which it can be reached.
//! * Out of this information the global table is constructed,
//!   containing only the best entry.

use libp2p::PeerId;
use state::Storage;
use prost::Message;
use std::sync::RwLock;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

use crate::connections::ConnectionModule;
use crate::node;
use crate::router::{
    table::{RoutingTable, RoutingUserEntry, RoutingConnectionEntry},
    neighbours::Neighbours,
};
use super::proto;
use crate::router::router_net_proto;
use crate::rpc::Rpc;


/// Mutable module state
/// Tables with all stats for each connection module
static LOCAL: Storage<RwLock<RoutingTable>> = Storage::new();
static INTERNET: Storage<RwLock<ConnectionTable>> = Storage::new();
static LAN: Storage<RwLock<ConnectionTable>> = Storage::new();


/// Connection entry for UserEntry
struct NeighbourEntry {
    /// node id of the neighbour
    id: PeerId,
    /// round trip time
    rtt: u32,
    /// hop count
    hc: u8,
    /// package loss
    pl: f32,
    /// time when the node was last updated
    last_update: SystemTime,
}

/// user entry for ConnectionTable
/// containing a BTreeMap of all neighbours over
/// which this user can be reached
struct UserEntry {
    /// user id
    id: PeerId,
    /// connection entries
    connections: BTreeMap<PeerId, NeighbourEntry>,
}

/// Connection table contains a hashed map of all reachable users.
/// The connectivity updates from each neighbour are collected here.
/// Out of this information the global routing table is generated.
pub struct ConnectionTable {
    table: HashMap<PeerId, UserEntry>,
}

impl ConnectionTable {
    /// Initialize connection tables
    /// Creates a table for each ConnectionModule
    /// and saves it to state.
    pub fn init() {
        {
            let internet = ConnectionTable { table: HashMap::new() };
            INTERNET.set(RwLock::new(internet));

            let lan = ConnectionTable { table: HashMap::new() };
            LAN.set(RwLock::new(lan));

            let local = RoutingTable { table: HashMap::new() };
            LOCAL.set(RwLock::new(local));
        }

        // create filled state for locally registered users
        {
            for user in node::user_accounts::UserAccounts::get_user_info() {
                Self::add_local_user(user.id);
            }
        }
    }

    /// add a new local user to state
    pub fn add_local_user(user_id: PeerId) {
        let node_id = node::Node::get_id();
        let mut routing_table = LOCAL.get().write().unwrap();

        let mut connections = Vec::new();
        connections.push(RoutingConnectionEntry {
            module: ConnectionModule::Local,
            node: node_id,
            rtt: 0,
            hc: 0,
            pl: 0.0,
        });

        let routing_user_entry = RoutingUserEntry {
            id: user_id.to_owned(),
            connections: connections,
        };

        routing_table.table.insert(user_id, routing_user_entry);
    }

    /// process received routing info table
    /// enter it into all modules where we are connected to
    pub fn process_received_routing_info( neighbour_id: PeerId, info: Vec<router_net_proto::RoutingInfoEntry> ) {

        for inf in &info{
            let c: &[u8] = &inf.user;
            let userid = PeerId::from_bytes(c).unwrap();
            log::info!("qual process_received_routing_info user={}, hc={}", userid, inf.hc[0]);
        }
        

        // try Lan module
        if let Some(rtt) = Neighbours::get_rtt(&neighbour_id , &ConnectionModule::Lan ){
            Self::fill_received_routing_info(ConnectionModule::Lan, neighbour_id, rtt, info.clone());
        }

        // try Internet module
        if let Some(rtt) = Neighbours::get_rtt(&neighbour_id , &ConnectionModule::Internet ){
            Self::fill_received_routing_info(ConnectionModule::Internet, neighbour_id, rtt, info.clone());
        }
    }

    /// populate connection table with incoming routing information
    fn fill_received_routing_info( conn: ConnectionModule, neighbour_id: PeerId, rtt: u32, info: Vec<router_net_proto::RoutingInfoEntry> ) {
        // loop through results and enter them to the table
        for entry in info {
            if let Ok(user_id) = PeerId::from_bytes(&entry.user){
                // calculate hop count
                // if hop count is > 255, return
                let hc;
                if entry.hc[0] < 255 {
                    hc = entry.hc[0] +1;
                }
                else {
                    return;
                }

                // fill structure
                let neighbour = NeighbourEntry {
                    id: neighbour_id,
                    rtt: entry.rtt +rtt,
                    hc,
                    pl: entry.pl,
                    last_update: SystemTime::now(),
                };

                // add it to state
                Self::add_connection(user_id, neighbour, conn.clone());
            }
        }
    }

    /// add connection to local state
    fn add_connection(user_id: PeerId, connection: NeighbourEntry, module: ConnectionModule) {
        // get access to the connection table
        let mut connection_table;
        match module {
            ConnectionModule::Internet => connection_table = INTERNET.get().write().unwrap(),
            ConnectionModule::Lan => connection_table = LAN.get().write().unwrap(),
            ConnectionModule::Ble => return,
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }        

        // check if user already exists
        if let Some(user) = connection_table.table.get_mut(&user_id) {
            user.connections.insert(connection.id, connection);
        } else {
            let mut connections_map = BTreeMap::new();
            connections_map.insert(connection.id, connection);

            let user = UserEntry { 
                id: user_id,
                connections: connections_map,
            };

            connection_table.table.insert(user_id, user);
        }
    }

    /// Create a routing table and set it to active routing table
    pub fn create_routing_table() {
        // create a new table
        let mut table = RoutingTable { table: HashMap::new() };

        // set static routes for local users
        // create them first, for that they are always routed to ourselves
        {
            table = Self::local_routes_to_intermediary_table(table);
        }

        // calculate from lan module
        table = Self::calculate_intermediary_table(table, ConnectionModule::Lan);

        // calculate from internet module
        table = Self::calculate_intermediary_table(table, ConnectionModule::Internet);

        // set table as new active routing table
        RoutingTable::set(table);
    }

    /// insert local routes into routing table
    fn local_routes_to_intermediary_table(mut table: RoutingTable) -> RoutingTable {
        // get local routes
        let local = LOCAL.get().read().unwrap();

        // fill it into routing table
        for (user_id, user) in &local.table {
            table.table.insert(user_id.to_owned(), user.to_owned());
        }

        table
    }

    /// calculate a routing table for a module
    fn calculate_intermediary_table(mut table: RoutingTable, conn: ConnectionModule) -> RoutingTable {
        // create vector for users to remove
        let mut expired_users: Vec<PeerId> = Vec::new();
        
        // get connections table
        let mut connection_table;
        match conn.clone() {
            ConnectionModule::Internet => connection_table = INTERNET.get().write().unwrap(),
            ConnectionModule::Lan => connection_table = LAN.get().write().unwrap(),
            ConnectionModule::Ble => return table,
            ConnectionModule::Local => return table,
            ConnectionModule::None => return table
        }

        // iterate over connection table
        for (user_id,user) in connection_table.table.iter_mut() {
            // find best entry
            if let Some(connection) = Self::find_best_connection(user) {
                // fill entry into routing table
                let routing_connection_entry = RoutingConnectionEntry {
                    module: conn.clone(),
                    node: connection.id,
                    rtt: connection.rtt,
                    hc: connection.hc,
                    pl: connection.pl,
                };

                // check if user entry already exists hashmap
                if let Some(routing_user_entry) = table.table.get_mut(&user.id) {
                    routing_user_entry.connections.push(routing_connection_entry);
                } else {
                    let mut connections = Vec::new();
                    connections.push(routing_connection_entry);

                    let routing_user_entry = RoutingUserEntry {
                        id: user_id.to_owned(),
                        connections: connections,
                    };
                    table.table.insert(user_id.to_owned(), routing_user_entry);
                }
            } else {
                // put user for removal as there is no connection in it
                expired_users.push(user_id.clone());
            }
        }

        // remove expired users
        for user_id in expired_users {
            connection_table.table.remove(&user_id);
        }
        
        table
    }

    /// find best entry
    /// and remove all old entries
    fn find_best_connection(user: &mut UserEntry) -> Option<NeighbourEntry> {
        // initialize helper variables
        let mut expired_connections: Vec<PeerId> = Vec::new();
        let mut return_entry = None;
        let mut rtt = u32::MAX;

        // create return value
        {
            let mut entry_found = None;

            // loop through all connections
            for (key, value) in &user.connections {
                let mut expired = true;

                // check if entry is expired
                if let Ok(duration) = value.last_update.elapsed() {
                    if duration < Duration::new(20, 0) {
                        expired = false;

                        if value.rtt < rtt {
                            rtt = value.rtt;
                            entry_found = Some(value);
                        }
                    }
                }

                // put connection for removal if expired
                if expired {
                    expired_connections.push(key.clone());
                }
            }

            if let Some(entry) = entry_found {
                return_entry = Some(NeighbourEntry {
                    id: entry.id.clone(),
                    rtt: entry.rtt,
                    hc: entry.hc,
                    pl: entry.pl,
                    last_update: entry.last_update.clone(),
                })
            }
        }

        // remove expired connections
        for node_id in expired_connections {
            user.connections.remove(&node_id);
        }

        return_entry
    }

    /// send protobuf RPC connections list
    pub fn rpc_send_connections_list() {
        // create connections list
        let connections_list = proto::ConnectionsList {
            lan: Self::rpc_create_connection_module_list(ConnectionModule::Lan),
            internet: Self::rpc_create_connection_module_list(ConnectionModule::Internet),
            ble: Self::rpc_create_connection_module_list(ConnectionModule::Ble),
            local: Self::rpc_create_connection_module_list(ConnectionModule::Local),
        };

        // create rpc connections list protobuf message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::ConnectionsList(
                connections_list
            )),
        };

        // encode message
        let mut buf = Vec::with_capacity(proto_message.encoded_len());
        proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // send message
        Rpc::send_message(buf, crate::rpc::proto::Modules::Router.into(), "".to_string(), Vec::new() );
    }

    /// create rpc connection module list
    fn rpc_create_connection_module_list( conn: ConnectionModule ) -> Vec<proto::ConnectionsUserEntry> {
        // create entry vector
        let mut connections_list: Vec<proto::ConnectionsUserEntry> = Vec::new();

        // request connection table from state
        let connection_table;
        match conn {
            ConnectionModule::Lan => connection_table = LAN.get().read().unwrap(),
            ConnectionModule::Internet => connection_table = INTERNET.get().read().unwrap(),
            ConnectionModule::Ble => return connections_list,
            ConnectionModule::Local => return connections_list,
            ConnectionModule::None => return connections_list,
        }

        // loop through all table entries per user
        for (id, entry) in &connection_table.table {
            // create user entry
            let mut user_entry = proto::ConnectionsUserEntry {
                user_id: id.to_bytes(),
                connections: Vec::new(),
            };

            // loop through all neighbour entries of a user entry
            for (id, neighbour) in &entry.connections {
                // add connection
                user_entry.connections.push(
                    proto::ConnectionEntry {
                        rtt: neighbour.rtt,
                        hop_count: neighbour.hc as u32,
                        via: id.to_bytes(),
                    }
                );
            }

            // add user entry to list
            connections_list.push(user_entry);
        }

        // return list
        connections_list
    }
}

