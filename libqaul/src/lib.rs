//! # libqaul
//! 
//! Library for qaul.net

// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use async_std::{task, io};
use futures::prelude::*;
use std::task::{Context, Poll};
use std::time::{SystemTime, Duration};


// crate modules
mod configuration;
mod connections;
mod node;
mod router;
mod services;
mod types;

use node::Node;
use node::users::Users;
use router::{Router, info::RouterInfo};
use router::flooder;
use connections::{Connections, ConnectionModule};
use services::Services;
use services::feed;
use configuration::Configuration;


pub async fn init() -> () {
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

    // listen for new commands from CLI
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // loop & poll network and CLI
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        // poll CLI
        loop {
            match stdin.poll_next_unpin(cx) {
                Poll::Ready(Some(input)) => {
                    if let Ok(line) = input {
                        match line.as_str() {
                            // node functions
                            "qaul peers" => {
                                // print information about the connections
                                conn.internet.info();
                                conn.lan.info();
                            }
                            // user functions
                            cmd if cmd.starts_with("user ") => {
                                Users::cli(cmd.strip_prefix("user ").unwrap());
                            },
                            // router module
                            cmd if cmd.starts_with("router ") => {
                                router::Router::cli(cmd.strip_prefix("router ").unwrap());
                            },
                            // send feed message
                            cmd if cmd.starts_with("feed ") => {
                                feed::Feed::cli(cmd.strip_prefix("feed ").unwrap(), &mut conn);
                            },
                            _ => log::error!("unknown command"),
                        }
                    }
                    else {
                        log::error!("CLI input error: {:?}", input);
                    }
                },
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break
            }
        }
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

        Poll::Pending
    }));
}
