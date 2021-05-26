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

pub mod mdns;
use mdns::QaulBehaviour;

use crate::configuration::Configuration;


pub struct Node {
    id: PeerId,
    keys: Keypair,
    topic: Topic,
}

static STATE: state::Storage<Node> = state::Storage::new();

// struct Node_State {
//     node: Option<Node>,
// }
//static mut NODE_STATE: Node_State = Node_State { node: None };


impl Node {
    // create a new node and save the parameters into config
    pub fn new(mut config: Configuration) -> Configuration {
        let keys_ed25519 = ed25519::Keypair::generate();
        config.node.keys = base64::encode(keys_ed25519.encode());

        //let keys = Keypair::generate_ed25519();
        let keys = Keypair::Ed25519(keys_ed25519);
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");

        let node = Node {id, keys, topic};
        
        // write keys & ID into the configuration
        config.node.id = id.to_string();
        

        // if let Keypair::Ed25519(edkeys) = keys {
        //     config.node.keys = base64::encode(edkeys.encode());
        // }
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
    pub fn init(config: Configuration) {
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
    }

    /**
     * get a cloned PeerId
     */
    pub fn get_id() -> PeerId {
        let node = STATE.get();
        node.id.clone()

        // if let Some(node) = NODE_STATE.node {
        //     node.id.clone()
        // }
        // else {
        //     panic!("node not initialized")
        // }
    }

    /**
     * get the string of a PeerId
     */
    pub fn get_id_string() -> String {
        let node = STATE.get();
        node.id.to_string()
        
        // if let Some(node) = NODE_STATE.node {
        //     node.id.to_string()
        // }
        // else {
        //     panic!("node not initialized")
        // }
    }

    /**
     * Get the Keypair for of this node
     */
    pub fn get_keys<'a>() -> &'a Keypair {
        let node = STATE.get();
        &node.keys
                
        // if let Some(node) = NODE_STATE.node {
        //     &node.keys
        // }
        // else {
        //     panic!("node not initialized")
        // }
    }

    /**
     * get the cloned Topic
     */
    pub fn get_topic() -> Topic {
        let node = STATE.get();
        node.topic.clone()
        
        // if let Some(node) = NODE_STATE.node {
        //     node.topic.clone()
        // }
        // else {
        //     panic!("node not initialized")
        // }
    }
}


/**
 * print peers
 */
pub async fn handle_list_peers(swarm: &mut Swarm<QaulBehaviour>) {
    info!("Discovered Peers:");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers = HashSet::new();
    for peer in nodes {
        unique_peers.insert(peer);
    }
    unique_peers.iter().for_each(|p| info!("{}", p));
}
