// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul RPC CLI Client
//!
//! This client uses all the functionality of the qaul.net
//! RPC system and

use async_std::io;
use futures_ticker::Ticker;
//use async_std::stream;
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use std::time::Duration;

use libqaul;

mod ble;
mod chat;
mod chatfile;
mod cli;
mod connections;
mod debug;
mod dtn;
mod feed;
mod group;
mod node;
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

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut futures_ticker = Ticker::new(Duration::from_millis(10));

    // loop and poll CLI and RPC
    loop {
        let evt = {
            let line_fut = stdin.next().fuse();
            let rpc_fut = futures_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(line_fut);
            pin_mut!(rpc_fut);

            select! {
                line = line_fut => Some(EventType::Cli(line.expect("can get line").expect("can read line from stdin"))),
                _rpc_ticker = rpc_fut => Some(EventType::Rpc(true)),
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
