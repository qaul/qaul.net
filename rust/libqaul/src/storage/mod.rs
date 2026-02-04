// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Storage Module
//!
//! contains:
//!
//! * configuration management
//! * database handling

use libp2p::PeerId;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub mod configuration;
pub mod database;

use configuration::Configuration;
use database::DataBase;
use state;

/// make storage path accessible (global state - deprecated)
static STORAGE_PATH: state::InitCell<String> = state::InitCell::new();

/// Storage module instance
///
/// This struct holds all storage-related state for a single libqaul instance.
/// It contains the configuration and database.
pub struct StorageModule {
    /// Path where all data is stored
    pub path: String,
    /// Configuration
    pub config: Arc<RwLock<Configuration>>,
    /// Database
    pub database: Arc<RwLock<DataBase>>,
}

impl StorageModule {
    /// Create a new StorageModule instance
    ///
    /// This initializes configuration and database via global state,
    /// then wraps references to them. This ensures backward compatibility
    /// with code that uses `Storage::get_path()`, `Configuration::get()`, etc.
    pub fn new(path: String) -> Self {
        // Initialize global state - this opens the database once
        Storage::init(path.clone());

        // Clone configuration from global state
        let config = Configuration::get().clone();

        // Get the database handle from global state (don't open again!)
        let db = DataBase::get_node_db();
        let database = DataBase {
            path: path.clone(),
            node: db,
            users: std::collections::BTreeMap::new(),
        };

        Self {
            path,
            config: Arc::new(RwLock::new(config)),
            database: Arc::new(RwLock::new(database)),
        }
    }

    /// Get the storage path
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Get data storage path for user account
    pub fn get_account_path(&self, account_id: PeerId) -> PathBuf {
        let storage_path = Path::new(&self.path);
        storage_path.join(account_id.to_base58())
    }
}

/// storage module structure (global state wrapper - for backward compatibility)
pub struct Storage {}

impl Storage {
    /// initialize storage module
    /// requires the path to the data storage folder
    ///
    /// Note: This uses global state. For new code, prefer using `StorageModule::new()`.
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
    ///
    /// Note: This uses global state. For new code, prefer using `StorageModule::get_path()`.
    pub fn get_path() -> String {
        STORAGE_PATH.get().clone()
    }

    /// get data storage path for user account
    ///
    /// Note: This uses global state. For new code, prefer using `StorageModule::get_account_path()`.
    pub fn get_account_path(account_id: PeerId) -> PathBuf {
        let storage_path_string = STORAGE_PATH.get().clone();
        let storage_path = Path::new(&storage_path_string);
        let account_storage_path = storage_path.join(account_id.to_base58());

        account_storage_path
    }
}
