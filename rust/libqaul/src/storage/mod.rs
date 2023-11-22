// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Storage Module
//!
//! contains:
//!
//! * configuration management
//! * database handling

use libp2p::PeerId;
use std::path::{Path, PathBuf};

pub mod configuration;
pub mod database;

use configuration::Configuration;
use database::DataBase;
use state;

/// make storage path accessible
static STORAGE_PATH: state::InitCell<String> = state::InitCell::new();

/// storage module structure
pub struct Storage {}

impl Storage {
    /// initialize storage module
    /// requires the path to the data storage folder
    pub fn init(path: String) {
        // put path to state
        STORAGE_PATH.set(path);

        // initialize configuration
        Configuration::init();

        // initialize data base
        DataBase::init();
    }

    /// get data storage path
    ///
    /// This will return the absolute path to the storage folder
    /// as a string, without a trailing slash.
    ///
    /// e.g. on Linux: /home/USERNAME/.config/qaul
    pub fn get_path() -> String {
        STORAGE_PATH.get().clone()
    }

    /// get data storage path for user account
    pub fn get_account_path(account_id: PeerId) -> PathBuf {
        let storage_path_string = STORAGE_PATH.get().clone();
        let storage_path = Path::new(&storage_path_string);
        let account_storage_path = storage_path.join(account_id.to_base58());

        account_storage_path
    }
}
