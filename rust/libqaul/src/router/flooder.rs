// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The flooder floods messages via floodsub/Gossipsub to the network.
//! It contains a ring buffer of messages to process.
//!
//! It sends the messages of the feed service.
//!
//! Most messages are repostings from incoming floods on some interface,
//! that need to be flooded via the other interfaces.

use crate::connections::ConnectionModule;
use libp2p::floodsub::Topic;
use std::collections::VecDeque;
use std::sync::RwLock;

pub struct FloodMessageContainer {
    pub message: Vec<u8>,
    pub topic: Topic,
    pub incoming_via: ConnectionModule,
}

pub struct Flooder {
    pub to_send: VecDeque<FloodMessageContainer>,
}

/// Instance-based flooder state.
pub struct FlooderState {
    pub inner: RwLock<Flooder>,
}

impl FlooderState {
    /// Create a new empty flooder state.
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Flooder {
                to_send: VecDeque::new(),
            }),
        }
    }

    /// Add a message to the flood queue.
    pub fn add(&self, message: Vec<u8>, topic: Topic, incoming_via: ConnectionModule) {
        let msg = FloodMessageContainer {
            message,
            topic,
            incoming_via,
        };
        let mut flooder = self.inner.write().unwrap();
        flooder.to_send.push_back(msg);
    }
}

impl Flooder {
    /// Add a message to the ring buffer for sending.
    /// Delegates to the global RouterState instance.
    pub fn add(message: Vec<u8>, topic: Topic, incoming_via: ConnectionModule) {
        super::RouterState::global().flooder.add(message, topic, incoming_via);
    }
}
