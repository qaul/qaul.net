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

/// mutable state of user requester
pub static USERREQUESTER: InitCell<RwLock<UserRequester>> = InitCell::new();

/// mutable state of the user responser
pub static USERRESPONSER: InitCell<RwLock<UserResponser>> = InitCell::new();

/// User Request Structure
pub struct UserRequest {
    pub neighbour_id: PeerId,
    pub user_ids: Vec<Vec<u8>>,
}

/// User Requester Module
pub struct UserRequester {
    pub to_send: VecDeque<UserRequest>,
}

impl UserRequester {
    /// Initialize and create the ring buffer.
    pub fn init() {
        let user_requester = UserRequester {
            to_send: VecDeque::new(),
        };
        USERREQUESTER.set(RwLock::new(user_requester));
    }

    /// Add a message to the ring buffer for sending.
    pub fn add(neighbour_id: &PeerId, user_ids: &Vec<Vec<u8>>) {
        let msg = UserRequest {
            neighbour_id: neighbour_id.clone(),
            user_ids: user_ids.clone(),
        };

        // add it to sending queue
        let mut user_requester = USERREQUESTER.get().write().unwrap();
        user_requester.to_send.push_back(msg);
    }
}

/// User Response Structure
pub struct UserResponse {
    pub neighbour_id: PeerId,
    pub users: super::router_net_proto::UserInfoTable,
}

/// Feed Responder
pub struct UserResponser {
    pub to_send: VecDeque<UserResponse>,
}

impl UserResponser {
    /// Initialize and create the ring buffer.
    pub fn init() {
        let user_responser = UserResponser {
            to_send: VecDeque::new(),
        };
        USERRESPONSER.set(RwLock::new(user_responser));
    }

    /// Add a message to the ring buffer for sending.
    pub fn add(neighbour_id: &PeerId, table: &super::router_net_proto::UserInfoTable) {
        let msg = UserResponse {
            neighbour_id: neighbour_id.clone(),
            users: table.clone(),
        };
        // add it to sending queue
        let mut user_responser = USERRESPONSER.get().write().unwrap();
        user_responser.to_send.push_back(msg);
    }
}
