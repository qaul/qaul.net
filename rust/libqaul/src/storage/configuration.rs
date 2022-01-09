// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Configuration
//! 
//! **Configure qaul.net via a config file, or from the commandline.**
//!
//! On the first startup a `config.yaml` file is saved.
//! It can be configured and will be read on the next startup.
//! All options are configurable from the commandline too.

use config::{Config, File};
use log::info;
use serde::{Deserialize, Serialize};
use state::Storage;
use std::{
    fs,
    path::Path,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// make configuration globally accessible mutable state
static CONFIG: Storage<RwLock<Configuration>> = Storage::new();

/// Configuration of the local Node
///
/// Here the keys and identity are stored
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Node {
    pub initialized: u8,
    pub id: String,
    pub keys: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            initialized: 0,
            id: String::from(""),
            keys: String::from(""),
        }
    }
}

/// LAN Connection Module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Lan {
    pub active: bool,
    pub listen: String,
}

impl Default for Lan {
    fn default() -> Self {
        Lan {
            active: true,
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
        }
    }
}

/// Internet Overlay Connection Module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Internet {
    pub active: bool,
    pub peers: Vec<String>,
    pub do_listen: bool,
    pub listen: String,
}

impl Default for Internet {
    fn default() -> Self {
        Internet {
            active: true,
            #[cfg(any(target_os = "android", target_os = "ios"))]
            peers: vec![String::from("/ip4/144.91.74.192/tcp/9229"); 1],
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            peers: vec![String::from("/ip4/144.91.74.192/tcp/9229"); 1],
            //peers: Vec::new(),
            do_listen: false,
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
        }
    }
}

/// local user accounts that are stored on this node
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserAccount {
    pub name: String,
    pub id: String,
    pub keys: String,
}

impl Default for UserAccount {
    fn default() -> Self {
        UserAccount {
            name: String::from(""),
            id: String::from(""),
            keys: String::from(""),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Configuration {
    pub node: Node,
    pub lan: Lan,
    pub internet: Internet,
    pub user_accounts: Vec<UserAccount>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            node: Node::default(),
            lan: Lan::default(),
            internet: Internet::default(),
            user_accounts: Vec::new(),
        }
    }
}

/// Configuration implementation of libqaul
impl Configuration {
    /// Initialize configuration
    pub fn init() {
        let mut settings = Config::default();

        // create configuration path
        let path_string = super::Storage::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("config.yaml");

        // Merge config if a Config file exists
        let config: Configuration = match settings.merge(File::with_name(&config_path.to_str().unwrap())) {
            Err(_) => {
                log::error!("no configuration file found, creating one.");
                Configuration::default()
            },
            Ok(c) => c
                .clone()
                .try_into()
                .expect("Couldn't Convert to `Configuration`, malformed config file."),
        };

        // There is no key for debug in the the configuration hence fails.

        // Add configuration options from environment variables (with a prefix of QAUL)
        // e.g. `QAUL_DEBUG=1 ./target/qaul` sets the `debug` key

        // match e.merge(Environment::with_prefix("QAUL")) {
        //     Ok(env) => settings = env.clone(),
        //     Err(e) => error!("Environment {:?}", e),
        // }

        // put configuration to state
        CONFIG.set(RwLock::new(config));
    }

    /// lend configuration for reading
    pub fn get<'a>() -> RwLockReadGuard<'a, Configuration> {
        let config = CONFIG.get().read().unwrap();
        config
    }

    /// lend configuration for writing
    pub fn get_mut<'a>() -> RwLockWriteGuard<'a, Configuration> {
        let config_mutable = CONFIG.get().write().unwrap();
        config_mutable
    }

    /// Returns true/false whether this node has been initialized,
    /// or needs to be created for the first time.
    pub fn is_node_initialized() -> bool {
        let config = CONFIG.get().read().unwrap();
        if config.node.initialized == 0 {
            return false;
        }
        true
    }

    /// Save current configuration to config.yaml file
    pub fn save() {
        let config = CONFIG.get();

        // create yaml configuration format
        let yaml = serde_yaml::to_string(config).expect("Couldn't encode into YAML values.");

        // create path to config file
        let path_string = super::Storage::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("config.yaml");

        info!("Writing to Path {:?}, {:?}", path, config_path);

        fs::write(config_path.clone(), yaml).expect(&format!("Could not write config to {:?}.", config_path));
    }
}
