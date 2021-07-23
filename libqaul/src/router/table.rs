/**
 * # Global Routing Table
 * 
 * This file contains the global routing table
 * 
 *   * contains all currently reachable users.
 *   * There is an entry for each user over which connection modules 
 *     it can be reached. Each connection module only contains 
 *     information of the best node.
 */

use libp2p::PeerId;
use state::Storage;
use std::sync::RwLock;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::connections::ConnectionModule;

/// mutable state of table
static ROUTINGTABLE: Storage<RwLock<RoutingTable>> = Storage::new();

/// table entry per user
#[derive(Debug, Clone)]
pub struct RoutingUserEntry {
    /// user id
    pub id: PeerId,
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
    /// Package loss
    /// how stable is the connection
    /// this only applies to modules where this is measured
    /// on all other modules this value is 0
    pub pl: f32,
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

    /// create serializable routing info for a specific neighbour
    pub fn create_routing_info( neighbour: Option<PeerId> ) -> RoutingInfoTable {
        let mut table: Vec<RoutingInfoEntry> = Vec::new();

        // get access to routing table
        let routing_table = ROUTINGTABLE.get().read().unwrap();

        // loop through routing table
        for (user_id, user) in routing_table.table.iter() {
            if user.connections.len() > 0 {
                // get first entry
                if let Some(neighbour_id) = neighbour {
                    // check if neighbour is best connection to it
                    if neighbour_id != user.connections[0].node {
                        table.push( RoutingInfoEntry {
                            user: user_id.to_bytes(),
                            rtt: user.connections[0].rtt,
                            hc: user.connections[0].hc,
                            pl: user.connections[0].pl,
                        });
                    }
                } else {
                    table.push( RoutingInfoEntry {
                        user: user_id.to_bytes(),
                        rtt: user.connections[0].rtt,
                        hc: user.connections[0].hc,
                        pl: user.connections[0].pl,
                    });
                }
            }
        }

        RoutingInfoTable(table)
    }

    /// Routing table's CLI commands
    /// 
    /// you get here with the commands:
    /// ```
    /// router table list
    /// ```
    pub fn cli(cmd: &str) {        
        match cmd {
            // display routing table
            cmd if cmd.starts_with("list") => {
                println!("Routing Table");
                println!("No. | User ID  | Connection Module | RTT in ms | Via Neighbour Node Id");

                let mut line = 1;
                let routing_table = ROUTINGTABLE.get().read().unwrap();

                // loop through all user table entries
                for (id, entry) in &routing_table.table {
                    // loop through all connection entries in a user entry
                    for connection in &entry.connections {
                        println!("{} | {:?} | {:?} | {} | {:?}", line, id, connection.module, connection.rtt, connection.node);
                        line += 1;    
                    }
                }
            },
            _ => log::error!("unknown router table command"),
        }
    }

}


/**
 * Serializable routing structures to send over the network
 */
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RoutingInfoEntry {
    /// user id
    pub user: Vec<u8>,
    /// round trip time
    pub rtt: u32,
    /// hop count
    pub hc: u8,
    /// package loss
    pub pl: f32,
}

/// serializable routing information to send to neighbours
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RoutingInfoTable (pub Vec<RoutingInfoEntry>);
