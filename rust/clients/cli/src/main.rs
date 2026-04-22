// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul RPC CLI Client
//!
//! This client uses all the functionality of the qaul
//! RPC system and

use futures::prelude::*;
use futures::{future::FutureExt, pin_mut, select};
use futures_ticker::Ticker;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use libqaul;

mod authentication;
mod ble;
mod chat;
mod chatfile;
mod cli;
mod connections;
mod crypto;
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

/// CLI-wide state passed to all modules instead of using globals.
pub struct CliState {
    /// The libqaul instance.
    pub instance: Arc<libqaul::Libqaul>,
    /// User accounts state (replaces the former USERACCOUNTS static).
    pub user_accounts: RwLock<Option<UserAccounts>>,
}

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

    // start libqaul in new thread and get instance
    let instance = libqaul::api::start_instance_in_thread(storage_path, None);

    // wait until libqaul finished initializing
    while !instance.is_initialized() {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // Print node info
    println!("Node ID: {}", instance.node_id());

    // Create CLI state
    let cli_state = Arc::new(CliState {
        instance,
        user_accounts: RwLock::new(None),
    });

    // initialize user accounts
    UserAccounts::init(&cli_state);

    // listen for new commands from CLI
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    // check RPC once every 10 milliseconds
    let mut futures_ticker = Ticker::new(Duration::from_millis(10));

    // loop and poll CLI and RPC
    loop {
        let evt = {
            let line_fut = lines.next_line().fuse();
            let rpc_fut = futures_ticker.next().fuse();

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
                    Cli::process_command(&cli_state, line);
                }
                EventType::Rpc => {
                    match libqaul::rpc::Rpc::receive_from_libqaul(&*cli_state.instance.state) {
                        Ok(data) => {
                            Rpc::received_message(&cli_state, data);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
