// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Configuration
//!
//! **Configure qaul via a config file, or from the command-line.**
//!
//! On the first startup a `config.yaml` file is saved.
//! It can be configured and will be read on the next startup.
//! All options are configurable from the command-line too.

use config::{Config, File};
use serde::{Deserialize, Serialize};
use state::InitCell;
use std::{
    fs,
    path::Path,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// make configuration globally accessible mutable state
static CONFIG: InitCell<RwLock<Configuration>> = InitCell::new();

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
    pub listen: Vec<String>,
}

impl Default for Lan {
    fn default() -> Self {
        Lan {
            active: true,
            listen: vec![
                String::from("/ip4/0.0.0.0/udp/0/quic-v1"),
                String::from("/ip4/0.0.0.0/tcp/0"),
                String::from("/ip6/::/udp/0/quic-v1"),
                String::from("/ip6/::/tcp/0"),
            ],
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct InternetPeer {
    pub address: String,
    pub name: String,
    pub enabled: bool,
}

/// Internet Overlay Connection Module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Internet {
    pub active: bool,
    pub peers: Vec<InternetPeer>,
    pub do_listen: bool,
    pub listen: Vec<String>,
}

impl Default for Internet {
    fn default() -> Self {
        let mut port: u16 = 0;
        if let Some(port_str) = super::super::get_default_config("port") {
            match port_str.parse::<u16>() {
                Ok(p) => {
                    port = p;
                }
                _ => {}
            }
        }
        // allow unused_variable needed for android
        #[allow(unused_variables)]
        let listen_ipv4_quic: String = format!("/ip4/0.0.0.0/udp/{}/quic-v1", port);
        #[allow(unused_variables)]
        let listen_ipv4: String = format!("/ip4/0.0.0.0/tcp/{}", port);
        #[allow(unused_variables)]
        let listen_ipv6_quic: String = format!("/ip6/::/udp/{}/quic-v1", port);
        #[allow(unused_variables)]
        let listen_ipv6: String = format!("/ip6/::/tcp/{}", port);

        Internet {
            active: true,
            peers: vec![
                InternetPeer {
                    address: String::from("/ip4/144.91.74.192/udp/9229/quic-v1"),
                    name: String::from("qaul Community Node [IPv4]"),
                    enabled: false,
                },
                InternetPeer {
                    address: String::from("/ip6/2a02:c207:3004:3887::1/udp/9229/quic-v1"),
                    name: String::from("qaul Community Node [IPv6]"),
                    enabled: false,
                },
            ],
            do_listen: false,
            #[cfg(any(target_os = "android", target_os = "ios"))]
            listen: vec![
                String::from("/ip4/0.0.0.0/udp/9229/quic-v1"),
                String::from("/ip4/0.0.0.0/tcp/9229"),
                String::from("/ip6/::/udp/9229/quic-v1"),
                String::from("/ip6/::/tcp/9229"),
            ],
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            listen: vec![listen_ipv4_quic, listen_ipv4, listen_ipv6_quic, listen_ipv6],
        }
    }
}

/// local user accounts that are stored on this node
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserAccount {
    pub name: String,
    pub id: String,
    pub keys: String,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub session_token: Option<String>,
    pub storage: StorageOptions,
}

impl Default for UserAccount {
    fn default() -> Self {
        UserAccount {
            name: String::from(""),
            id: String::from(""),
            keys: String::from(""),
            password_hash: None,
            password_salt: None,
            session_token: None,
            storage: StorageOptions::default(),
        }
    }
}

/// Debugging Configuration Options
///
/// The following options can be configured:
///
/// * logging to file
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DebugOption {
    pub log: bool,
}

impl Default for DebugOption {
    fn default() -> Self {
        DebugOption { log: false }
    }
}

/// Routing Configuration Options
///
/// The following options can be configured:
/// All units are second
/// because rtt is measured as micro seconds
/// * routing options
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RoutingOptions {
    //Sending the table every 10 seconds to direct neighbours.
    pub sending_table_period: u64,
    //Pinging every neighbour all 5 seconds.
    pub ping_neighbour_period: u64,
    //Hop count penalty.
    pub hop_count_penalty: u64,
    //How long a route is stored until it is removed.
    pub maintain_period_limit: u64,
}

impl Default for RoutingOptions {
    fn default() -> Self {
        RoutingOptions {
            sending_table_period: 10,   //10 seconds, unit seconds
            ping_neighbour_period: 5,   //5  seconds, unit: seconds
            hop_count_penalty: 10,      //10 seconds, unit: second
            maintain_period_limit: 300, //5min, unit: second
        }
    }
}

/// Storage Configuration Options
///
/// The following options can be configured:
/// size_total units are MB
/// * storage options
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StorageOptions {
    //storage node users
    pub users: Vec<String>,
    //Sending the table every 10 seconds to direct neighbours.
    pub size_total: u32,
}

impl Default for StorageOptions {
    fn default() -> Self {
        StorageOptions {
            users: vec![],
            size_total: 1024, //1024 MB
        }
    }
}

/// Configuration Structure of libqaul
///
/// This structure contains the entire configuration of libqaul.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Configuration {
    pub node: Node,
    pub lan: Lan,
    pub internet: Internet,
    pub user_accounts: Vec<UserAccount>,
    pub debug: DebugOption,
    pub routing: RoutingOptions,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            node: Node::default(),
            lan: Lan::default(),
            internet: Internet::default(),
            user_accounts: Vec::new(),
            debug: DebugOption::default(),
            routing: RoutingOptions::default(),
        }
    }
}

/// Configuration implementation of libqaul
impl Configuration {
    /// Initialize configuration
    pub fn init() {
        // create configuration path
        let path_string = super::Storage::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("config.yaml");

        let config: Configuration = match Config::builder()
            .add_source(File::with_name(&config_path.to_str().unwrap()))
            .build()
        {
            Err(_) => {
                log::error!("no configuration file found, creating one.");
                Configuration::default()
            }
            Ok(c) => c.try_deserialize::<Configuration>().unwrap(),
        };

        // put configuration to state
        CONFIG.set(RwLock::new(config));
    }

    /// Load a configuration file for upgrading purposes
    ///
    /// This function is only to be used for the upgrading procedure.
    /// Libqaul uses the `init()` function to load and initialize the configuration!
    pub fn load(path: &str) -> Option<Configuration> {
        if let Ok(c) = Config::builder().add_source(File::with_name(path)).build() {
            return Some(c.try_deserialize::<Configuration>().unwrap());
        }
        None
    }

    /// lend configuration for reading
    pub fn get<'a>() -> RwLockReadGuard<'a, Configuration> {
        let config = CONFIG.get().read().unwrap();
        config
    }

    /// get user account
    pub fn get_user(user_id: String) -> Option<UserAccount> {
        let config = CONFIG.get().read().unwrap();
        for user in &config.user_accounts {
            if user.id == user_id {
                return Some(user.clone());
            }
        }
        None
    }

    /// CHANGE: remove this function & save configuration directly via UserAccount
    pub fn update_user_storage(user_id: String, opt: &StorageOptions) {
        let mut config = CONFIG.get().write().unwrap();
        for i in 0..config.user_accounts.len() {
            if let Some(user) = config.user_accounts.get_mut(i) {
                if user.id == user_id {
                    user.storage = opt.clone();
                    break;
                }
            }
        }
    }

    /// CHANGE: remove this function and save configuration directly via UserAccount
    pub fn update_total_size(user_id: String, size: u32) {
        let mut config = CONFIG.get().write().unwrap();
        for i in 0..config.user_accounts.len() {
            if let Some(user) = config.user_accounts.get_mut(i) {
                if user.id == user_id {
                    user.storage.size_total = size;
                    break;
                }
            }
        }
    }

    /// lend configuration for writing
    pub fn get_mut<'a>() -> RwLockWriteGuard<'a, Configuration> {
        let config_mutable = CONFIG.get().write().unwrap();
        config_mutable
    }

    /// Enable/disable logging to file for debugging
    pub fn enable_debug_log(enable: bool) {
        let mut config_mutable = CONFIG.get().write().unwrap();
        config_mutable.debug.log = enable;
    }

    /// Check if logging to file for debugging is enabled
    pub fn get_debug_log() -> bool {
        let config_mutable = CONFIG.get().read().unwrap();
        config_mutable.debug.log
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
        let yaml = serde_yaml_ng::to_string(config).expect("Couldn't encode into YAML values.");

        // create path to config file
        let path_string = super::Storage::get_path();
        let path = Path::new(path_string.as_str());
        let config_path = path.join("config.yaml");

        log::trace!("Writing to Path {:?}, {:?}", path, config_path);

        fs::write(config_path.clone(), yaml)
            .expect(&format!("Could not write config to {:?}.", config_path));
    }
}
