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

use crate::connections::ConnectionModule;

/// mutable state of table
static TABLE: Storage<RwLock<Table>> = Storage::new();

/// table entry per user
pub struct UserEntry {
    /// user id
    id: PeerId,
    /// best routing entry per connection module
    connections: Vec<ConnectionEntry>,
}

/// connection entry per 
pub struct ConnectionEntry {
    /// connections module
    module: ConnectionModule,
    /// node id
    /// via which the user can be reached
    node: PeerId,
    /// round trip time
    /// addition of all round trip times for all hops
    rtt: u32,
    /// hop count
    /// how many hops has the connection
    hc: u8,
    /// Package loss
    /// how stable is the connection
    /// this only applies to modules where this is measured
    /// on all other modules this value is 0
    pl: f32,
}

/**
 * Global Routing Table Implementation
 * 
 * This is the table to turn to when checking where to send
 * a package.
 */
pub struct Table {
    table: HashMap<PeerId, UserEntry>
}

impl Table {
    pub fn init() {
        // create global routing table and save it to state
        let table = Table { table: HashMap::new() };
        TABLE.set(RwLock::new(table));
    }
}


/**
 * Serializable routing structures to send over the network
 */
pub struct TableEntrySerde {
    /// user id
    user: Vec<u8>,
    /// round trip time
    rtt: u32,
    /// hop count
    hc: u8,
    /// package loss
    pl: f32,
}

/// serializable routing information to send to neighbours
pub struct TableSerde (Vec<TableEntrySerde>);
