/**
 * # Qaul Connections Modules
 * 
 * The modules define how and where to connect to network interfaces.
 */

pub mod events;
pub mod lan;
pub mod internet;

use libp2p::{
    noise::{Keypair, X25519Spec},
};
use serde::{Serialize, Deserialize};

use crate::node::Node;
use lan::Lan;
use internet::Internet;


/// enum with all connection modules
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ConnectionModule {
    /// This is a local user and does not need
    /// any further routing.
    Local,
    /// Lan module, for all kind of lan connections,
    /// neighbour nodes are found over mdns.
    Lan,
    /// Connect statically to remote nodes.
    Internet,
    /// BLE module
    Ble,
    /// no connection module known for this
    None,
}

/// Collection of all connections of libqaul
/// each collection is a libp2p swarm
pub struct Connections {
    pub lan: Option<Lan>,
    pub internet: Option<Internet>,
}

impl Connections {
    /// initialize connections
    pub async fn init() -> Connections  {
        // create transport encryption keys for noise protocol
        let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(Node::get_keys())
        .expect("can create auth keys");

        // initialize Lan module
        let lan = Lan::init(auth_keys.clone()).await;

        // initialize Internet overlay module
        let internet = Internet::init(auth_keys).await;

        let conn = Connections{ lan: Some(lan), internet: Some(internet) };

        conn
    }

    /// Initialize connections for android
    /// This is here for debugging reasons
    pub async fn init_android() -> Connections  {
        log::info!("init_android() start");


        // create transport encryption keys for noise protocol
        let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(Node::get_keys())
        .expect("can create auth keys");

        log::info!("init_android() auth_keys generated");


        // initialize Lan module
        let lan = Lan::init(auth_keys.clone()).await;

        log::info!("init_android() lan initialized");

        // initialize Internet overlay module
        let internet = Internet::init(auth_keys).await;

        log::info!("init_android() internet initialized");

        //let conn = Connections{ lan: None, internet: Some(internet) };
        let conn = Connections{ lan: None, internet: Some(internet) };

        conn
    }
   
}

