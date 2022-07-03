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
mod connections;
mod node;
mod router;
mod rpc;
mod services;
pub mod storage;
mod types;
pub mod utilities;

use connections::{ble::Ble, internet::Internet, ConnectionModule, Connections};

use node::Node;
use router::{feed_requester, flooder, info::RouterInfo, neighbours::Neighbours, Router};

use rpc::sys::Sys;
use rpc::Rpc;
use services::messaging::Messaging;
use services::Services;

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

use crate::utilities::timestamp::Timestamp;

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
    Sys(bool),
    Flooding(bool),
    FeedRequest(bool),
    FeedResponse(bool),
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

    // only use the pretty logger on desktop systems
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    pretty_env_logger::init();

    // initialize storage module.
    // This will initialize configuration & data base
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
            let feedreq_fut = feedreq_ticker.next().fuse();
            let feedresp_fut = feedresp_ticker.next().fuse();
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
                feedreq_fut,
                feedresp_fut,
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
                            log::info!("internet connection closed: {:?}", peer_id);
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
                _feedreq_event = feedreq_fut => Some(EventType::FeedRequest(true)),
                _feedresp_event = feedresp_fut => Some(EventType::FeedResponse(true)),
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
                        //make data
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
                EventType::RoutingInfo(_) => {
                    // send routing info to neighbours
                    // check scheduler
                    if let Some((neighbour_id, connection_module, data)) =
                        RouterInfo::check_scheduler()
                    {
                        log::error!(
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
                        log::error!("sending messaging message dat len={}", data.len());
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
                                // TODO: deliver it locally
                            }
                            ConnectionModule::None => {
                                // TODO: DTN behaviour
                                // reschedule it for the moment
                            }
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
