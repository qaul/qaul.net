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
        table::RoutingTable,
        users::Users,
        connections::ConnectionTable,
        router_net_proto,
    },
    utilities::timestamp::Timestamp,
};

use crate::services::{
    feed::{Feed},
};
use crate::feed_requester::FeedRequester;


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

    /// propagation ID
    /// 
    /// A number that is increased by one 
    /// on each propagation cycle
    propagation_id: u32,

    /// propagation update time
    /// 
    /// timestamp of the last propagation update
    propagation_timestamp: u64,
}

/// An entry for the scheduler neighbour list
/// that contains the time stamp 
#[derive(Clone, Debug, Copy)]
struct SchedulerEntry {
    /// time of the last send
    timestamp: SystemTime,
    is_first: bool,
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
            propagation_id: 0,
            propagation_timestamp: Timestamp::get_timestamp(),
        };
        SCHEDULER.set(RwLock::new(scheduler));
    }

    /// This loops over all neighbours
    /// and checks if there is any timeout.
    /// If it finds a timeout it returns the node id
    /// to send a routing information to.
    pub fn check_scheduler() -> Option<(PeerId, ConnectionModule, Vec<u8>)> {
        let mut found_neighbour: Option<PeerId> = None;
        let mut neighbour_last_sent: u64 = 0;
        let mut neighbour_is_first: bool = false;
        let mut propagation_id: u32;
        let mut propagation_timestamp: u64;
         

        {
            // get state for reading
            let scheduler = SCHEDULER.get().read().unwrap();

            // loop over all neighbours
            for (id, ctx) in scheduler.neighbours.iter() {
                if ctx.timestamp + scheduler.interval < SystemTime::now() {
                    found_neighbour = Some(id.clone());
                    neighbour_last_sent = Timestamp::get_timestamp_by(&ctx.timestamp);
                    neighbour_is_first = ctx.is_first;
                    break;
                }
            }

            // get propagation information
            propagation_id = scheduler.propagation_id;
            propagation_timestamp = scheduler.propagation_timestamp;
        }

        // check if we have to update the propagation ID
        if Timestamp::get_timestamp() >= propagation_timestamp + 10 * 1000 {
            propagation_id += 1;
            propagation_timestamp = Timestamp::get_timestamp();

            // get scheduler for writing
            let mut scheduler = SCHEDULER.get().write().unwrap();
            scheduler.propagation_id = propagation_id;
            scheduler.propagation_timestamp = propagation_timestamp;

            // update propagation ID
            super::connections::ConnectionTable::update_propagation_id(propagation_id);
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
                    entry.is_first = false;
                }

                // create routing information
                let data = Self::create(node_id.clone(), neighbour_last_sent, neighbour_is_first);

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
                is_first: true,
            });
        }
    }


    /// Create routing information for a neighbour node,
    /// encode the information and return the byte code.
    pub fn create(neighbour: PeerId, last_sent: u64, is_first: bool) -> Vec<u8> {
        // create RouterInfo
        let node_id = Node::get_id();
        let routes = RoutingTable::create_routing_info(neighbour, last_sent);
        let online_user_ids = RoutingTable::get_online_user_ids(last_sent);

        log::info!("online users={} routes={}", online_user_ids.len(), routes.entry.len());
        for online in &online_user_ids{
            log::info!("online user={}", online);
        }

        // log::info!("sending_routing_info count={}", routes.entry.len());
        // for inf in &routes.entry{
        //     let c: &[u8] = &inf.user;
        //     let userid = PeerId::from_bytes(c).unwrap();
        //     log::info!("qaul sending_routing_info user={}, hc={}, propg_id={}", userid, inf.hc[0], inf.pgid);
        // }
        let users = Users::get_user_info_table_by_ids(&online_user_ids);
        for user in &users.info{
            let userid = PeerId::from_bytes(&user.id).unwrap();
            log::info!("user={}", userid);
        }
        
        //create latest Feed ids table
        let mut feeds = router_net_proto::FeedIdsTable{
            ids: Vec::new(),
        };

        if is_first == true{
            let ids = Feed::get_latest_message_ids(5);
            for id in ids{
                feeds.ids.push(id.clone());
            }
        }

        let timestamp = Timestamp::get_timestamp();
        let router_info = router_net_proto::RouterInfoMessage {
            node: node_id.clone().to_bytes(),
            routes: Some(routes),
            users: Some(users),
            feeds: Some(feeds),
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

    //creating feed request message
    pub fn create_feed_request(ids: &Vec<Vec<u8>>)-> Vec<u8>{

        let node_id = Node::get_id();

        //create latest Feed ids table
        let feeds = router_net_proto::FeedIdsTable{
            ids: ids.clone(),
        };

        let timestamp = Timestamp::get_timestamp();
        let router_info = router_net_proto::FeedRequstMessage {
            feeds: Some(feeds)
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

        match decoding_result {
            Ok(container) => {
                // TODO: check signature
                //let signature = container.signature;

                // decode message
                let message_result = router_net_proto::RouterInfoContent::decode(&container.message[..]);

                match message_result {
                    Ok(content) => {
                        let message_info = router_net_proto::RouterInfoMessage::decode(&content.content[..]);
                        if let Ok(message) = message_info {
                            // collect users and routes
                            let messages = message;
                            let users = messages.users;
                            let routes = messages.routes;
                            let feeds = messages.feeds;

                            match users {
                                Some(router_net_proto::UserInfoTable { info }) => {
                                    Users::add_user_info_table(info);
                                },
                                _ => {},
                            }

                            match routes {
                                Some(router_net_proto::RoutingInfoTable { entry} ) => {
                                    ConnectionTable::process_received_routing_info(received.received_from, entry);
                                },
                                _ => {},
                            }
                            match feeds{
                                Some(router_net_proto::FeedIdsTable { ids } ) => {
                                    let missing_ids = Feed::process_received_feed_ids(&ids);
                                    if missing_ids.len() > 0 {                                        
                                        FeedRequester::add(&received.received_from, &missing_ids);
                                    }
                                },
                                _ => {},
                            }
                        }
                    },
                    Err(msg) => {
                        log::error!("RouterInfoContent decode {:?}", msg);
                    },
                }
        },
        Err(msg) => {
            log::error!("RouterInfoContainer decode {:?}", msg);
        },
    }
}
}