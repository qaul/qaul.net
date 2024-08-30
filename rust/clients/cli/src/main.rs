// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul RPC CLI Client
//!
//! This client uses all the functionality of the qaul
//! RPC system and

use tokio::io::{self, AsyncBufReadExt, BufReader};
use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
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
mod authentication;

use cli::Cli;
use rpc::Rpc;
use user_accounts::UserAccounts;

/// Event Types of the async loop
enum EventType {
    Cli(String),
    Rpc,
}

#[tokio::main]
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
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable.
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    let mut futures_ticker = Ticker::new(Duration::from_millis(10));

    // loop and poll CLI and RPC
    loop {
        let evt = {
            let line_fut = lines.next_line().fuse();
            let rpc_fut = futures_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(line_fut);
            pin_mut!(rpc_fut);

            select! {
                line = line_fut => {
                    match line {
                        Ok(Some(line_str)) => Some(EventType::Cli(line_str)),
                        Ok(None) => None,  // EOF
                        Err(_) => None,    // Error reading
                    }
                },
                _rpc_ticker = rpc_fut => Some(EventType::Rpc),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Cli(line) => {
                    Cli::process_command(line);
                }
                EventType::Rpc => match libqaul::api::receive_rpc() {
                    Ok(data) => {
                        Rpc::received_message(data);
                    }
                    _ => {}
                },
            }
        }
    }
}
