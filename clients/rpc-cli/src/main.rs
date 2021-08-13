//! # qaul RPC CLI Client
//! 
//! This client uses all the functionality of the qaul.net
//! RPC system and 

use futures_ticker::Ticker;
use async_std::io;
//use async_std::stream;
use futures::prelude::*;
use futures::{ pin_mut, select, future::FutureExt };
use std::time::Duration;

use libqaul;

mod cli;
mod rpc;
mod node;

use cli::Cli;
use rpc::Rpc;

/// Events of the async loop
enum EventType {
    Cli(String),
    Rpc(bool),
}

#[async_std::main]
async fn main() {
    // start libqaul in new thread
    libqaul::threaded::start();

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
                },
                EventType::Rpc(_) => {
                    match libqaul::threaded::receive_rpc_from_libqaul() {
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
