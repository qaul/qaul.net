// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Database
//!
//! Embedded sled database.

use libp2p::PeerId;
use sled;
use state::InitCell;
use std::{collections::BTreeMap, path::Path, sync::RwLock};

use crate::router::users::UserData;

/// make database globally accessible
static DATABASE: InitCell<RwLock<DataBase>> = InitCell::new();

/// DataBase Module
#[derive(Clone, Debug)]
pub struct DataBase {
    /// storage path
    pub path: String,
    /// node data base
    pub node: sled::Db,
    /// user data bases
    /// each user account has an own data base
    /// that is opened on request
    pub users: BTreeMap<Vec<u8>, sled::Db>,
}

impl DataBase {
    /// Initialize data base,
    /// open data base from disk and set it to global state.
    pub fn init() {
        // create node data base path
        let path_string = super::Storage::get_path();
        let path = Path::new(path_string.as_str());
        let db_path = path.join("node.db");

        // open node data base
        let node = sled::Config::default()
            .path(db_path)
            .open()
            .expect("Failed to open sled db");

        // create data base structure
        let database = DataBase {
            path: path_string,
            node,
            users: BTreeMap::new(),
        };

        // put data base structure to state
        DATABASE.set(RwLock::new(database));
    }

    /// get node DB
    pub fn get_node_db() -> sled::Db {
        let database = DATABASE.get().read().unwrap();
        database.node.clone()
    }

    /// get a user account data base
    ///
    /// Each user account has an own storage folder
    /// with a data base.
    /// The data base is opened on request.
    pub fn get_user_db(account_id: PeerId) -> sled::Db {
        // check if user account data base is already open
        if let Some(db) = Self::user_db_opened(account_id) {
            return db;
        }
        // otherwise open it from disk and save it to state
        else {
            // get data base structure
            let mut database = DATABASE.get().write().unwrap();

            // create path
            let path = Path::new(database.path.as_str());
            let db_folder = path.join(account_id.to_base58());
            let db_path = db_folder.join("user.db");

            // open data base from disk
            let db = sled::Config::default()
                .path(db_path)
                .open()
                .expect("Failed to open sled db");

            // save data base to state
            database.users.insert(account_id.to_bytes(), db.clone());

            // return data base handle
            db
        }
    }

    /// check if user account data base has already been opened
    fn user_db_opened(account_id: PeerId) -> Option<sled::Db> {
        // get data base structure
        let database = DATABASE.get().read().unwrap();

        // check if data base is opened and return the output
        if let Some(db) = database.users.get(&account_id.to_bytes()) {
            return Some(db.clone());
        } else {
            return None;
        }
    }
}

/// Data Base Users Storage
#[derive(Clone, Debug)]
pub struct DbUsers {}

impl DbUsers {
    /// Add a user to the DB
    pub fn add_user(user: UserData) {
        // get node data base
        let db = DataBase::get_node_db();

        // open tree from data base
        let tree: sled::Tree = db.open_tree("users").unwrap();

        // clone key
        let key = user.key.clone();

        // save user
        let user_bytes = bincode::serialize(&user).unwrap();
        if let Err(e) = tree.insert(key.as_slice(), user_bytes) {
            log::error!("Error saving user to data base: {}", e);
        } else {
            if let Err(e) = tree.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }
    }

    // get user table
    pub fn get_tree() -> sled::Tree {
        // get data base
        let db = DataBase::get_node_db();

        // open tree from data base
        db.open_tree("users").unwrap()
    }
}
