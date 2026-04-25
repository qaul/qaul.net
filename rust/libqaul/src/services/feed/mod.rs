// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Feed Service
//!
//! The feed service sends and receives feed messages into the network.
//! Feed messages are not encrypted and for everybody to read.
//! They should reach everyone in the network.

//use bs58::decode;
use libp2p::{
    identity::{Keypair, PublicKey},
    PeerId,
};
use prost::Message;
use serde::{Deserialize, Serialize};
use sled;
use std::collections::BTreeMap;
use std::{convert::TryInto, sync::RwLock};

use crate::node::{
    user_accounts::{UserAccount, UserAccounts},
    Node,
};

use crate::connections::{internet::Internet, lan::Lan, ConnectionModule};
use crate::router;
use crate::router::flooder::Flooder;
use crate::rpc::Rpc;
use crate::storage::database::DataBase;
use crate::utilities::timestamp;

pub use qaul_proto::qaul_net_feed as proto_net;
/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_feed as proto;

/// For storing in data base
#[derive(Serialize, Deserialize, Clone)]
pub struct FeedMessageData {
    /// index of message in the data base
    pub index: u64,
    /// hash of the message
    pub message_id: Vec<u8>,
    /// user ID of the sender
    pub sender_id: Vec<u8>,
    /// time sent in milli seconds
    pub timestamp_sent: u64,
    /// time received in milli seconds
    pub timestamp_received: u64,
    /// the message content
    pub content: String,
}

/// qaul Feed storage and logic
pub struct Feed {
    /// in memory BTreeMap of messages
    pub messages: BTreeMap<Vec<u8>, proto_net::FeedMessageContent>,

    /// sled data base tree for message_id to last index
    ///
    /// value: bincode of `u64`
    pub tree_ids: sled::Tree,

    /// sled data base tree of
    ///
    /// value: bincode of `FeedMessageData`
    pub tree: sled::Tree,
    /// last recent message
    pub last_message: u64,
}

/// Instance-based feed state owning feed messages and database references.
/// Replaces the global FEED static for multi-instance use.
pub struct FeedState {
    /// Feed inner state.
    pub inner: RwLock<Feed>,
    /// Sled database backing (kept alive for tree references).
    /// Wrapped in RwLock so `init_production` can swap it after construction.
    _db: RwLock<sled::Db>,
}


impl FeedState {
    /// Create a FeedState from production sled database.
    pub fn from_production(db: sled::Db, tree: sled::Tree, tree_ids: sled::Tree, last_message: u64) -> Self {
        Self {
            inner: RwLock::new(Feed {
                messages: BTreeMap::new(),
                tree,
                tree_ids,
                last_message,
            }),
            _db: RwLock::new(db),
        }
    }

    /// Create a new empty FeedState with a temporary in-memory database.
    pub fn new() -> Self {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("feed").unwrap();
        let tree_ids = db.open_tree("feed_id").unwrap();
        Self {
            inner: RwLock::new(Feed {
                messages: BTreeMap::new(),
                tree,
                tree_ids,
                last_message: 0,
            }),
            _db: RwLock::new(db),
        }
    }

    /// Swap the temporary database with a production one and reload the trees.
    /// Called during `Feed::init()` after `QaulState` is available.
    pub fn init_production(&self, db: sled::Db, tree: sled::Tree, tree_ids: sled::Tree, last_message: u64) {
        {
            let mut feed = self.inner.write().unwrap();
            feed.tree = tree;
            feed.tree_ids = tree_ids;
            feed.last_message = last_message;
        }
        {
            let mut db_lock = self._db.write().unwrap();
            *db_lock = db;
        }
    }

    /// Save a message to the in-memory BTreeMap and the sled database.
    pub fn save_message(&self, signature: Vec<u8>, message: proto_net::FeedMessageContent) {
        let mut feed = self.inner.write().unwrap();
        let sender_id = message.sender.clone();
        let content = message.content.clone();
        let timestamp_sent = message.time;

        feed.messages.insert(signature.clone(), message);

        let last_message = feed.last_message + 1;
        let timestamp_received = timestamp::Timestamp::get_timestamp();

        let message_data = FeedMessageData {
            index: last_message,
            message_id: signature,
            sender_id,
            timestamp_sent,
            timestamp_received,
            content,
        };

        let message_data_bytes = bincode::serialize(&message_data).unwrap();
        if let Err(e) = feed.tree.insert(&last_message.to_be_bytes(), message_data_bytes) {
            log::error!("Error saving feed message to data base: {}", e);
        } else if let Err(e) = feed.tree.flush() {
            log::error!("Error when flushing data base to disk: {}", e);
        }

        let last_message_bytes = bincode::serialize(&last_message).unwrap();
        if let Err(e) = feed.tree_ids.insert(&message_data.message_id[..], last_message_bytes) {
            log::error!("Error saving feed id to data base: {}", e);
        } else if let Err(e) = feed.tree_ids.flush() {
            log::error!("Error when flushing data base to disk: {}", e);
        }

        feed.last_message = last_message;
    }

    /// Save a message received via sync. Returns early if it already exists.
    pub fn save_message_by_sync(&self, message_id: &[u8], sender_id: &[u8], content: String, time: u64) {
        let mut feed = self.inner.write().unwrap();
        if let Some(_index) = feed.tree_ids.get(&message_id[..]).unwrap() {
            return;
        }

        let msg_content = proto_net::FeedMessageContent {
            sender: sender_id.to_vec(),
            content: content.clone(),
            time,
        };
        feed.messages.insert(message_id.to_vec(), msg_content);

        let last_message = feed.last_message + 1;
        let timestamp_received = timestamp::Timestamp::get_timestamp();

        let message_data = FeedMessageData {
            index: last_message,
            message_id: message_id.to_vec(),
            sender_id: sender_id.to_vec(),
            timestamp_sent: time,
            timestamp_received,
            content,
        };

        let message_data_bytes = bincode::serialize(&message_data).unwrap();
        if let Err(e) = feed.tree.insert(&last_message.to_be_bytes(), message_data_bytes) {
            log::error!("Error saving feed message to data base: {}", e);
        } else if let Err(e) = feed.tree.flush() {
            log::error!("Error when flushing data base to disk: {}", e);
        }

        let last_message_bytes = bincode::serialize(&last_message).unwrap();
        if let Err(e) = feed.tree_ids.insert(&message_id[..], last_message_bytes) {
            log::error!("Error saving feed id to data base: {}", e);
        } else if let Err(e) = feed.tree_ids.flush() {
            log::error!("Error when flushing data base to disk: {}", e);
        }

        feed.last_message = last_message;
    }

    /// Get messages newer than `last_message` from the database.
    pub fn get_messages(&self, last_message: u64) -> proto::FeedMessageList {
        let feed = self.inner.read().unwrap();
        let mut feed_list = proto::FeedMessageList {
            feed_message: Vec::with_capacity(feed.last_message.saturating_sub(last_message) as usize),
            pagination: None,
        };

        if feed.last_message > last_message {
            let first_message = last_message + 1;
            let first_message_bytes = first_message.to_be_bytes().to_vec();
            for res in feed.tree.range(first_message_bytes.as_slice()..) {
                match res {
                    Ok((_id, message_bytes)) => {
                        let message: FeedMessageData = bincode::deserialize(&message_bytes).unwrap();
                        let sender_id_base58 = bs58::encode(&message.sender_id).into_string();
                        let time_sent = timestamp::Timestamp::create_time();
                        let time_rfc3339 = humantime::format_rfc3339(time_sent).to_string();

                        feed_list.feed_message.push(proto::FeedMessage {
                            sender_id: message.sender_id.clone(),
                            sender_id_base58,
                            message_id: message.message_id.clone(),
                            message_id_base58: bs58::encode(message.message_id).into_string(),
                            time_sent: time_rfc3339.clone(),
                            timestamp_sent: message.timestamp_sent,
                            time_received: time_rfc3339,
                            timestamp_received: message.timestamp_received,
                            content: message.content.clone(),
                            index: message.index,
                        });
                    }
                    Err(e) => {
                        log::error!("Error retrieving feed message from data base: {}", e);
                    }
                }
            }
        }

        feed_list
    }

    /// Get paginated messages.
    pub fn get_paginated_messages(&self, offset: u32, limit: u32) -> proto::FeedMessageList {
        let feed = self.inner.read().unwrap();
        build_feed_list_from(&feed.tree, offset, limit)
    }

    /// Get latest message IDs.
    pub fn get_latest_message_ids(&self, count: usize) -> Vec<Vec<u8>> {
        let feed = self.inner.read().unwrap();
        let mut msg_count: usize = count;
        if feed.last_message < (count as u64) {
            msg_count = feed.last_message as usize;
        }
        let mut ids = Vec::with_capacity(msg_count);

        let first_message = feed.last_message - (msg_count as u64);
        let first_message_bytes = first_message.to_be_bytes().to_vec();
        for res in feed.tree.range(first_message_bytes.as_slice()..) {
            match res {
                Ok((_id, message_bytes)) => {
                    let message: FeedMessageData = bincode::deserialize(&message_bytes).unwrap();
                    ids.push(message.message_id.clone());
                }
                Err(e) => {
                    log::error!("Error retrieving feed message from data base: {}", e);
                }
            }
        }
        ids
    }

    /// Return IDs from `ids` that are not present in the database.
    pub fn process_received_feed_ids(&self, ids: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let feed = self.inner.read().unwrap();
        let mut missing_ids = Vec::with_capacity(ids.len());
        for id in ids {
            if feed.tree_ids.get(&id[..]).unwrap().is_none() {
                missing_ids.push(id.clone());
            }
        }
        missing_ids
    }

    /// Retrieve full message data for each of the given IDs.
    pub fn get_messages_by_ids(&self, ids: &[Vec<u8>]) -> Vec<(Vec<u8>, Vec<u8>, String, u64)> {
        let feed = self.inner.read().unwrap();
        let mut res = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(index_bytes) = feed.tree_ids.get(&id[..]).unwrap() {
                let index: u64 = bincode::deserialize(&index_bytes).unwrap();
                if let Some(message_bytes) = feed.tree.get(index.to_be_bytes()).unwrap() {
                    let message: FeedMessageData = bincode::deserialize(&message_bytes).unwrap();
                    res.push((id.clone(), message.sender_id.clone(), message.content.clone(), message.timestamp_sent));
                }
            }
        }
        res
    }
}

impl Feed {
    /// initialize feed module
    pub fn init(state: &crate::QaulState) {
        // get database and initialize tree
        let db = DataBase::get_node_db(state);
        let tree: sled::Tree = match db.open_tree("feed") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open feed tree: {}", e);
                return;
            }
        };
        let tree_ids: sled::Tree = match db.open_tree("feed_id") {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Failed to open feed_id tree: {}", e);
                return;
            }
        };

        // get last key
        let last_message: u64;
        match tree.iter().last() {
            Some(Ok((ivec, _))) => {
                let i = ivec.to_vec();
                match i.try_into() {
                    Ok(arr) => {
                        last_message = u64::from_be_bytes(arr);
                    }
                    Err(e) => {
                        log::error!("couldn't convert ivec to u64: {:?}", e);
                        last_message = 0;
                    }
                }
            }
            None => {
                last_message = 0;
            }
            Some(Err(e)) => {
                log::error!("Sled feed table error: {}", e);
                last_message = 0;
            }
        }

        // swap temporary DB with production DB in existing state
        state.services.feed.init_production(db, tree, tree_ids, last_message);
    }

    /// Send message via all swarms
    pub fn send(
        state: &crate::QaulState,
        user_account: &UserAccount,
        content: String,
        lan: Option<&mut Lan>,
        internet: Option<&mut Internet>,
    ) {
        // create timestamp
        let timestamp = timestamp::Timestamp::get_timestamp();

        // create feed message
        let msg = proto_net::FeedMessageContent {
            sender: user_account.id.to_bytes(),
            content: content.clone(),
            time: timestamp,
        };

        // encode feed message
        let mut buf = Vec::with_capacity(msg.encoded_len());
        msg.encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // sign message
        let signature = Self::sign_message(&buf, &user_account.keys);

        // create signed container
        let container = proto_net::FeedContainer {
            signature,
            message: buf,
        };

        // encode container
        let mut buf = Vec::with_capacity(container.encoded_len());
        container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // save message in feed store
        state.services.feed.save_message(container.signature.clone(), msg);

        // flood via floodsub
        if let Some(lan) = lan {
            lan.swarm
                .behaviour_mut()
                .floodsub
                .publish(Node::get_topic(state), buf.clone());
        }
        if let Some(internet) = internet {
            internet
                .swarm
                .behaviour_mut()
                .floodsub
                .publish(Node::get_topic(state), buf.clone());
        }
        crate::connections::ble::Ble::send_feed_message(state, Node::get_topic(state), buf);
    }

    /// Process a received message
    pub fn received(
        state: &crate::QaulState,
        via_conn: ConnectionModule,
        _via_node: PeerId,
        feed_container: proto_net::FeedContainer,
    ) {
        match proto_net::FeedMessageContent::decode(&feed_container.message[..]) {
            Ok(feed_content) => {
                let message = feed_content.clone();

                if let Ok(user_id_decoded) = PeerId::from_bytes(&message.sender) {
                    // check if sending user public is in user store
                    let rs = state.get_router();
                    let result = router::users::Users::get_pub_key(&rs, &user_id_decoded);

                    if let Some(key) = result {
                        // validate message
                        if !Self::validate_message(&feed_container, &key) {
                            log::error!(
                                "Validation of feed message {:?} failed: {:?}",
                                feed_container.signature,
                                message.content
                            );
                            log::error!("  sender id:  {}", user_id_decoded);
                            let (key_type, key_base58) =
                                crate::router::users::Users::get_protobuf_public_key(&key);
                            log::error!("  sender key [{}]: {}", key_type, key_base58);
                            return;
                        }

                        // check if message exists is in feed store
                        let mut new_message = true;

                        {
                            let feed = state.services.feed.inner.read().unwrap();

                            if feed.messages.contains_key(&feed_container.signature) {
                                new_message = false;
                            }
                        }

                        // check if message exists
                        if new_message {
                            // write message to store
                            state.services.feed.save_message(feed_container.signature.clone(), feed_content);

                            // display message
                            log::trace!("message received:");
                            log::trace!(
                                "Timestamp - {}, Signature- {:?}",
                                message.time,
                                feed_container.signature
                            );
                            log::trace!(" Message Content {}", message.content);

                            // encode container
                            let mut buf = Vec::with_capacity(feed_container.encoded_len());
                            feed_container
                                .encode(&mut buf)
                                .expect("Vec<u8> provides capacity as needed");

                            // forward message
                            Flooder::add(&rs, buf, Node::get_topic(state), via_conn);
                        } else {
                            log::trace!(
                                "message key {:?} already in store",
                                feed_container.signature
                            );
                        }
                    } else {
                        log::error!("Sender of feed message not known: {}", user_id_decoded);
                        return;
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// Sign a message with the private key
    /// The signature can be validated with the corresponding public key.
    pub fn sign_message(buf: &[u8], keys: &Keypair) -> Vec<u8> {
        match keys.sign(buf) {
            Ok(sig) => sig,
            Err(e) => {
                log::error!("Failed to sign feed message: {}", e);
                Vec::new()
            }
        }
    }

    /// validate a message via the public key of the sender
    pub fn validate_message(msg: &proto_net::FeedContainer, key: &PublicKey) -> bool {
        key.verify(&msg.message, &msg.signature)
    }

    /// Process incoming RPC request messages for feed module
    pub fn rpc(
        state: &crate::QaulState,
        data: Vec<u8>,
        user_id: Vec<u8>,
        lan: Option<&mut Lan>,
        internet: Option<&mut Internet>,
        request_id: String,
    ) {
        match proto::Feed::decode(&data[..]) {
            Ok(feed) => {
                match feed.message {
                    Some(proto::feed::Message::Request(feed_request)) => {
                        // get feed messages from data base
                        // Pagination is optional: when limit is set to 0, we fallback to the previous index-based impl
                        let feed_list = if feed_request.limit > 0 {
                            state.services.feed.get_paginated_messages(feed_request.offset, feed_request.limit)
                        } else {
                            state.services.feed.get_messages(feed_request.last_index)
                        };

                        // pack message
                        let proto_message = proto::Feed {
                            message: Some(proto::feed::Message::Received(feed_list)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            state,
                            buf,
                            crate::rpc::proto::Modules::Feed.into(),
                            request_id,
                            Vec::new(),
                        );
                    }
                    Some(proto::feed::Message::Send(send_feed)) => {
                        // print message
                        log::trace!("feed message received: {}", send_feed.content);

                        // get user account from user_id
                        let user_account;
                        match PeerId::from_bytes(&user_id) {
                            Ok(user_id_decoded) => {
                                match UserAccounts::get_by_id(state,user_id_decoded) {
                                    Some(account) => {
                                        user_account = account;
                                        // send the message
                                        Self::send(state, &user_account, send_feed.content, lan, internet);
                                    }
                                    None => {
                                        log::error!(
                                            "user account id not found: {:?}",
                                            user_id_decoded.to_base58()
                                        );
                                        return;
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("user account id could'nt be encoded: {:?}", e);
                            }
                        }
                    }
                    _ => {
                        log::error!("Unhandled Protobuf Feed Message");
                    }
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }
}

/// Build a paginated feed message list from the sled database tree.
///
/// This is a free function (outside `impl Feed`) so it can be unit-tested
/// without initialising the Feed module.
fn build_feed_list_from(tree: &sled::Tree, offset: u32, limit: u32) -> proto::FeedMessageList {
    let total_messages = tree.len() as u32;
    let remaining_messages = total_messages.saturating_sub(offset) as usize;

    // vector cap is determined by either the number of remaining messages, or the limit - whichever is smaller.
    // if no limit is provided, will iterate over all messages
    let page_capacity = if limit == 0 {
        remaining_messages
    } else {
        remaining_messages.min(limit as usize)
    };

    let mut feed_list = proto::FeedMessageList {
        feed_message: Vec::with_capacity(page_capacity),
        pagination: None,
    };

    // We iterate in reverse (newest-first) so that offset=0 returns the most
    // recent messages — the natural order for a feed UI.
    // We build the iter by reversing all messages in the tree, then skipping
    // past the offset and taking up until:
    //   - the limit, if provided, or;
    //   - the tree length
    let take = if limit > 0 {
        limit as usize
    } else {
        usize::MAX
    };
    let iter = tree.iter().rev().skip(offset as usize).take(take);

    for res in iter {
        match res {
            Ok((_key, message_bytes)) => {
                let message: FeedMessageData = match bincode::deserialize(&message_bytes) {
                    Ok(m) => m,
                    Err(e) => {
                        log::error!("Failed to deserialize feed message: {}", e);
                        continue;
                    }
                };

                let sender_id_base58 = bs58::encode(&message.sender_id).into_string();

                let time_sent = timestamp::Timestamp::create_time();
                let time_rfc3339 = humantime::format_rfc3339(time_sent).to_string();

                let feed_message = proto::FeedMessage {
                    sender_id: message.sender_id.clone(),
                    sender_id_base58,
                    message_id: message.message_id.clone(),
                    message_id_base58: bs58::encode(&message.message_id).into_string(),
                    time_sent: time_rfc3339.clone(),
                    timestamp_sent: message.timestamp_sent,
                    time_received: time_rfc3339,
                    timestamp_received: message.timestamp_received,
                    content: message.content.clone(),
                    index: message.index,
                };

                feed_list.feed_message.push(feed_message);
            }
            Err(e) => {
                log::error!("Error reading feed message from database: {}", e);
            }
        }
    }

    let has_more = limit > 0 && offset.saturating_add(limit) < total_messages;

    feed_list.pagination = Some(proto::PaginationMetadata {
        has_more,
        total: total_messages,
        offset,
        limit,
    });

    feed_list
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a temporary sled tree with `n` FeedMessageData entries.
    fn make_feed_tree(n: u64) -> (sled::Db, sled::Tree) {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("test_feed").unwrap();
        for i in 1..=n {
            let key = i.to_be_bytes();
            let data = FeedMessageData {
                index: i,
                message_id: vec![i as u8],
                sender_id: vec![0xAA, 0xBB],
                timestamp_sent: 1000 + i,
                timestamp_received: 2000 + i,
                content: format!("message {}", i),
            };
            tree.insert(&key, bincode::serialize(&data).unwrap())
                .unwrap();
        }
        (db, tree)
    }

    #[test]
    fn empty_messages() {
        let (_db, tree) = make_feed_tree(0);
        let list = build_feed_list_from(&tree, 0, 0);

        assert_eq!(list.feed_message.len(), 0);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 0);
    }

    #[test]
    fn pagination_echoes_offset_and_limit() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 3, 7);

        let p = list.pagination.unwrap();
        assert_eq!(p.offset, 3);
        assert_eq!(p.limit, 7);
    }

    #[test]
    fn first_page_returns_newest_messages() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 0, 2);

        assert_eq!(list.feed_message.len(), 2);
        // offset=0 should return the two most recent messages (index 5, 4)
        assert_eq!(list.feed_message[0].index, 5);
        assert_eq!(list.feed_message[1].index, 4);
        let p = list.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn middle_page() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 2, 2);

        assert_eq!(list.feed_message.len(), 2);
        // skipping 2 newest (5, 4), should return (3, 2)
        assert_eq!(list.feed_message[0].index, 3);
        assert_eq!(list.feed_message[1].index, 2);
        let p = list.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn last_page_partial() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 4, 2);

        assert_eq!(list.feed_message.len(), 1);
        // skipping 4 newest (5, 4, 3, 2), only index 1 remains
        assert_eq!(list.feed_message[0].index, 1);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn offset_beyond_total_returns_no_messages() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 10, 2);

        assert_eq!(list.feed_message.len(), 0);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }

    #[test]
    fn limit_larger_than_total_returns_all_messages() {
        let (_db, tree) = make_feed_tree(5);
        let list = build_feed_list_from(&tree, 0, 100);

        assert_eq!(list.feed_message.len(), 5);
        let p = list.pagination.unwrap();
        assert!(!p.has_more);
        assert_eq!(p.total, 5);
    }
}
