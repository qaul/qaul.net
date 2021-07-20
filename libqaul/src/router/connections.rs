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
use std::sync::RwLock;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

use crate::connections::ConnectionModule;
use crate::router::{
    table::{RoutingInfoTable, RoutingTable, RoutingUserEntry, RoutingConnectionEntry},
    neighbours::Neighbours,
};


/// Mutable module state
/// Tables with all stats for each connection module
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
        let internet = ConnectionTable { table: HashMap::new() };
        INTERNET.set(RwLock::new(internet));

        let lan = ConnectionTable { table: HashMap::new() };
        LAN.set(RwLock::new(lan));
    }

    /// process received routing info table
    /// enter it into all modules where we are connected to
    pub fn process_received_routing_info( neighbour_id: PeerId, info: RoutingInfoTable ) {
        // try Lan module
        if let Some(rtt) = Neighbours::get_rtt(&neighbour_id , &ConnectionModule::Lan ){
            Self::populate(ConnectionModule::Lan, neighbour_id, rtt, info.clone());
        }

        // try Internet module
        if let Some(rtt) = Neighbours::get_rtt(&neighbour_id , &ConnectionModule::Internet ){
            Self::populate(ConnectionModule::Internet, neighbour_id, rtt, info.clone());
        }
    }

    /// populate connection table with incoming routing information
    pub fn populate( conn: ConnectionModule, neighbour_id: PeerId, rtt: u32, info: RoutingInfoTable ) {
        // get access to the connection table
        let mut connection_table;
        match conn {
            ConnectionModule::Internet => connection_table = INTERNET.get().write().unwrap(),
            ConnectionModule::Lan => connection_table = LAN.get().write().unwrap(),
            ConnectionModule::None => return
        }

        // loop through results and enter them to the table
        for entry in info.0 {
            if let Ok(user_id) = PeerId::from_bytes(&entry.user){
                let neighbour = NeighbourEntry {
                    id: neighbour_id,
                    rtt: entry.rtt +rtt,
                    hc: entry.hc,
                    pl: entry.pl,
                    last_update: SystemTime::now(),
                };
                // add entry to table
                if let Some(user) = connection_table.table.get_mut(&user_id) {
                    user.connections.insert(neighbour_id, neighbour);
                } else {
                    let mut connections_map = BTreeMap::new();
                    connections_map.insert(neighbour.id, neighbour);

                    let user = UserEntry { 
                        id: user_id,
                        connections: connections_map,
                    };

                    connection_table.table.insert(user_id, user);
                }
            }
        }
    }

    /// Create a routing table and set it to active routing table
    pub fn create_routing_table() {
        // create a new table
        let mut table = RoutingTable { table: HashMap::new() };

        // calculate from lan module
        table = Self::calculate_intermediary_table(table, ConnectionModule::Lan);

        // calculate from internet module
        table = Self::calculate_intermediary_table(table, ConnectionModule::Internet);

        // set table as new active routing table
        RoutingTable::set(table);
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

    /// Connections table's CLI commands
    /// 
    /// you get here with the commands:
    /// ```
    /// router connections list
    /// ```
    pub fn cli(cmd: &str) {        
        match cmd {
            // display routing table
            cmd if cmd.starts_with("list") => {
                // display LAN table
                Self::cli_display_list(ConnectionModule::Lan);

                // display Inernet table
                Self::cli_display_list(ConnectionModule::Internet);
            },
            _ => log::error!("unknown user command"),
        }
    }

    /// print list of connection module on terminal 
    fn cli_display_list( conn: ConnectionModule ) {
        println!("{:?}", conn);
        println!("No. | User ID");
        println!("      * RTT in ms | hop count | Via Neighbour Node Id");

        let mut line = 1;
        let connection_table;
        
        match conn {
            ConnectionModule::Lan => connection_table = LAN.get().read().unwrap(),
            ConnectionModule::Internet => connection_table = INTERNET.get().read().unwrap(),
            ConnectionModule::None => return,
        }

        // loop through all table entries per user
        for (id, entry) in &connection_table.table {
            println!("{} | {:?}", line, id);
            // loop through all neighbour entries of a user entry
            for (id, neighbour) in &entry.connections {
                println!("      * {} | {} | {:?}", neighbour.rtt, neighbour.hc, id);
            }
            line += 1;
        }
    }

}

