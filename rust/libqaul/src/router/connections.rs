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

use crate::connections::ConnectionModule;
use crate::node;
use crate::router::{
    table::{RoutingTable, RoutingUserEntry, RoutingConnectionEntry},
    neighbours::Neighbours,
};
use super::proto;
use crate::router::router_net_proto;
use crate::rpc::Rpc;
use crate::utilities::timestamp::Timestamp;


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
    /// link quality
    lq: u32,
    /// time when the node was last updated
    last_update: u64,
}

/// user entry for ConnectionTable
/// containing a BTreeMap of all neighbours over
/// which this user can be reached
struct UserEntry {
    /// user id
    id: PeerId,
    /// propagation id
    pub pgid: u32,
    /// when was the propagation id last updated
    pub pgid_update: u64,
    /// DEPRECATED: do we really need this?
    pub pgid_update_hc: u8,
    //online time
    pub online_time: u64,
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

        //routing table creating is done every 1 seconds. 
        //by considerate neighbour sending is done before creating routing Table.
        //we set local user online time forward 3 seconds
        let now_ts = Timestamp::get_timestamp()  + 3000;
        connections.push(RoutingConnectionEntry {
            module: ConnectionModule::Local,
            node: node_id,
            rtt: 0,
            hc: 0,
            lq: 0,
            last_update: now_ts
        });

        let routing_user_entry = RoutingUserEntry {
            id: user_id.to_owned(),
            pgid: 1,
            pgid_update: now_ts,
            pgid_update_hc: 1,
            online_time: now_ts,
            connections,
        };
        routing_table.table.insert(user_id, routing_user_entry);
    }

    /// process received routing info table
    /// enter it into all modules where we are connected to
    pub fn process_received_routing_info( neighbour_id: PeerId, info: Vec<router_net_proto::RoutingInfoEntry> ) {

        // log::info!("process_received_routing_info count={}", info.len());
        // for inf in &info{
        //     let c: &[u8] = &inf.user;
        //     let userid = PeerId::from_bytes(c).unwrap();
        //     log::info!("receive_routing_info user={}, hc={}, propg_id={}", userid, inf.hc[0], inf.pgid);
        // }

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
                    lq: Self::calculate_linkquality(entry.rtt +rtt, hc),
                    last_update: Timestamp::get_timestamp(),
                };

                // add it to state
                Self::add_connection(user_id, entry.pgid, neighbour, conn.clone());
            }
        }
    }

    /// calculate link quality
    /// 
    /// returns the calculated link quality for a connection.
    /// 
    /// The link quality is currently calculated using the 
    /// round trip time (rtt) and adding a penalty for each hop
    /// according hop count (hc).
    /// 
    /// The smaller the value is better is the link quality.
    pub fn calculate_linkquality(rtt: u32, hc: u8) -> u32 {
        
        // get the router configuration
        let config = super::Router::get_configuration();

        // calculate link quality
        // `hop_count_penalty` is seconds unit, thus it must be converted micro seconds
        let lq = rtt + (hc as u32 * (config.hop_count_penalty as u32) * 1000_000);

        // return link quality
        lq
    }

    /// add connection to local state
    fn add_connection(user_id: PeerId, pgid: u32, connection: NeighbourEntry, module: ConnectionModule) {
        // get access to the connection table
        let mut connection_table;
        match module {
            ConnectionModule::Internet => connection_table = INTERNET.get().write().unwrap(),
            ConnectionModule::Lan => connection_table = LAN.get().write().unwrap(),
            ConnectionModule::Ble => return,
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }        

        let now_ts = Timestamp::get_timestamp();
        // check if user already exists
        if let Some(user) = connection_table.table.get_mut(&user_id) {
            //check alreay exist and pgid is new
            // if (connection.hc == 1 || pgid > user.pgid) ||
            //  (pgid == user.pgid && connection.hc < user.pgid_update_hc){
            //     user.pgid = pgid;
            //     user.pgid_update = Timestamp::get_timestamp();
            //     user.pgid_update_hc = connection.hc;
            //     user.connections.insert(connection.id, connection);
            // }else if pgid == user.pgid{
            //     if let Some(conn) = user.connections.get_mut(&connection.id){
            //         if connection.lq < conn.lq {
            //            conn.lq = connection.lq;
            //            conn.last_update = Timestamp::get_timestamp();
            //         }
            //     }
            // }
            //log::error!("receive_inode hc={}, propg_id={}", connection.hc, pgid);
            
            if connection.hc == 1 || pgid > user.pgid {                
                user.pgid = pgid;
                user.pgid_update = now_ts;
                user.pgid_update_hc = connection.hc;
                user.connections.remove(&connection.id);
                user.connections.insert(connection.id, connection); 
            }else if pgid == user.pgid{
                //check last update
                if (now_ts - user.pgid_update <= (10 * 1000)) && connection.hc < user.pgid_update_hc {
                    user.pgid_update = now_ts;
                    user.pgid_update_hc = connection.hc;
                    user.connections.remove(&connection.id);
                    user.connections.insert(connection.id, connection);
                }else if let Some(conn) = user.connections.get_mut(&connection.id){
                    if connection.lq < conn.lq {
                        conn.lq = connection.lq;
                        conn.hc = connection.hc;
                        conn.last_update = now_ts;
                        user.connections.remove(&connection.id);
                        user.connections.insert(connection.id, connection);
                    }
                }
            }else if pgid < user.pgid {
                if user.pgid_update_hc == connection.hc {
                    //reboot node case
                    if (user.pgid - pgid) > (connection.hc as u32) { 
                        user.pgid = pgid;
                        user.pgid_update = now_ts;
                        user.pgid_update_hc = connection.hc;
                        user.connections.remove(&connection.id);
                        user.connections.insert(connection.id, connection);        
                    }
                }
            } 

        } else {
            //log::error!("receive_inode_insert hc={}, propg_id={}", connection.hc, pgid);
            let mut connections_map = BTreeMap::new();
            let hc = connection.hc;
            connections_map.insert(connection.id, connection);

            let user = UserEntry { 
                id: user_id,
                pgid: pgid,
                pgid_update: now_ts,
                pgid_update_hc: hc,
                online_time: now_ts,
                connections: connections_map,
            };

            connection_table.table.insert(user_id, user);
        }
    }

    /// update propagation id for local users
    pub fn update_propagation_id(propagation_id: u32){
        //update local user's propagation id
        let mut local = LOCAL.get().write().unwrap();
        for (_user_id, user) in local.table.iter_mut() {
            user.pgid = propagation_id;
            // QUESTION: is this of any use?
            user.pgid_update = Timestamp::get_timestamp();
            user.connections.get_mut(0).unwrap().last_update = Timestamp::get_timestamp();
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

            let (b_expired_pgid, connection_entry) = Self::find_best_connection(user);
            if b_expired_pgid == false {
                if let Some(connection) = connection_entry{
                    // fill entry into routing table
                    let routing_connection_entry = RoutingConnectionEntry {
                        module: conn.clone(),
                        node: connection.id,
                        rtt: connection.rtt,
                        hc: connection.hc,
                        lq: connection.lq,
                        last_update: connection.last_update
                    };

                    // check if user entry already exists hashmap
                    if let Some(routing_user_entry) = table.table.get_mut(&user.id) {
                        routing_user_entry.connections.push(routing_connection_entry);
                    } else {
                        let mut connections = Vec::new();
                        connections.push(routing_connection_entry);

                        let routing_user_entry = RoutingUserEntry {
                            id: user_id.to_owned(),
                            pgid: user.pgid,
                            pgid_update: user.pgid_update,
                            pgid_update_hc: user.pgid_update_hc,
                            online_time: user.online_time,
                            connections,
                        };
                        table.table.insert(user_id.to_owned(), routing_user_entry);
                    }                    
                }else{
                    if let None = table.table.get(&user.id) {
                        let routing_user_entry = RoutingUserEntry {
                            id: user_id.to_owned(),
                            pgid: user.pgid,
                            pgid_update: user.pgid_update,
                            pgid_update_hc: user.pgid_update_hc,
                            online_time: user.online_time,
                            connections: Vec::new(),
                        };
                        table.table.insert(user_id.to_owned(), routing_user_entry);
                    }                    
                }
            }else{
                expired_users.push(user_id.clone());
            }

            // find best entry
            // if let Some(connection) = Self::find_best_create_routing_table
            //     let routing_connection_entry = RoutingConnectionEntry {
            //         module: conn.clone(),
            //         node: connection.id,
            //         rtt: connection.rtt,
            //         hc: connection.hc,
            //         lq: connection.lq,
            //         propagation_id: user.pgid,
            //     };

            //     // check if user entry already exists hashmap
            //     if let Some(routing_user_entry) = table.table.get_mut(&user.id) {
            //         routing_user_entry.connections.push(routing_connection_entry);
            //     } else {create_routing_table
            //         let mut connections = Vec::new();
            //         connections.push(routing_connection_entry);

            //         let routing_user_entry = RoutingUserEntry {
            //             id: user_id.to_owned(),
            //             pgid: user.pgid,
            //             pgid_update: user.pgid_update,
            //             pgid_update_hc: user.pgid_update_hc,
            //             connections,
            //         };
            //         table.table.insert(user_id.to_owned(), routing_user_entry);
            //     }
            // } else {
            //     // put user for removal as there is no connection in it
            //     expired_users.push(user_id.clone());
            // }
        }

        // remove expired users
        for user_id in expired_users {
            connection_table.table.remove(&user_id);
        }
        
        table
    }

    /// find best entry
    /// and remove all old entries
    fn find_best_connection(user: &mut UserEntry) -> (bool, Option<NeighbourEntry>) {
        // initialize helper variables
        let mut expired_connections: Vec<PeerId> = Vec::new();
        let mut return_entry = None;
        let mut lq = u32::MAX;

        //remove user after 5min from last pgid updated
        //config.maintain_period_limit is seconds unit, need to convert into mili seconds
        let config = super::Router::get_configuration();
        if Timestamp::get_timestamp() - user.pgid_update >= (config.maintain_period_limit * 1000){
            return (true, None);
        }

        // create return value
        {
            let mut entry_found = None;

            // loop through all connections
            for (key, value) in &user.connections {
                let mut expired = true;

                // check if entry is expired
                // entry expires after 20 seconds, unit is mili seconds
                let now = Timestamp::get_timestamp();
                //if now - value.last_update < (20 * 1000 * (value.hc as u64)){
                if now - value.last_update < (2 * (config.sending_table_period * 1000) * (value.hc as u64)){
                    expired = false;

                    if value.lq < lq {
                        lq = value.lq;
                        entry_found = Some(value);
                    }
                }

                // put connection for removal if expired
                if expired {
                    log::error!("expired entry={},  hc={}", (now - value.last_update), value.hc);
                    expired_connections.push(key.clone());
                }
            }

            if let Some(entry) = entry_found {
                return_entry = Some(NeighbourEntry {
                    id: entry.id.clone(),
                    rtt: entry.rtt,
                    hc: entry.hc,
                    lq: entry.lq,
                    last_update: entry.last_update.clone(),
                })
            }
        }

        // remove expired connections
        for node_id in expired_connections {
            user.connections.remove(&node_id);
        }

        (false, return_entry)
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
