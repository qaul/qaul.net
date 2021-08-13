//! Discovered user table
//! 
//! This table contains all users known to this node.

use libp2p::{
    PeerId,
    identity::PublicKey,
};
use prost::Message;
use serde::{Serialize, Deserialize};
use state::Storage;
use std::collections::BTreeMap;
use std::sync::RwLock;

use crate::node;
use super::proto;
use crate::rpc::Rpc;

/// mutable state of users table
static USERS: Storage<RwLock<Users>> = Storage::new();

/// implementation of all known users for routing references
pub struct Users {
    pub users: BTreeMap<PeerId, User>,
}

impl Users {
    /// Initialize the router::users::Users module
    /// this module is automatically initialized
    /// when the router module is initialized
    pub fn init() {
        {
            // create users table and save it to state
            let users = Users { users: BTreeMap::new() };
            USERS.set(RwLock::new(users));
        }

        // fill with locally registered users
        for user in node::user_accounts::UserAccounts::get_user_info() {
            Self::add(user.id, user.key, user.name.clone());
        }
    }

    /// add a new user to the users list
    pub fn add( id: PeerId, key: PublicKey, name: String ) {
        let mut users = USERS.get().write().unwrap();
        users.users.insert( id, User { id, key, name } );
    }

    /// add a new user to the users list, and check whether the 
    /// User ID matches the public key
    pub fn add_with_check( id: PeerId, key: PublicKey, name: String ) {
        // check if user is valid
        if id != key.clone().into_peer_id() { 
            return 
        }

        let mut users = USERS.get().write().unwrap();

        // check if user already exists
        if users.users.contains_key(&id) {
            return
        }
        users.users.insert( id, User { id, key, name } );
    }

    /// get the public key of a known user
    pub fn get_pub_key( user_id: &PeerId ) -> Option<PublicKey> {
        let store = USERS.get().read().unwrap();
        let result = store.users.get(user_id);
        match result {
            Some(user) => Some(user.key.clone()),
            None => None
        }
    }

    /// create and send the user info table for the
    /// RouterInfo message which is sent regularily to neighbours
    pub fn get_user_info_table() -> UserInfoTable {
        let store = USERS.get().read().unwrap();
        let mut users = Vec::new();
        for (_id, value) in &store.users {
            users.push(UserInfo {
                id: value.id.to_bytes(),
                key: value.key.clone().into_protobuf_encoding(),
                name: value.name.clone(),
            });
        }
        UserInfoTable(users)
    }

    /// add new users from the received bytes of a UserInfoTable
    pub fn add_user_info_table(users: UserInfoTable) {
        // loop through it and add it to the users list
        for value in users.0.iter() {
            let id_result = PeerId::from_bytes(&value.id);
            let key_result = PublicKey::from_protobuf_encoding(&value.key);

            if let (Ok(id), Ok(key)) = (id_result, key_result) {
                Self::add_with_check(
                    id,
                    key,
                    value.name.clone()
                );
            }
        }
    }

    /// users CLI commands
    /// 
    /// you invoke this with the commands:
    /// ```
    /// router users list
    /// ```
    pub fn cli(cmd: &str) {        
        match cmd {
            // list all users
            cmd if cmd.starts_with("list") => {
                let mut line = 1;
                println!("All known Users");
                println!("No. | User Name | User Id | Public Key");

                let users = USERS.get().read().unwrap();

                for (id, user) in &users.users {
                    println!("{} | {} | {:?} | {:?}", line, user.name, id, user.key);
                    line += 1;
                }
            },
            _ => log::error!("unknown router users command"),
        }
    }

    /// Process RPC request and send reply
    pub fn rpc(router_message: proto::Router) {
        match router_message.message {
            Some(proto::router::Message::UserRequest(_user_request)) => {
                // get users store
                let users = USERS.get().read().unwrap();

                // fill them into the list
                let mut user_list = proto::UserList {
                    user: Vec::new(),
                };
                for (id, user) in &users.users {
                    // get RPC key values
                    let (key_type, key_base58) = Self::get_protobuf_public_key(user.key.clone());

                    // create user entry message
                    let user_entry = proto::UserEntry {
                        name: user.name.clone(),
                        id: id.to_bytes(),
                        id_base58: id.to_base58(),
                        key: user.key.clone().into_protobuf_encoding(),
                        key_type,
                        key_base58,
                        connectivity: 0,
                    };

                    // add entry to list
                    user_list.user.push(user_entry);
                }

                // create message
                let proto_message = proto::Router{
                    message: Some(proto::router::Message::UserList(
                        user_list)),
                };

                // encode message
                let mut buf = Vec::with_capacity(proto_message.encoded_len());
                proto_message.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

                // send message
                Rpc::send_message(buf, crate::rpc::proto::Modules::Router.into(), "".to_string(), Vec::new() );
            },
            _ => {},
        }
    }

    /// create the qaul RPC definitions of a public key
    /// 
    /// Returns a tuple with the key type & the base58 encoded
    /// (key_type: String, key_base58: String)
    fn get_protobuf_public_key(key: PublicKey) -> (String, String) {
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


pub struct User {
    pub id: PeerId,
    pub key: PublicKey,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserInfo {
    pub id: Vec<u8>,
    pub key: Vec<u8>,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserInfoTable(pub Vec<UserInfo>);
