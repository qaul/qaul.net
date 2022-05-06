// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # libqaul
//!
//! Library for qaul.net

// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use state::Storage;
use std::time::Duration;
use std::fs::File;
use filetime::FileTime;
use std::collections::BTreeMap;
use crate::utilities::timestamp::Timestamp;
use crate::utilities::filelogger::FileLogger;

// crate modules
pub mod api;
pub mod storage;
pub mod utilities;
mod connections;
mod node;
mod router;
mod rpc;
mod services;
mod types;


use connections::{
    ConnectionModule, Connections,
    internet::Internet,   
    ble::Ble,
};

use node::Node;
use router::{
    flooder,
    info::RouterInfo, 
    Router,
    neighbours::Neighbours,
};

use rpc::Rpc;
use rpc::sys::Sys;
use services::Services;
use services::messaging::Messaging;

/// check this when the library finished initializing
static INITIALIZED: Storage<bool> = Storage::new();

/// To see logs on android we need the android logger
#[cfg(target_os = "android")]
extern crate log;
#[cfg(target_os = "android")]
use log::Level;

#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(target_os = "android")]
use android_logger::Config;

/// Events of the async loop
enum EventType {
    Rpc(bool),
    Sys(bool),
    Flooding(bool),
    RoutingInfo(bool),
    ReConnecting(bool),
    RoutingTable(bool),
    Messaging(bool),
}

/// initialize and start libqaul
/// and poll all the necessary modules
/// 
/// Provide a path where libqaul can save all data.
pub async fn start(storage_path: String) -> () {
    log::info!("start initializing libqaul");

    // initialize rpc system
    let libqaul_rpc_receive = Rpc::init();
    let libqaul_sys_receive = Sys::init();

    let storage_p = storage_path.clone();
    // initialize storage module.
    // This will initialize configuration & data base
    storage::Storage::init(storage_path);

    // --- initialize logger ---
    // prepare logger path
    // the path of the log file follows the following naming convention:
    // errror_234324232.log
    let logger_path: String;
    if storage_p.len() == 0{
        logger_path = "./logs/".to_string();
    }else{
        logger_path = storage_p + "/logs/";
    }
    let cur_time_as_ms = Timestamp::get_timestamp();
    let logger_file = logger_path.clone() + "error_" + &cur_time_as_ms.to_string() + ".log";

    // create log directory
    std::fs::create_dir_all(logger_path.clone()).unwrap();

    // maintain log files
    let paths = std::fs::read_dir(logger_path.clone()).unwrap();
    // --- logger init-end ---

    let mut logfiles: BTreeMap<i64, String> = BTreeMap::new();
    let mut logfile_times: Vec<i64> = vec![];
    for path in paths{
        let filename = String::from(path.as_ref().unwrap().path().to_str().unwrap());
        let metadata = std::fs::metadata(filename.clone()).unwrap();
        //print!("path={}", path.unwrap().path().display());
        let mtime = FileTime::from_last_modification_time(&metadata);
        //println!("{}", mtime.seconds());
        logfile_times.push(mtime.seconds());
        logfiles.insert(mtime.seconds(), filename);
    }
    logfile_times.sort();

    if logfile_times.len() > 2{
        for i in 0..(logfile_times.len() - 2) {
            if let Some(filename) = logfiles.get(&logfile_times[i]) {
                std::fs::remove_file(std::path::Path::new(filename)).unwrap();
            }
        }    
    }

    // logging on android with android logger
    #[cfg(target_os = "android")]
    {
        let env_logger = Box::new(android_logger::AndroidLogger::new(Config::default().with_min_level(Level::Info)));
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(simplelog::LevelFilter::Error, simplelog::Config::default(), File::create(logger_file).unwrap()));
        multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info).unwrap();
    }

    // logging on ios
    #[cfg(target_os = "ios")]
    {
        let env_logger = Box::new(pretty_env_logger::formatted_builder().filter(None, log::LevelFilter::Info).build());
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(simplelog::LevelFilter::Error, simplelog::Config::default(), File::create(logger_file).unwrap()));
        multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info).unwrap();
    }

    // only use the simple logger on desktop systems
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        // find rust env var
        let mut env_log_level = String::from("error");
        for (key, value) in std::env::vars() {
            if key == "RUST_LOG"{
                env_log_level = value;
                break;
            }
        }

        // define log level
        let mut level_filter = log::LevelFilter::Error;
        if env_log_level == "warn"{
            level_filter = log::LevelFilter::Warn;
        } else if  env_log_level == "debug" {
            level_filter = log::LevelFilter::Debug;
        } else if env_log_level == "info" {
            level_filter = log::LevelFilter::Info;
        } else if env_log_level == "trace" {
            level_filter = log::LevelFilter::Trace;
        }

        let env_logger = Box::new(pretty_env_logger::formatted_builder().filter(None, level_filter).build());
        let w_logger = FileLogger::new(*simplelog::WriteLogger::new(simplelog::LevelFilter::Error, simplelog::Config::default(), File::create(logger_file).unwrap()));
        multi_log::MultiLogger::init(vec![env_logger, Box::new(w_logger)], log::Level::Info).unwrap();
    }

    log::error!("this is test");
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

    // send routing info periodically to neighbours
    let mut routing_info_ticker = Ticker::new(Duration::from_millis(100));

    // try to connect to intertnet neighbour if there is no connection in internet
    let mut connection_ticker = Ticker::new(Duration::from_millis(1000));

    // re-create routing table periodically
    let mut routing_table_ticker = Ticker::new(Duration::from_millis(1000));

    // manage the message sending
    let mut messaging_ticker = Ticker::new(Duration::from_millis(10));

    // set initialized flag
    INITIALIZED.set(true);

    log::info!("initializing finished, start event loop");

    loop {
        let evt = {
            let lan_fut = lan.swarm.next().fuse();
            let internet_fut = internet.swarm.next().fuse();
            let rpc_fut = rpc_ticker.next().fuse();
            let sys_fut = sys_ticker.next().fuse();
            let flooding_fut = flooding_ticker.next().fuse();
            let routing_info_fut = routing_info_ticker.next().fuse();
            let connection_fut = connection_ticker.next().fuse();
            let routing_table_fut = routing_table_ticker.next().fuse();
            let messaging_fut = messaging_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(
                lan_fut,
                internet_fut,
                rpc_fut,
                sys_fut,
                flooding_fut,
                routing_info_fut,
                connection_fut,
                routing_table_fut,
                messaging_fut,
            );

            select! {
                lan_event = lan_fut => {
                    log::info!("Unhandled lan connection module event: {:?}", lan_event);
                    match lan_event.unwrap() {
                        libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::info!("lan connection closed: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Lan, peer_id);
                        },
                        libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {                            
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::info!("lan connection banned: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Lan, peer_id);
                        }
                        _ => {}
                    }
                    None
                },
                internet_event = internet_fut => {
                    log::info!("Unhandled internet connection module event: {:?}", internet_event);
                    match internet_event.unwrap() {
                        libp2p::swarm::SwarmEvent::UnknownPeerUnreachableAddr{address, ..} => {
                            Internet::add_reconnection(address);
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished{endpoint, ..} =>{
                            //remove from attempting connections
                            match endpoint{
                                libp2p::core::ConnectedPoint::Dialer{address} =>{
                                    Internet::remove_reconnection(address);
                                }
                                _ => {}
                            }                            
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed{peer_id, endpoint, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::info!("internet connection closed: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Internet, peer_id);

                            //add new reconnection
                            match endpoint{
                                libp2p::core::ConnectedPoint::Dialer{address} =>{
                                    Internet::add_reconnection(address);
                                }
                                _ => {}
                            }                            
                        }
                        libp2p::swarm::SwarmEvent::BannedPeer {peer_id, ..} => {
                            //remove from neighbour table, after then scheduler will auto remove this neighbour
                            log::info!("internet connection banned: {:?}", peer_id);
                            Neighbours::delete(ConnectionModule::Internet, peer_id);
                        }
                        _ => {}
                    }
                    None
                },
                _rpc_event = rpc_fut => Some(EventType::Rpc(true)),
                _sys_event = sys_fut => Some(EventType::Sys(true)),
                _flooding_event = flooding_fut => Some(EventType::Flooding(true)),
                _routing_info_event = routing_info_fut => Some(EventType::RoutingInfo(true)),
                _connection_event = connection_fut => Some(EventType::ReConnecting(true)),
                _routing_table_event = routing_table_fut => Some(EventType::RoutingTable(true)),
                _messaging_event = messaging_fut => Some(EventType::Messaging(true)),
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
                        );
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
                EventType::RoutingInfo(_) => {
                    // send routing info to neighbours
                    // check scheduler
                    if let Some((neighbour_id, connection_module, data)) =
                        RouterInfo::check_scheduler()
                    {
                        log::info!(
                            "sending routing information via {:?} to {:?}",
                            connection_module,
                            neighbour_id
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
                EventType::ReConnecting(_) =>{
                    if let Some(addr) = Internet::check_reconnection() {
                        log::info!("redial....: {:?}", addr);
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
                        log::info!(
                            "sending messaging message via {:?} to {:?}",
                            connection_module,
                            neighbour_id
                        );
                        // send messaging message via the best module
                        match connection_module {
                            ConnectionModule::Lan => {
                                lan
                                    .swarm
                                    .behaviour_mut()
                                    .qaul_messaging
                                    .send_qaul_messaging_message(neighbour_id, data);
                            },
                            ConnectionModule::Internet => {
                                internet
                                    .swarm
                                    .behaviour_mut()
                                    .qaul_messaging
                                    .send_qaul_messaging_message(neighbour_id, data);
                            },
                            ConnectionModule::Ble => {
                                Ble::send_messaging_message(neighbour_id, data);
                            },
                            ConnectionModule::Local => {
                                let message = qaul_messaging::types::QaulMessagingReceived {
                                    received_from: neighbour_id,
                                    data,
                                };
                                // forward to messaging module
                                Messaging::received(message);
                            },
                            ConnectionModule::None => {
                                // TODO: DTN behaviour
                                // reschedule it for the moment

                            },
                        }
                    }
                }
            }
        }
    }
}

/// ANDROID TESTING
/// initialize libqaul for android
/// and poll all the necessary modules
///
/// This function is here to test the initialization of libqaul
/// on android.
pub async fn start_android(storage_path: String) -> () {
    start(storage_path).await
}
