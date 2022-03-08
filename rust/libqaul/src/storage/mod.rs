// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Storage Module
//! 
//! contains:
//! 
//! * configuration management
//! * database handling

pub mod configuration;
pub mod database;
pub mod logger;

use configuration::Configuration;
use database::DataBase;
use state;

/// make storage path accessible
static STORAGE_PATH: state::Storage<String> = state::Storage::new();

/// storage module structure
pub struct Storage {

}

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
    pub fn get_path() -> String {
        STORAGE_PATH.get().clone()
    }
}
