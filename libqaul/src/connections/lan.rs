/**
 * # LAN Connection Module
 * 
 * **Discover other qaul nodes on the local LAN and connect to them.**
 * 
 * This module advertises the node via mdns in the local network.
 * By default it listens to all interfaces and connects to a random port.
 * 
 * The module is configured in the configuration file:
 * 
 * ```toml
 * [lan]
 * active = true
 * listen = "/ip4/0.0.0.0/tcp/0"
 * ```
 */

use libp2p::{
    core::upgrade,
    noise::{NoiseConfig, X25519Spec, AuthenticKeypair},
    tcp::TcpConfig,
    mplex,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    Transport,
    floodsub::{Floodsub, FloodsubEvent},
    swarm::{
        Swarm, NetworkBehaviourEventProcess, SwarmBuilder, ExpandedSwarm,
        protocols_handler::ProtocolsHandler,
        IntoProtocolsHandler, NetworkBehaviour
    },
    NetworkBehaviour,
};
use futures::channel::mpsc;
use std::collections::HashSet;
use log::info;
use async_std::task;
use mpsc::UnboundedReceiver;
use crate::types::QaulMessage;
use crate::node::Node;
use crate::services::{
    page,
    page::{PageMode, PageRequest, PageResponse},
    feed::FeedMessage,
};
use crate::configuration::Configuration;


#[derive(NetworkBehaviour)]
pub struct QaulLanBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<QaulMessage>,
}

pub struct Lan {
    pub swarm: ExpandedSwarm<QaulLanBehaviour, <<<QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::InEvent, <<<QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::OutEvent, <QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler>, 
    pub receiver: UnboundedReceiver<QaulMessage>,
}

impl Lan {
    /**
     * Initialize swarm for LAN connection module
     */
    pub async fn init(config: Configuration, auth_keys: AuthenticKeypair<X25519Spec>) -> (Configuration, Lan) {
        // create a multi producer, single consumer queue
        let (response_sender, response_rcv) = mpsc::unbounded();
    
        // create a TCP transport
        let transp = TcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        // create behaviour
        let mut behaviour = QaulLanBehaviour {
            floodsub: Floodsub::new(Node::get_id()),
            mdns: Mdns::new(MdnsConfig::default()).await.expect("can create mdns"),
            response_sender,
        };
        behaviour.floodsub.subscribe(Node::get_topic());

        // swarm libp2p connection management
        let mut swarm = SwarmBuilder::new(transp, behaviour, Node::get_id())
            .executor(Box::new(|fut| {
                task::spawn(fut);
            }))
            .build();
        
        // connect swarm to the listening interface in 
        // the configuration config.lan.listen
        Swarm::listen_on(
            &mut swarm,
            config.lan.listen
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");

        let lan = Lan { swarm: swarm, receiver: response_rcv };

        (config, lan)
    }

    /**
     * Print information about this connection
     */
    pub fn info(&self) {
        println!("# Lan Connection Module");
        // number of peers connected
        println!("{} peer(s) connected", self.swarm.network_info().num_peers());

        // List mdns peers
        println!("Discovered mdns peers:");
        let nodes = self.swarm.behaviour().mdns.discovered_nodes();
        let mut unique_peers = HashSet::new();
        for peer in nodes {
            unique_peers.insert(peer);
        }
        unique_peers.iter().for_each(|p| println!("  {}", p));
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    info!("MdnsEvent::Discovered, peer {:?} to floodsub added", peer);
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
                        info!("MdnsEvent::Expired, peer {:?} from floodsub removed", peer);
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                // feed Message
                if let Ok(resp) = serde_json::from_slice::<FeedMessage>(&msg.data) {
                    info!("Feed from {}", msg.source);
                    info!("{}", resp.message);
                }
                // Pages Messages
                else if let Ok(resp) = serde_json::from_slice::<PageResponse>(&msg.data) {
                    //if resp.receiver == node::get_id_string() {
                        info!("Response from {}", msg.source);
                        resp.data.iter().for_each(|r| info!("{:?}", r));
                    //}
                } else if let Ok(req) = serde_json::from_slice::<PageRequest>(&msg.data) {
                    match req.mode {
                        PageMode::ALL => {
                            info!("Received All req: {:?} from {:?}", req, msg.source);
                            page::respond_with_public_pages(
                                self.response_sender.clone(),
                                msg.source.to_string(),
                            );
                        }
                        PageMode::One(ref peer_id) => {
                            if peer_id.to_string() == Node::get_id_string() {
                                info!("Received req: {:?} from {:?}", req, msg.source);
                                page::respond_with_public_pages(
                                    self.response_sender.clone(),
                                    msg.source.to_string(),
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
