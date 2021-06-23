use libp2p::{
    swarm::Swarm,
};
use serde::{Serialize, Deserialize};
// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo

use crate::node::Node;
use crate::connections::lan::QaulLanBehaviour;


#[derive(Debug, Serialize, Deserialize)]
pub struct FeedMessage {
    pub message: String,
}


pub fn send(cmd: &str, swarm: &mut Swarm<QaulLanBehaviour>) {
    let rest = cmd.strip_prefix("f ");

    let msg = FeedMessage {
        message: rest.unwrap().to_string(),
    };

    // fload via MDNS
    let json = serde_json::to_string(&msg).expect("can jsonify request");
    swarm.behaviour_mut().floodsub.publish(Node::get_topic(), json.as_bytes());
}
