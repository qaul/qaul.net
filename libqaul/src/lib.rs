use libp2p::{
    Transport,
};
use log::{error, info};
// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use async_std::io;
use futures::prelude::*;
use futures::{ pin_mut, select, future::FutureExt };

// create modules
mod configuration;
mod connections;
mod node;
mod services;
mod types;

use types::EventType;
use node::Node;
use connections::Connections;
use services::page;
use services::feed;
use configuration::Configuration;



pub async fn init() -> ! {
    pretty_env_logger::init();

    // Load configuration
    let mut config = Configuration::new().unwrap();
    println!("{:?}", config);

    // initialize node
    config = Node::init(config);

    // Initialize Connection Modules
    let (config, mut conn) = Connections::init(config).await;

    // listen for new commands from CLI
    let mut stdin = io::BufReader::new(io::stdin()).lines();


    // event loop: listen STDIN, Swarm & Channel responses
    loop {
        let evt = {
            let cli_fut = stdin.next().fuse();
            let lan_event_fut = conn.lan.swarm.next().fuse();
            let lan_message_fut = conn.lan.receiver.next().fuse();
            let internet_event_fut = conn.internet.swarm.next().fuse();
            let internet_message_fut = conn.internet.receiver.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(cli_fut, lan_event_fut, lan_message_fut, internet_event_fut, internet_message_fut);

            select! {
                cli = cli_fut => Some(EventType::Cli(cli.expect("can get line").expect("can read line from stdin"))),
                lan_event = lan_event_fut => {
                    info!("Unhandled Lan Swarm Event: {:?}", lan_event);
                    None
                },
                lan_message = lan_message_fut => Some(EventType::Message(lan_message.expect("response exists"))),
                internet_event = internet_event_fut => {
                    info!("Unhandled Internet Swarm Event: {:?}", internet_event);
                    None
                },
                internet_message = internet_message_fut => Some(EventType::Message(internet_message.expect("response exists"))),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Message(resp) => {
                    let json = serde_json::to_string(&resp).expect("can jsonify response");
                    conn.lan.swarm.behaviour_mut().floodsub.publish(Node::get_topic(), json.as_bytes());
                    conn.internet.swarm.behaviour_mut().floodsub.publish(Node::get_topic(), json.as_bytes());
                }
                EventType::Cli(cli) => match cli.as_str() {
                    // node functions
                    "q peers" => {
                        // print information about the connections
                        conn.internet.info();
                        conn.lan.info();
                    }
                    // feed functions
                    cmd if cmd.starts_with("f ") => {
                        feed::send(cmd, &mut conn.lan.swarm, &mut conn.internet.swarm);
                    },
                    // pages functions
                    cmd if cmd.starts_with("p ls") => {
                        page::handle_list_pages(cmd, &mut conn.lan.swarm, &mut conn.internet.swarm).await

                    },
                    cmd if cmd.starts_with("p create") => page::handle_create_page(cmd).await,
                    cmd if cmd.starts_with("p publish") => page::handle_publish_page(cmd).await,
                    // unknown command
                    _ => error!("unknown command"),
                },
            }
        }
    }
}



