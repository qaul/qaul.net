// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Qaul Community Router
//!
//! This module implements all the tables and logic of the
//! qaul router.

use prost::Message;
use std::sync::Arc;
use std::sync::OnceLock;

pub mod connections;
pub mod feed_requester;
pub mod flooder;
pub mod info;
pub mod neighbours;
pub mod table;
pub mod user_requester;
pub mod users;

use crate::storage::configuration::{Configuration, RoutingOptions};
use connections::ConnectionTable;
use neighbours::Neighbours;
use table::RoutingTable;
use users::Users;

pub use qaul_proto::qaul_net_router_net_info as router_net_proto;
/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_router as proto;

/// mutable state of router,
/// used for storing the router configuration (global state - deprecated)
static ROUTER: InitCell<RwLock<Router>> = InitCell::new();

/// Instance-based router state that owns all routing sub-state.
///
/// This replaces the scattered global statics across router submodules.
/// Each `RouterState` instance is fully independent, enabling multiple
/// nodes to run in the same process.
pub struct RouterState {
    /// Router configuration
    pub configuration: RoutingOptions,

    /// Neighbour tables per connection module
    pub neighbours: neighbours::NeighboursState,

    /// Connection tables per connection module
    pub connections: connections::ConnectionTableState,

    /// Global routing table
    pub routing_table: table::RoutingTableState,

    /// Routing info scheduler
    pub scheduler: info::SchedulerState,

    /// Flood message queue
    pub flooder: flooder::FlooderState,

    /// Feed request queue
    pub feed_requester: feed_requester::FeedRequesterState,

    /// Feed response queue
    pub feed_responser: feed_requester::FeedResponserState,

    /// User request queue
    pub user_requester: user_requester::UserRequesterState,

    /// User response queue
    pub user_responser: user_requester::UserResponserState,

    /// Users table
    pub users: users::UsersState,
}

impl RouterState {
    /// Create a new RouterState with default empty tables.
    /// This does NOT touch any global state or databases.
    /// Suitable for simulation.
    pub fn new(config: RoutingOptions) -> Self {
        let interval = config.sending_table_period;
        Self {
            configuration: config,
            neighbours: neighbours::NeighboursState::new(),
            connections: connections::ConnectionTableState::new(),
            routing_table: table::RoutingTableState::new(),
            scheduler: info::SchedulerState::new(interval),
            flooder: flooder::FlooderState::new(),
            feed_requester: feed_requester::FeedRequesterState::new(),
            feed_responser: feed_requester::FeedResponserState::new(),
            user_requester: user_requester::UserRequesterState::new(),
            user_responser: user_requester::UserResponserState::new(),
            users: users::UsersState::new(),
        }
    }

    /// Get a reference to the global RouterState instance.
    /// Panics if the global state has not been initialized.
    pub fn global() -> &'static Arc<RouterState> {
        GLOBAL_ROUTER_STATE.get().expect("RouterState not initialized")
    }

    /// Initialize the global RouterState from the current Configuration.
    /// Called during Router::init() for backward compatibility.
    fn init_global() {
        let config = Configuration::get();
        let state = Arc::new(RouterState::new(config.routing.clone()));
        let _ = GLOBAL_ROUTER_STATE.set(state);
    }
}

/// Router Module - holds all router state for a single instance
///
/// This struct wraps the router configuration and provides instance-based
/// access to routing functionality. The actual routing tables are still
/// stored in global state for backward compatibility
pub struct RouterModule {
    /// Router configuration
    pub configuration: RoutingOptions,
    /// Sending table period in seconds
    pub sending_table_period: u64,
}

impl RouterModule {
    /// Create a new RouterModule from configuration (instance-based)
    ///
    /// Note: This creates the instance but the actual routing tables
    /// are still initialized via global state for backward compatibility.
    pub fn new(config: &Configuration) -> Self {
        Self {
            configuration: config.routing.clone(),
            sending_table_period: config.routing.sending_table_period,
        }
    }

    /// Get router configuration
    pub fn get_configuration(&self) -> &RoutingOptions {
        &self.configuration
    }
}

/// qaul community router access
pub struct Router {}

impl Router {
    /// Initialize the qaul router
    pub fn init() {
        // Initialize the global RouterState instance.
        // All sub-module state (flooder, feed_requester, user_requester, etc.)
        // lives inside RouterState. Per-module globals are no longer needed.
        RouterState::init_global();

        // initialize direct neighbours table (database-backed)
        Neighbours::init();

        // initialize users table (database-backed)
        Users::init();

        // initialize the routing information collection
        // tables per connection module (database-backed)
        ConnectionTable::init();

        // RouterInfo scheduler is already initialized as part of RouterState::init_global().
        // No separate RouterInfo::init() call needed.
    }

    /// Get router configuration from state
    pub fn get_configuration() -> RoutingOptions {
        RouterState::global().configuration.clone()
    }

    /// Process incoming RPC request messages and send them to
    /// the submodules
    pub fn rpc(data: Vec<u8>, request_id: String) {
        match proto::Router::decode(&data[..]) {
            Ok(router) => {
                match router.message {
                    Some(proto::router::Message::RoutingTableRequest(_request)) => {
                        // send routing table list
                        RoutingTable::rpc_send_routing_table(request_id);
                    }
                    Some(proto::router::Message::ConnectionsRequest(_request)) => {
                        // send connections list
                        ConnectionTable::rpc_send_connections_list(request_id);
                    }
                    Some(proto::router::Message::NeighboursRequest(_request)) => {
                        // send neighbours list
                        Neighbours::rpc_send_neighbours_list(request_id);
                    }
                    _ => {}
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
