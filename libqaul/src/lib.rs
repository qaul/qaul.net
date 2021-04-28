use libp2p::{
    core::upgrade,
    noise::{Keypair, NoiseConfig, X25519Spec},
    tcp::TcpConfig,
    mplex,
    mdns::{Mdns, MdnsConfig},
    floodsub::{Floodsub},
    swarm::{Swarm, SwarmBuilder},
    Transport,
};
use log::{error, info};
// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use async_std::{io, task};
use futures::channel::mpsc;
use futures::prelude::*;
use futures::{ pin_mut, select, future::FutureExt };

mod node;
use node::mdns;
mod services;
use services::page;
use services::feed;


pub enum EventType {
    Response(mdns::QaulMessage),
    Input(String),
}


pub async fn init() {
    pretty_env_logger::init();

    // Node ID
    info!("Peer Id: {}", node::get_id());
    let (response_sender, mut response_rcv) = mpsc::unbounded();

    // create user ID/keys
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(node::get_keys())
        .expect("can create auth keys");
    
    // create a TCP transport
    let transp = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // create mDNS advertising, libp2p NetworkBehaviour
    let mut behaviour = mdns::QaulBehaviour {
        floodsub: Floodsub::new(node::get_id()),
        // TODO: most probably without the await.expect ...? maybe await.unwrap_or
        mdns: Mdns::new(MdnsConfig::default()).await.expect("can create mdns"),
        response_sender,
    };
    behaviour.floodsub.subscribe(node::get_topic());

    // listen for new commands from CLI
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // swarm libp2p connection management
    let mut swarm = SwarmBuilder::new(transp, behaviour, node::get_id())
        .executor(Box::new(|fut| {
            task::spawn(fut);
        }))
        .build();
    
    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get a local socket"),
    )
    .expect("swarm can be started");


    // event loop: listen STDIN, Swarm & Channel responses
    loop {
        let evt = {
            let line_fut = stdin.next().fuse();
            let event_fut = swarm.next().fuse();
            let response_fut = response_rcv.next().fuse();

            // This Macro is shown wrong by Rust-Language-Server > 0.2.400
            // You need to downgrade to version 0.2.400 if this happens to you
            pin_mut!(line_fut, event_fut, response_fut);

            select! {
                line = line_fut => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
                event = event_fut => {
                    info!("Unhandled Swarm Event: {:?}", event);
                    None
                },
                response = response_fut => Some(EventType::Response(response.expect("response exists"))),
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Response(resp) => {
                    let json = serde_json::to_string(&resp).expect("can jsonify response");
                    swarm.behaviour_mut().floodsub.publish(node::get_topic(), json.as_bytes());
                }
                EventType::Input(line) => match line.as_str() {
                    // node functions
                    "q ls" => node::handle_list_peers(&mut swarm).await,
                    // feed functions
                    cmd if cmd.starts_with("f ") => feed::send(cmd, &mut swarm),
                    // pages functions
                    cmd if cmd.starts_with("p ls") => page::handle_list_pages(cmd, &mut swarm).await,
                    cmd if cmd.starts_with("p create") => page::handle_create_page(cmd).await,
                    cmd if cmd.starts_with("p publish") => page::handle_publish_page(cmd).await,
                    // unknown command
                    _ => error!("unknown command"),
                },
            }
        }
    }
}



