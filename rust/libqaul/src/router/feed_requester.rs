// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Feed Requester
//!
//! As feed messages flooded in the entire network.
//! In case a user joins the network newly or a feed message
//! was missed, the latest feed id's are synchronized via the
//! qaul router info service.
//!
//! With each routing information the last feed messages are
//! advertised and can be requested from the sending node.

use libp2p::PeerId;
use state::InitCell;
use std::collections::VecDeque;
use std::sync::RwLock;

/// mutable state of feed requester
pub static FEEDREQUESTER: InitCell<RwLock<FeedRequester>> = InitCell::new();

/// mutable state of the feed responser
pub static FEEDRESPONSER: InitCell<RwLock<FeedResponser>> = InitCell::new();

/// Feed Request Structure
pub struct FeedRequest {
    pub neighbour_id: PeerId,
    pub feed_ids: Vec<Vec<u8>>,
}

/// Feed Requester Module
pub struct FeedRequester {
    pub to_send: VecDeque<FeedRequest>,
}

impl FeedRequester {
    /// Initialize the flooder and create the ring buffer.
    pub fn init() {
        let feed_requester = FeedRequester {
            to_send: VecDeque::new(),
        };
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

/// Feed Response Structure
pub struct FeedResponse {
    pub neighbour_id: PeerId,
    pub feeds: Vec<(Vec<u8>, Vec<u8>, String, u64)>,
}

/// Feed Responder
pub struct FeedResponser {
    pub to_send: VecDeque<FeedResponse>,
}

impl FeedResponser {
    /// Initialize the flooder and create the ring buffer.
    pub fn init() {
        let feed_responser = FeedResponser {
            to_send: VecDeque::new(),
        };
        FEEDRESPONSER.set(RwLock::new(feed_responser));
    }

    /// Add a message to the ring buffer for sending.
    pub fn add(neighbour_id: &PeerId, feeds: &Vec<(Vec<u8>, Vec<u8>, String, u64)>) {
        let mut msg = FeedResponse {
            neighbour_id: neighbour_id.clone(),
            feeds: vec![],
        };
        for (message_id, sender_id, content, time) in feeds {
            msg.feeds.push((
                message_id.clone(),
                sender_id.clone(),
                content.clone(),
                *time,
            ));
        }

        // add it to sending queue
        let mut feed_responser = FEEDRESPONSER.get().write().unwrap();
        feed_responser.to_send.push_back(msg);
    }
}
