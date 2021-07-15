/**
 * # Connection Module Connectivity Collection Table
 * 
 * This file contains the collected collectivity information
 * per connection module
 * 
 *   * each connection module has it's own routing table containing
 *     all currently reachable users via this module.
 *   * Each user has an entry for each node over which it can be reached.
 *   * Out of this information the global table is constructed, 
 *     containing only the best entry.
 */

use libp2p::PeerId;
use state::Storage;
use std::sync::RwLock;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::SystemTime;

// mutable state of tables
static INTERNET: Storage<RwLock<ConnectionTable>> = Storage::new();
static LAN: Storage<RwLock<ConnectionTable>> = Storage::new();


/**
 * Table with all stats for each connection module
 * 
 * The connectivity updates from each neighbour are collected here.
 * Out of this information the global routing table is generated.
 */

/// Connection entry for UserEntry
pub struct NeighbourEntry {
    /// node id of the neighbour
    node: PeerId,
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
pub struct UserEntry {
    /// user id
    id: PeerId,
    /// connection entries
    connections: BTreeMap<PeerId, NeighbourEntry>,
}

/// Connection table, 
/// containing a hashed map of all reachable users
pub struct ConnectionTable {
    table: HashMap<PeerId, UserEntry>,
}

impl ConnectionTable {
    pub fn init() {
        // create intermediary tables for eachConnectionModule
        // and save it to state
        let internet = ConnectionTable { table: HashMap::new() };
        INTERNET.set(RwLock::new(internet));

        let lan = ConnectionTable { table: HashMap::new() };
        INTERNET.set(RwLock::new(lan));
    }
}

