/**
 * User Module
 * 
 * In qaul.net each user is defined by the following things
 * 
 * * user ID (hash of the public key)
 * * Public / private key
 * * user name (optional)
 */

use libp2p::{
    PeerId,
    identity::{Keypair, ed25519},
};
use log::{error, info};
use crate::configuration;
use crate::configuration::Configuration;
use crate::router;
use state::Storage;
use std::sync::RwLock;

// mutable state of users table
static USERS: Storage<RwLock<Users>> = Storage::new();

pub struct User {
    pub id: PeerId,
    pub keys: Keypair,
    pub name: String,
}

pub struct Users {
    pub users: Vec<User>,
}

impl Users {
    pub fn init() {
        let mut users = Users { users: Vec::new() };

        // check if there are users defined in configuration
        let config = Configuration::get();
        let config_users = config.users.clone();
        let mut iter = IntoIterator::into_iter(config_users);

        while let Some(user) = iter.next() {
            let mut basedecode = base64::decode(&user.keys).unwrap();
            let keys = Keypair::Ed25519(ed25519::Keypair::decode( &mut basedecode).unwrap());
            let id = PeerId::from(keys.public());

            // check if saved ID and the id from the keypair are equal
            if id.to_string() == user.id {
                info!("user id's of '{}' match {}", user.name, user.id);
            } 
            else {
                error!("------------------------------------");
                error!("ERROR: user id's of '{}' are not equal", user.name);
                error!("{}  {}", id.to_string(), user.id);
                error!("------------------------------------");
            }

            // push to users table
            users.users.push(
                User {
                    name: user.name.clone(),
                    id,
                    keys: keys.clone(),
                }
            );
        }

        // save users to state
        USERS.set(RwLock::new(users));
    }

    pub fn create( name: String ) {
        // create user
        let keys_ed25519 = ed25519::Keypair::generate();
        let keys_config = base64::encode(keys_ed25519.encode());
        let keys = Keypair::Ed25519(keys_ed25519);
        let id = PeerId::from(keys.public());
        let user = User {id, keys, name: name.clone()};

        // save it to state
        let mut users = USERS.get().write().unwrap();
        users.users.push(user);

        // save it to config
        {
            let mut config = Configuration::get_mut();
            config.users.push( configuration::User {
                name: name.clone(),
                id: id.to_string(),
                keys: keys_config,
            });
        }
        Configuration::save();

        // display id
        info!("created User '{}' {:?}", name, id);
    }

    /**
     * Return the number of registered user accounts on this node.
     */
    pub fn len() -> usize {
        let users = USERS.get().read().unwrap();
        users.users.len()
    }

    /**
     * Return the default user.
     * The first registered user account is returned.
     */
    pub fn get_default_user() -> User {
        let users = USERS.get().read().unwrap();
        let user = users.users.first().unwrap();
        User {
            id: user.id.clone(), 
            keys: user.keys.clone(),
            name: user.name.clone(),
        }
    }

    /// to fill the routing table get all users
    pub fn get_user_info() -> Vec<router::users::User> {
        let mut user_info = Vec::new();

        let users = USERS.get().read().unwrap();
        for user in &users.users {
            user_info.push(router::users::User {
                id: user.id,
                key: user.keys.public(),
                name: user.name.clone(),
            });
        }

        user_info
    }

    /// Process command line instructions for the 
    /// user accounts
    pub fn cli(cmd: &str) {        
        match cmd {
            // list all user accounts
            "list" => {
                println!("User Accounts:");
                let users = USERS.get().read().unwrap();
                for user in &users.users {
                    println!("'{}' {:?}", user.name, user.id);
                }
            },
            // create a new user account
            cmd if cmd.starts_with("create ") => {
                Self::create( String::from(cmd.strip_prefix("create ").unwrap()) );
            },
            _ => error!("unknown user command"),
        }
    }
}

