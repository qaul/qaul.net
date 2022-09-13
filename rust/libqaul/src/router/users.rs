// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Discovered user table
//!
//! This table contains all users known to this node.

use libp2p::{identity::PublicKey, PeerId};
use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::digest::DynDigest;
use sha2::{Digest, Sha512};
use state::Storage;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::RwLock;

use super::router_net_proto;
use super::table::RoutingTable;
use crate::node::user_accounts::UserAccounts;
use crate::rpc::Rpc;
use crate::services::group::group_id::GroupId;
use crate::storage::database::DbUsers;
use crate::utilities::qaul_id::QaulId;

/// Import protobuf users RPC message definition generated by
/// the rust module prost-build.
pub mod proto {
    include!("qaul.rpc.users.rs");
}

/// Import protobuf router_net_info definitions
pub mod proto_net {
    include!("qaul.net.router_net_info.rs");
}

/// mutable state of users table
static USERS: Storage<RwLock<Users>> = Storage::new();

/// implementation of all known users for routing references
pub struct Users {
    /// the BTreeMap key is the 8 byte qaul ID (q8id)
    pub users: BTreeMap<Vec<u8>, User>,
}

impl Users {
    /// Initialize the router::users::Users module
    /// this module is automatically initialized
    /// when the router module is initialized
    pub fn init() {
        {
            // create users table and save it to state
            let users = Users {
                users: BTreeMap::new(),
            };
            USERS.set(RwLock::new(users));
        }

        // fill user table with users from data base
        let tree = DbUsers::get_tree();
        let mut users = USERS.get().write().unwrap();
        // iterate over all values in db
        for res in tree.iter() {
            if let Ok((_vec, user)) = res {
                // encode values from bytes
                let q8id = QaulId::bytes_to_q8id(user.id.clone());
                let id = PeerId::from_bytes(&user.id).unwrap();
                let key = PublicKey::from_protobuf_encoding(&user.key).unwrap();
                // fill result into user table
                users.users.insert(
                    q8id,
                    User {
                        id,
                        key,
                        name: user.name,
                        verified: user.verified,
                        blocked: user.blocked,
                    },
                );
            }
        }
    }

    /// add a new user
    ///
    /// This user will be added to the users list in memory and to the data base
    pub fn add(id: PeerId, key: PublicKey, name: String, verified: bool, blocked: bool) {
        // save user to the data base
        DbUsers::add_user(UserData {
            id: id.to_bytes(),
            key: key.clone().to_protobuf_encoding(),
            name: name.clone(),
            verified,
            blocked,
        });

        // add user to the users table
        let q8id = QaulId::to_q8id(id.clone());
        let mut users = USERS.get().write().unwrap();
        users.users.insert(
            q8id,
            User {
                id,
                key,
                name,
                verified,
                blocked,
            },
        );
    }

    /// add a new user to the users list, and check whether the
    /// User ID matches the public key
    /// and save it to the data base
    pub fn add_with_check(id: PeerId, key: PublicKey, name: String) {
        // check if user is valid
        if id != key.clone().to_peer_id() {
            log::error!("user id & key do not match {}", id.to_base58());
            return;
        }

        // check if user already exists
        {
            let q8id = QaulId::to_q8id(id.clone());
            let users = USERS.get().read().unwrap();

            // check if user already exists
            if users.users.contains_key(&q8id) {
                return;
            }
        }
        // add user
        Self::add(id, key, name, false, false);
    }

    /// check missed users from ids
    pub fn get_missed_ids(ids: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut res: Vec<Vec<u8>> = vec![];
        let users = USERS.get().read().unwrap();
        for id in ids {
            if !users.users.contains_key(id) {
                res.push(id.clone());
            }
        }
        return res;
    }

    /// get the public key of a known user
    pub fn get_pub_key(user_id: &PeerId) -> Option<PublicKey> {
        // get q8id
        let q8id = QaulId::to_q8id(user_id.to_owned());

        // get public key
        Self::get_pub_key_by_q8id(&q8id)
    }

    /// get the public key of a known user by it's q8id
    pub fn get_pub_key_by_q8id(q8id: &Vec<u8>) -> Option<PublicKey> {
        let store = USERS.get().read().unwrap();
        let result = store.users.get(q8id);
        match result {
            Some(user) => Some(user.key.clone()),
            None => None,
        }
    }

    /// get user by q8id
    pub fn get_user_id_by_q8id(q8id: Vec<u8>) -> Option<PeerId> {
        let store = USERS.get().read().unwrap();

        if let Some(user) = store.users.get(&q8id) {
            return Some(user.id);
        }

        None
    }

    /// create and send the user info table for the
    /// RouterInfo message which is sent regularly to neighbours
    ///
    /// This is a wrapper function for the PeerIds for the function
    /// `get_user_info_table_by_q8ids(q8ids)`
    pub fn _get_user_info_table_by_ids(ids: &Vec<PeerId>) -> router_net_proto::UserInfoTable {
        let mut q8ids: Vec<Vec<u8>> = Vec::new();
        for id in ids {
            // convert qaul ID to q8id
            let q8id = QaulId::to_q8id(id.to_owned());
            q8ids.push(q8id);
        }

        Self::get_user_info_table_by_q8ids(&q8ids)
    }

    /// create and send the user info table for the
    /// RouterInfo message which is sent regularly to neighbours
    pub fn get_user_info_table_by_q8ids(q8ids: &Vec<Vec<u8>>) -> router_net_proto::UserInfoTable {
        let store = USERS.get().read().unwrap();
        let mut users = router_net_proto::UserInfoTable { info: Vec::new() };

        for q8id in q8ids {
            if let Some(value) = store.users.get(q8id) {
                let user_info = router_net_proto::UserInfo {
                    id: value.id.to_bytes(),
                    key: value.key.clone().to_protobuf_encoding(),
                    name: value.name.clone(),
                };
                users.info.push(user_info);
            }
        }
        users
    }

    /// add new users from the received bytes of a UserInfoTable
    pub fn add_user_info_table(users: &Vec<router_net_proto::UserInfo>) {
        // loop through it and add it to the users list
        for value in users {
            let id_result = PeerId::from_bytes(&value.id);
            let key_result = PublicKey::from_protobuf_encoding(&value.key);

            if let (Ok(id), Ok(key)) = (id_result, key_result) {
                Self::add_with_check(id, key, value.name.clone());
            }
        }
    }

    fn compare(a: &[u8], b: &[u8]) -> Ordering {
        for (ai, bi) in a.iter().zip(b.iter()) {
            match ai.cmp(&bi) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }

        /* if every single element was equal, compare length */
        a.len().cmp(&b.len())
    }

    /// get security number
    fn get_security_number(my_user: &PeerId, user_id: &Vec<u8>) -> Result<Vec<u8>, String> {
        let q8id = QaulId::bytes_to_q8id(user_id.clone());
        let q8id_my = QaulId::to_q8id(my_user.clone());

        // find user from users
        let users = USERS.get().read().unwrap();
        if !users.users.contains_key(&q8id) {
            return Err("user no exists".to_string());
        }

        if !users.users.contains_key(&q8id_my) {
            return Err("my user is not existed".to_string());
        }
        let mut key1 = users
            .users
            .get(&q8id_my)
            .unwrap()
            .key
            .to_protobuf_encoding();
        let mut key2 = users.users.get(&q8id).unwrap().key.to_protobuf_encoding();

        // merge two keys
        let mut data: Vec<u8> = vec![];
        match Self::compare(&key1, &key2) {
            Ordering::Less => {
                data.append(&mut key1);
                data.append(&mut key2);
            }
            _ => {
                data.append(&mut key2);
                data.append(&mut key1);
            }
        }

        let mut key_data = data.clone();
        data.clear();

        for _ in 0..5200 {
            data.append(&mut key_data);
            let hash = Sha512::digest(&data);
            //log::error!("len: {}", hash.len());
            let mut hash_vec = hash[..64].to_vec();
            data.clear();
            data.append(&mut hash_vec);
        }
        Ok(data[..16].to_vec())
    }

    /// Process incoming RPC request messages
    pub fn rpc(data: Vec<u8>, user_id: Vec<u8>) {
        let account_id = PeerId::from_bytes(&user_id).unwrap();

        match proto::Users::decode(&data[..]) {
            Ok(users) => {
                match users.message {
                    Some(proto::users::Message::UserRequest(_user_request)) => {
                        // get users store
                        let users = USERS.get().read().unwrap();

                        // create empty user list
                        let mut user_list = proto::UserList { user: Vec::new() };

                        // get user account
                        if let Some(account) = UserAccounts::get_default_user() {
                            // fill them into the list
                            for (_id, user) in &users.users {
                                // get RPC key values
                                let (_key_type, key_base58) =
                                    Self::get_protobuf_public_key(user.key.clone());

                                // create group id
                                let group_id =
                                    GroupId::from_peers(&account.id, &user.id).to_bytes();

                                // create user entry message
                                let user_entry = proto::UserEntry {
                                    name: user.name.clone(),
                                    id: user.id.to_bytes(),
                                    group_id,
                                    key_base58,
                                    connectivity: 0,
                                    verified: user.verified,
                                    blocked: user.blocked,
                                };

                                // add entry to list
                                user_list.user.push(user_entry);
                            }
                        }

                        // create message
                        let proto_message = proto::Users {
                            message: Some(proto::users::Message::UserList(user_list)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Users.into(),
                            "".to_string(),
                            Vec::new(),
                        );
                    }
                    Some(proto::users::Message::UserOnlineRequest(_user_online_request)) => {
                        // get users store
                        let users = USERS.get().read().unwrap();

                        // get all online user ids by passing last_sent=0
                        let online_user_ids = RoutingTable::get_online_user_ids(0);

                        // create empty user list
                        let mut user_list = proto::UserList { user: Vec::new() };

                        // get user account
                        if let Some(account) = UserAccounts::get_default_user() {
                            // fill them into the list
                            for id in &online_user_ids {
                                if let Some(user) = users.users.get(id) {
                                    // get RPC key values
                                    let (_key_type, key_base58) =
                                        Self::get_protobuf_public_key(user.key.clone());

                                    // create group id
                                    let group_id =
                                        GroupId::from_peers(&account.id, &user.id).to_bytes();

                                    // create user entry message
                                    let user_entry = proto::UserEntry {
                                        name: user.name.clone(),
                                        id: user.id.to_bytes(),
                                        group_id,
                                        key_base58,
                                        connectivity: 0,
                                        verified: user.verified,
                                        blocked: user.blocked,
                                    };

                                    // add entry to list
                                    user_list.user.push(user_entry);
                                }
                            }
                        }

                        // create message
                        let proto_message = proto::Users {
                            message: Some(proto::users::Message::UserList(user_list)),
                        };

                        // encode message
                        let mut buf = Vec::with_capacity(proto_message.encoded_len());
                        proto_message
                            .encode(&mut buf)
                            .expect("Vec<u8> provides capacity as needed");

                        // send message
                        Rpc::send_message(
                            buf,
                            crate::rpc::proto::Modules::Users.into(),
                            "".to_string(),
                            Vec::new(),
                        );
                    }
                    Some(proto::users::Message::UserUpdate(updated_user)) => {
                        log::info!("UserUpdate protobuf RPC message");

                        // create user id from bytes
                        if let Ok(user_id) = PeerId::from_bytes(&updated_user.id) {
                            // get users store
                            let mut users = USERS.get().write().unwrap();

                            let q8id = QaulId::to_q8id(user_id);

                            // search for user in list and update entry
                            match users.users.get_mut(&q8id) {
                                Some(user_result) => {
                                    let user = User {
                                        id: user_id,
                                        key: user_result.key.clone(),
                                        name: user_result.name.clone(),
                                        verified: updated_user.verified,
                                        blocked: updated_user.blocked,
                                    };

                                    // update list
                                    *user_result = user;

                                    // save to data base
                                    DbUsers::add_user(UserData {
                                        id: user_id.to_bytes(),
                                        key: user_result.key.clone().to_protobuf_encoding(),
                                        name: user_result.name.clone(),
                                        verified: updated_user.verified,
                                        blocked: updated_user.blocked,
                                    });
                                }
                                None => {
                                    log::error!("updated user is unknown: {}", user_id.to_base58())
                                }
                            }
                        } else {
                            log::error!("PeerId couldn't be created");
                        }
                    }
                    Some(proto::users::Message::SecurityNumberRequest(secure_req)) => {
                        match Self::get_security_number(&account_id, &secure_req.user_id) {
                            Ok(x) => {
                                let mut security_number_blocks: Vec<u32> = vec![];
                                for i in 0..x.len() / 2 {
                                    let number = x[i * 2] as u32 + (x[i * 2 + 1] as u32 * 256);
                                    security_number_blocks.push(number);
                                }

                                // create message
                                let proto_message = proto::Users {
                                    message: Some(proto::users::Message::SecurityNumberResponse(
                                        proto::SecurityNumberResponse {
                                            user_id: secure_req.user_id.clone(),
                                            security_hash: x.clone(),
                                            security_number_blocks,
                                        },
                                    )),
                                };

                                // encode message
                                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                                proto_message
                                    .encode(&mut buf)
                                    .expect("Vec<u8> provides capacity as needed");

                                // send message
                                Rpc::send_message(
                                    buf,
                                    crate::rpc::proto::Modules::Users.into(),
                                    "".to_string(),
                                    Vec::new(),
                                );
                            }
                            Err(error) => {
                                log::error!("get secure unmber error {}", error);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {
                log::error!("{:?}", error);
            }
        }
    }

    /// create the qaul RPC definitions of a public key
    ///
    /// Returns a tuple with the key type & the base58 encoded
    /// (key_type: String, key_base58: String)
    pub fn get_protobuf_public_key(key: PublicKey) -> (String, String) {
        // extract values
        let key_type: String;
        let key_base58: String;

        match key {
            PublicKey::Ed25519(key) => {
                key_type = "Ed25519".to_owned();
                key_base58 = bs58::encode(key.encode()).into_string();
            }
            _ => {
                key_type = "UNDEFINED".to_owned();
                key_base58 = "UNDEFINED".to_owned();
            }
        }

        (key_type, key_base58)
    }
}

/// user structure
pub struct User {
    pub id: PeerId,
    pub key: PublicKey,
    pub name: String,
    pub verified: bool,
    pub blocked: bool,
}

/// user structure for storing it in the data base
#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: Vec<u8>,
    pub key: Vec<u8>,
    pub name: String,
    pub verified: bool,
    pub blocked: bool,
}
