//! # libqaul
//! 
//! Library for qaul.net

// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use futures_ticker::Ticker;
use futures::prelude::*;
use futures::{ pin_mut, select, future::FutureExt };
use std::time::Duration;


// crate modules
pub mod api;
mod configuration;
mod connections;
mod node;
mod router;
mod rpc;
mod services;
mod types;

use configuration::Configuration;
use connections::{Connections, ConnectionModule};
use node::Node;
use node::user_accounts::UserAccounts;
use router::{Router, info::RouterInfo};
use router::flooder;
use rpc::Rpc;
use services::Services;
use services::feed;

/// Events of the async loop
enum EventType {
    Cli(String),
    Rpc(bool),
    Flooding(bool),
    RoutingInfo(bool),
    RoutingTable(bool),
}

/// initialize and start libqaul
/// and poll all the necessary modules
pub async fn start() -> () {
    // initialize rpc system
    let libqaul_receive = Rpc::init();

    pretty_env_logger::init();

    // initialize & load configuration
    Configuration::init();

    // initialize node & user accounts
    Node::init();

    // initialize router
    Router::init();
    
    // initialize Connection Modules
    let mut conn = Connections::init().await;

    // initialize services
    Services::init();


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

    loop {
        let evt = {
            let lan_fut = conn.lan.swarm.next().fuse();
            let internet_fut = conn.internet.swarm.next().fuse();
            let rpc_fut = rpc_ticker.next().fuse();
            let flooding_fut = flooding_ticker.next().fuse();
            let routing_info_fut = routing_info_ticker.next().fuse();
            let routing_table_fut = routing_table_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(
                lan_fut, 
                internet_fut, 
                rpc_fut, 
                flooding_fut, 
                routing_info_fut, 
                routing_table_fut
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
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Cli(_line) => {},
                EventType::Rpc(_) => {
                    if let Ok(rpc_message) = libqaul_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Rpc::process_received_message(rpc_message, &mut conn);
                    }
                },
                EventType::Flooding(_) => {
                    // send messages in the flooding queue
                    // get sending queue
                    let mut flooder = flooder::FLOODER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(msg) = flooder.to_send.pop_front() {
                        // check which swarm to send to
                        if !matches!(msg.incoming_via, ConnectionModule::Lan) {
                            conn.lan.swarm.behaviour_mut().floodsub.publish( msg.topic.clone(), msg.message.clone());
                        }
                        if !matches!(msg.incoming_via, ConnectionModule::Internet) {
                            conn.internet.swarm.behaviour_mut().floodsub.publish( msg.topic, msg.message);
                        }
                    }
                },
                EventType::RoutingInfo(_) => {
                    // send routing info to neighbours
                    // check scheduler
                    if let Some((neighbour_id, connection_module, data)) = RouterInfo::check_scheduler() {
                        log::info!("sending routing information via {:?} to {:?}", connection_module, neighbour_id);
                        // send routing information
                        match connection_module {
                            ConnectionModule::Lan => conn.lan.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                            ConnectionModule::Internet => conn.internet.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                            ConnectionModule::Local => {},
                            ConnectionModule::None => {},
                        }
                    }
                },
                EventType::RoutingTable(_) => {
                    // create new routing table
                    router::connections::ConnectionTable::create_routing_table();
                },
            }
        }
    }
}

/// DEPRECATED! This function will be deleted soon
/// initialize and start libqaul and read the CLI line in
pub async fn start_cli() -> () {
    // initialize rpc system
    let libqaul_receive = Rpc::init();

    pretty_env_logger::init();

    // initialize & load configuration
    Configuration::init();

    // initialize node & user accounts
    Node::init();

    // initialize router
    Router::init();
    
    // initialize Connection Modules
    let mut conn = Connections::init().await;

    // initialize services
    Services::init();


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

    // TODO: remove this as soon as rpc-cli has all functionality
    // listen for new commands from CLI
    let mut stdin = async_std::io::BufReader::new(async_std::io::stdin()).lines();

    loop {
        let evt = {
            let cli_fut = stdin.next().fuse();
            let lan_fut = conn.lan.swarm.next().fuse();
            let internet_fut = conn.internet.swarm.next().fuse();
            let rpc_fut = rpc_ticker.next().fuse();
            let flooding_fut = flooding_ticker.next().fuse();
            let routing_info_fut = routing_info_ticker.next().fuse();
            let routing_table_fut = routing_table_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(
                cli_fut,
                lan_fut, 
                internet_fut, 
                rpc_fut, 
                flooding_fut, 
                routing_info_fut, 
                routing_table_fut
            );

            select! {
                // TODO: delete this as soon as rpc-cli is full function
                cli_event = cli_fut => Some(EventType::Cli(cli_event.expect("can get line").expect("can read line from stdin"))),
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
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Cli(line) => {
                    cli(line, &mut conn).await;
                }
                EventType::Rpc(_) => {
                    if let Ok(rpc_message) = libqaul_receive.try_recv() {
                        // we received a message, send it to RPC crate
                        Rpc::process_received_message(rpc_message, &mut conn);
                    }
                },
                EventType::Flooding(_) => {
                    // send messages in the flooding queue
                    // get sending queue
                    let mut flooder = flooder::FLOODER.get().write().unwrap();

                    // loop over messages to send & flood them
                    while let Some(msg) = flooder.to_send.pop_front() {
                        // check which swarm to send to
                        if !matches!(msg.incoming_via, ConnectionModule::Lan) {
                            conn.lan.swarm.behaviour_mut().floodsub.publish( msg.topic.clone(), msg.message.clone());
                        }
                        if !matches!(msg.incoming_via, ConnectionModule::Internet) {
                            conn.internet.swarm.behaviour_mut().floodsub.publish( msg.topic, msg.message);
                        }
                    }
                },
                EventType::RoutingInfo(_) => {
                    // send routing info to neighbours
                    // check scheduler
                    if let Some((neighbour_id, connection_module, data)) = RouterInfo::check_scheduler() {
                        log::info!("sending routing information via {:?} to {:?}", connection_module, neighbour_id);
                        // send routing information
                        match connection_module {
                            ConnectionModule::Lan => conn.lan.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                            ConnectionModule::Internet => conn.internet.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                            ConnectionModule::Local => {},
                            ConnectionModule::None => {},
                        }
                    }
                },
                EventType::RoutingTable(_) => {
                    // create new routing table
                    router::connections::ConnectionTable::create_routing_table();
                },
            }
        }
    }
}

/// DEPRECATED! this function will be deleted soon
// TODO: get rid of this by implementing all functionality into rpc-cli
async fn cli( line: String, conn: &mut Connections ) {
    match line.as_str() {
        // node functions
        "modules info" => {
            // print information about the connections
            conn.internet.info();
            conn.lan.info();
        }
        // user functions
        cmd if cmd.starts_with("user ") => {
            UserAccounts::cli(cmd.strip_prefix("user ").unwrap());
        },
        // router module
        cmd if cmd.starts_with("router ") => {
            router::Router::cli(cmd.strip_prefix("router ").unwrap());
        },
        // send feed message
        cmd if cmd.starts_with("feed ") => {
            feed::Feed::cli(cmd.strip_prefix("feed ").unwrap(), conn);
        },
        _ => log::error!("unknown command"),
    }
}
