//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protbuf rpc messages.

use crossbeam_channel::{unbounded, Sender, Receiver, TryRecvError};
use state::Storage;
use futures_ticker::Ticker;
use futures::prelude::*;
use futures::{ pin_mut, select, future::FutureExt };
use futures::executor::block_on;
use std::{
    thread,
    time::{SystemTime, Duration},
};

use crate::configuration::Configuration;
use crate::connections::{Connections, ConnectionModule};
use crate::node::Node;
use crate::router;
use crate::router::{Router, info::RouterInfo};
use crate::router::flooder;
use crate::rpc::Rpc;
use crate::services::Services;

mod api;


/// receiving end of the mpsc channel
static EXTERN_RECEIVE: Storage<Receiver<Vec<u8>>> = Storage::new();
/// sending end of the mpsc channel
static EXTERN_SEND: Storage<Sender<Vec<u8>>> = Storage::new();
/// sending end of th mpsc channel for libqaul to send
static LIBQAUL_SEND: Storage<Sender<Vec<u8>>> = Storage::new();


/// send rpc message from the outside to the inside 
/// of the worker thread of libqaul.
pub fn send_rpc_to_libqaul(binary_message: Vec<u8>) {
    let sender = EXTERN_SEND.get().clone();
    match sender.send(binary_message) {
        Ok(()) => {},
        Err(err) => {
            // log error message
            log::error!("{:?}", err);
        },
    }
}


/// check the receiving rpc channel if there
/// are new messages from inside libqaul for 
/// the outside.
pub fn receive_rpc_from_libqaul() -> Result<Vec<u8>, TryRecvError> {
    let receiver = EXTERN_RECEIVE.get().clone();
    receiver.try_recv()
}


/// start libqaul in an own thread
pub fn start() {
    log::info!("start");

    // create channels
    let (libqaul_send, extern_receive) = unbounded();
    let (extern_send, libqaul_receive) = unbounded();

    // save to state
    EXTERN_RECEIVE.set(extern_receive);
    EXTERN_SEND.set(extern_send);
    LIBQAUL_SEND.set(libqaul_send.clone());

    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            log::info!("spawn");

            let libqaul = LibqaulThread{
                sender: libqaul_send,
                receiver: libqaul_receive,
            };
            log::info!("before start");
            libqaul.start().await;
        }
    ));

    log::info!("return start");
}

/// Events of the async loop
enum EventType {
    Rpc(bool),
    Flooding(bool),
    RoutingInfo(bool),
    RoutingTable(bool),
}

/// The worker thread of libqaul
struct LibqaulThread {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
}


impl LibqaulThread {
    /// initialize and start libqaul
    /// and poll all the necessary modules
    async fn start(&self) -> () {
        pretty_env_logger::init();

        // initialize & load configuration
        Configuration::init();

        // initialize node & user accounts
        Node::init();

        // initialize router
        Router::init();
        let router_interval = Duration::from_millis(1000);
        let router_time = SystemTime::now();
        
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
                    EventType::Rpc(_) => {
                        if let Ok(rpc_message) = self.receiver.try_recv() {
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
}

/// send an rpc message from inside libqaul thread
/// to the extern.
pub fn send_rpc_to_extern(message: Vec<u8>) {
    let sender = LIBQAUL_SEND.get().clone();
    match sender.send(message) {
        Ok(()) => {},
        Err(err) => {
            // log error message
            log::error!("{:?}", err);
        },
    }
}

