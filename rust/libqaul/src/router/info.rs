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

use crate::utilities::qaul_id::QaulId;
use libp2p::PeerId;
use prost::Message;
use qaul_info::QaulInfoReceived;
use state::InitCell;
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, SystemTime},
};

use crate::{
    connections::ConnectionModule,
    node::Node,
    router::{
        connections::ConnectionTable, neighbours::Neighbours, router_net_proto,
        table::RoutingTable, users::Users,
    },
    utilities::timestamp::Timestamp,
};

use crate::feed_requester::FeedRequester;
use crate::feed_requester::FeedResponser;
use crate::services::feed::Feed;

use crate::router::user_requester::UserRequester;
use crate::router::user_requester::UserResponser;

/// mutable state of Neighbours table per ConnectionModule
static SCHEDULER: InitCell<RwLock<Scheduler>> = InitCell::new();

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
pub struct RouterInfo {}

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
                log::debug!("node is not a neighbour anymore: {:?}", node_id);
                // delete this entry
                scheduler.neighbours.remove(&node_id);
            } else {
                // update timer
                if let Some(entry) = scheduler.neighbours.get_mut(&node_id) {
                    entry.timestamp = SystemTime::now();
                    entry.is_first = false;
                }

                // create routing information
                let data = Self::create(node_id.clone(), neighbour_last_sent, neighbour_is_first);

                // create result
                return Some((node_id, module, data));
            }
        }

        None
    }

    /// add new neighbour entry
    pub fn add_neighbour(node_id: PeerId) {
        let exists;
        log::trace!("add new neighbour {:?} to RouterInfo scheduler", node_id);
        // check if a neighbour entry exists
        {
            let scheduler = SCHEDULER.get().read().unwrap();
            exists = scheduler.neighbours.contains_key(&node_id);
        }

        // if it does not exist add it to scheduler
        if !exists {
            let mut scheduler = SCHEDULER.get().write().unwrap();
            let interval = scheduler.interval.clone();
            scheduler.neighbours.insert(
                node_id,
                SchedulerEntry {
                    timestamp: SystemTime::now() - interval,
                    is_first: true,
                },
            );
        }
    }

    /// Create routing information for a neighbour node,
    /// encode the information and return the byte code.
    pub fn create(neighbour: PeerId, last_sent: u64, is_first: bool) -> Vec<u8> {
        let node_id = Node::get_id();

        // create routing table
        let routes = RoutingTable::create_routing_info(neighbour, last_sent);

        // create latest Feed ids table
        let mut feeds = router_net_proto::FeedIdsTable { ids: Vec::new() };

        if is_first == true {
            let ids = Feed::get_latest_message_ids(5);
            for id in ids {
                feeds.ids.push(id.clone());
            }
        }

        // create router info protobuf message
        let timestamp = Timestamp::get_timestamp();
        let router_info = router_net_proto::RouterInfoMessage {
            node: node_id.clone().to_bytes(),
            routes: Some(routes),
            //users: Some(users),
            feeds: Some(feeds),
            timestamp,
        };

        // encode router info message
        let mut buf = Vec::with_capacity(router_info.encoded_len());
        router_info
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // create router info content message
        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            router_info_module: router_net_proto::RouterInfoModule::RouterInfo as i32,
            content: buf,
            time: timestamp,
        };

        // encode content message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        // sign data
        let keys = Node::get_keys();
        let signature = keys.sign(&buf).unwrap();

        // create signed container
        let router_info_container = router_net_proto::RouterInfoContainer {
            signature,
            message: buf,
        };

        // encode container message
        let mut buf = Vec::with_capacity(router_info_container.encoded_len());
        router_info_container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// creating feed request message
    pub fn create_feed_request(ids: &Vec<Vec<u8>>) -> Vec<u8> {
        let node_id = Node::get_id();

        //create latest Feed ids table
        let feeds = router_net_proto::FeedIdsTable { ids: ids.clone() };

        let timestamp = Timestamp::get_timestamp();
        let router_info = router_net_proto::FeedRequestMessage { feeds: Some(feeds) };

        let mut buf = Vec::with_capacity(router_info.encoded_len());
        router_info
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            router_info_module: router_net_proto::RouterInfoModule::FeedRequest as i32,
            content: buf,
            time: timestamp,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

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
        router_info_container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// create_feed_response
    pub fn create_feed_response(messages: &Vec<(Vec<u8>, Vec<u8>, String, u64)>) -> Vec<u8> {
        let node_id = Node::get_id();

        // create latest Feed ids table
        let mut feeds = router_net_proto::FeedResponseTable { messages: vec![] };
        for (message_id, sender_id, content, time) in messages {
            let feed = router_net_proto::FeedMessage {
                message_id: message_id.clone(),
                sender_id: sender_id.clone(),
                content: content.clone(),
                time: *time,
            };
            feeds.messages.push(feed);
        }

        let timestamp = Timestamp::get_timestamp();
        let router_info = router_net_proto::FeedResponseMessage { feeds: Some(feeds) };

        let mut buf = Vec::with_capacity(router_info.encoded_len());
        router_info
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            router_info_module: router_net_proto::RouterInfoModule::FeedResponse as i32,
            content: buf,
            time: timestamp,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

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
        router_info_container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// creating user request message
    pub fn create_user_request(ids: &Vec<Vec<u8>>) -> Vec<u8> {
        let node_id = Node::get_id();

        //create latest Feed ids table
        let users = router_net_proto::UserIdTable { ids: ids.clone() };

        let timestamp = Timestamp::get_timestamp();
        let mut buf = Vec::with_capacity(users.encoded_len());
        users
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            router_info_module: router_net_proto::RouterInfoModule::UserRequest as i32,
            content: buf,
            time: timestamp,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

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
        router_info_container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// create_user_response
    pub fn create_user_response(users: &router_net_proto::UserInfoTable) -> Vec<u8> {
        let node_id = Node::get_id();
        let timestamp = Timestamp::get_timestamp();

        let mut buf = Vec::with_capacity(users.encoded_len());
        users
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        let router_info_proto = router_net_proto::RouterInfoContent {
            id: node_id.to_bytes(),
            router_info_module: router_net_proto::RouterInfoModule::UserResponse as i32,
            content: buf,
            time: timestamp,
        };

        // encode message
        let mut buf = Vec::with_capacity(router_info_proto.encoded_len());
        router_info_proto
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

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
        router_info_container
            .encode(&mut buf)
            .expect("Vec<u8> provides capacity as needed");

        buf
    }

    /// process received qaul_info message
    pub fn received(received: QaulInfoReceived) {
        // decode message to structure
        let decoding_result = router_net_proto::RouterInfoContainer::decode(&received.data[..]);

        match decoding_result {
            Ok(container) => {
                // TODO: check signature
                //let signature = container.signature;message.ids

                // decode message
                let message_result =
                    router_net_proto::RouterInfoContent::decode(&container.message[..]);

                match message_result {
                    Ok(content) => {
                        match router_net_proto::RouterInfoModule::try_from(
                            content.router_info_module,
                        ) {
                            Ok(router_net_proto::RouterInfoModule::RouterInfo) => {
                                let message_info = router_net_proto::RouterInfoMessage::decode(
                                    &content.content[..],
                                );
                                if let Ok(message) = message_info {
                                    // collect users and routes
                                    let messages = message;
                                    //let users = messages.users;
                                    let routes = messages.routes;
                                    let feeds = messages.feeds;

                                    match routes {
                                        Some(router_net_proto::RoutingInfoTable { entry }) => {
                                            //check missed user ids
                                            let mut user_ids: Vec<Vec<u8>> = vec![];
                                            for e in &entry {
                                                user_ids.push(e.user.clone());
                                            }
                                            let missed_users = Users::get_missed_ids(&user_ids);
                                            if missed_users.len() > 0 {
                                                UserRequester::add(
                                                    &received.received_from,
                                                    &missed_users,
                                                );
                                            }

                                            //process routing table
                                            ConnectionTable::process_received_routing_info(
                                                received.received_from,
                                                &entry,
                                            );
                                        }
                                        _ => {}
                                    }
                                    match feeds {
                                        Some(router_net_proto::FeedIdsTable { ids }) => {
                                            let missing_ids = Feed::process_received_feed_ids(&ids);
                                            if missing_ids.len() > 0 {
                                                FeedRequester::add(
                                                    &received.received_from,
                                                    &missing_ids,
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Ok(router_net_proto::RouterInfoModule::FeedRequest) => {
                                let message_info = router_net_proto::FeedRequestMessage::decode(
                                    &content.content[..],
                                );
                                if let Ok(message) = message_info {
                                    match message.feeds {
                                        Some(table) => {
                                            let feeds = Feed::get_messges_by_ids(&table.ids);
                                            if feeds.len() > 0 {
                                                FeedResponser::add(&received.received_from, &feeds);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Ok(router_net_proto::RouterInfoModule::FeedResponse) => {
                                let message_info = router_net_proto::FeedResponseMessage::decode(
                                    &content.content[..],
                                );
                                if let Ok(message) = message_info {
                                    match message.feeds {
                                        Some(table) => {
                                            let mut user_ids: Vec<Vec<u8>> = vec![];
                                            for feed in table.messages {
                                                user_ids.push(QaulId::bytes_to_q8id(
                                                    feed.sender_id.clone(),
                                                ));
                                                Feed::save_message_by_sync(
                                                    &feed.message_id,
                                                    &feed.sender_id,
                                                    feed.content,
                                                    feed.time,
                                                );
                                            }
                                            // check missed users
                                            let missed_users = Users::get_missed_ids(&user_ids);
                                            if missed_users.len() > 0 {
                                                UserRequester::add(
                                                    &received.received_from,
                                                    &missed_users,
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Ok(router_net_proto::RouterInfoModule::UserRequest) => {
                                let message_info =
                                    router_net_proto::UserIdTable::decode(&content.content[..]);
                                if let Ok(message) = message_info {
                                    let table = Users::get_user_info_table_by_q8ids(&message.ids);
                                    UserResponser::add(&received.received_from, &table);
                                }
                            }
                            Ok(router_net_proto::RouterInfoModule::UserResponse) => {
                                let message_info =
                                    router_net_proto::UserInfoTable::decode(&content.content[..]);
                                if let Ok(message) = message_info {
                                    Users::add_user_info_table(&message.info);
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(msg) => {
                        log::error!("RouterInfoContent decode {:?}", msg);
                    }
                }
            }
            Err(msg) => {
                log::error!("RouterInfoContainer decode {:?}", msg);
            }
        }
    }
}
