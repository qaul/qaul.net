/**
 * # Qaul Routing Info
 * 
 * Packaging and unpacking of routing information messages.
 */

use libp2p::PeerId;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    node::Node,
    router::{
        table::{RoutingTable, RoutingInfoTable},
        users::UserInfoTable,
    },
};


/// Serializable routing information entry about one 
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RouterInfo {
    /// node id
    pub node: Vec<u8>,
    /// routes information list 
    pub routes: RoutingInfoTable,
    /// user information list
    pub users: UserInfoTable,
    /// timestamp, when this was generated
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RouterInfoMessage {
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}

impl RouterInfo {
    /// Create routing information for a neighbouring node
    /// and return bytes code of it.
    pub fn create(neighbour: Option<PeerId>) -> Vec<u8> {
        // create RouterInfo
        let node_id = Node::get_id();
        let routes = RoutingTable::create_routing_info(neighbour);
        let users = UserInfoTable(Vec::new());

        let time = SystemTime::now();
        let duration = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = u64::try_from(duration.as_millis()).unwrap();

        let router_info = RouterInfo {
            node: node_id.to_bytes(),
            routes,
            users,
            timestamp,
        };

        // convert to bytes data
        let data = bincode::serialize(&router_info).unwrap();

        // sign data
        let keys = Node::get_keys();
        let signature = keys.sign(&data).unwrap();

        // create signed message
        let message = RouterInfoMessage {
            data,
            signature,
        };

        // return binary data
        bincode::serialize(&message).unwrap()
    }
}