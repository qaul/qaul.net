// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Node Module
//!
//! Creates a node on first startup.
//! Loads the node definition from configuration and keeps it in
//! local state.
//! Provides state information of the local node to libqaul.

pub mod user_accounts;

use base64;
use base64::Engine;
use libp2p::identity::ed25519;
use libp2p::{floodsub::Topic, identity::Keypair, PeerId};
use prost::Message;
use std::sync::{Arc, RwLock};

use crate::connections::{internet::Internet, lan::Lan};
use crate::rpc::Rpc;
use crate::storage::configuration::Configuration;
use crate::utilities::qaul_id::QaulId;
use user_accounts::UserAccounts;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_node as proto;

/// Node Module - holds node identity and user accounts for a single instance
///
/// This struct is used for instance-based access to node state.
/// It wraps the node identity (PeerId, Keypair) and user accounts.
pub struct NodeModule {
    /// The node identity
    pub node: Arc<NodeIdentity>,
    /// User accounts managed by this node
    pub user_accounts: Arc<RwLock<UserAccounts>>,
}

impl NodeModule {
    /// Create a new NodeModule from configuration
    ///
    /// This is the instance-based API. For backward compatibility,
    /// use `Node::init()` which uses global state.
    pub fn new(config: &Configuration) -> Self {
        // Create user accounts first
        let user_accounts = UserAccounts::create_from_config(config);

        // Create or load node identity
        let node = if config.node.initialized == 0 {
            NodeIdentity::generate()
        } else {
            NodeIdentity::from_config(config)
        };

        Self {
            node: Arc::new(node),
            user_accounts: Arc::new(RwLock::new(user_accounts)),
        }
    }

    /// Get the node's PeerId
    pub fn id(&self) -> PeerId {
        self.node.id
    }

    /// Get reference to the node's Keypair
    pub fn keys(&self) -> &Keypair {
        &self.node.keys
    }

    /// Get the node's small ID
    pub fn small_id(&self) -> Vec<u8> {
        QaulId::to_q8id(self.node.id)
    }

    /// Get the node's topic
    pub fn topic(&self) -> Topic {
        self.node.topic.clone()
    }
}

/// Node Identity - holds the cryptographic identity of a node
pub struct NodeIdentity {
    pub id: PeerId,
    pub keys: Keypair,
    pub topic: Topic,
}

impl NodeIdentity {
    /// Generate a new node identity
    pub fn generate() -> Self {
        let keys = Keypair::generate_ed25519();
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");

        Self { id, keys, topic }
    }

    /// Load node identity from configuration
    pub fn from_config(config: &Configuration) -> Self {
        let mut basedecode = base64::engine::general_purpose::STANDARD
            .decode(&config.node.keys)
            .unwrap();
        let ed25519_keys = ed25519::Keypair::try_from_bytes(&mut basedecode).unwrap();
        let keys = Keypair::from(ed25519_keys);
        let id = PeerId::from(keys.public());
        let topic = Topic::new("pages");

        // Verify ID matches
        if id.to_string() != config.node.id {
            log::error!("------------------------------------");
            log::error!("ERROR: id's are not equal");
            log::error!("{}  {}", id.to_string(), config.node.id);
            log::error!("------------------------------------");
        } else {
            log::trace!("id's match {}", config.node.id);
        }

        Self { id, keys, topic }
    }

    /// Encode keys for configuration storage
    pub fn keys_to_config(&self) -> String {
        base64::engine::general_purpose::STANDARD
            .encode(self.keys.clone().try_into_ed25519().unwrap().to_bytes())
    }
}

/// This Node (global state wrapper - delegates to QaulState)
pub struct Node;

impl Node {
    /// Get a snapshot of the NodeIdentity from QaulState.
    fn state() -> Arc<NodeIdentity> {
        crate::QaulState::global().get_node()
    }

    /// start an existing node from the config parameters
    ///
    /// This is now a no-op: node identity is created during `Libqaul::new()`
    /// via `NodeModule::new()` and stored in `QaulState`. Kept for backward
    /// compatibility so existing call sites don't break.
    pub fn init() {
        // UserAccounts::init() is also a no-op now; call it for compat.
        UserAccounts::init();

        if !Configuration::is_node_initialized() {
            // Save the generated node identity to configuration
            log::trace!("Create a new node (saving identity to config).");
            Self::save_to_config();
        } else {
            log::trace!("Node already initialized from configuration.");
        }
    }

    /// Save the current node identity (from QaulState) into the configuration file.
    /// Called when a brand-new node is created.
    fn save_to_config() {
        let node = Self::state();
        {
            let mut config = Configuration::get_mut();
            config.node.keys = node.keys_to_config();
            config.node.id = node.id.to_string();
            config.node.initialized = 1;
        }
        Configuration::save();
        log::trace!("Peer Id: {}", node.id);
    }

    /// get a cloned PeerId
    pub fn get_id() -> PeerId {
        Self::state().id
    }

    /// get small node ID
    pub fn get_q8id() -> Vec<u8> {
        QaulId::to_q8id(Self::state().id)
    }

    /// get the string of a PeerId
    pub fn get_id_string() -> String {
        Self::state().id.to_string()
    }

    /// Get the Keypair of this node (cloned)
    pub fn get_keys() -> Keypair {
        Self::state().keys.clone()
    }

    /// get the cloned Topic
    pub fn get_topic() -> Topic {
        Self::state().topic.clone()
    }

    /// Process incoming RPC request messages for node module
    pub fn rpc(
        data: Vec<u8>,
        lan: Option<&mut Lan>,
        internet: Option<&mut Internet>,
        request_id: String,
    ) {
        match proto::Node::decode(&data[..]) {
            Ok(node) => {
                match node.message {
                    Some(proto::node::Message::GetNodeInfo(_)) => {
                        Rpc::increase_message_counter();

                        // create address list
                        let mut addresses: Vec<String> = Vec::new();
                        if let Some(internet_connection) = internet {
                            // listener addresses
                            for address in internet_connection.swarm.listeners() {
                                addresses.push(address.to_string());
                            }
                            // external addresses
                            for address in internet_connection.swarm.external_addresses() {
                                addresses.push(address.to_string());
                            }
                        } else if let Some(lan_connection) = lan {
                            // listener addresses
                            for address in lan_connection.swarm.listeners() {
                                addresses.push(address.to_string());
                            }
                            // external addresses
                            for address in lan_connection.swarm.external_addresses() {
                                addresses.push(address.to_string());
                            }
                        } else {
                            log::error!("lan & internet swarms missing");
                        }

                        // create response message
                        let proto_nodeinformation = proto::NodeInformation {
                            id_base58: Self::get_id_string(),
                            addresses,
                        };

                        let proto_message = proto::Node {
                            message: Some(proto::node::Message::Info(proto_nodeinformation)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Node.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    _ => {
                        log::error!("rpc message undefined");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}
