use config::{Config, File};
use serde::{Deserialize, Serialize};

/// Configuration of the local Node
///
/// Here the keys and identity are stored
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Node {
    pub initialized: u8,
    pub id: String,
    pub keys: String,
}

impl Node {}

/// LAN Connection Module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Lan {
    pub active: bool,
    pub listen: String,
}

impl Lan {}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct InternetPeer {
    pub address: String,
    pub enabled: bool,
}

/// Internet Overlay Connection Module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Internet {
    pub active: bool,
    pub peers: Vec<InternetPeer>,
    pub do_listen: bool,
    pub listen: String,
}

impl Internet {}

/// local user accounts that are stored on this node
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserAccount {
    pub name: String,
    pub id: String,
    pub keys: String,
    pub storage: StorageOptions,
}

impl UserAccount {}

/// Debugging Configuration Options
///
/// The following options can be configured:
///
/// * logging to file
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DebugOption {
    pub log: bool,
}

impl DebugOption {}

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

impl RoutingOptions {}

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

impl StorageOptions {}

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

impl Configuration {}

/// Configuration implementation of libqaul
impl Configuration {
    /// Initialize configuration
    pub fn load(path: &str) -> Option<Configuration> {
        let mut settings = Config::default();

        // Merge config if a Config file exists
        if let Ok(c) = settings.merge(File::with_name(path)) {
            return Some(c.clone().try_into().unwrap());
        }
        None
    }
}
