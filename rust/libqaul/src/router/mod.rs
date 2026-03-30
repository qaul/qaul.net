// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Qaul Community Router
//!
//! This module implements all the tables and logic of the
//! qaul router.

use prost::Message;
use std::sync::Arc;

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

/// Import protobuf message definition
pub use qaul_proto::qaul_net_router_net_info as router_net_proto;
pub use qaul_proto::qaul_rpc_router as proto;


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

    /// Initialize the RouterState from the current Configuration
    /// and store it in QaulState.
    fn init_into_qaul_state(qaul_state: &crate::QaulState) {
        let config = Configuration::get(qaul_state);
        let state = Arc::new(RouterState::new(config.routing.clone()));
        qaul_state.replace_router(state);
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
    pub fn init(qaul_state: &crate::QaulState) {

        // Initialize the RouterState and store it in QaulState.
        RouterState::init_into_qaul_state(qaul_state);

        let rs = qaul_state.get_router();

        // initialize direct neighbours table (database-backed)
        Neighbours::init_with_state(qaul_state, &rs);

        // initialize users table (database-backed)
        Users::init_with_state(qaul_state, &rs);

    }

    /// Get router configuration from an explicit state reference
    pub fn get_configuration_from(router: &RouterState) -> RoutingOptions {
        router.configuration.clone()
    }

    /// Process incoming RPC request messages and send them to
    /// the submodules
    pub fn rpc(state: &crate::QaulState, router_state: &RouterState, data: Vec<u8>, request_id: String) {
        match proto::Router::decode(&data[..]) {
            Ok(router) => {
                match router.message {
                    Some(proto::router::Message::RoutingTableRequest(_request)) => {
                        // send routing table list
                        RoutingTable::rpc_send_routing_table(state, router_state, request_id);
                    }
                    Some(proto::router::Message::ConnectionsRequest(_request)) => {
                        // send connections list
                        ConnectionTable::rpc_send_connections_list(state, router_state, request_id);
                    }
                    Some(proto::router::Message::NeighboursRequest(_request)) => {
                        // send neighbours list
                        Neighbours::rpc_send_neighbours_list(state, router_state, request_id);
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
