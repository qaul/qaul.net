// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # bridge client for Matrix<-->Qaul Bridge
//!
//! This client uses all the functionality of the qaul-cli
//! and implements Matrix bridge over

use async_std::io;
use futures_ticker::Ticker;
use uuid::Uuid;
//use async_std::stream;
use crate::relay_bot::MATRIX_CONFIG;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use std::thread;
use std::time::Duration;

use libqaul;

mod ble;
mod chat;
mod chatfile;
mod cli;
mod configuration;
mod connections;
mod debug;
mod dtn;
mod feed;
mod group;
mod node;
mod relay_bot;
mod router;
mod rpc;
mod rtc;
mod user_accounts;
mod users;

use cli::Cli;
use rpc::Rpc;
use user_accounts::UserAccounts;

/// Events of the async loop
enum EventType {
    Cli(String),
    Rpc(bool),
}

#[async_std::main]
async fn main() {
    // get current working directory
    let path = std::env::current_dir().unwrap();
    let storage_path = path.as_path().to_str().unwrap().to_string();

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start_with_config(storage_path, None);

    // wait until libqaul finished initializing
    while libqaul::api::initialization_finished() == false {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // initialize user accounts
    UserAccounts::init();
    // listen for new commands from CLI
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    thread::spawn(|| {
        // connect the matrix bot with the qaul-cli
        match relay_bot::connect() {
            Ok(_) => {
                println!("Matrix-Bridge connecting");
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    });

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut futures_ticker = Ticker::new(Duration::from_millis(10));
    let mut feed_ticker = Ticker::new(Duration::from_secs(3));
    let mut group_ticker = Ticker::new(Duration::from_secs(3));
    // loop and poll CLI and RPC
    loop {
        let evt = {
            let line_fut = stdin.next().fuse();
            let rpc_fut = futures_ticker.next().fuse();
            let feed_fut = feed_ticker.next().fuse();
            let group_fut = group_ticker.next().fuse();
            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(line_fut);
            pin_mut!(rpc_fut);
            pin_mut!(feed_fut);
            pin_mut!(group_fut);
            select! {
                line = line_fut => Some(EventType::Cli(line.expect("can get line").expect("can read line from stdin"))),
                _rpc_ticker = rpc_fut => Some(EventType::Rpc(true)),
                _feed_ticker = feed_fut => {
                    let config = MATRIX_CONFIG.get().read().unwrap();
                    let last_index = &config.feed.last_index;
                    // Check unread messages from Libqaul
                    feed::Feed::request_feed_list(*last_index);
                    None
                }
                _group_ticker = group_fut => {
                    let config = MATRIX_CONFIG.get().read().unwrap();
                    group::Group::group_list();
                    let qaul_groups: Vec<Uuid> = config.room_map.keys().cloned().collect();

                    // Check unread messages from Libqaul groups
                    for group in qaul_groups {
                        let matrix_room = config.room_map.get(&group).unwrap();
                            let last_index_grp = matrix_room.last_index;
                        let group_id = group.as_bytes().to_vec();
                        chat::Chat::request_chat_conversation(group_id,last_index_grp);
                    }None
                }
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Cli(line) => {
                    Cli::process_command(line);
                }
                EventType::Rpc(_) => match libqaul::api::receive_rpc() {
                    Ok(data) => {
                        Rpc::received_message(data);
                    }
                    _ => {}
                },
            }
        }
    }
}
