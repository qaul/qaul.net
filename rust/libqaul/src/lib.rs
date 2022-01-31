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

use connections::{ConnectionModule, Connections};
use node::Node;
use router::{
    flooder,
    info::RouterInfo, 
    Router,
};
use rpc::Rpc;
use services::Services;
use services::messaging::Messaging;
use utilities::Utilities;

/// check this when the library finished initializing
static INITIALIZED: Storage<bool> = Storage::new();

/// To see logs on android we need the android logger
#[cfg(target_os = "android")]
#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
use log::Level;

#[cfg(target_os = "android")]
extern crate android_logger;
#[cfg(target_os = "android")]
use android_logger::Config;

/// Initialize android logger
/// This sends logs to android logger, otherwise
/// the logs are not visible on android.
/// This function is only activated on android OS
#[cfg(target_os = "android")]
pub fn initialize_android_logging() {
    android_logger::init_once(
        Config::default()
            // show all logs
            .with_min_level(Level::Trace), //.with_min_level(Level::Info)
                                           // display them under the tag 'libqaul'
                                           //.with_tag("libqaul")
    );
}

/// Events of the async loop
enum EventType {
    Rpc(bool),
    Flooding(bool),
    RoutingInfo(bool),
    RoutingTable(bool),
    Messaging(bool),
}

/// initialize and start libqaul
/// and poll all the necessary modules
/// 
/// Provide a path where libqaul can save all data.
pub async fn start(storage_path: String) -> () {
    // initialize logging on android
    #[cfg(target_os = "android")]
    initialize_android_logging();

    // initialize rpc system
    let libqaul_receive = Rpc::init();

    pretty_env_logger::init();

    // initialize storage module.
    // initializes configuration.
    storage::Storage::init(storage_path);

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

    //initialize utilities
    Utilities::init();
    
    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut rpc_ticker = Ticker::new(Duration::from_millis(10));

    // check flooding message queue periodically
    let mut flooding_ticker = Ticker::new(Duration::from_millis(100));

    // send routing info periodically to neighbours
    let mut routing_info_ticker = Ticker::new(Duration::from_millis(100));

    // re-create routing table periodically
    let mut routing_table_ticker = Ticker::new(Duration::from_millis(1000));

    // manage the message sending
    let mut messaging_ticker = Ticker::new(Duration::from_millis(10));

    // set initialized flag
    INITIALIZED.set(true);

    loop {
        let evt = {
            let lan_fut = lan.swarm.next().fuse();
            let internet_fut = internet.swarm.next().fuse();
            let rpc_fut = rpc_ticker.next().fuse();
            let flooding_fut = flooding_ticker.next().fuse();
            let routing_info_fut = routing_info_ticker.next().fuse();
            let routing_table_fut = routing_table_ticker.next().fuse();
            let messaging_fut = messaging_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(
                lan_fut,
                internet_fut,
                rpc_fut,
                flooding_fut,
                routing_info_fut,
                routing_table_fut,
                messaging_fut,
            );

            select! {
                lan_event = lan_fut => {
                    log::info!("Unhandled lan connection module event: {:?}", lan_event);
                    None
                },
                internet_event = internet_fut => {
                    log::info!("Unhandled internet connection module event: {:?}", internet_event);
                    None
                },
                _rpc_event = rpc_fut => Some(EventType::Rpc(true)),
                _flooding_event = flooding_fut => Some(EventType::Flooding(true)),
                _routing_info_event = routing_info_fut => Some(EventType::RoutingInfo(true)),
                _routing_table_event = routing_table_fut => Some(EventType::RoutingTable(true)),
                _messaging_event = messaging_fut => Some(EventType::Messaging(true)),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Rpc(_) => {
                    if let Ok(rpc_message) = libqaul_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Rpc::process_received_message(
                            rpc_message,
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
                                .publish(msg.topic, msg.message);
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
                            ConnectionModule::Ble => {}
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
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
                            ConnectionModule::Ble => {},
                            ConnectionModule::Local => {
                                // TODO: deliver it locally
                            },
                            ConnectionModule::None => {
                                // TODO: DNT behaviour
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
    // does it log?
    log::info!("start_android");

    // initialize rpc system
    let libqaul_receive = Rpc::init();

    log::info!("start_android Rpc::init()");

    // initialize storage.
    // initializes & loads configuration.
    storage::Storage::init(storage_path);

    log::info!("start_android Configuration::init()");

    // initialize node & user accounts
    Node::init();

    log::info!("start_android Node::init()");

    // initialize router
    Router::init();

    log::info!("start_android Router::init()");

    // initialize Connection Modules
    let conn = Connections::init_android().await;
    let mut internet = conn.internet.unwrap();
    let mut lan = conn.lan.unwrap();

    log::info!("start_android Connections::init().await");

    // initialize services
    Services::init();

    log::info!("start_android Services::init()");

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut rpc_ticker = Ticker::new(Duration::from_millis(10));

    log::info!("start_android rpc_ticker");

    // check flooding message queue periodically
    let mut flooding_ticker = Ticker::new(Duration::from_millis(100));

    log::info!("start_android flooding_ticker");

    // send routing info periodically to neighbours
    let mut routing_info_ticker = Ticker::new(Duration::from_millis(100));

    log::info!("start_android routing_info_ticker");

    // re-create routing table periodically
    let mut routing_table_ticker = Ticker::new(Duration::from_millis(1000));

    log::info!("start_android routing_table_ticker");

    // manage the message sending
    let mut messaging_ticker = Ticker::new(Duration::from_millis(10));

    // set initialized flag
    INITIALIZED.set(true);

    log::info!("start_android INITIALIZED.set(true)");

    loop {
        let evt = {
            let lan_fut = lan.swarm.next().fuse();
            let internet_fut = internet.swarm.next().fuse();
            let rpc_fut = rpc_ticker.next().fuse();
            let flooding_fut = flooding_ticker.next().fuse();
            let routing_info_fut = routing_info_ticker.next().fuse();
            let routing_table_fut = routing_table_ticker.next().fuse();
            let messaging_fut = messaging_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(
                lan_fut,
                internet_fut,
                rpc_fut,
                flooding_fut,
                routing_info_fut,
                routing_table_fut,
                messaging_fut,
            );

            select! {
                lan_event = lan_fut => {
                    log::info!("Unhandled lan connection module event: {:?}", lan_event);
                    None
                },
                internet_event = internet_fut => {
                    log::info!("Unhandled internet connection module event: {:?}", internet_event);
                    None
                },
                _rpc_event = rpc_fut => Some(EventType::Rpc(true)),
                _flooding_event = flooding_fut => Some(EventType::Flooding(true)),
                _routing_info_event = routing_info_fut => Some(EventType::RoutingInfo(true)),
                _routing_table_event = routing_table_fut => Some(EventType::RoutingTable(true)),
                _messaging_event = messaging_fut => Some(EventType::Messaging(true)),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Rpc(_) => {
                    if let Ok(rpc_message) = libqaul_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Rpc::process_received_message(rpc_message, None, Some(&mut internet));
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
                                .publish(msg.topic, msg.message);
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
                            ConnectionModule::Ble => {}
                            ConnectionModule::Local => {}
                            ConnectionModule::None => {}
                        }
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
                            "sending routing information via {:?} to {:?}",
                            connection_module,
                            neighbour_id
                        );
                        // send routing information
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
                            ConnectionModule::Ble => {},
                            ConnectionModule::Local => {},
                            ConnectionModule::None => {},
                        }
                    }
                }
            }
        }
    }
}
