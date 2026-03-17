// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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
use std::collections::VecDeque;
use std::sync::RwLock;

/// Feed Request Structure
pub struct FeedRequest {
    pub neighbour_id: PeerId,
    pub feed_ids: Vec<Vec<u8>>,
}

/// Feed Requester Module
pub struct FeedRequester {
    pub to_send: VecDeque<FeedRequest>,
}

/// Instance-based feed requester state.
pub struct FeedRequesterState {
    pub inner: RwLock<FeedRequester>,
}

impl FeedRequesterState {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(FeedRequester {
                to_send: VecDeque::new(),
            }),
        }
    }

    pub fn add(&self, neighbour_id: &PeerId, feed_ids: &[Vec<u8>]) {
        let msg = FeedRequest {
            neighbour_id: neighbour_id.clone(),
            feed_ids: feed_ids.to_vec(),
        };
        let mut feed_requester = self.inner.write().unwrap();
        feed_requester.to_send.push_back(msg);
    }
}

impl FeedRequester {
    /// Add a message to the ring buffer for sending.
    /// Delegates to the global RouterState instance.
    pub fn add(neighbour_id: &PeerId, feed_ids: &[Vec<u8>]) {
        super::RouterState::global().feed_requester.add(neighbour_id, feed_ids);
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

/// Instance-based feed responser state.
pub struct FeedResponserState {
    pub inner: RwLock<FeedResponser>,
}

impl FeedResponserState {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(FeedResponser {
                to_send: VecDeque::new(),
            }),
        }
    }

    pub fn add(&self, neighbour_id: &PeerId, feeds: &[(Vec<u8>, Vec<u8>, String, u64)]) {
        let msg = FeedResponse {
            neighbour_id: neighbour_id.clone(),
            feeds: feeds.to_vec(),
        };
        let mut feed_responser = self.inner.write().unwrap();
        feed_responser.to_send.push_back(msg);
    }
}

impl FeedResponser {
    /// Add a message to the ring buffer for sending.
    /// Delegates to the global RouterState instance.
    pub fn add(neighbour_id: &PeerId, feeds: &[(Vec<u8>, Vec<u8>, String, u64)]) {
        super::RouterState::global().feed_responser.add(neighbour_id, feeds);
    }
}
