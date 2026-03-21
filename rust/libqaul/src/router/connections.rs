// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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
use prost::Message;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::RwLock;

use super::proto;
use crate::connections::ConnectionModule;
use crate::router::router_net_proto;
use crate::router::table::{RoutingConnectionEntry, RoutingTable, RoutingUserEntry};
use crate::rpc::Rpc;
use crate::utilities::qaul_id::QaulId;
use crate::utilities::timestamp::Timestamp;

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
    /// user q8id
    id: Vec<u8>,
    /// propagation id
    pub pgid: u32,
    /// when was the propagation id last updated
    pub pgid_update: u64,
    /// DEPRECATED: do we really need this?
    pub pgid_update_hc: u8,
    /// online time
    pub online_time: u64,
    /// connection entries
    connections: BTreeMap<PeerId, NeighbourEntry>,
}

/// Connection table contains a hashed map of all reachable users.
/// The connectivity updates from each neighbour are collected here.
/// Out of this information the global routing table is generated.
pub struct ConnectionTable {
    table: HashMap<Vec<u8>, UserEntry>,
}

/// Instance-based connection table state owning all per-module tables.
/// Replaces the global LOCAL/INTERNET/LAN/BLE statics for multi-instance use.
pub struct ConnectionTableState {
    pub local: RwLock<RoutingTable>,
    pub internet: RwLock<ConnectionTable>,
    pub lan: RwLock<ConnectionTable>,
    pub ble: RwLock<ConnectionTable>,
}

impl ConnectionTableState {
    /// Create a new empty ConnectionTableState.
    pub fn new() -> Self {
        Self {
            local: RwLock::new(RoutingTable {
                table: HashMap::new(),
            }),
            internet: RwLock::new(ConnectionTable {
                table: HashMap::new(),
            }),
            lan: RwLock::new(ConnectionTable {
                table: HashMap::new(),
            }),
            ble: RwLock::new(ConnectionTable {
                table: HashMap::new(),
            }),
        }
    }

    /// Process received routing info and fill into the appropriate module tables.
    /// This is the instance-based version for simulation use.
    pub fn process_received_routing_info(
        &self,
        neighbour_id: PeerId,
        info: &[router_net_proto::RoutingInfoEntry],
        neighbours_state: &super::neighbours::NeighboursState,
        config: &crate::storage::configuration::RoutingOptions,
    ) {
        // try Lan module
        if let Some(rtt) = neighbours_state.get_rtt(&neighbour_id, &ConnectionModule::Lan) {
            self.fill_received_routing_info(ConnectionModule::Lan, neighbour_id, rtt, info, config);
        }

        // try Internet module
        if let Some(rtt) = neighbours_state.get_rtt(&neighbour_id, &ConnectionModule::Internet) {
            self.fill_received_routing_info(
                ConnectionModule::Internet,
                neighbour_id,
                rtt,
                info,
                config,
            );
        }

        // try Bluetooth module
        if let Some(rtt) = neighbours_state.get_rtt(&neighbour_id, &ConnectionModule::Ble) {
            self.fill_received_routing_info(ConnectionModule::Ble, neighbour_id, rtt, info, config);
        }
    }

    /// Populate connection table with incoming routing information (instance-based).
    fn fill_received_routing_info(
        &self,
        conn: ConnectionModule,
        neighbour_id: PeerId,
        rtt: u32,
        info: &[router_net_proto::RoutingInfoEntry],
        config: &crate::storage::configuration::RoutingOptions,
    ) {
        for entry in info {
            let hc;
            if entry.hc[0] < 255 {
                hc = entry.hc[0] + 1;
            } else {
                return;
            }

            let total_rtt = entry.rtt + rtt;
            let neighbour = NeighbourEntry {
                id: neighbour_id,
                rtt: total_rtt,
                hc,
                lq: Self::calculate_linkquality_with_config(total_rtt, hc, config),
                last_update: Timestamp::get_timestamp(),
            };

            self.add_connection_instance(entry.user.clone(), entry.pgid, neighbour, conn);
        }
    }

    /// calculate link quality with explicit config
    fn calculate_linkquality_with_config(
        rtt: u32,
        hc: u8,
        config: &crate::storage::configuration::RoutingOptions,
    ) -> u32 {
        rtt + (hc as u32 * (config.hop_count_penalty as u32) * 1000_000)
    }

    /// add connection to instance state
    fn add_connection_instance(
        &self,
        user_q8id: Vec<u8>,
        pgid: u32,
        connection: NeighbourEntry,
        module: ConnectionModule,
    ) {
        let connection_table_lock = match module {
            ConnectionModule::Internet => &self.internet,
            ConnectionModule::Lan => &self.lan,
            ConnectionModule::Ble => &self.ble,
            ConnectionModule::Local | ConnectionModule::None => return,
        };

        let mut connection_table = connection_table_lock.write().unwrap();
        let now_ts = Timestamp::get_timestamp();

        if let Some(user) = connection_table.table.get_mut(&user_q8id) {
            if connection.hc == 1 || pgid > user.pgid {
                user.pgid = pgid;
                user.pgid_update = now_ts;
                user.pgid_update_hc = connection.hc;
                user.connections.remove(&connection.id);
                user.connections.insert(connection.id, connection);
            } else if pgid == user.pgid {
                if (now_ts - user.pgid_update <= (10 * 1000)) && connection.hc < user.pgid_update_hc
                {
                    user.pgid_update = now_ts;
                    user.pgid_update_hc = connection.hc;
                    user.connections.remove(&connection.id);
                    user.connections.insert(connection.id, connection);
                } else if let Some(conn) = user.connections.get_mut(&connection.id) {
                    if connection.lq < conn.lq {
                        conn.lq = connection.lq;
                        conn.hc = connection.hc;
                        conn.last_update = now_ts;
                        user.connections.remove(&connection.id);
                        user.connections.insert(connection.id, connection);
                    }
                }
            } else if pgid < user.pgid {
                if user.pgid_update_hc == connection.hc {
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
            let mut connections_map = BTreeMap::new();
            let hc = connection.hc;
            connections_map.insert(connection.id, connection);

            let user = UserEntry {
                id: user_q8id.clone(),
                pgid,
                pgid_update: now_ts,
                pgid_update_hc: hc,
                online_time: now_ts,
                connections: connections_map,
            };

            connection_table.table.insert(user_q8id, user);
        }
    }

    /// Update propagation id for local users (instance-based).
    pub fn update_propagation_id(&self, propagation_id: u32) {
        let mut local = self.local.write().unwrap();
        for (_user_id, user) in local.table.iter_mut() {
            user.pgid = propagation_id;
            user.pgid_update = Timestamp::get_timestamp();
            user.connections.get_mut(0).unwrap().last_update = Timestamp::get_timestamp();
        }
    }

    /// Add a local user to the instance state.
    pub fn add_local_user(&self, user_id: PeerId, node_id: PeerId) {
        let mut routing_table = self.local.write().unwrap();
        let mut connections = Vec::with_capacity(1);
        let now_ts = Timestamp::get_timestamp() + 3000;
        connections.push(RoutingConnectionEntry {
            module: ConnectionModule::Local,
            node: node_id,
            rtt: 0,
            hc: 0,
            lq: 0,
            last_update: now_ts,
        });

        let user_q8id = QaulId::to_q8id(user_id);
        let routing_user_entry = RoutingUserEntry {
            id: user_q8id.clone(),
            pgid: 1,
            pgid_update: now_ts,
            pgid_update_hc: 1,
            online_time: now_ts,
            connections,
        };
        routing_table.table.insert(user_q8id, routing_user_entry);
    }

    /// Create a routing table from the instance state (instance-based version of create_routing_table).
    pub fn create_routing_table(
        &self,
        config: &crate::storage::configuration::RoutingOptions,
    ) -> RoutingTable {
        let mut table = RoutingTable {
            table: HashMap::new(),
        };

        // set static routes for local users
        {
            let local = self.local.read().unwrap();
            for (user_id, user) in &local.table {
                table.table.insert(user_id.to_owned(), user.to_owned());
            }
        }

        // calculate from each module
        table = self.calculate_intermediary_table_instance(table, ConnectionModule::Lan, config);
        table =
            self.calculate_intermediary_table_instance(table, ConnectionModule::Internet, config);
        table = self.calculate_intermediary_table_instance(table, ConnectionModule::Ble, config);

        table
    }

    /// Instance-based version of calculate_intermediary_table.
    fn calculate_intermediary_table_instance(
        &self,
        mut table: RoutingTable,
        conn: ConnectionModule,
        config: &crate::storage::configuration::RoutingOptions,
    ) -> RoutingTable {
        let connection_table_lock = match conn {
            ConnectionModule::Internet => &self.internet,
            ConnectionModule::Lan => &self.lan,
            ConnectionModule::Ble => &self.ble,
            ConnectionModule::Local | ConnectionModule::None => return table,
        };

        let mut connection_table = connection_table_lock.write().unwrap();
        let mut expired_users = Vec::with_capacity(connection_table.table.len());

        for (user_id, user) in connection_table.table.iter_mut() {
            let (b_expired_pgid, connection_entry) =
                ConnectionTable::find_best_connection_with_config(user, config);
            if !b_expired_pgid {
                if let Some(connection) = connection_entry {
                    let routing_connection_entry = RoutingConnectionEntry {
                        module: conn,
                        node: connection.id,
                        rtt: connection.rtt,
                        hc: connection.hc,
                        lq: connection.lq,
                        last_update: connection.last_update,
                    };

                    if let Some(routing_user_entry) = table.table.get_mut(&user.id) {
                        routing_user_entry
                            .connections
                            .push(routing_connection_entry);
                    } else {
                        let mut connections = Vec::with_capacity(1);
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
                } else {
                    if !table.table.contains_key(&user.id) {
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
            } else {
                expired_users.push(user_id.clone());
            }
        }

        for user_id in expired_users {
            connection_table.table.remove(&user_id);
        }

        table
    }
}

impl ConnectionTable {
    /// Initialize connection tables
    ///
    /// The per-module tables (local, internet, lan, ble) already exist inside
    /// `RouterState::global().connections`. This method only populates the
    /// local-user entries for every registered user account.
    /// Initialize connection tables with an explicit router state reference.
    /// Populates the local-user entries for every registered user account.
    pub fn init_with_state(state: &crate::QaulState, router: &super::RouterState) {
        // create filled state for locally registered users
        for user in node::user_accounts::UserAccounts::get_user_info(state) {
            Self::add_local_user(state, router, user.id);
        }
    }

    /// add a new local user to state
    pub fn add_local_user(state: &crate::QaulState, router: &super::RouterState, user_id: PeerId) {
        let node_id = node::Node::get_id(state);
        let mut routing_table = router.connections.local.write().unwrap();

        let mut connections = Vec::with_capacity(1);

        // routing table creating is done every 1 seconds.
        // by considerate neighbour sending is done before creating routing Table.
        // we set local user online time forward 3 seconds
        let now_ts = Timestamp::get_timestamp() + 3000;
        connections.push(RoutingConnectionEntry {
            module: ConnectionModule::Local,
            node: node_id,
            rtt: 0,
            hc: 0,
            lq: 0,
            last_update: now_ts,
        });

        let user_q8id = QaulId::to_q8id(user_id);

        let routing_user_entry = RoutingUserEntry {
            id: user_q8id.clone(),
            pgid: 1,
            pgid_update: now_ts,
            pgid_update_hc: 1,
            online_time: now_ts,
            connections,
        };
        routing_table.table.insert(user_q8id, routing_user_entry);
    }

    /// process received routing info table
    /// enter it into all modules where we are connected to
    pub fn process_received_routing_info(
        router: &super::RouterState,
        neighbour_id: PeerId,
        info: &[router_net_proto::RoutingInfoEntry],
    ) {
        // try Lan module
        if let Some(rtt) = Neighbours::get_rtt(router, &neighbour_id, &ConnectionModule::Lan) {
            Self::fill_received_routing_info(router, ConnectionModule::Lan, neighbour_id, rtt, info);
        }

        // try Internet module
        if let Some(rtt) = Neighbours::get_rtt(router, &neighbour_id, &ConnectionModule::Internet) {
            Self::fill_received_routing_info(router, ConnectionModule::Internet, neighbour_id, rtt, info);
        }

        // try Bluetooth module
        if let Some(rtt) = Neighbours::get_rtt(router, &neighbour_id, &ConnectionModule::Ble) {
            Self::fill_received_routing_info(router, ConnectionModule::Ble, neighbour_id, rtt, info);
        }
    }

    /// populate connection table with incoming routing information
    fn fill_received_routing_info(
        router: &super::RouterState,
        conn: ConnectionModule,
        neighbour_id: PeerId,
        rtt: u32,
        info: &[router_net_proto::RoutingInfoEntry],
    ) {
        log::trace!("fill_received_routing_info {}", info.len());
        // loop through results and enter them to the table
        for entry in info {
            // calculate hop count
            // if hop count is > 255, return
            let hc;
            if entry.hc[0] < 255 {
                hc = entry.hc[0] + 1;
            } else {
                return;
            }

            let total_rtt = entry.rtt + rtt;
            // fill structure
            let neighbour = NeighbourEntry {
                id: neighbour_id,
                rtt: total_rtt,
                hc,
                lq: Self::calculate_linkquality_from_router(router, total_rtt, hc),
                last_update: Timestamp::get_timestamp(),
            };

            // add it to state
            Self::add_connection(router, entry.user.clone(), entry.pgid, neighbour, conn);
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
        // `hop_count_penalty` is seconds unit, thus it must be converted to micro seconds
        let penalty = (hc as u64) * (config.hop_count_penalty as u64) * 1_000_000;
        let lq = (rtt as u64).saturating_add(penalty).min(u32::MAX as u64) as u32;

        // return link quality
        lq
    }

    /// add connection to local state
    fn add_connection(
        router: &super::RouterState,
        user_q8id: Vec<u8>,
        pgid: u32,
        connection: NeighbourEntry,
        module: ConnectionModule,
    ) {
        // get access to the connection table
        let mut connection_table;
        match module {
            ConnectionModule::Internet => connection_table = router.connections.internet.write().unwrap(),
            ConnectionModule::Lan => connection_table = router.connections.lan.write().unwrap(),
            ConnectionModule::Ble => connection_table = router.connections.ble.write().unwrap(),
            ConnectionModule::Local => return,
            ConnectionModule::None => return,
        }

        let now_ts = Timestamp::get_timestamp();
        // check if user already exists
        if let Some(user) = connection_table.table.get_mut(&user_q8id) {
            if connection.hc == 1 || pgid > user.pgid {
                user.pgid = pgid;
                user.pgid_update = now_ts;
                user.pgid_update_hc = connection.hc;
                log::trace!("add_connection={}", connection.last_update);
                user.connections.remove(&connection.id);
                user.connections.insert(connection.id, connection);
            } else if pgid == user.pgid {
                //check last update
                if (now_ts - user.pgid_update <= (10 * 1000)) && connection.hc < user.pgid_update_hc
                {
                    user.pgid_update = now_ts;
                    user.pgid_update_hc = connection.hc;
                    user.connections.remove(&connection.id);
                    user.connections.insert(connection.id, connection);
                } else if let Some(conn) = user.connections.get_mut(&connection.id) {
                    if connection.lq < conn.lq {
                        conn.lq = connection.lq;
                        conn.hc = connection.hc;
                        conn.last_update = now_ts;
                        user.connections.remove(&connection.id);
                        user.connections.insert(connection.id, connection);
                    }
                }
            } else if pgid < user.pgid {
                if user.pgid_update_hc == connection.hc {
                    //reboot node case
                    if user.pgid.saturating_sub(pgid) > (connection.hc as u32) {
                        user.pgid = pgid;
                        user.pgid_update = now_ts;
                        user.pgid_update_hc = connection.hc;
                        user.connections.remove(&connection.id);
                        user.connections.insert(connection.id, connection);
                    }
                }
            }
        } else {
            let mut connections_map = BTreeMap::new();
            let hc = connection.hc;
            connections_map.insert(connection.id, connection);

            let user = UserEntry {
                id: user_q8id.clone(),
                pgid: pgid,
                pgid_update: now_ts,
                pgid_update_hc: hc,
                online_time: now_ts,
                connections: connections_map,
            };

            connection_table.table.insert(user_q8id, user);
        }
    }

    /// update propagation id for local users
    pub fn update_propagation_id(router: &super::RouterState, propagation_id: u32) {
        //update local user's propagation id
        let mut local = router.connections.local.write().unwrap();
        for (_user_id, user) in local.table.iter_mut() {
            user.pgid = propagation_id;
            // QUESTION: is this of any use?
            user.pgid_update = Timestamp::get_timestamp();
            if let Some(conn) = user.connections.get_mut(0) {
                conn.last_update = Timestamp::get_timestamp();
            }
        }
    }

    /// Create a routing table and set it to active routing table
    pub fn create_routing_table(router: &super::RouterState) {
        // create a new table
        let mut table = RoutingTable {
            table: HashMap::new(),
        };

        // set static routes for local users
        // create them first, for that they are always routed to ourselves
        {
            table = Self::local_routes_to_intermediary_table(router, table);
        }

        // calculate from lan module
        table = Self::calculate_intermediary_table(router, table, ConnectionModule::Lan);

        // calculate from internet module
        table = Self::calculate_intermediary_table(router, table, ConnectionModule::Internet);

        // calculate from ble module
        table = Self::calculate_intermediary_table(router, table, ConnectionModule::Ble);

        // set table as new active routing table
        RoutingTable::set(router, table);
    }

    /// insert local routes into routing table
    fn local_routes_to_intermediary_table(router: &super::RouterState, mut table: RoutingTable) -> RoutingTable {
        // get local routes
        let local = router.connections.local.read().unwrap();

        // fill it into routing table
        for (user_id, user) in &local.table {
            table.table.insert(user_id.to_owned(), user.to_owned());
        }

        table
    }

    /// calculate a routing table for a module
    fn calculate_intermediary_table(
        router: &super::RouterState,
        mut table: RoutingTable,
        conn: ConnectionModule,
    ) -> RoutingTable {
        // get connections table
        let mut connection_table;
        match conn {
            ConnectionModule::Internet => connection_table = router.connections.internet.write().unwrap(),
            ConnectionModule::Lan => connection_table = router.connections.lan.write().unwrap(),
            ConnectionModule::Ble => connection_table = router.connections.ble.write().unwrap(),
            ConnectionModule::Local => return table,
            ConnectionModule::None => return table,
        }
        // create vector for users to remove
        let mut expired_users = Vec::with_capacity(connection_table.table.len());

        // iterate over connection table
        for (user_id, user) in connection_table.table.iter_mut() {
            let (b_expired_pgid, connection_entry) = Self::find_best_connection(router, user);
            if !b_expired_pgid {
                if let Some(connection) = connection_entry {
                    // fill entry into routing table
                    let routing_connection_entry = RoutingConnectionEntry {
                        module: conn,
                        node: connection.id,
                        rtt: connection.rtt,
                        hc: connection.hc,
                        lq: connection.lq,
                        last_update: connection.last_update,
                    };

                    // check if user entry already exists hashmap
                    if let Some(routing_user_entry) = table.table.get_mut(&user.id) {
                        routing_user_entry
                            .connections
                            .push(routing_connection_entry);
                    } else {
                        let mut connections = Vec::with_capacity(1);
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
                } else {
                    if !table.table.contains_key(&user.id) {
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
            } else {
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
    fn find_best_connection(router: &super::RouterState, user: &mut UserEntry) -> (bool, Option<NeighbourEntry>) {
        Self::find_best_connection_with_config(user, &router.configuration)
    }

    /// find best entry with explicit config (for instance-based and global use)
    fn find_best_connection_with_config(
        user: &mut UserEntry,
        config: &crate::storage::configuration::RoutingOptions,
    ) -> (bool, Option<NeighbourEntry>) {
        // initialize helper variables
        let mut expired_connections = Vec::with_capacity(user.connections.len());
        let mut return_entry = None;
        let mut lq = u32::MAX;

        //remove user after 5min from last pgid updated
        //config.maintain_period_limit is seconds unit, need to convert into mili seconds
        if Timestamp::get_timestamp() - user.pgid_update >= (config.maintain_period_limit * 1000) {
            return (true, None);
        }

        // create return value
        {
            let mut entry_found = None;
            let now = Timestamp::get_timestamp();

            // loop through all connections
            for (key, value) in &user.connections {
                let mut expired = true;

                // check if entry is expired
                // entry expires after 20 seconds, unit is mili seconds
                if now - value.last_update
                    < (config.sending_table_period * 1000 * (value.hc as u64 + 1))
                {
                    expired = false;

                    if value.lq < lq {
                        lq = value.lq;
                        entry_found = Some(value);
                    }
                }

                // put connection for removal if expired
                if expired {
                    log::info!(
                        "expired entry={},  hc={}",
                        (now - value.last_update),
                        value.hc
                    );
                    expired_connections.push(key.clone());
                }
            }

            if let Some(entry) = entry_found {
                return_entry = Some(NeighbourEntry {
                    id: entry.id.clone(),
                    rtt: entry.rtt,
                    hc: entry.hc,
                    lq: entry.lq,
                    last_update: entry.last_update,
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
    pub fn rpc_send_connections_list(state: &crate::QaulState, router: &super::RouterState, request_id: String) {
        // create connections list
        let connections_list = proto::ConnectionsList {
            lan: Self::rpc_create_connection_module_list(router, ConnectionModule::Lan),
            internet: Self::rpc_create_connection_module_list(router, ConnectionModule::Internet),
            ble: Self::rpc_create_connection_module_list(router, ConnectionModule::Ble),
            local: Self::rpc_create_connection_module_list(router, ConnectionModule::Local),
        };

        // create rpc connections list protobuf message
        let proto_message = proto::Router {
            message: Some(proto::router::Message::ConnectionsList(connections_list)),
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

    /// create rpc connection module list
    fn rpc_create_connection_module_list(
        router: &super::RouterState,
        conn: ConnectionModule,
    ) -> Vec<proto::ConnectionsUserEntry> {
        // request connection table from state
        let connection_table;
        match conn {
            ConnectionModule::Lan => connection_table = router.connections.lan.read().unwrap(),
            ConnectionModule::Internet => connection_table = router.connections.internet.read().unwrap(),
            ConnectionModule::Ble => connection_table = router.connections.ble.read().unwrap(),
            ConnectionModule::Local => return Vec::new(),
            ConnectionModule::None => return Vec::new(),
        }

        // create entry vector
        let mut connections_list = Vec::with_capacity(connection_table.table.len());

        // loop through all table entries per user
        for (id, entry) in &connection_table.table {
            // create user entry
            let mut user_entry = proto::ConnectionsUserEntry {
                user_id: id.clone(),
                connections: Vec::with_capacity(entry.connections.len()),
            };

            // loop through all neighbour entries of a user entry
            for (id, neighbour) in &entry.connections {
                // add connection
                user_entry.connections.push(proto::ConnectionEntry {
                    rtt: neighbour.rtt,
                    hop_count: neighbour.hc as u32,
                    via: id.to_bytes(),
                });
            }

            // add user entry to list
            connections_list.push(user_entry);
        }

        // return list
        connections_list
    }
}
