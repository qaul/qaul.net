// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul RPC CLI Client
//! 
//! This client uses all the functionality of the qaul.net
//! RPC system and 

use tokio::io::{self, AsyncBufReadExt};
use tokio::select;
use futures::{ pin_mut };
use options::Config;
use structopt::StructOpt;
use std::time::Duration;

use libqaul;

mod cli;
mod rpc;
mod node;
mod user_accounts;
mod connections;
mod users;
mod router;
mod feed;
mod chat;
mod debug;
mod ble;
mod options;

use cli::Cli;
use rpc::Rpc;
use user_accounts::UserAccounts;

/// Events of the async loop
enum EventType {
    Cli(String),
    Rpc(bool),
}

#[tokio::main]
async fn main() {
    // Extract the CLI options
    let Config { options } = options::Config::from_args();

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start(options.storage_path.unwrap_or("".to_string()));

    //#[cfg(target_os = "windows")]
    //libqaul::api::start(".\\".to_string());
    //#[cfg(not(target_os = "windows"))]
    //libqaul::api::start("./".to_string());

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
    let mut rpc_interval = tokio::time::interval(Duration::from_millis(10));

    // loop and poll CLI and RPC
    loop {
        let evt = {
            let line_fut = stdin.next_line();
            let rpc_fut = rpc_interval.tick();

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
                },
                EventType::Rpc(_) => {
                    match libqaul::api::receive_rpc() {
                        Ok(data) => {
                            Rpc::received_message(data);
                        },
                        _ => {},
                    }
                },
            }
        }
    }
}
