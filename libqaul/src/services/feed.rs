use libp2p::{
    identity::{Keypair, PublicKey},
    PeerId,
};
use bincode;
use humantime;
use log::{info, error};
use serde::{Serialize, Deserialize};
use state::Storage;
use std::sync::RwLock;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::node;
use crate::node::Node; 
use crate::connections::{Connections, ConnectionModule};
use crate::router;
use crate::router::flooder::Flooder;
//use crate::connections::internet::QaulInternetBehaviour;
//use crate::connections::lan::QaulLanBehaviour;


// mutable state of feed messages
static FEED: Storage<RwLock<Feed>> = Storage::new();


#[derive(Debug, Clone)]
pub struct FeedMessage {
    /// the user id of the user sending this message
    pub sender: PeerId,
    /// the content of the message
    pub content: String,
    /// the time when this message was sent in seconds
    pub time: SystemTime,
}

impl FeedMessage {
    pub fn format_to_send( &self ) -> FeedMessageSend {
        FeedMessageSend {
            sender: self.sender.to_bytes(),
            content: self.content.clone(),
            time: self.time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
        }
    }

    pub fn format_from_send( message: &FeedMessageSend ) -> Self {
        FeedMessage {
            sender: PeerId::from_bytes(&message.sender).unwrap(),
            content: message.content.clone(),
            time: SystemTime::from(UNIX_EPOCH + Duration::from_secs_f64(message.time)),
        }
    }
}

/**
 * Serializable format of the feed message
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedMessageSend {
    /// the user id of the user sending this message
    pub sender: Vec<u8>,
    /// the content of the message
    pub content: String,
    /// the time when this message was sent in seconds
    pub time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedMessageSendContainer {
    pub message: FeedMessageSend,
    pub id: Vec<u8>,
}

pub struct Feed {
    pub messages: BTreeMap< Vec<u8>, FeedMessage>,
}

impl Feed {
    pub fn init() {
        // create feed messages state
        let feed = Feed { messages: BTreeMap::new() };
        FEED.set(RwLock::new(feed));
    }

    /**
     * Send message via all swarms
     */
    pub fn send(user: &node::users::User, cmd: &str,  conn: &mut Connections )
    {
        let msg = FeedMessage {
            sender: user.id,
            content: cmd.to_string(),
            time: SystemTime::now(),
        };
        let msg_send = msg.clone().format_to_send();

        // sign message
        let signature = Self::sign_message(msg_send.clone(), user.keys.clone());
        // create signed container
        let container = FeedMessageSendContainer { id: signature , message: msg_send };

        // create message json
        // TODO: couldn't it be sent directly as bytes?
        let json = serde_json::to_string(&container).expect("can jsonify request");
        
        // save message in feed store
        {
            let mut feed = FEED.get().write().unwrap();
            feed.messages.insert(container.id.clone(), msg.clone());
        }

        // flood via floodsub
        conn.lan.swarm.behaviour_mut().floodsub.publish(Node::get_topic(), json.as_bytes());
        conn.internet.swarm.behaviour_mut().floodsub.publish(Node::get_topic(), json.as_bytes());
    }

    /**
     * Process a received message
     */
    pub fn received( via_conn: ConnectionModule, _via_node: PeerId, container: FeedMessageSendContainer ) {
        let message = FeedMessage::format_from_send( &container.message );
        
        // check if sending user public is in user store
        let result = router::users::Users::get_pub_key(&message.sender);
        
        let mut user_known = false;
        let mut msg_valid = false;
        if let Some(key) = result {
            user_known = true;
            // validate message
            if Self::validate_message(&container, key) {
                msg_valid = true;
            } else {
                error!("Validation of message {:?}, from {} failed", container.id, message.sender);
            }
        } else {
            error!("User not known: {}", message.sender);
        }
        
        info!("user known: {}, message valid: {}", user_known, msg_valid);

        // get feed store
        let mut feed = FEED.get().write().unwrap();

        // check if message exists
        if !feed.messages.contains_key(&container.id) {
            // write message to store
            feed.messages.insert(container.id.clone(), message.clone());

            // display message
            info!("message received:");
            info!("{}, {:?}", humantime::format_rfc3339(message.time), container.id);
            info!("  '{}'", container.message.content);

            // forward message
            let json = serde_json::to_string(&container).expect("can jsonify request");
            let bytes = json.as_bytes();
            Flooder::add(bytes.to_vec(), Node::get_topic(), via_conn);
        } else {
            info!("message key {:?} already in store", container.id);
        }
    }

    /**
     * Sign a message with the private key
     * The signature can be validated with the corresponding public key.
     */
    pub fn sign_message ( message: FeedMessageSend, keys: Keypair ) -> Vec<u8> {
        let buf = bincode::serialize(&message).unwrap();
        keys.sign(&buf).unwrap()
    }

    pub fn validate_message( msg: &FeedMessageSendContainer, key: PublicKey ) -> bool {
        let buf = bincode::serialize(&msg.message).unwrap();
        key.verify(&buf, &msg.id)
    }

    pub fn cli(cmd: &str, connections: &mut Connections ) {        
        match cmd {
            // list all messages
            "list" => {
                println!("feed messages:");
                let feed = FEED.get().read().unwrap();
                for (id,message) in &feed.messages {
                    // print meta data
                    println!("{}, {:?}", humantime::format_rfc3339(message.time), id);
                    // print message
                    println!("  '{}'", message.content);
                }
            },
            // send a new feed message
            cmd if cmd.starts_with("send ") => {
                // check if a user exists
                if node::users::Users::len() < 1 {
                    // please register a user account first
                    println!("Please create a user account first:");
                    println!("  user create USERNAME");
                } else {
                    // use the first user account for it
                    let user = node::users::Users::get_default_user();
                    // send the message
                    Self::send( &user, cmd.strip_prefix("send ").unwrap(), connections );
                }
            },
            _ => error!("unknown user command"),
        }
    }
}


