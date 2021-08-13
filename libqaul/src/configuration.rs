/**
 * # Configuration
 * 
 * **Configure qaul.net via a config file, or from the commandline.**
 * 
 * On the first startup a `config.toml` file is saved.
 * It can be configured and will be read on the next startup.
 * All options are configurable from the commandline too.
 */

use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use toml;
use std::{
    fs, 
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use log::error;
use state::Storage;

// make configuration globally accessible mutable state
static CONFIG: Storage<RwLock<Configuration>> = Storage::new();

/**
 * Configuration of the local Node
 * 
 * Here the keys and identity are stored
 */
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

/**
 * LAN Connection Module
 */
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

/**
 * Internet Overlay Connection Module
 */
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

/**
 * local user accounts that are stored on this node
 */
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

impl Configuration {
    pub fn init() {
        let mut s = Config::new();

        // FIXME: There is a problem in the rs-config library, that empty vectors of 
        //       structs cannot be initialized. The only way to do that is to load
        //       a json file. 
        // Workaround: In order to be able to start with an empty configuration we 
        //       save a default.json file to initialize the config.
        Configuration::create_default_json();

        // set default values via `./default.json`
        let mut d = s.clone();
        match d.merge(File::with_name("default")) {
            Ok(default) => s = default.clone(),
            Err(e) => error!("./default.json {:?}", e),
        }

        // get `./config.toml`
        //s.merge(File::with_name("config")).unwrap();
        let mut c = s.clone();
        match c.merge(File::with_name("config")) {
            Ok(conf) => s = conf.clone(),
            Err(e) => {
                error!("./config.toml {:?}", e);
            },
        }

        // Add configuration options from environment variables (with a prefix of QAUL)
        // e.g. `QAUL_DEBUG=1 ./target/qaul` sets the `debug` key
        //s.merge(Environment::with_prefix("QAUL")).unwrap();
        let mut e = s.clone();
        match e.merge(Environment::with_prefix("QAUL")) {
            Ok(env) => s = env.clone(),
            Err(e) => error!("Environment {:?}", e),
        }

        let config: Configuration = s.try_into().unwrap();
        
        // put configuration to state
        CONFIG.set(RwLock::new(config));
    }

    /**
     * lend configuration for reading
     */
    pub fn get<'a>() -> RwLockReadGuard<'a, Configuration> {
        let config = CONFIG.get().read().unwrap();
        config
    }

    /**
     * lend configuration for writing
     */
    pub fn get_mut<'a>() -> RwLockWriteGuard<'a, Configuration> {
        let config_mutable = CONFIG.get().write().unwrap();
        config_mutable
    }

    /**
     * Returns true/false whether this node has been initialized, 
     * or needs to be created for the first time.
     */
    pub fn is_node_initialized() -> bool {
        let config = CONFIG.get().read().unwrap();
        if config.node.initialized == 0 {
            return false
        }
        true
    }

    /**
     * Save current configuration to config.toml file
     */
    pub fn save() {
        let config = CONFIG.get();
        let toml_string = toml::to_string(config).expect("Could not encode TOML value");
        println!("{}", toml_string);
        fs::write("config.toml", toml_string).expect("Could not write to file!"); 
    }

    /**
     * Create a default.json configuration file.
     * This is a workaround in order to be able to initialize an empty configuration.
     */
    fn create_default_json() {
        // default json configuration string
        let json_string = "{
            \"node\":{\"initialized\":0,\"id\":\"\",\"keys\":\"\"},
            \"lan\":{\"active\":true,\"listen\":\"/ip4/0.0.0.0/tcp/0\"},
            \"internet\":{\"active\":true,\"peers\":[],\"do_listen\":false,\"listen\":\"/ip4/0.0.0.0/tcp/0\"},
            \"user_accounts\":[]
        }".to_string();
        // save file
        fs::write("default.json", json_string).expect("Could not write to file!"); 
    }
}
