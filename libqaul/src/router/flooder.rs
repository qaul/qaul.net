/**
 * The flooder floods messages via floodsub/Gossipsub to the network.
 * It contains a ring buffer of messages to process.
 * 
 * It sends the messages of the feed service.
 * 
 * Most messages are repostings from incoming floods on some interface,
 * that need to be flooded via the other interfaces.
 */

use libp2p::floodsub::Topic;
use state::Storage;
use std::sync::RwLock;
use std::collections::VecDeque;
use crate::connections::ConnectionModule;

// mutable state of feed messages
pub static FLOODER: Storage<RwLock<Flooder>> = Storage::new();


pub struct FloodMessageContainer {
    pub message: Vec<u8>,
    pub topic: Topic,
    pub incoming_via: ConnectionModule,
}

pub struct Flooder {
    pub to_send: VecDeque<FloodMessageContainer>,
}

impl Flooder {
    /**
     * Initialize the flooder and create the ring buffer.
     */
    pub fn init() {
        let flooder = Flooder { to_send: VecDeque::new() };
        FLOODER.set(RwLock::new(flooder));
    }

    /**
     * Add a message to the ring buffer for sending.
     */
    pub fn add(message: Vec<u8>, topic: Topic, incoming_via: ConnectionModule) {
        let msg = FloodMessageContainer {
            message,
            topic,
            incoming_via,
        };

        // add it to sending queue
        let mut flooder = FLOODER.get().write().unwrap();
        flooder.to_send.push_back(msg);    
    }
}
