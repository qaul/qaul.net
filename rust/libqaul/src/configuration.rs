//! # Configuration
//! **Configure qaul.net via a config file, or from the commandline.**
//!
//! On the first startup a `config.toml` file is saved.
//! It can be configured and will be read on the next startup.
//! All options are configurable from the commandline too.

use config::{Config, File};
use log::info;
use serde::{Deserialize, Serialize};
use state::Storage;
use std::{
    fs,
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
            //peers: vec![String::from(""); 0],
            peers: Vec::new(),
            do_listen: false,
            listen: String::from("/ip4/0.0.0.0/tcp/9229"),
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
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
    pub fn init(path: Option<&str>) {
        let mut settings = Config::default();

        let config: Configuration = if let Some(path) = path {
            // Merge config if a Config file exists
            match settings.merge(File::with_name(path)) {
                Err(_) => Configuration::default(),
                Ok(c) => c
                    .clone()
                    .try_into()
                    .expect("Couldn't Convert to `Configuration`, malformed config file."),
            }
        } else {
            Configuration::default()
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

    /// Save current configuration to config.toml file
    pub fn save() {
        let config = CONFIG.get();

        let yaml = serde_yaml::to_string(config).expect("Couldn't encode into YAML values.");

        let path = format!("config.yaml");

        info!("Writing to Path : {:?}", path);

        fs::write(path.as_str(), yaml).expect(&format!("Could not write config to {}.", path));
    }

    /// FOR DEBUGGING ANDROID
    ///
    /// Initialize a default the configuration for android
    pub fn init_android() {
        // create Node configuration
        let node = Node {
            initialized: 1,
            id: String::from("12D3KooWMRmfDGEuKWX6RRPrP2mVc293kfff7XuM1hP91HWr3HsS"),
            keys: String::from("fBgCB+NsT0jEkOmfXRYyuH5ELCODSCDNbG7I8RdAnrSsgnIXOhLqpReH2hQRgDVcr0IzoTRNVkRXO+iN/m7NLQ=="),
        };

        // create Lan configuration
        let lan = Lan {
            active: true,
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
        };

        // create Internet configuration
        let internet = Internet {
            active: true,
            peers: vec![String::from("/ip4/144.91.74.192/tcp/9229"); 1],
            //peers: Vec::new(),
            do_listen: false,
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
        };

        // create UserAccount configuration
        let mut user_accounts = Vec::new();
        user_accounts.push(UserAccount {
            name: String::from("DEBUG ONLY"),
            id: String::from("12D3KooWKAGBaQKMcGzpnrAYE8JQq2NmYnSYJMcBj8DdJ9QVDEPf"),
            keys: String::from("POQCEaXwvi4P7V9VVu84fwhm3tEYzGIPQg4jw8LMuECK0gfQSROZJww/sN9dIqe7m33KcoriZe/ImV6XseQVdg=="),
        });

        // create Configuration structure
        let config = Configuration {
            node,
            lan,
            internet,
            user_accounts,
        };

        // save to state
        CONFIG.set(RwLock::new(config));
    }
}
