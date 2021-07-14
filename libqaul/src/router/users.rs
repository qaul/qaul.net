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
//use log::{error, info};

// mutable state of users table
static USERS: Storage<RwLock<Users>> = Storage::new();

pub struct Users {
    pub users: BTreeMap<PeerId, User>,
}

impl Users {
    pub fn init() {
        // create users table and save it to state
        let users = Users { users: BTreeMap::new() };
        USERS.set(RwLock::new(users));

        // TODO: fill BTreeMap from storage
    }

    pub fn add( id: PeerId, key: PublicKey, name: String ) {
        let mut users = USERS.get().write().unwrap();
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
}

pub struct User {
    pub id: PeerId,
    pub key: PublicKey,
    pub name: String,
}

impl User {

}
