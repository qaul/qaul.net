// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qauld - qaul Daemon
//! 
//! qaul Daemon is running headless on servers in the background

//use futures_ticker::Ticker;
//use async_std::io;
//use futures::prelude::*;
//use futures::{ pin_mut, select, future::FutureExt };
use std::{thread, time::Duration};

use libqaul;

#[async_std::main]
async fn main() {
    // TODO: make configuration file location configurable

    // get current working directory
    let path = std::env::current_dir().unwrap(); 
    let storage_path = path.as_path().to_str().unwrap().to_string();

    // start libqaul in new thread and save configuration file to current working path
    libqaul::api::start(storage_path);

    // wait until libqaul finished initializing
    while libqaul::api::initialization_finished() == false {
        // wait a little while
        std::thread::sleep(Duration::from_millis(10));
    }

    // TODO: open a unix socket to communicate with libqaul

    // check RPC once every 10 milliseconds
    // TODO: interval is only in unstable. Use it once it is stable. 
    //       https://docs.rs/async-std/1.5.0/async_std/stream/fn.interval.html
    //let mut rpc_interval = async_std::stream::interval(Duration::from_millis(10));
    
    //let mut futures_ticker = Ticker::new(Duration::from_millis(10));


    // loop
    loop {
        thread::sleep(Duration::from_millis(10));

        /*
        let evt = {
            let rpc_fut = futures_ticker.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(rpc_fut);

            select! {
                _rpc_ticker = rpc_fut => Some(EventType::Rpc(true)),
            }
        };

        if let Some(event) = evt {
            match event {
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
        */
    }
}
