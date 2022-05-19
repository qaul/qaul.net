// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The flooder floods messages via floodsub/Gossipsub to the network.
//! It contains a ring buffer of messages to process.
//! 
//! It sends the messages of the feed service.
//! 
//! Most messages are repostings from incoming floods on some interface,
//! that need to be flooded via the other interfaces.


use state::Storage;
use std::sync::RwLock;
use std::collections::VecDeque;
use libp2p::PeerId;

// mutable state of feed messages
pub static FEEDREQUESTER: Storage<RwLock<FeedRequester>> = Storage::new();

pub struct FeedRequest {
    pub neighbour_id: PeerId,
    pub feed_ids: Vec<Vec<u8>>,
}

pub struct FeedRequester {
    pub to_send: VecDeque<FeedRequest>,
}

impl FeedRequester {
    /// Initialize the flooder and create the ring buffer.
    pub fn init() {
        let feed_requester = FeedRequester { to_send: VecDeque::new() };
        FEEDREQUESTER.set(RwLock::new(feed_requester));
    }

    /// Add a message to the ring buffer for sending.
    pub fn add(neighbour_id: &PeerId, feed_ids: &Vec<Vec<u8>>) {
        let msg = FeedRequest {
            neighbour_id: neighbour_id.clone(),
            feed_ids: feed_ids.clone(),
        };

        // add it to sending queue
        let mut feed_requester = FEEDREQUESTER.get().write().unwrap();
        feed_requester.to_send.push_back(msg);
    }
}


pub static FEEDRESPONSER: Storage<RwLock<FeedResponser>> = Storage::new();
pub struct FeedResponse {
    pub neighbour_id: PeerId,
    pub feeds: Vec<(Vec<u8>, Vec<u8>, String, u64)>,
}

pub struct FeedResponser {
    pub to_send: VecDeque<FeedResponse>,
}

impl FeedResponser {
    /// Initialize the flooder and create the ring buffer.
    pub fn init() {
        let feed_responser = FeedResponser { to_send: VecDeque::new() };
        FEEDRESPONSER.set(RwLock::new(feed_responser));
    }

    /// Add a message to the ring buffer for sending.
    pub fn add(neighbour_id: &PeerId, feeds: &Vec<(Vec<u8>, Vec<u8>, String, u64)>) {
        let mut msg = FeedResponse {
            neighbour_id: neighbour_id.clone(),
            feeds: vec![],
        };
        for (message_id, sender_id, content, time) in feeds{
            msg.feeds.push((message_id.clone(), sender_id.clone(), content.clone(), *time));
        }

        // add it to sending queue
        let mut feed_responser = FEEDRESPONSER.get().write().unwrap();
        feed_responser.to_send.push_back(msg);
    }
}
