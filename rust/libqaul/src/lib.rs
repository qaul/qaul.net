// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # libqaul
//!
//! Library for qaul

use crate::rpc::authentication::Authentication;
use crossbeam_channel::Receiver;
use filetime::FileTime;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use state::InitCell;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

// crate modules
pub mod api;
mod connections;
pub mod node;
mod router;
mod rpc;
mod services;
pub mod storage;
pub mod utilities;

use connections::{ble::Ble, internet::Internet, ConnectionModule, Connections, ConnectionsModule};
use node::{Node, NodeModule};
use router::{
    feed_requester, flooder, info::RouterInfo, neighbours::Neighbours, user_requester, Router,
    RouterModule,
};
use rpc::sys::Sys;
use rpc::{Rpc, RpcModule};
use services::messaging::Messaging;
use services::{Services, ServicesModule};
use storage::StorageModule;
use utilities::filelogger::FileLogger;
use utilities::timestamp::Timestamp;
use utilities::upgrade;

/// check this when the library finished initializing
static INITIALIZED: InitCell<bool> = InitCell::new();

/// default configs
static DEFCONFIGS: InitCell<BTreeMap<String, String>> = InitCell::new();

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
        if let Some(def_cfg) = def_config {
            DEFCONFIGS.set(def_cfg.clone());
        } else {
            DEFCONFIGS.set(BTreeMap::new());
        }

        // initialize rpc system
        let rpc_receiver = Rpc::init();
        let sys_receiver = Sys::init();

        // initialize storage module (instance-based)
        // This also sets up global state for backward compatibility
        let storage = Arc::new(StorageModule::new(storage_path.clone()));

        // Initialize logger
        Self::init_logger(&storage_path);

        log::trace!("test log to ensure that logging is working");

        // initialize node module (instance-based)
        let node = {
            let config = storage.config.read().unwrap();
            Arc::new(NodeModule::new(&config))
        };

        // Also initialize global state for backward compatibility
        Node::init();

        // initialize router module (instance-based)
        let router = {
            let config = storage.config.read().unwrap();
            Arc::new(RouterModule::new(&config))
        };

        // Also initialize global state for backward compatibility
        Router::init();

        // initialize services module (instance-based)
        let services = Arc::new(std::sync::RwLock::new(ServicesModule::new()));
        {
            let mut svc = services.write().unwrap();
            svc.initialize();
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
    fn init_logger(storage_path: &str) {
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
            let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
                simplelog::LevelFilter::Error,
                simplelog::Config::default(),
                File::create(log_file_path).unwrap(),
            ));
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
            let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
                simplelog::LevelFilter::Error,
                simplelog::Config::default(),
                File::create(log_file_path).unwrap(),
            ));
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
            let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
                simplelog::LevelFilter::Error,
                simplelog::Config::default(),
                File::create(log_file_path).unwrap(),
            ));
            multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info)
                .unwrap();
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
        let conn = Connections::init().await;
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

        // Mark as initialized
        self.initialized.store(true, Ordering::SeqCst);
        INITIALIZED.set(true);

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
                                Neighbours::delete(ConnectionModule::Lan, peer_id);
                            },
                            libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                                lan.swarm.behaviour_mut().process_events(behaviour);
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
                                            if Internet::is_active_connection(&addr){
                                                Internet::add_reconnection(addr);
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
                                        Internet::remove_reconnection(address.clone());
                                        Internet::add_connection(address.to_string(), &peer_id);
                                    }
                                    _ => {}
                                }
                            }
                            libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, endpoint, ..} => {
                                log::trace!("internet connection closed: {:?}", peer_id);
                                Neighbours::delete(ConnectionModule::Internet, peer_id);

                                match endpoint {
                                    libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                        if Internet::is_active_connection(&address){
                                            Internet::add_reconnection(address);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                                internet.swarm.behaviour_mut().process_events(behaviour);
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

    /// Handle a single event from the event loop
    async fn handle_event(
        &self,
        event: EventType,
        lan: &mut connections::lan::Lan,
        internet: &mut connections::internet::Internet,
    ) {
        match event {
            EventType::Rpc => {
                if let Ok(rpc_message) = self.rpc_receiver.try_recv() {
                    Rpc::process_received_message(rpc_message, Some(lan), Some(internet)).await;
                }
            }
            EventType::Sys => {
                if let Ok(sys_message) = self.sys_receiver.try_recv() {
                    Sys::process_received_message(sys_message, Some(lan), Some(internet));
                }
            }
            EventType::Flooding => {
                let mut flooder = flooder::FLOODER.get().write().unwrap();
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
                        Ble::send_feed_message(msg.topic, msg.message);
                    }
                }
            }
            EventType::FeedRequest => {
                let mut feed_requester = feed_requester::FEEDREQUESTER.get().write().unwrap();
                while let Some(request) = feed_requester.to_send.pop_front() {
                    let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_feed_request(&request.feed_ids);
                    Self::send_via_module(
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::FeedResponse => {
                let mut feed_responser = feed_requester::FEEDRESPONSER.get().write().unwrap();
                while let Some(request) = feed_responser.to_send.pop_front() {
                    let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_feed_response(&request.feeds);
                    Self::send_via_module(
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::UserRequest => {
                let mut user_requester = user_requester::USERREQUESTER.get().write().unwrap();
                while let Some(request) = user_requester.to_send.pop_front() {
                    let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_user_request(&request.user_ids);
                    Self::send_via_module(
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::UserResponse => {
                let mut user_responser = user_requester::USERRESPONSER.get().write().unwrap();
                while let Some(request) = user_responser.to_send.pop_front() {
                    let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                    if connection_module == ConnectionModule::None {
                        log::error!(
                            "sending feed requests, node is not a neighbour anymore: {:?}",
                            request.neighbour_id
                        );
                        continue;
                    }
                    let data = RouterInfo::create_user_response(&request.users);
                    Self::send_via_module(
                        connection_module,
                        request.neighbour_id,
                        data,
                        lan,
                        internet,
                    );
                }
            }
            EventType::RoutingInfo => {
                if let Some((neighbour_id, connection_module, data)) = RouterInfo::check_scheduler()
                {
                    log::trace!(
                        "sending routing information via {:?} to {:?}, {:?}",
                        connection_module,
                        neighbour_id,
                        Timestamp::get_timestamp()
                    );
                    Self::send_via_module(connection_module, neighbour_id, data, lan, internet);
                }
            }
            EventType::ReConnecting => {
                if let Some(addr) = Internet::check_reconnection() {
                    log::trace!("redial....: {:?}", addr);
                    Internet::peer_redial(&addr, &mut internet.swarm).await;
                    Internet::set_redialed(&addr);
                }
            }
            EventType::RoutingTable => {
                router::connections::ConnectionTable::create_routing_table();
            }
            EventType::Messaging => {
                if let Some((neighbour_id, connection_module, data)) = Messaging::check_scheduler()
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
                            Ble::send_messaging_message(neighbour_id, data);
                        }
                        ConnectionModule::Local => {
                            let message = qaul_messaging::types::QaulMessagingReceived {
                                received_from: neighbour_id,
                                data,
                            };
                            Messaging::received(message);
                        }
                        ConnectionModule::None => {}
                    }
                }
            }
            EventType::Retransmit => {
                services::messaging::retransmit::MessagingRetransmit::process();
            }
        }
    }

    /// Helper to send data via the appropriate connection module
    fn send_via_module(
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
                Ble::send_routing_info(neighbour_id, data);
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

/// Get default config values
pub fn get_default_config(pattern: &str) -> Option<String> {
    let def_config = DEFCONFIGS.get();
    if let Some(v) = def_config.get(&pattern.to_string()) {
        return Some(v.clone());
    }
    None
}

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

/// initialize and start libqaul with a optional custom configuration options
/// and poll all the necessary modules
///
/// Input Values:
///
/// * Provide a path where libqaul can save all data.
/// * Optionally you can provide the following configuration values:
///   * listening port of the Internet connection module (default = randomly assigned)
pub async fn start(storage_path: String, def_config: Option<BTreeMap<String, String>>) -> () {
    // print storage path
    println!("storage path: {}", storage_path);

    // check if we need to upgrade our stored data
    if upgrade::Upgrade::init(storage_path.clone()) == false {
        println!("upgrade to new version failed");
        // restart node
        std::process::exit(0);
    }

    // check configuration options
    if let Some(def_cfg) = def_config {
        DEFCONFIGS.set(def_cfg.clone());
    } else {
        DEFCONFIGS.set(BTreeMap::new());
    }

    // initialize rpc system
    let libqaul_rpc_receive = Rpc::init();
    let libqaul_sys_receive = Sys::init();

    // initialize storage module.
    // This will initialize configuration & data base
    storage::Storage::init(storage_path.clone());

    // --- initialize logger ---
    // prepare logger path
    // the path of the log file follows the following naming convention:
    // error_234324232.log
    let path = Path::new(&storage_path);
    let log_path = path.join("logs");

    // create log directory if missing
    std::fs::create_dir_all(&log_path).unwrap();

    // create log file name
    let log_file_name: String =
        "error_".to_string() + Timestamp::get_timestamp().to_string().as_str() + ".log";
    let log_file_path = log_path.join(log_file_name);

    // maintain log files
    let paths = std::fs::read_dir(log_path).unwrap();
    // --- logger init-end ---

    let mut logfiles: BTreeMap<i64, String> = BTreeMap::new();
    let mut logfile_times: Vec<i64> = vec![];
    for path in paths {
        let filename = String::from(path.as_ref().unwrap().path().to_str().unwrap());
        let metadata = std::fs::metadata(filename.clone()).unwrap();
        //print!("path={}", path.unwrap().path().display());
        let mtime = FileTime::from_last_modification_time(&metadata);
        //println!("{}", mtime.seconds());
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
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
            simplelog::LevelFilter::Error,
            simplelog::Config::default(),
            File::create(log_file_path).unwrap(),
        ));
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
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
            simplelog::LevelFilter::Error,
            simplelog::Config::default(),
            File::create(log_file_path).unwrap(),
        ));
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
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(
            simplelog::LevelFilter::Error,
            simplelog::Config::default(),
            File::create(log_file_path).unwrap(),
        ));
        multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info)
            .unwrap();
    }

    log::trace!("test log to ensure that logging is working");

    // initialize node & user accounts
    Node::init();

    // initialize router
    Router::init();

    Authentication::init();

    // initialize Connection Modules
    let conn = Connections::init().await;
    let mut internet = conn.internet.unwrap();
    let mut lan = conn.lan.unwrap();

    // initialize services
    Services::init();

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut rpc_ticker = Ticker::new(Duration::from_millis(10));

    // check SYS once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut sys_ticker = Ticker::new(Duration::from_millis(10));

    // check flooding message queue periodically
    let mut flooding_ticker = Ticker::new(Duration::from_millis(100));

    // check feed request to neighbour
    let mut feedreq_ticker = Ticker::new(Duration::from_millis(100));

    // check feed request to neighbour
    let mut feedresp_ticker = Ticker::new(Duration::from_millis(100));

    // check user request to neighbour
    let mut userreq_ticker = Ticker::new(Duration::from_millis(100));

    // check user request to neighbour
    let mut userresp_ticker = Ticker::new(Duration::from_millis(100));

    // send routing info periodically to neighbours
    let mut routing_info_ticker = Ticker::new(Duration::from_millis(100));

    // try to connect to intertnet neighbour if there is no connection in internet
    let mut connection_ticker = Ticker::new(Duration::from_millis(1000));

    // re-create routing table periodically
    let mut routing_table_ticker = Ticker::new(Duration::from_millis(1000));

    // manage the message sending
    let mut messaging_ticker = Ticker::new(Duration::from_millis(10));

    // manage the message retransmit
    let mut retransmit_ticker = Ticker::new(Duration::from_millis(1000));

    // set initialized flag
    INITIALIZED.set(true);

    log::trace!("initializing finished, start event loop");

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

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
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
                    //log::trace!("Unhandled lan connection module event: {:?}", lan_event);
                    match lan_event.unwrap() {
                        libp2p::swarm::SwarmEvent::ConnectionEstablished{peer_id,  ..} => {
                            log::trace!("lan connection established: {:?}", peer_id);
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::trace!("lan connection closed: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Lan, peer_id);
                        },
                        // libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {
                        //     //remove from neighbour table, after then scheduler will auto remove this neighbour
                        //     log::trace!("lan connection banned: {:?}", peer_id);
                        //     Neighbours::delete(ConnectionModule::Lan, peer_id);
                        // },
                        libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                            lan.swarm.behaviour_mut().process_events(behaviour);
                        }
                        _ => {}
                    }
                    None
                },
                internet_event = internet_fut => {
                    //log::trace!("Unhandled internet connection module event: {:?}", internet_event);
                    match internet_event.unwrap() {
                        libp2p::swarm::SwarmEvent::OutgoingConnectionError{error, ..} => {
                            // Get list of addresses which we failed to connect to
                            // Since `UnknownPeerUnreachableAddr` error was removed, we need to parse
                            // list of outgoing connection errors to get list of addresses
                            match error {
                                libp2p::swarm::DialError::Transport(unreachable_addrs) => {
                                    for (addr, _) in unreachable_addrs {

                                        // check if address is active
                                        if Internet::is_active_connection(&addr){
                                            Internet::add_reconnection(addr);
                                        }

                                    }
                                },
                                _ => {
                                    log::trace!("INTERNET Outgoing Connection Error");
                                }
                            }
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished{peer_id, endpoint, ..} => {
                            // remove from attempting connections
                            match endpoint{
                                libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                    log::info!("connection established! peer={}, endpoint={}", peer_id.to_base58(), address.to_string());
                                    Internet::remove_reconnection(address.clone());
                                    Internet::add_connection(address.to_string(), &peer_id);
                                }
                                _ => {}
                            }
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, endpoint, ..} => {
                            // remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::trace!("internet connection closed: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Internet, peer_id);

                            // add new reconnection
                            match endpoint {
                                libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                    //check if address is active
                                    if Internet::is_active_connection(&address){
                                        Internet::add_reconnection(address);
                                    }
                                }
                                _ => {}
                            }
                        }
                        // libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {
                        //     // remove from neighbour table, after then scheduler will auto remove this neighbour
                        //     log::trace!("internet connection banned: {:?}", peer_id);
                        //     Neighbours::delete(ConnectionModule::Internet, peer_id);
                        // }
                        libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                            internet.swarm.behaviour_mut().process_events(behaviour);
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
            match event {
                EventType::Rpc => {
                    if let Ok(rpc_message) = libqaul_rpc_receive.try_recv() {
                        // we received a message, send it to RPC
                        Rpc::process_received_message(
                            rpc_message,
                            Some(&mut lan),
                            Some(&mut internet),
                        )
                        .await;
                    }
                }
                EventType::Sys => {
                    if let Ok(sys_message) = libqaul_sys_receive.try_recv() {
                        // we received a message, send it to SYS
                        Sys::process_received_message(
                            sys_message,
                            Some(&mut lan),
                            Some(&mut internet),
                        );
                    }
                }
                EventType::Flooding => {
                    // send messages in the flooding queue
                    // get sending queue
                    let mut flooder = flooder::FLOODER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(msg) = flooder.to_send.pop_front() {
                        // check which swarm to send to
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
                            Ble::send_feed_message(msg.topic, msg.message);
                        }
                    }
                }
                EventType::FeedRequest => {
                    // get sending queue
                    let mut feed_requester = feed_requester::FEEDREQUESTER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(request) = feed_requester.to_send.pop_front() {
                        let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                        if connection_module == ConnectionModule::None {
                            log::error!(
                                "sending feed requests, node is not a neighbour anymore: {:?}",
                                request.neighbour_id
                            );
                            continue;
                        }

                        let data = RouterInfo::create_feed_request(&request.feed_ids);
                        match connection_module {
                            ConnectionModule::Lan => lan
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Internet => internet
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Ble => {
                                Ble::send_routing_info(request.neighbour_id, data);
                            }
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
                    }
                }
                EventType::FeedResponse => {
                    // get sending queue
                    let mut feed_responser = feed_requester::FEEDRESPONSER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(request) = feed_responser.to_send.pop_front() {
                        let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                        if connection_module == ConnectionModule::None {
                            log::error!(
                                "sending feed requests, node is not a neighbour anymore: {:?}",
                                request.neighbour_id
                            );
                            continue;
                        }

                        let data = RouterInfo::create_feed_response(&request.feeds);
                        match connection_module {
                            ConnectionModule::Lan => lan
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Internet => internet
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Ble => {
                                Ble::send_routing_info(request.neighbour_id, data);
                            }
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
                    }
                }
                EventType::UserRequest => {
                    // send messages in the flooding queue
                    // get sending queue
                    let mut user_requester = user_requester::USERREQUESTER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(request) = user_requester.to_send.pop_front() {
                        let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                        if connection_module == ConnectionModule::None {
                            log::error!(
                                "sending feed requests, node is not a neighbour anymore: {:?}",
                                request.neighbour_id
                            );
                            continue;
                        }

                        let data = RouterInfo::create_user_request(&request.user_ids);
                        match connection_module {
                            ConnectionModule::Lan => lan
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Internet => internet
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Ble => {
                                Ble::send_routing_info(request.neighbour_id, data);
                            }
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
                    }
                }
                EventType::UserResponse => {
                    // get sending queue
                    let mut user_responser = user_requester::USERRESPONSER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(request) = user_responser.to_send.pop_front() {
                        let connection_module = Neighbours::is_neighbour(&request.neighbour_id);
                        if connection_module == ConnectionModule::None {
                            log::error!(
                                "sending feed requests, node is not a neighbour anymore: {:?}",
                                request.neighbour_id
                            );
                            continue;
                        }

                        // make data
                        let data = RouterInfo::create_user_response(&request.users);
                        match connection_module {
                            ConnectionModule::Lan => lan
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Internet => internet
                                .swarm
                                .behaviour_mut()
                                .qaul_info
                                .send_qaul_info_message(request.neighbour_id, data),
                            ConnectionModule::Ble => {
                                Ble::send_routing_info(request.neighbour_id, data);
                            }
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
                    }
                }

                EventType::RoutingInfo => {
                    // send routing info to neighbours
                    // check scheduler
                    if let Some((neighbour_id, connection_module, data)) =
                        RouterInfo::check_scheduler()
                    {
                        log::trace!(
                            "sending routing information via {:?} to {:?}, {:?}",
                            connection_module,
                            neighbour_id,
                            Timestamp::get_timestamp()
                        );
                        // send routing information
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
                                Ble::send_routing_info(neighbour_id, data);
                            }
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
                    }
                }
                EventType::ReConnecting => {
                    if let Some(addr) = Internet::check_reconnection() {
                        log::trace!("redial....: {:?}", addr);
                        Internet::peer_redial(&addr, &mut internet.swarm).await;
                        Internet::set_redialed(&addr);
                    }
                }
                EventType::RoutingTable => {
                    // create new routing table
                    router::connections::ConnectionTable::create_routing_table();
                }
                EventType::Messaging => {
                    // send scheduled messages
                    if let Some((neighbour_id, connection_module, data)) =
                        Messaging::check_scheduler()
                    {
                        log::trace!(
                            "sending messaging message via {:?} to {}",
                            connection_module,
                            neighbour_id.to_base58()
                        );
                        // send messaging message via the best module
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
                                Ble::send_messaging_message(neighbour_id, data);
                            }
                            ConnectionModule::Local => {
                                let message = qaul_messaging::types::QaulMessagingReceived {
                                    received_from: neighbour_id,
                                    data,
                                };
                                // forward to messaging module
                                Messaging::received(message);
                            }
                            ConnectionModule::None => {
                                // TODO: DTN behaviour
                                // reschedule it for the moment
                            }
                        }
                    }
                }
                EventType::Retransmit => {
                    // check if there are messages to retransmit
                    services::messaging::retransmit::MessagingRetransmit::process();
                }
            }
        }
    }
}
