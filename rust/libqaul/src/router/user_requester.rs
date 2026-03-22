// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! User Requester
//!
//! When routing information is received and contains unknown users,
//! the user information is requested from the sending node.

use libp2p::PeerId;
use std::collections::VecDeque;
use std::sync::RwLock;

/// User Request Structure
pub struct UserRequest {
    pub neighbour_id: PeerId,
    pub user_ids: Vec<Vec<u8>>,
}

/// User Requester Module
pub struct UserRequester {
    pub to_send: VecDeque<UserRequest>,
}

/// Instance-based user requester state.
pub struct UserRequesterState {
    pub inner: RwLock<UserRequester>,
}

impl UserRequesterState {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(UserRequester {
                to_send: VecDeque::new(),
            }),
        }
    }

    pub fn add(&self, neighbour_id: &PeerId, user_ids: &[Vec<u8>]) {
        let msg = UserRequest {
            neighbour_id: neighbour_id.clone(),
            user_ids: user_ids.to_vec(),
        };
        let mut user_requester = self.inner.write().unwrap();
        user_requester.to_send.push_back(msg);
    }
}

impl UserRequester {
    /// Add a message to the ring buffer for sending.
    /// Delegates to the provided RouterState instance.
    pub fn add(router: &super::RouterState, neighbour_id: &PeerId, user_ids: &[Vec<u8>]) {
        router.user_requester.add(neighbour_id, user_ids);
    }
}

/// User Response Structure
pub struct UserResponse {
    pub neighbour_id: PeerId,
    pub users: super::router_net_proto::UserInfoTable,
}

/// User Responder
pub struct UserResponser {
    pub to_send: VecDeque<UserResponse>,
}

/// Instance-based user responser state.
pub struct UserResponserState {
    pub inner: RwLock<UserResponser>,
}

impl UserResponserState {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(UserResponser {
                to_send: VecDeque::new(),
            }),
        }
    }

    pub fn add(&self, neighbour_id: &PeerId, table: &super::router_net_proto::UserInfoTable) {
        let msg = UserResponse {
            neighbour_id: neighbour_id.clone(),
            users: table.clone(),
        };
        let mut user_responser = self.inner.write().unwrap();
        user_responser.to_send.push_back(msg);
    }
}

impl UserResponser {
    /// Add a message to the ring buffer for sending.
    /// Delegates to the provided RouterState instance.
    pub fn add(router: &super::RouterState, neighbour_id: &PeerId, table: &super::router_net_proto::UserInfoTable) {
        router.user_responser.add(neighbour_id, table);
    }
}
