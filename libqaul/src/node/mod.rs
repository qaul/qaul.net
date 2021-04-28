use libp2p::{
    PeerId,
    identity,
    floodsub::Topic,
    swarm::Swarm,
};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use log::info;

pub mod mdns;
use mdns::QaulBehaviour;


// pub struct Node {
//     id: PeerId,
//     keys: identity::Keypair,
//     topic: Topic,
// }

// impl Node {
//     pub fn new() {
        
//     }
// }


/**
 * Keypair for node
 */
static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());

/**
 * get a keys reference
 */
pub fn get_keys<'a>() -> &'a identity::Keypair {
    &KEYS
}


/**
 * PeerId handling of libp2p
 * 
 * The PeerId is the identity of a node in the network.
 * Each node can have several users.
 */
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));

/**
 * get a cloned PeerId
 */
pub fn get_id() -> PeerId {
    PEER_ID.clone()
}

/**
 * get the string of a PeerId
 */
pub fn get_id_string() -> String {
    PEER_ID.to_string()
}


/**
 * TOPIC
 */
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

/**
 * get a cloned Topic
 */
pub fn get_topic() -> Topic {
    TOPIC.clone()
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
