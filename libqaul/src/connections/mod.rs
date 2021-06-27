/**
 * # Qaul Connections Modules
 * 
 * The modules define how and where to connect to network interfaces.
 */

use libp2p::{
    noise::{Keypair, X25519Spec},
    PeerId
};
use crate::node::Node;
use crate::configuration::Configuration;
use futures::executor::block_on;

pub mod lan;
use lan::Lan;
pub mod internet;
use internet::Internet;


pub struct Connections {
    pub lan: Lan,
    pub internet: Internet,
}

impl Connections {
    pub async fn init( config: Configuration ) -> (Configuration, Connections)  {
        // create transport encryption keys for noise protocol
        let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(Node::get_keys())
        .expect("can create auth keys");

        // initialize Lan module
        let (config, lan) = lan::init(config, auth_keys.clone()).await;

        // initialize Internet overlay module
        let (config, internet) = Internet::init(config, auth_keys).await;

        let con = Connections{ lan: lan, internet: internet };

        (config, con)
    }
}

