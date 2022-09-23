// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # libqaul
//!
//! Library for qaul.net

// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use crate::utilities::filelogger::FileLogger;
use crate::utilities::timestamp::Timestamp;
use filetime::FileTime;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use state::Storage;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

// crate modules
pub mod api;
mod connections;
pub mod node;
mod router;
mod rpc;
mod services;
pub mod storage;
mod types;
pub mod utilities;

use connections::{ble::Ble, internet::Internet, ConnectionModule, Connections};
use node::Node;
use router::{
    feed_requester, flooder, info::RouterInfo, neighbours::Neighbours, user_requester, Router,
};
use rpc::sys::Sys;
use rpc::Rpc;
use services::messaging::Messaging;
use services::Services;

/// check this when the library finished initializing
static INITIALIZED: Storage<bool> = Storage::new();

/// default configs
static DEFCONFIGS: Storage<BTreeMap<String, String>> = Storage::new();

/// To see logs on android we need the android logger
#[cfg(target_os = "android")]
extern crate log;
#[cfg(target_os = "android")]
use log::Level;

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

/// Events of the async loop
enum EventType {
    Rpc(bool),
    Sys(bool),
    Flooding(bool),
    FeedRequest(bool),
    FeedResponse(bool),
    UserRequest(bool),
    UserResponse(bool),
    RoutingInfo(bool),
    ReConnecting(bool),
    RoutingTable(bool),
    Messaging(bool),
    Retransmit(bool),
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
    log::trace!("start initializing libqaul");

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
            Config::default().with_min_level(Level::Info),
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
                        libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::trace!("lan connection closed: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Lan, peer_id);
                        },
                        libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::trace!("lan connection banned: {:?}", peer_id);
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
                    //log::trace!("Unhandled internet connection module event: {:?}", internet_event);
                    match internet_event.unwrap() {
                        libp2p::swarm::SwarmEvent::OutgoingConnectionError{error, ..} => {
                            // Get list of addresses which we failed to connect to
                            // Since `UnknownPeerUnreachableAddr` error was removed, we need to parse
                            // list of outgoing connection errors to get list of addresses
                            match error {
                                libp2p::swarm::DialError::Transport(unreachable_addrs) => {
                                    for (addr, _) in unreachable_addrs {
                                        Internet::add_reconnection(addr);
                                    }
                                },
                                _ => {}
                            }
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished{endpoint, ..} =>{
                            //remove from attempting connections
                            match endpoint{
                                libp2p::core::ConnectedPoint::Dialer{address, ..} =>{
                                    Internet::remove_reconnection(address);
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
                                    Internet::add_reconnection(address);
                                }
                                _ => {}
                            }
                        }
                        libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::trace!("internet connection banned: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Internet, peer_id);
                        },
                        libp2p::swarm::SwarmEvent::Behaviour(behaviour) => {
                            internet.swarm.behaviour_mut().process_events(behaviour);
                        }
                        _ => {}
                    }
                    None
                },
                _rpc_event = rpc_fut => Some(EventType::Rpc(true)),
                _sys_event = sys_fut => Some(EventType::Sys(true)),
                _flooding_event = flooding_fut => Some(EventType::Flooding(true)),
                _feedreq_event = feedreq_fut => Some(EventType::FeedRequest(true)),
                _feedresp_event = feedresp_fut => Some(EventType::FeedResponse(true)),
                _userreq_event = userreq_fut => Some(EventType::UserRequest(true)),
                _userresp_event = userresp_fut => Some(EventType::UserResponse(true)),
                _routing_info_event = routing_info_fut => Some(EventType::RoutingInfo(true)),
                _connection_event = connection_fut => Some(EventType::ReConnecting(true)),
                _routing_table_event = routing_table_fut => Some(EventType::RoutingTable(true)),
                _messaging_event = messaging_fut => Some(EventType::Messaging(true)),
                _retransmit_event = retransmit_fut => Some(EventType::Retransmit(true)),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Rpc(_) => {
                    if let Ok(rpc_message) = libqaul_rpc_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Rpc::process_received_message(
                            rpc_message,
                            Some(&mut lan),
                            Some(&mut internet),
                        )
                        .await;
                    }
                }
                EventType::Sys(_) => {
                    if let Ok(sys_message) = libqaul_sys_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Sys::process_received_message(
                            sys_message,
                            Some(&mut lan),
                            Some(&mut internet),
                        );
                    }
                }
                EventType::Flooding(_) => {
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
                EventType::FeedRequest(_) => {
                    // send messages in the flooding queue
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
                        //make dataMessaging
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
                EventType::FeedResponse(_) => {
                    // send messages in the flooding queue
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

                        //make data
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
                EventType::UserRequest(_) => {
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
                        //make dataMessaging
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
                EventType::UserResponse(_) => {
                    // send messages in the flooding queue
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

                        //make data
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

                EventType::RoutingInfo(_) => {
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
                EventType::ReConnecting(_) => {
                    if let Some(addr) = Internet::check_reconnection() {
                        log::trace!("redial....: {:?}", addr);
                        Internet::peer_redial(&addr, &mut internet.swarm).await;
                        Internet::set_redialed(&addr);
                    }
                }
                EventType::RoutingTable(_) => {
                    // create new routing table
                    router::connections::ConnectionTable::create_routing_table();
                }
                EventType::Messaging(_) => {
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
                EventType::Retransmit(_) => {
                    services::messaging::retransmit::MessagingRetransmit::process();
                }
            }
        }
    }
}
