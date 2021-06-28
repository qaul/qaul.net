/**
 * # Internet Overlay Connection Module
 * 
 * **Statically connect to nodes in the Internet.**
 * 
 * This module connects to predefined nodes in the internet.
 * The addresses of the peers to connect to are read from 
 * the config file:
 * 
 * ```toml
 * [internet]
 * active = true
 * peers = ["/ip4/144.91.74.192/tcp/9229"]
 * do_listen = false
 * listen = "/ip4/0.0.0.0/tcp/9229"
 * ```
 */

use libp2p::{
    core::upgrade,
    noise::{NoiseConfig, X25519Spec, AuthenticKeypair},
    tcp::TcpConfig,
    mplex,
    identify::{Identify, IdentifyConfig, IdentifyEvent},
    Transport,
    floodsub::{Floodsub, FloodsubEvent},
    swarm::{
        Swarm, NetworkBehaviourEventProcess, SwarmBuilder, ExpandedSwarm,
        protocols_handler::ProtocolsHandler,
        IntoProtocolsHandler, NetworkBehaviour,
    },
    NetworkBehaviour,
};
use log::info;
use async_std::task;
use futures::channel::mpsc;
use mpsc::{UnboundedReceiver, UnboundedSender};
use crate::types::QaulMessage;
use crate::node::Node;
use crate::services::{
    page,
    page::{PageMode, PageRequest, PageResponse},
    feed::FeedMessage,
};
use crate::configuration::Configuration;


#[derive(NetworkBehaviour)]
pub struct QaulInternetBehaviour {
    pub floodsub: Floodsub,
    pub identify: Identify,
    #[behaviour(ignore)]
    pub response_sender: UnboundedSender<QaulMessage>,
}

pub struct Internet {
    pub swarm: ExpandedSwarm<QaulInternetBehaviour, <<<QaulInternetBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::InEvent, <<<QaulInternetBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::OutEvent, <QaulInternetBehaviour as NetworkBehaviour>::ProtocolsHandler>, 
    pub receiver: UnboundedReceiver<QaulMessage>,
}

impl Internet {
    /**
     * Initialize swarm for Internet overlay connection module
     */
    pub async fn init(config: Configuration, auth_keys: AuthenticKeypair<X25519Spec>) -> (Configuration, Self) {
        // create a multi producer, single consumer queue
        let (response_sender, mut response_rcv) = mpsc::unbounded();
    
        // create a TCP transport
        let transp = TcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        // create behaviour
        let mut behaviour = QaulInternetBehaviour {
            floodsub: Floodsub::new(Node::get_id()),
            identify: Identify::new(IdentifyConfig::new("/ipfs/0.1.0".into(), Node::get_keys().public())),
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
        // the configuration config.internet.listen
        Swarm::listen_on(
            &mut swarm,
            config.internet.listen
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");

        // connect to remote peers that are specified in 
        // the configuration config.internet.peers
        Self::peer_connect(&config, &mut swarm);

        // construct internet object
        let internet = Internet { swarm: swarm, receiver: response_rcv };

        (config, internet)
    }

    /**
     * connect to remote peers that are specified in 
     * the configuration config.internet.peers
     */
    pub fn peer_connect( config: &Configuration, swarm: &mut Swarm<QaulInternetBehaviour> ) {
        for addr in &config.internet.peers {
            let tried = addr.clone();
            match addr.parse() {
                Ok(addr) => match swarm.dial_addr(addr) {
                    Ok(_) => info!("peer {:?} dialed", tried),
                    Err(error) => info!("peer {} swarm dial error: {:?}", tried, error),
                },
                Err(error) => info!("peer address {} parse error: {:?}", tried, error),
            }
        }
    }

    /**
     * Print information about this connection
     */
    pub fn info(&self) {
        println!("# Internet Connection Module");
        // number of peers connected
        println!("{} peer(s) connected", self.swarm.network_info().num_peers());
    }
}

impl NetworkBehaviourEventProcess<IdentifyEvent> for QaulInternetBehaviour {
    fn inject_event(&mut self, event: IdentifyEvent) {
        info!("{:?}", event);
        match event {
            IdentifyEvent::Received { peer_id, info } => {
                self.floodsub.add_node_to_partial_view(peer_id);
                info!("added peer_id {:?} to floodsub", peer_id);
            },
            _ => info!("unhandled event"),
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for QaulInternetBehaviour {
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
