use libp2p::{
    PeerId,
    identity::{Keypair, ed25519},
    floodsub::Topic,
    swarm::Swarm,
};
use std::collections::HashSet;
use log::info;
use base64;
use state;
use crate::configuration::Configuration;
use crate::connections::lan::QaulLanBehaviour;


pub struct Node {
    id: PeerId,
    keys: Keypair,
    topic: Topic,
}

static STATE: state::Storage<Node> = state::Storage::new();

impl Node {
    // start an existing node from the config parameters
    pub fn init( mut config: Configuration) -> Configuration {
        if config.node.initialized == 0 {
            // create a new node and save it to configuration
            config = Self::new(config);
        }
        else {
            // instantiate node from configuration
            config = Self::from_config(config);
        }

        config
    }

    // create a new node and save the parameters into config
    fn new(mut config: Configuration) -> Configuration {
        // create node & configuration
        let keys_ed25519 = ed25519::Keypair::generate();
        config.node.keys = base64::encode(keys_ed25519.encode());
        let keys = Keypair::Ed25519(keys_ed25519);
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");
        let node = Node {id, keys, topic};
        config.node.id = id.to_string();
        config.node.initialized = 1;

        // save configuration file
        Configuration::save(&config);

        // display id
        info!("Peer Id: {}", node.id.clone());

        // save node to state
        STATE.set(node);

        config
    }

    // start an existing node from the config parameters
    fn from_config(config: Configuration) -> Configuration {
        let mut basedecode = base64::decode(&config.node.keys).unwrap();
        let keys = Keypair::Ed25519(ed25519::Keypair::decode( &mut basedecode).unwrap());
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");

        // check if saved ID and the id from the keypair are equal
        if id.to_string() == config.node.id {
            info!("id's match {}", config.node.id);
        } 
        else {
            info!("------------------------------------");
            info!("ERROR: id's are not equal");
            info!("{}  {}", id.to_string(), config.node.id);
            info!("------------------------------------");
        }

        let node = Node {id, keys, topic};
        STATE.set(node);

        config
    }

    /**
     * get a cloned PeerId
     */
    pub fn get_id() -> PeerId {
        let node = STATE.get();
        node.id.clone()
    }

    /**
     * get the string of a PeerId
     */
    pub fn get_id_string() -> String {
        let node = STATE.get();
        node.id.to_string()
    }

    /**
     * Get the Keypair for of this node
     */
    pub fn get_keys<'a>() -> &'a Keypair {
        let node = STATE.get();
        &node.keys
    }

    /**
     * get the cloned Topic
     */
    pub fn get_topic() -> Topic {
        let node = STATE.get();
        node.topic.clone()
    }
}


/**
 * print peers
 */
pub async fn print_peers(swarm: &mut Swarm<QaulLanBehaviour>) {
    // List all connections
    println!("{} peer(s) connected", swarm.network_info().num_peers());

    // List mdns peers
    println!("Discovered mdns peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| info!("{}", p));
}
