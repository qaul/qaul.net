/**
 * Discovered user table
 * 
 * This table contains all users known to this node.
 */

use libp2p::{
    PeerId,
    identity::PublicKey,
};
use std::collections::BTreeMap;
use state::Storage;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::node;

// mutable state of users table
static USERS: Storage<RwLock<Users>> = Storage::new();

pub struct Users {
    pub users: BTreeMap<PeerId, User>,
}

impl Users {
    pub fn init() {
        {
            // create users table and save it to state
            let users = Users { users: BTreeMap::new() };
            USERS.set(RwLock::new(users));
        }

        // fill with locally registered users
        for user in node::users::Users::get_user_info() {
            Self::add(user.id, user.key, user.name.clone());
        }
    }

    pub fn add( id: PeerId, key: PublicKey, name: String ) {
        let mut users = USERS.get().write().unwrap();
        users.users.insert( id, User { id, key, name } );
    }

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

    pub fn get_pub_key( user_id: &PeerId ) -> Option<PublicKey> {
        let store = USERS.get().read().unwrap();
        let result = store.users.get(user_id);
        match result {
            Some(user) => Some(user.key.clone()),
            None => None
        }
    }

    pub fn get_as_bytes() -> Vec<u8> {
        let store = USERS.get().read().unwrap();
        let mut users = Vec::new();
        for (_id, value) in &store.users {
            users.push(UserInfo {
                id: value.id.to_bytes(),
                key: value.key.clone().into_protobuf_encoding(),
                name: value.name.clone(),
            });
        }
        let encoded: Vec<u8> = bincode::serialize(&users).unwrap();
        encoded
    }

    pub fn add_from_bytes(data: Vec<u8>) {
        let decoded: UserInfoTable = bincode::deserialize(&data[..]).unwrap();

        // loop through it and add it to the users list
        for value in decoded.0.iter() {
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
