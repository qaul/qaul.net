// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # libqaul
//!
//! Library for qaul

use crossbeam_channel::Receiver;
use filetime::FileTime;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::rpc::authentication::AuthenticationState;
use crate::rpc::sys::SysRpcState;
use crate::rpc::RpcState;
use crate::utilities::filelogger::FileLoggerState;

// crate modules
pub mod api;
pub mod connections;
pub mod node;
pub mod router;
pub mod rpc;
mod search;
pub mod services;
pub mod storage;
pub mod utilities;

use connections::{ble::Ble, internet::Internet, ConnectionModule, Connections, ConnectionsModule};
use node::{Node, NodeModule};
use router::{info::RouterInfo, Router, RouterModule};
use rpc::sys::Sys;
use rpc::{Rpc, RpcModule};
use services::messaging::Messaging;
use services::ServicesModule;
use storage::StorageModule;
use utilities::filelogger::FileLogger;
use utilities::timestamp::Timestamp;
use utilities::upgrade;

/// Top-level state container that owns all instance-based state.
///
/// `Libqaul` owns this via `Arc<QaulState>`. All event-loop handlers
/// and RPC dispatch functions receive `&QaulState` instead of reaching
/// into global statics.
pub struct QaulState {
    /// Router state (routing tables, neighbours, flooder, scheduler, etc.)
    /// Wrapped in RwLock because QaulState is created before Router::init()
    /// populates the real state.
    pub router: std::sync::RwLock<Arc<router::RouterState>>,
    /// Services state (messaging, feed, chat, crypto, groups, dtn)
    pub services: services::ServicesState,
    /// User accounts state
    pub user_accounts: node::user_accounts::UserAccountsState,
    /// Authentication state (challenge-response sessions)
    pub auth: AuthenticationState,
    /// RPC channel state (external ↔ libqaul channels)
    pub rpc: RpcState,
    /// SYS RPC channel state (external ↔ libqaul SYS channels)
    pub sys: SysRpcState,
    /// Connection states (Internet reconnections, BLE)
    pub connections: connections::ConnectionsState,
    /// Node identity (PeerId, Keypair, topic)
    /// Wrapped in RwLock because QaulState is created before Node::init()
    /// loads the real identity from config.
    pub node: std::sync::RwLock<Arc<node::NodeIdentity>>,
    /// Configuration state
    pub config: storage::configuration::ConfigurationState,
    /// Database state
    pub database: storage::database::DatabaseState,
    /// Storage path (set during Storage::init)
    pub storage_path: std::sync::RwLock<String>,
    /// File logger state
    pub filelogger: FileLoggerState,
    /// Default configuration values passed at startup.
    pub default_configs: BTreeMap<String, String>,
    /// Whether the instance has finished initializing (event loop started).
    pub initialized: AtomicBool,
}

impl QaulState {
    /// Replace the router state with the real one after initialization.
    pub fn replace_router(&self, router_state: Arc<router::RouterState>) {
        *self.router.write().unwrap() = router_state;
    }

    /// Replace the node identity with the real one after initialization.
    pub fn replace_node(&self, node_identity: Arc<node::NodeIdentity>) {
        *self.node.write().unwrap() = node_identity;
    }

    /// Get a snapshot of the current router state.
    pub fn get_router(&self) -> Arc<router::RouterState> {
        self.router.read().unwrap().clone()
    }

    /// Get a snapshot of the current node identity.
    pub fn get_node(&self) -> Arc<node::NodeIdentity> {
        self.node.read().unwrap().clone()
    }

    /// Create a `QaulState` suitable for simulation / testing.
    /// Uses temporary in-memory databases and default configuration.
    pub fn new_for_simulation() -> Self {
        let config = storage::configuration::Configuration::default();
        Self {
            router: std::sync::RwLock::new(Arc::new(router::RouterState::new(config.routing.clone()))),
            services: services::ServicesState::new(),
            user_accounts: node::user_accounts::UserAccountsState::new(),
            auth: AuthenticationState::new(),
            rpc: RpcState::new(),
            sys: SysRpcState::new(),
            connections: connections::ConnectionsState::new(),
            node: std::sync::RwLock::new(Arc::new(node::NodeIdentity::generate())),
            config: storage::configuration::ConfigurationState::from_config(config),
            database: storage::database::DatabaseState::new_temporary(),
            storage_path: std::sync::RwLock::new(String::new()),
            filelogger: FileLoggerState::new(),
            default_configs: BTreeMap::new(),
            initialized: AtomicBool::new(false),
        }
    }

    /// Create a `QaulState` with defaults suitable for production.
    ///
    /// Configuration and database fields start with defaults and are
    /// populated later by `Storage::init()`.
    pub fn new_production(
        node_identity: Arc<node::NodeIdentity>,
        user_accounts: node::user_accounts::UserAccountsState,
        router_state: Arc<router::RouterState>,
        default_configs: BTreeMap<String, String>,
    ) -> Self {
        Self {
            router: std::sync::RwLock::new(router_state),
            services: services::ServicesState::new(),
            user_accounts,
            auth: AuthenticationState::new(),
            rpc: RpcState::new(),
            sys: SysRpcState::new(),
            connections: connections::ConnectionsState::new(),
            node: std::sync::RwLock::new(node_identity),
            config: storage::configuration::ConfigurationState::new(),
            database: storage::database::DatabaseState::new_temporary(),
            storage_path: std::sync::RwLock::new(String::new()),
            filelogger: FileLoggerState::new(),
            default_configs,
            initialized: AtomicBool::new(false),
        }
    }
}

/// Libqaul - Main library instance
///
/// # Usage
///
/// ```rust,ignore
/// // Create a new instance
/// let libqaul = Libqaul::new(storage_path, None).await;
///
/// // Run the event loop
/// libqaul.run().await;
/// ```
pub struct Libqaul {
    /// Unified instance-based state (owns all sub-states).
    /// Event-loop handlers and RPC dispatch use this instead of globals.
    pub state: Arc<QaulState>,

    /// Storage module (configuration and database)
    pub storage: Arc<StorageModule>,

    /// Node module (identity and user accounts)
    pub node: Arc<NodeModule>,

    /// Router module (routing configuration)
    pub router: Arc<RouterModule>,

    /// Services module (messaging, feed, chat, etc.)
    pub services: Arc<std::sync::RwLock<ServicesModule>>,

    /// Connections module (network connections state)
    pub connections: Arc<std::sync::RwLock<ConnectionsModule>>,

    /// RPC module state
    pub rpc: Arc<std::sync::RwLock<RpcModule>>,

    /// RPC channel receiver (from external to libqaul)
    rpc_receiver: Receiver<Vec<u8>>,

    /// SYS channel receiver (from external to libqaul)
    sys_receiver: Receiver<Vec<u8>>,

    /// Whether this instance has finished initializing
    initialized: AtomicBool,
}

impl Libqaul {
    /// Create a new Libqaul instance
    ///
    /// This initializes all modules (storage, node, router, connections, services)
    /// and returns an instance ready to run.
    ///
    /// # Arguments
    ///
    /// * `storage_path` - Path where libqaul can save all data
    /// * `def_config` - Optional configuration values (e.g., listening port)
    ///
    /// # Returns
    ///
    /// A new `Libqaul` instance wrapped in `Arc` for shared ownership
    pub async fn new(
        storage_path: String,
        def_config: Option<BTreeMap<String, String>>,
    ) -> Arc<Self> {
        // print storage path
        println!("storage path: {}", storage_path);

        // check if we need to upgrade our stored data
        if upgrade::Upgrade::init(storage_path.clone()) == false {
            println!("upgrade to new version failed");
            std::process::exit(0);
        }

        // check configuration options
        let def_configs = def_config.unwrap_or_default();

        // Default configs are stored in QaulState and applied after Configuration::init().

        // Bootstrap QaulState early with defaults so that Storage::init(),
        // Configuration::init(), and DataBase::init() can write into it.
        let qaul_state = {
            // Create a temporary NodeIdentity; will be replaced after config is loaded.
            let temp_node = Arc::new(node::NodeIdentity::generate());
            let temp_router = Arc::new(router::RouterState::new(
                storage::configuration::RoutingOptions::default(),
            ));
            Arc::new(QaulState::new_production(
                temp_node,
                node::user_accounts::UserAccountsState::new(),
                temp_router,
                def_configs,
            ))
        };
        // Initialize storage module.
        // Storage::init() now writes storage_path, config, and database
        // into QaulState.
        let storage = Arc::new(StorageModule::new(&qaul_state, storage_path.clone()));

        // Apply default port override from CLI-provided config, if any.
        if let Some(port_str) = qaul_state.default_configs.get("port") {
            if let Ok(port) = port_str.parse::<u16>() {
                let mut config = qaul_state.config.inner.write().unwrap();
                config.internet.listen = {
                    let listen_ipv4_quic = format!("/ip4/0.0.0.0/udp/{}/quic-v1", port);
                    let listen_ipv4 = format!("/ip4/0.0.0.0/tcp/{}", port);
                    let listen_ipv6_quic = format!("/ip6/::/udp/{}/quic-v1", port);
                    let listen_ipv6 = format!("/ip6/::/tcp/{}", port);
                    vec![listen_ipv4_quic, listen_ipv4, listen_ipv6_quic, listen_ipv6]
                };
            }
        }

        // Initialize logger (pass shared config handle so FileLogger can read
        // the enable/disable flag that FileLoggerState will toggle at runtime).
        Self::init_logger(&storage_path, qaul_state.filelogger.config_handle());

        log::trace!("test log to ensure that logging is working");

        // initialize node module (instance-based)
        let node = {
            let config = storage.config.read().unwrap();
            Arc::new(NodeModule::new(&config))
        };

        // initialize router module (instance-based)
        let router = {
            let config = storage.config.read().unwrap();
            Arc::new(RouterModule::new(&config))
        };

        // Also initialize global router state for backward compatibility
        Router::init(&*qaul_state);

        // Now update QaulState with the real node identity, user accounts,
        // and router state (config/database are already populated by Storage::init).
        {
            let config = storage::configuration::Configuration::get(&qaul_state);
            let user_accounts = node::user_accounts::UserAccounts::create_from_config(&config);
            *qaul_state.user_accounts.inner.write().unwrap() = user_accounts;

            // Router::init() already stored real RouterState into QaulState.
            qaul_state.replace_node(Arc::clone(&node.node));
            qaul_state.filelogger.enable(config.debug.log);
        }

        // Now that real node identity and user accounts are loaded, register
        // local users in the router's connection table.
        {
            let rs = qaul_state.get_router();
            let node_id = crate::node::Node::get_id(&qaul_state);
            for user in crate::node::user_accounts::UserAccounts::get_user_info(&qaul_state) {
                rs.connections.add_local_user(user.id, node_id);
            }
        }

        // Initialize node global state for backward compatibility.
        // This now delegates to qaul_state.node and saves config if needed.
        Node::init(&qaul_state);

        // Use the QaulState's channel receivers
        let rpc_receiver = qaul_state.rpc.libqaul_receive.clone();
        let sys_receiver = qaul_state.sys.libqaul_receive.clone();

        // initialize services module (instance-based)
        let services = Arc::new(std::sync::RwLock::new(ServicesModule::new()));
        {
            let mut svc = services.write().unwrap();
            svc.initialize(&*qaul_state);
        }

        // Also initialize global state for backward compatibility
        // Note: Services::init() is now redundant since ServicesModule::initialize()
        // does the same thing, but we keep it for explicit backward compatibility
        // Services::init();

        // Create connections module (instance-based wrapper)
        let connections = Arc::new(std::sync::RwLock::new(ConnectionsModule::new()));

        // Create RPC module (instance-based wrapper)
        let rpc_module = Arc::new(std::sync::RwLock::new(RpcModule::new()));
        {
            let mut rpc = rpc_module.write().unwrap();
            rpc.set_initialized();
        }

        let instance = Arc::new(Self {
            state: qaul_state,
            storage,
            node,
            router,
            services,
            connections,
            rpc: rpc_module,
            rpc_receiver,
            sys_receiver,
            initialized: AtomicBool::new(false),
        });

        instance
    }

    /// Initialize the logger with appropriate configuration for the platform
    fn init_logger(storage_path: &str, log_config: Arc<std::sync::RwLock<utilities::filelogger::FileLoggerConfig>>) {
        let path = Path::new(storage_path);
        let log_path = path.join("logs");

        // create log directory if missing
        std::fs::create_dir_all(&log_path).unwrap();

        // create log file name
        let log_file_name: String =
            "error_".to_string() + Timestamp::get_timestamp().to_string().as_str() + ".log";
        let log_file_path = log_path.join(log_file_name);

        // maintain log files (keep only last 2)
        let paths = std::fs::read_dir(&log_path).unwrap();

        let mut logfiles: BTreeMap<i64, String> = BTreeMap::new();
        let mut logfile_times: Vec<i64> = vec![];
        for path in paths {
            let filename = String::from(path.as_ref().unwrap().path().to_str().unwrap());
            let metadata = std::fs::metadata(filename.clone()).unwrap();
            let mtime = FileTime::from_last_modification_time(&metadata);
            logfile_times.push(mtime.seconds());
            logfiles.insert(mtime.seconds(), filename);
        }
        logfile_times.sort();

        if logfile_times.len() > 2 {
            for i in 0..(logfile_times.len() - 2) {
                if let Some(filename) = logfiles.get(&logfile_times[i]) {
                    std::fs::remove_file(std::path::Path::new(filename)).unwrap();
                }
            }
        }

        // logging on android with android logger
        #[cfg(target_os = "android")]
        {
            let env_logger = Box::new(android_logger::AndroidLogger::new(
                Config::default().with_max_level(log::LevelFilter::Info),
            ));
            let w_logger = FileLogger::new(
                *simplelog::WriteLogger::new(
                    simplelog::LevelFilter::Error,
                    simplelog::Config::default(),
                    File::create(log_file_path).unwrap(),
                ),
                log_config.clone(),
            );
            multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info)
                .unwrap();
        }

        // logging on ios
        #[cfg(target_os = "ios")]
        {
            let env_logger = Box::new(
                pretty_env_logger::formatted_builder()
                    .filter(None, log::LevelFilter::Info)
                    .build(),
            );
            let w_logger = FileLogger::new(
                *simplelog::WriteLogger::new(
                    simplelog::LevelFilter::Error,
                    simplelog::Config::default(),
                    File::create(log_file_path).unwrap(),
                ),
                log_config.clone(),
            );
            multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info)
                .unwrap();
        }

        // only use the simple logger on desktop systems
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            // find rust env var
            let mut env_log_level = String::from("error");
            for (key, value) in std::env::vars() {
                if key == "RUST_LOG" {
                    env_log_level = value;
                    break;
                }
            }

            // define log level
            let mut level_filter = log::LevelFilter::Error;
            if env_log_level == "warn" {
                level_filter = log::LevelFilter::Warn;
            } else if env_log_level == "debug" {
                level_filter = log::LevelFilter::Debug;
            } else if env_log_level == "info" {
                level_filter = log::LevelFilter::Info;
            } else if env_log_level == "trace" {
                level_filter = log::LevelFilter::Trace;
            }

            let env_logger = Box::new(
                pretty_env_logger::formatted_builder()
                    .filter(None, level_filter)
                    .build(),
            );
            let w_logger = FileLogger::new(
                *simplelog::WriteLogger::new(
                    simplelog::LevelFilter::Error,
                    simplelog::Config::default(),
                    File::create(log_file_path).unwrap(),
                ),
                log_config.clone(),
            );
            // Ignore error if global logger was already set (e.g. multi-instance tests).
            let _ = multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info);
        }
    }

    /// Check if this instance has finished initializing
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Get the storage path for this instance
    pub fn storage_path(&self) -> &str {
        self.storage.get_path()
    }

    /// Get reference to the storage module
    pub fn storage(&self) -> &StorageModule {
        &self.storage
    }

    /// Get reference to the node module
    pub fn node(&self) -> &NodeModule {
        &self.node
    }

    /// Get the node's PeerId
    pub fn node_id(&self) -> libp2p::PeerId {
        self.node.id()
    }

    /// Get reference to the router module
    pub fn router(&self) -> &RouterModule {
        &self.router
    }

    /// Get reference to the services module
    pub fn services(&self) -> &std::sync::RwLock<ServicesModule> {
        &self.services
    }

    /// Get reference to the connections module
    pub fn connections(&self) -> &std::sync::RwLock<ConnectionsModule> {
        &self.connections
    }

    /// Get reference to the RPC module
    pub fn rpc_module(&self) -> &std::sync::RwLock<RpcModule> {
        &self.rpc
    }

    /// Run the main event loop
    ///
    /// This method runs forever, processing events from all modules.
    /// It should be called after `new()` returns.
    pub async fn run(&self) {
        // initialize Connection Modules
        let conn = Connections::init(&*self.state).await;
        let mut internet = conn.internet.unwrap();
        let mut lan = conn.lan.unwrap();

        // Set up all the tickers for periodic tasks
        let mut rpc_ticker = Ticker::new(Duration::from_millis(10));
        let mut sys_ticker = Ticker::new(Duration::from_millis(10));
        let mut flooding_ticker = Ticker::new(Duration::from_millis(100));
        let mut feedreq_ticker = Ticker::new(Duration::from_millis(100));
        let mut feedresp_ticker = Ticker::new(Duration::from_millis(100));
        let mut userreq_ticker = Ticker::new(Duration::from_millis(100));
        let mut userresp_ticker = Ticker::new(Duration::from_millis(100));
        let mut routing_info_ticker = Ticker::new(Duration::from_millis(100));
        let mut connection_ticker = Ticker::new(Duration::from_millis(1000));
        let mut routing_table_ticker = Ticker::new(Duration::from_millis(1000));
        let mut messaging_ticker = Ticker::new(Duration::from_millis(10));
        let mut retransmit_ticker = Ticker::new(Duration::from_millis(1000));

        // Mark as initialized (both on Libqaul and QaulState for API compatibility)
        self.initialized.store(true, Ordering::SeqCst);
        self.state.initialized.store(true, Ordering::SeqCst);

        log::trace!("initializing finished, start event loop");

        // Run the event loop
        self.event_loop(
            &mut lan,
            &mut internet,
            &mut rpc_ticker,
            &mut sys_ticker,
            &mut flooding_ticker,
            &mut feedreq_ticker,
            &mut feedresp_ticker,
            &mut userreq_ticker,
            &mut userresp_ticker,
            &mut routing_info_ticker,
            &mut connection_ticker,
            &mut routing_table_ticker,
            &mut messaging_ticker,
            &mut retransmit_ticker,
        )
        .await;
    }

    /// Internal event loop implementation
    #[allow(clippy::too_many_arguments)]
    async fn event_loop(
        &self,
        lan: &mut connections::lan::Lan,
        internet: &mut connections::internet::Internet,
        rpc_ticker: &mut Ticker,
        sys_ticker: &mut Ticker,
        flooding_ticker: &mut Ticker,
        feedreq_ticker: &mut Ticker,
        feedresp_ticker: &mut Ticker,
        userreq_ticker: &mut Ticker,
        userresp_ticker: &mut Ticker,
        routing_info_ticker: &mut Ticker,
        connection_ticker: &mut Ticker,
        routing_table_ticker: &mut Ticker,
        messaging_ticker: &mut Ticker,
        retransmit_ticker: &mut Ticker,
    ) {
        // Take a snapshot of the router state once; it doesn't change after init.
        let router = self.state.get_router();
        loop {
            let evt = {
                let lan_fut = lan.swarm.next().fuse();
                let internet_fut = internet.swarm.next().fuse();
                let rpc_fut = rpc_ticker.next().fuse();
                let sys_fut = sys_ticker.next().fuse();
                let flooding_fut = flooding_ticker.next().fuse();
                let feedreq_fut = feedreq_ticker.next().fuse();
                let feedresp_fut = feedresp_ticker.next().fuse();
                let userreq_fut = userreq_ticker.next().fuse();
                let userresp_fut = userresp_ticker.next().fuse();
                let routing_info_fut = routing_info_ticker.next().fuse();
                let connection_fut = connection_ticker.next().fuse();
                let routing_table_fut = routing_table_ticker.next().fuse();
                let messaging_fut = messaging_ticker.next().fuse();
                let retransmit_fut = retransmit_ticker.next().fuse();

                pin_mut!(
                    lan_fut,
                    internet_fut,
                    rpc_fut,
                    sys_fut,
                    flooding_fut,
                    feedreq_fut,
                    feedresp_fut,
                    userreq_fut,
                    userresp_fut,
                    routing_info_fut,
                    connection_fut,
                    routing_table_fut,
                    messaging_fut,
                    retransmit_fut,
                );

                select! {
                    lan_event = lan_fut => {
                        match lan_event.unwrap() {
                            libp2p::swarm::SwarmEvent::ConnectionEstablished{peer_id,  ..} => {
                                log::trace!("lan connection established: {:?}", peer_id);
                            }
                            libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, ..} => {
                                log::trace!("lan connection closed: {:?}", peer_id);
                                router.neighbours.delete(ConnectionModule::Lan, peer_id);
                            },
                            libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                                lan.swarm.behaviour_mut().process_events(&*self.state, behaviour);
                            }
                            _ => {}
                        }
                        None
                    },
                    internet_event = internet_fut => {
                        match internet_event.unwrap() {
                            libp2p::swarm::SwarmEvent::OutgoingConnectionError{error, ..} => {
                                match error {
                                    libp2p::swarm::DialError::Transport(unreachable_addrs) => {
                                        for (addr, _) in unreachable_addrs {
                                            if Internet::is_active_connection(&*self.state, &addr){
                                                self.state.connections.internet.add_reconnection(addr);
                                            }
                                        }
                                    },
                                    _ => {
                                        log::trace!("INTERNET Outgoing Connection Error");
                                    }
                                }
                            }
                            libp2p::swarm::SwarmEvent::ConnectionEstablished{peer_id, endpoint, ..} => {
                                match endpoint{
                                    libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                        log::info!("connection established! peer={}, endpoint={}", peer_id.to_base58(), address.to_string());
                                        self.state.connections.internet.remove_reconnection(address.clone());
                                        self.state.connections.internet.add_connection(address.to_string(), &peer_id);
                                    }
                                    _ => {}
                                }
                            }
                            libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, endpoint, ..} => {
                                log::trace!("internet connection closed: {:?}", peer_id);
                                router.neighbours.delete(ConnectionModule::Internet, peer_id);

                                match endpoint {
                                    libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                        if Internet::is_active_connection(&*self.state, &address){
                                            self.state.connections.internet.add_reconnection(address);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                                internet.swarm.behaviour_mut().process_events(&*self.state, behaviour);
                            }
                            _ => {}
                        }
                        None
                    },
                    _rpc_event = rpc_fut => Some(EventType::Rpc),
                    _sys_event = sys_fut => Some(EventType::Sys),
                    _flooding_event = flooding_fut => Some(EventType::Flooding),
                    _feedreq_event = feedreq_fut => Some(EventType::FeedRequest),
                    _feedresp_event = feedresp_fut => Some(EventType::FeedResponse),
                    _userreq_event = userreq_fut => Some(EventType::UserRequest),
                    _userresp_event = userresp_fut => Some(EventType::UserResponse),
                    _routing_info_event = routing_info_fut => Some(EventType::RoutingInfo),
                    _connection_event = connection_fut => Some(EventType::ReConnecting),
                    _routing_table_event = routing_table_fut => Some(EventType::RoutingTable),
                    _messaging_event = messaging_fut => Some(EventType::Messaging),
                    _retransmit_event = retransmit_fut => Some(EventType::Retransmit),
                }
            };

            if let Some(event) = evt {
                self.handle_event(event, lan, internet).await;
            }
        }
    }

    /// Handle a single event from the event loop.
    async fn handle_event(
        &self,
        event: EventType,
        lan: &mut connections::lan::Lan,
        internet: &mut connections::internet::Internet,
    ) {
        // Reuse the router snapshot taken in event_loop() instead of cloning the Arc again.
        let router = self.state.get_router();
        match event {
            EventType::Rpc => {
                if let Ok(rpc_message) = self.rpc_receiver.try_recv() {
                    Rpc::process_received_message(&*self.state, rpc_message, Some(lan), Some(internet)).await;
                }
            }
            EventType::Sys => {
                if let Ok(sys_message) = self.sys_receiver.try_recv() {
                    Sys::process_received_message(&*self.state, sys_message, Some(lan), Some(internet));
                }
            }
            EventType::Flooding => {
                let mut flooder = router.flooder.inner.write().unwrap();
                while let Some(msg) = flooder.to_send.pop_front() {
                    if !matches!(msg.incoming_via, ConnectionModule::Lan) {
                        lan.swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(msg.topic.clone(), msg.message.clone());
                    }
                    if !matches!(msg.incoming_via, ConnectionModule::Internet) {
                        internet
                            .swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(msg.topic.clone(), msg.message.clone());
                    }
                    if !matches!(msg.incoming_via, ConnectionModule::Ble) {
                        Ble::send_feed_message(&*self.state, msg.topic, msg.message);
                    }
                }
            }
            EventType::FeedRequest => {
                let mut feed_requester =
                    router.feed_requester.inner.write().unwrap();
                while let Some(request) = feed_requester.to_send.pop_front() {
                    let connection_module =
                        router.neighbours.is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_feed_request(&*self.state, &request.feed_ids);
                    Self::send_via_module(
                        &*self.state,
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::FeedResponse => {
                let mut feed_responser =
                    router.feed_responser.inner.write().unwrap();
                while let Some(request) = feed_responser.to_send.pop_front() {
                    let connection_module =
                        router.neighbours.is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_feed_response(&*self.state, &request.feeds);
                    Self::send_via_module(
                        &*self.state,
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::UserRequest => {
                let mut user_requester =
                    router.user_requester.inner.write().unwrap();
                while let Some(request) = user_requester.to_send.pop_front() {
                    let connection_module =
                        router.neighbours.is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_user_request(&*self.state, &request.user_ids);
                    Self::send_via_module(
                        &*self.state,
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::UserResponse => {
                let mut user_responser =
                    router.user_responser.inner.write().unwrap();
                while let Some(request) = user_responser.to_send.pop_front() {
                    let connection_module =
                        router.neighbours.is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_user_response(&*self.state, &request.users);
                    Self::send_via_module(
                        &*self.state,
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::RoutingInfo => {
                if let Some((neighbour_id, connection_module, data)) = RouterInfo::check_scheduler(&*self.state, &router)
                {
                    log::trace!(
                        "sending routing information via {:?} to {:?}, {:?}",
                        connection_module,
                        neighbour_id,
                        Timestamp::get_timestamp()
                    );
                    Self::send_via_module(&*self.state, connection_module, neighbour_id, data, lan, internet);
                }
            }
            EventType::ReConnecting => {
                if let Some(addr) = self.state.connections.internet.check_reconnection() {
                    log::trace!("redial....: {:?}", addr);
                    Internet::peer_redial(&addr, &mut internet.swarm).await;
                    self.state.connections.internet.set_redialed(&addr);
                }
            }
            EventType::RoutingTable => {
                let table = router
                    .connections
                    .create_routing_table(&router.configuration);
                router.routing_table.set(table);
            }
            EventType::Messaging => {
                if let Some((neighbour_id, connection_module, data)) =
                    self.state
                        .services
                        .messaging
                        .check_scheduler(&router.routing_table)
                {
                    log::trace!(
                        "sending messaging message via {:?} to {}",
                        connection_module,
                        neighbour_id.to_base58()
                    );
                    match connection_module {
                        ConnectionModule::Lan => {
                            lan.swarm
                                .behaviour_mut()
                                .qaul_messaging
                                .send_qaul_messaging_message(neighbour_id, data);
                        }
                        ConnectionModule::Internet => {
                            internet
                                .swarm
                                .behaviour_mut()
                                .qaul_messaging
                                .send_qaul_messaging_message(neighbour_id, data);
                        }
                        ConnectionModule::Ble => {
                            Ble::send_messaging_message(&*self.state, neighbour_id, data);
                        }
                        ConnectionModule::Local => {
                            let message = qaul_messaging::types::QaulMessagingReceived {
                                received_from: neighbour_id,
                                data,
                            };
                            Messaging::received(&*self.state, message);
                        }
                        ConnectionModule::None => {}
                    }
                }
            }
            EventType::Retransmit => {
                // MessagingRetransmit::process() uses Messaging::state() for
                // unconfirmed messages, RoutingTable::get_online_users, and
                // Messaging::schedule_message — all routed through QaulState.
                services::messaging::retransmit::MessagingRetransmit::process(&*self.state);
            }
        }
    }

    /// Helper to send data via the appropriate connection module
    fn send_via_module(
        state: &crate::QaulState,
        connection_module: ConnectionModule,
        neighbour_id: libp2p::PeerId,
        data: Vec<u8>,
        lan: &mut connections::lan::Lan,
        internet: &mut connections::internet::Internet,
    ) {
        match connection_module {
            ConnectionModule::Lan => lan
                .swarm
                .behaviour_mut()
                .qaul_info
                .send_qaul_info_message(neighbour_id, data),
            ConnectionModule::Internet => internet
                .swarm
                .behaviour_mut()
                .qaul_info
                .send_qaul_info_message(neighbour_id, data),
            ConnectionModule::Ble => {
                Ble::send_routing_info(state, neighbour_id, data);
            }
            ConnectionModule::Local => {}
            ConnectionModule::None => {}
        }
    }
}

/// Create a new Libqaul instance
///
/// This is the new API that returns an `Arc<Libqaul>` for shared ownership.
/// Use this for integration tests or when you need multiple instances.
///
/// # Example
///
/// ```rust,ignore
/// let instance = libqaul::start_instance(storage_path, None).await;
/// instance.run().await;
/// ```
pub async fn start_instance(
    storage_path: String,
    def_config: Option<BTreeMap<String, String>>,
) -> Arc<Libqaul> {
    Libqaul::new(storage_path, def_config).await
}

/// To see logs on android we need the android logger
#[cfg(target_os = "android")]
extern crate log;

#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(target_os = "android")]
use android_logger::Config;

/// Event Types of the async loop
enum EventType {
    Rpc,
    Sys,
    Flooding,
    FeedRequest,
    FeedResponse,
    UserRequest,
    UserResponse,
    RoutingInfo,
    ReConnecting,
    RoutingTable,
    Messaging,
    Retransmit,
}

/// Legacy entry point — removed in favor of `Libqaul::new()` + `Libqaul::run()`.
/// Use `start_instance()` or `api::start_instance_in_thread()` instead.
#[deprecated(since = "2.0.0", note = "Use start_instance() instead")]
pub async fn start(storage_path: String, def_config: Option<BTreeMap<String, String>>) -> () {
    let instance = start_instance(storage_path, def_config).await;
    instance.run().await;
}
