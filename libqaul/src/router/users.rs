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
    /// Initialize the router::users::Users module
    /// this module is automatically initialized
    /// when the router module is initionalized
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
