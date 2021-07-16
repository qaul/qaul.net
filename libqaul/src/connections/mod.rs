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


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ConnectionModule {
    Lan,
    Internet,
    None,
}

pub struct Connections {
    pub lan: Lan,
    pub internet: Internet,
}

impl Connections {
    pub async fn init() -> Connections  {
        // create transport encryption keys for noise protocol
        let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(Node::get_keys())
        .expect("can create auth keys");

        // initialize Lan module
        let lan = Lan::init(auth_keys.clone()).await;

        // initialize Internet overlay module
        let internet = Internet::init(auth_keys).await;

        let conn = Connections{ lan: lan, internet: internet };

        conn
    }
}

