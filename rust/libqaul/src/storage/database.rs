// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Database
//! 
//! Embedded sled database. 

use state::Storage;
use serde::{Serialize, Deserialize};
use sled_extensions::{
    DbExt,
    bincode::Tree,
};
use crate::router::users::UserData;

/// make database globally accessible
static DATABASE: Storage<sled_extensions::Db> = Storage::new();


/// DataBase Module
#[derive(Clone, Debug)]
pub struct DataBase {
    //pub node: sled_extensions::Db,
    //pub users: BTreeMap<PeerId, sled_extensions::Db>,
}

impl DataBase {
    /// Initialize data base,
    /// open data base from disk and set it to global state.
    pub fn init() {
        // create node data base path
        let path_string = super::Storage::get_path();
        let path = std::path::Path::new(path_string.as_str());
        let db_path = path.join("node.db");

        // open data base
        let db = sled_extensions::Config::default()
            .path(db_path)
            .open()
            .expect("Failed to open sled db");

        // set database to state
        DATABASE.set(db);
    }

    // get DB
    pub fn get_db() -> sled_extensions::Db {
        let db = DATABASE.get();
        db.clone()
    }
}

/// Data Base Users Storage
#[derive(Clone, Debug)]
pub struct DbUsers {
}

impl DbUsers {
    /// Add a user to the DB
    pub fn add_user(user: UserData) {
        // get data base
        let db = DATABASE.get();

        // open tree from data base
        let tree: Tree<UserData> = db.open_bincode_tree("users").unwrap();

        // clone key
        let key = user.key.clone();

        // save user
        if let Err(e) = tree.insert(key.as_slice(), user) {
            log::error!("Error saving user to data base: {}", e);
        } 
        else {
            if let Err(e) = tree.flush() {
                log::error!("Error when flushing data base to disk: {}", e);
            }
        }
    }

    // get user table
    pub fn get_tree() -> Tree<UserData> {
        // get data base
        let db = DATABASE.get();

        // open tree from data base
        db.open_bincode_tree("users").unwrap()
    }
}

