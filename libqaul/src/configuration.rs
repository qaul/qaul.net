use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use toml;
use std::fs;
use log::{error, info};


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
    pub peers: Vec<String>,
    pub listen: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            initialized: 0,
            id: String::from(""),
            keys: String::from(""),
            peers: vec![String::from(""); 0],
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
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
    pub listen: String,
}

impl Default for Internet {
    fn default() -> Self {
        Internet {
            active: true,
            peers: vec![String::from(""); 0],
            listen: String::from("/ip4/0.0.0.0/tcp/0"),
        }
    }
}

/**
 * local user accounts that are stored on this node
 */
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct User {
    pub name: String,
    pub id: String,
    pub keys: String,
}

impl Default for User {
    fn default() -> Self {
        User {
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
    pub users: Vec<User>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            node: Node::default(),
            lan: Lan::default(),
            internet: Internet::default(),
            users: vec![User::default(); 0],
        }
    }
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
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
                info!("./config.toml {:?}", e);
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

        // // deserialize configuration
        // //s.try_into()
        // let return_conf: Configuration;
        // let t = s.clone();
        // match t.into() {
        //     Ok(myconf) => return_conf = myconf,
        //     Err(e) => error!("try_into() {:?}", e),
        // }

        s.try_into()
    }

    /**
     * Save current configuration to config.toml file
     */
    pub fn save(self: &Self) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
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
            \"internet\":{\"active\":true,\"peers\":[],\"accept_incoming\":false,\"listen\":\"/ip4/0.0.0.0/tcp/9229\"},
            \"users\":[]
        }".to_string();
        // save file
        fs::write("default.json", json_string).expect("Could not write to file!"); 
    }
}
