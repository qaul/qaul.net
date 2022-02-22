// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Routing Info
//! 
//! This module communicates the routing information with it's direct
//! neighbours, on which information the global routing table is build.
//! 
//! It receives and sends the routing information to the neighbours, 
//! via the qaul_info swarm behaviour.
//! 
//! It is packaging and unpacking the routing information messages.
//! 
//! It has an own list of neighbour nodes with a timer on each
//! of them to make sure, each node is only sent the routing
//! table over one of the interfaces.
//! The timer needs to be polled manually.

use libp2p::PeerId;
use prost::Message;
use state::Storage;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, SystemTime},
};

use qaul_info::{
    QaulInfoReceived, 
    //QaulInfoSend,
};

use crate::{
    connections::ConnectionModule,
    node::Node,
    router::{
        neighbours::Neighbours,
        table::{RoutingTable, RoutingInfoTable},
        users::{Users, UserInfoTable},
        connections::ConnectionTable,
        router_net_proto,
    },
    utilities::timestamp::Timestamp,
};

/// mutable state of Neighbours table per ConnectionModule
static SCHEDULER: Storage<RwLock<Scheduler>> = Storage::new();


/// global scheduler state
#[derive(Clone, Debug)]
pub struct Scheduler {
    /// list of all neighbours, to schedule sending of 
    /// routing information.
    /// If a node is interconnected via several connection
    /// modules, the table is only sent on one of them.
    neighbours: HashMap<PeerId, SchedulerEntry>,

    /// interval in which updated routing information
    /// shall be sent to the neighbouring nodes.
    interval: Duration,
}

/// An entry for the scheduler neighbour list
/// that contains the time stamp 
#[derive(Clone, Debug, Copy)]
struct SchedulerEntry {
    /// time of the last send
    timestamp: SystemTime,
}

/// Serializable routing information message 
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RouterInfoMessage {
    /// node id
    pub node: Vec<u8>,
    /// routes information list 
    pub routes: RoutingInfoTable,
    /// user information list
    pub users: UserInfoTable,
    /// timestamp, when this was generated
    pub timestamp: u64,
}

/// Signed message container
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RouterInfoContainer {
    /// data contains the binary encoded RouterInfoMessage
    pub data: Vec<u8>,
    /// signature of RouterInfoMessage
    pub signature: Vec<u8>,
}

/// RouterInfo Module
#[derive(Clone, Debug)]
pub struct RouterInfo {
}

impl RouterInfo {
    /// Initialize router info and scheduler
    /// with the interval in seconds that the 
    /// routing information shall be sent
    /// to neighbours.
    pub fn init(interval_seconds: u64) {
        // neighbours list for routing info scheduler
        let scheduler = Scheduler { 
            neighbours: HashMap::new(),
            interval: Duration::from_secs(interval_seconds),
        };
        SCHEDULER.set(RwLock::new(scheduler));
    }

    /// This loops over all neighbours
    /// and checks if there is any timeout.
    /// If it finds a timeout it returns the node id
    /// to send a routing information to.
    pub fn check_scheduler() -> Option<(PeerId, ConnectionModule, Vec<u8>)> {
        let mut found_neighbour: Option<PeerId> = None;

        {
            // get state for reading
            let scheduler = SCHEDULER.get().read().unwrap();

            // loop over all neighbours
            for (id, time) in scheduler.neighbours.iter() {
                if time.timestamp + scheduler.interval < SystemTime::now() {
                    found_neighbour = Some(id.clone());
                    break
                }
            }
        }

        // process finding
        if let Some(node_id) = found_neighbour {
            // Check whether this node is 
            // still connected and over which connection module
            // we can approach it.
            let module = Neighbours::is_neighbour(&node_id);

            // get SCHEDULER for writing
            let mut scheduler = SCHEDULER.get().write().unwrap();

            if module == ConnectionModule::None {
                log::error!("node is not a neighbour anymore: {:?}", node_id);
                // delete this entry
                scheduler.neighbours.remove(&node_id);
            }
            else {
                // update timer
                if let Some(entry) = scheduler.neighbours.get_mut(&node_id){
                    entry.timestamp = SystemTime::now();
                }

                // create routing information
                let data = Self::create(Some(node_id.clone()));

                // create result
                return Some((node_id, module, data))
            }
        }
        
        None
    }

    /// add new neighbour entry
    pub fn add_neighbour(node_id: PeerId) {
        let exists;
        log::info!("add new neighbour {:?} to RouterInfo scheduler", node_id);
        // check if a neighbour entry exists
        {
            let scheduler = SCHEDULER.get().read().unwrap();
            exists = scheduler.neighbours.contains_key(&node_id);
        }

        // if it does not exist add it to scheduler
        if !exists {
            let mut scheduler = SCHEDULER.get().write().unwrap();
            let interval = scheduler.interval.clone();
            scheduler.neighbours.insert(node_id, SchedulerEntry {
                timestamp: SystemTime::now() - interval,
            });
        }
    }

    /// Create routing information for a neighbour node,
    /// encode the information and return the byte code.
    pub fn create(neighbour: Option<PeerId>) -> Vec<u8> {
        // create RouterInfo
        let node_id = Node::get_id();
        let routes = RoutingTable::create_routing_info(neighbour);
        let users = Users::get_user_info_table();

        let timestamp = Timestamp::get_timestamp();

        let router_info = router_net_proto::RouterInfoMessage {
            node: node_id.clone().to_bytes(),
            routes: Some(routes),
            users: Some(users),
            timestamp,
        };

        let mut buf = Vec::with_capacity(router_info.encoded_len());
        router_info.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            content: buf,
            time: timestamp,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        // convert to bytes data
        //let data = bincode::serialize(&router_info).unwrap();

        // sign data
        let keys = Node::get_keys();
        let signature = keys.sign(&buf).unwrap();

        // create signed container
        let router_info_container = router_net_proto::RouterInfoContainer {
            signature,
            message: buf,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_container.encoded_len());
        router_info_container.encode(&mut buf).expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// process received qaul_info message
    pub fn received( received: QaulInfoReceived ) {
        // decode message to structure
        let decoding_result = router_net_proto::RouterInfoContainer::decode(&received.data[..]);

        if let Ok(container) = decoding_result {
            // TODO: check signature

            // unstuff data
            let message_result = router_net_proto::RouterInfoContent::decode(&container.message[..]);

            if let Ok(content) = message_result {
                let message_info = router_net_proto::RouterInfoMessage::decode(&content.content[..]);
                if let Ok(message) = message_info {
                    let messages = message;
                    let users = messages.users;
                    let routes = messages.routes;

                    if let Some(router_net_proto::UserInfoTable { info }) = users {
                        Users::add_user_info_table(info);
                    } else {
                        todo!("None => add_user_info_table")
                    }

                    if let Some(router_net_proto::RoutingInfoTable { entry}) = routes {
                        ConnectionTable::process_received_routing_info(received.received_from, entry);
                    } else {
                        todo!("None => process_received_routing_info")
                    }
                }
            } else {
                log::error!("RouterInfoContent decoding error")
            }
                } else {
            log::error!("info - RouterInfoContainer decode")
        }
}
}