use log::{error, info};
// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use async_std::{task, io};
use futures::prelude::*;
use std::task::{Context, Poll};

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
                            _ => error!("unknown command"),
                        }
                    }
                    else {
                        error!("CLI input error: {:?}", input);
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
                    info!("Lan SwarmEvent: {:?}", event);
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
                    info!("Internet SwarmEvent: {:?}", event);
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
                // send routing information
                match connection_module {
                    ConnectionModule::Lan => conn.lan.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                    ConnectionModule::Internet => conn.internet.swarm.behaviour_mut().qaul_info.send_qaul_info_message(neighbour_id, data),
                    ConnectionModule::None => {},
                }
            }
        }

        Poll::Pending
    }));
}
