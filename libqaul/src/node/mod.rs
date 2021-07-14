pub mod users;

use libp2p::{
    PeerId,
    identity::{Keypair, ed25519},
    floodsub::Topic,
};
use log::{error, info};
use base64;
use state;
use crate::configuration::Configuration;
use users::Users;


pub struct Node {
    id: PeerId,
    keys: Keypair,
    topic: Topic,
}

static STATE: state::Storage<Node> = state::Storage::new();

impl Node {
    // start an existing node from the config parameters
    pub fn init() {
        // initialize node
        {
            if !Configuration::is_node_initialized() {
                // create a new node and save it to configuration
                info!("Create a new node.");
                Self::new();
            }
            else {
                // instantiate node from configuration
                info!("Setup node from configuration.");
                Self::from_config();
            }
        }

        // initialize users of this node
        Users::init();
    }

    // create a new node and save the parameters into config
    fn new() {
        // create node
        let keys_ed25519 = ed25519::Keypair::generate();
        let keys = Keypair::Ed25519(keys_ed25519.clone());
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");
        let node = Node {id, keys, topic};

        // save node to configuration file
        {
            let mut config = Configuration::get_mut();
            config.node.keys = base64::encode(keys_ed25519.encode());
            config.node.id = id.to_string();
            config.node.initialized = 1;
        }
        Configuration::save();

        // display id
        info!("Peer Id: {}", node.id.clone());

        // save node to state
        STATE.set(node);
    }

    // start an existing node from the config parameters
    fn from_config() {
        let config = Configuration::get();
        let mut basedecode = base64::decode(&config.node.keys).unwrap();
        let keys = Keypair::Ed25519(ed25519::Keypair::decode( &mut basedecode).unwrap());
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");

        // check if saved ID and the id from the keypair are equal
        if id.to_string() == config.node.id {
            info!("id's match {}", config.node.id);
        } 
        else {
            error!("------------------------------------");
            error!("ERROR: id's are not equal");
            error!("{}  {}", id.to_string(), config.node.id);
            error!("------------------------------------");
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
    }

    /**
     * get the string of a PeerId
     */
    pub fn get_id_string() -> String {
        let node = STATE.get();
        node.id.to_string()
    }

    /**
     * Get the Keypair of this node
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
