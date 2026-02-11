// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # LAN Connection Module
//!
//! **Discover other qaul nodes on the local LAN and connect to them.**
//!
//! This module advertises the node via mdns in the local network.
//! By default it listens to all interfaces and connects to a random port.
//!
//! The module is configured in the configuration file:
//!
//! ```yaml
//! lan:
//!   active: true
//!   listen:
//!   - /ip4/0.0.0.0/udp/0/quic-v1
//!   - /ip4/0.0.0.0/tcp/0
//!   - /ip6/::/udp/0/quic-v1
//!   - /ip6/::/tcp/0
//! ```

use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    identity::Keypair,
    mdns, noise, ping,
    swarm::{NetworkBehaviour, Swarm},
    tcp, yamux, SwarmBuilder,
};
use prost::Message;
use std::time::Duration;

use crate::connections::{events, ConnectionModule};
use crate::node::Node;
use crate::services::feed::proto_net;
use crate::services::feed::Feed;
use crate::storage::configuration::Configuration;
use qaul_info::{QaulInfo, QaulInfoEvent};
use qaul_messaging::{QaulMessaging, QaulMessagingEvent};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "QaulLanEvent")]
pub struct QaulLanBehaviour {
    pub floodsub: Floodsub,
    pub mdns: mdns::async_io::Behaviour,
    pub ping: ping::Behaviour,
    pub qaul_info: QaulInfo,
    pub qaul_messaging: QaulMessaging,
}

impl QaulLanBehaviour {
    pub fn process_events(&mut self, event: QaulLanEvent) {
        match event {
            QaulLanEvent::QaulInfo(ev) => {
                self.qaul_info_event(ev);
            }
            QaulLanEvent::QaulMessaging(ev) => {
                self.qaul_messaging_event(ev);
            }
            QaulLanEvent::Ping(ev) => {
                self.ping_event(ev);
            }
            QaulLanEvent::Mdns(ev) => {
                self.mdsn_event(ev);
            }
            QaulLanEvent::Floodsub(ev) => {
                self.floodsub_event(ev);
            }
        }
    }
    fn qaul_info_event(&mut self, event: QaulInfoEvent) {
        events::qaul_info_event(event, ConnectionModule::Lan);
    }
    fn qaul_messaging_event(&mut self, event: QaulMessagingEvent) {
        events::qaul_messaging_event(event, ConnectionModule::Lan);
    }
    fn ping_event(&mut self, event: ping::Event) {
        events::ping_event(event, ConnectionModule::Lan);
    }
    fn mdsn_event(&mut self, event: mdns::Event) {
        match event {
            mdns::Event::Discovered(discovered_list) => {
                for (peer_id, _addr) in discovered_list {
                    log::trace!(
                        "MdnsEvent::Discovered, peer {:?} to floodsub added",
                        peer_id.clone()
                    );
                    self.floodsub.add_node_to_partial_view(peer_id);
                }
            }
            mdns::Event::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    // TODO: why to remove it at all? does it not get removed automatically, when disconnected?
                    //
                    // Why should we check again if a node is in discovered list, once it has expired?
                    // Apparently the expired node is not instantly removed from the discovered_nodes list too.
                    if !self.mdns.discovered_nodes().any(|p| p == &peer) {
                        log::trace!("MdnsEvent::Expired, peer {:?} from floodsub removed", peer);
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }

    fn floodsub_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                // feed Message
                if let Ok(resp) = proto_net::FeedContainer::decode(&msg.data[..]) {
                    Feed::received(ConnectionModule::Lan, msg.source, resp);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum QaulLanEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
    QaulInfo(QaulInfoEvent),
    QaulMessaging(QaulMessagingEvent),
}

impl From<FloodsubEvent> for QaulLanEvent {
    fn from(event: FloodsubEvent) -> Self {
        Self::Floodsub(event)
    }
}

impl From<mdns::Event> for QaulLanEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}

impl From<ping::Event> for QaulLanEvent {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}

impl From<QaulInfoEvent> for QaulLanEvent {
    fn from(event: QaulInfoEvent) -> Self {
        Self::QaulInfo(event)
    }
}

impl From<QaulMessagingEvent> for QaulLanEvent {
    fn from(event: QaulMessagingEvent) -> Self {
        Self::QaulMessaging(event)
    }
}

pub struct Lan {
    pub swarm: Swarm<QaulLanBehaviour>,
}

impl Lan {
    /// Initialize swarm for LAN connection module
    pub async fn init(node_keys: &Keypair) -> Lan {
        log::trace!("Lan::init() start");

        // create ping configuration
        let mut ping_config = ping::Config::new();

        let config = Configuration::get();
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Lan::init() ping_config");

        // create MDNS behaviour
        // TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), Node::get_id()).unwrap();

        // create behaviour
        let mut behaviour: QaulLanBehaviour = QaulLanBehaviour {
            floodsub: Floodsub::new(Node::get_id()),
            mdns,
            ping: ping::Behaviour::new(ping_config),
            qaul_info: QaulInfo::new(Node::get_id()),
            qaul_messaging: QaulMessaging::new(Node::get_id()),
        };
        behaviour.floodsub.subscribe(Node::get_topic());

        let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
            .with_async_std()
            .with_tcp(
                tcp::Config::new().nodelay(true),
                noise::Config::new,
                yamux::Config::default,
            )
            .unwrap()
            .with_quic()
            .with_behaviour(|key| {
                log::trace!("internal LAN node ID: {:?}", key.public().to_peer_id());
                Ok(behaviour)
            })
            .unwrap()
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        log::trace!("Lan::init() swarm created");

        // connect swarm to the defined listening interfaces in
        // the configuration array config.lan.listen
        let config = Configuration::get();

        for listen in &config.lan.listen {
            Swarm::listen_on(&mut swarm, listen.parse().expect("can get a local socket"))
                .expect("swarm can be started");
        }

        let lan = Lan { swarm };

        lan
    }
}
