//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protbuf rpc messages.

use crossbeam_channel::{unbounded, Sender, Receiver, RecvError};
use state::Storage;
use async_std::task;
use futures::prelude::*;
use std::{
    task::{Context, Poll},
    thread,
    time::{SystemTime, Duration},
};

use crate::node::Node;
use crate::router;
use crate::router::{Router, info::RouterInfo};
use crate::router::flooder;
use crate::connections::{Connections, ConnectionModule};
use crate::services::Services;
use crate::configuration::Configuration;

mod api;

// pub struct RpcThread {
//     sender: Sender<Vec<u8>>,
//     receiver: Receiver<Vec<u8>>,
// }


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
pub fn receive_rpc_from_libqaul() -> Result<Vec<u8>, RecvError> {
    let receiver = EXTERN_RECEIVE.get().clone();
    receiver.recv()
}


/// start libqaul in an own thread
pub fn start() {
    // create channels
    let (libqaul_send, extern_receive) = unbounded();
    let (extern_send, libqaul_receive) = unbounded();

    // save to state
    EXTERN_RECEIVE.set(extern_receive);
    EXTERN_SEND.set(extern_send);
    LIBQAUL_SEND.set(libqaul_send.clone());

    // Spawn new thread
    thread::spawn(move|| {
        async move {
            let libqaul = LibqaulThread{
                sender: libqaul_send,
                receiver: libqaul_receive,
            };
            libqaul.start().await;
        }
    });
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

        // loop & poll network and CLI
        task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
            // poll LAN connection
            loop {
                match conn.lan.swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => {
                        log::debug!("Lan SwarmEvent: {:?}", event);
                        // if let SwarmEvent::NewListenAddr(addr) = event {
                        //     println!("Listening on {:?}", addr);
                        // }
                    }
                    Poll::Ready(None) => return Poll::Ready(()),
                    Poll::Pending => break,
                }
            }
            // poll Internet connection
            loop {
                match conn.internet.swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => {
                        log::debug!("Internet SwarmEvent: {:?}", event);
                        // if let SwarmEvent::NewListenAddr(addr) = event {
                        //     println!("Listening on {:?}", addr);
                        // }
                    }
                    Poll::Ready(None) => return Poll::Ready(()),
                    Poll::Pending => break,
                }
            }
            // send messages in the flooding queue
            loop {
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
                break
            }
            // send router info to neighbours
            {
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
            }
            // create routing table periodically (every second)
            {
                let time_now = SystemTime::now();
                if let Ok(passed) = time_now.duration_since(router_time) {
                    if passed > router_interval {
                        router::connections::ConnectionTable::create_routing_table();
                    }
                } else {
                    log::error!("error in time duration calculation");
                }
            }
            // poll RPC
            {
                match self.receiver.try_recv() {
                    Ok(rpc_message) => {
                        // we received a message
                        // better do this blocking and join the tasks afterwards
                    },
                    _ => {
                        // we received an error
                        // an error can happen, when the buffer was empty.
                    },
                }
            }

            Poll::Pending
        }));
    }

    /// send an rpc message from inside libqaul thread
    /// to the extern.
    fn send_rpc_to_extern(&self, message: Vec<u8>) {
        match self.sender.send(message) {
            Ok(()) => {},
            Err(err) => {
                // log error message
                log::error!("{:?}", err);
            },
        }
    }
}
