// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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
    core::transport::ListenerId,
    floodsub,
    identity::Keypair,
    mdns, noise, ping,
    swarm::{NetworkBehaviour, Swarm},
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use prost::Message;
use std::time::Duration;

use crate::connections::transport::{
    Transport, TransportCapabilities, TransportError, TransportStatus,
};
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
    pub floodsub: floodsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
    pub qaul_info: QaulInfo,
    pub qaul_messaging: QaulMessaging,
}

impl QaulLanBehaviour {
    pub fn process_events(&mut self, state: &crate::QaulState, event: QaulLanEvent) {
        match event {
            QaulLanEvent::QaulInfo(ev) => {
                self.qaul_info_event(state, ev);
            }
            QaulLanEvent::QaulMessaging(ev) => {
                self.qaul_messaging_event(state, ev);
            }
            QaulLanEvent::Ping(ev) => {
                self.ping_event(state, ev);
            }
            QaulLanEvent::Mdns(ev) => {
                self.mdsn_event(ev);
            }
            QaulLanEvent::Floodsub(ev) => {
                self.floodsub_event(state, ev);
            }
        }
    }
    fn qaul_info_event(&mut self, state: &crate::QaulState, event: QaulInfoEvent) {
        events::qaul_info_event(state, event, ConnectionModule::Lan);
    }
    fn qaul_messaging_event(&mut self, state: &crate::QaulState, event: QaulMessagingEvent) {
        events::qaul_messaging_event(state, event, ConnectionModule::Lan);
    }
    fn ping_event(&mut self, state: &crate::QaulState, event: ping::Event) {
        events::ping_event(state, event, ConnectionModule::Lan);
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

    fn floodsub_event(&mut self, state: &crate::QaulState, event: floodsub::Event) {
        match event {
            floodsub::Event::Message(msg) => {
                // feed Message
                if let Ok(resp) = proto_net::FeedContainer::decode(&msg.data[..]) {
                    Feed::received(state, ConnectionModule::Lan, msg.source, resp);
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum QaulLanEvent {
    Floodsub(floodsub::Event),
    Mdns(mdns::Event),
    Ping(ping::Event),
    QaulInfo(QaulInfoEvent),
    QaulMessaging(QaulMessagingEvent),
}

impl From<floodsub::Event> for QaulLanEvent {
    fn from(event: floodsub::Event) -> Self {
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
    status: TransportStatus,
    listener_ids: Vec<ListenerId>,
}

impl Transport for Lan {
    fn id(&self) -> &'static str {
        "lan"
    }

    fn label(&self) -> &'static str {
        "LAN"
    }

    fn module(&self) -> ConnectionModule {
        ConnectionModule::Lan
    }

    fn capabilities(&self) -> TransportCapabilities {
        TransportCapabilities {
            supports_runtime_toggle: true,
            supports_peer_list: false,
            is_local_only: true,
        }
    }

    fn status(&self) -> &TransportStatus {
        &self.status
    }

    fn stop(&mut self, state: &crate::QaulState) -> Result<(), TransportError> {
        if self.status == TransportStatus::Disabled {
            return Ok(());
        }

        for id in self.listener_ids.drain(..) {
            self.swarm.remove_listener(id);
        }

        let peers: Vec<PeerId> = self.swarm.connected_peers().cloned().collect();
        for peer in peers {
            let _ = self.swarm.disconnect_peer_id(peer);
        }

        // persist to config
        {
            let mut config = Configuration::get_mut(state);
            config.lan.active = false;
        }
        Configuration::save(state);

        self.status = TransportStatus::Disabled;
        log::info!("LAN transport stopped");
        Ok(())
    }

    fn start(&mut self, state: &crate::QaulState) -> Result<(), TransportError> {
        if self.status == TransportStatus::Running {
            return Ok(());
        }

        let config = Configuration::get(state);
        for listen in &config.lan.listen {
            match listen.parse() {
                Ok(addr) => match Swarm::listen_on(&mut self.swarm, addr) {
                    Ok(id) => {
                        self.listener_ids.push(id);
                    }
                    Err(e) => {
                        log::error!("LAN start: failed to listen on {}: {}", listen, e);
                    }
                },
                Err(e) => {
                    log::error!("LAN start: invalid address {}: {}", listen, e);
                }
            }
        }
        drop(config);

        // persist to config
        {
            let mut config = Configuration::get_mut(state);
            config.lan.active = true;
        }
        Configuration::save(state);

        self.status = TransportStatus::Running;
        log::info!("LAN transport started");
        Ok(())
    }

    fn send_qaul_info_message(&mut self, _state: &crate::QaulState, peer_id: PeerId, data: Vec<u8>) {
        if !self.is_enabled() {
            return;
        }
        self.swarm
            .behaviour_mut()
            .qaul_info
            .send_qaul_info_message(peer_id, data);
    }

    fn send_qaul_messaging_message(&mut self, _state: &crate::QaulState, peer_id: PeerId, data: Vec<u8>) {
        if !self.is_enabled() {
            return;
        }
        self.swarm
            .behaviour_mut()
            .qaul_messaging
            .send_qaul_messaging_message(peer_id, data);
    }

    fn publish_floodsub(&mut self, _state: &crate::QaulState, topic: floodsub::Topic, data: Vec<u8>) {
        if !self.is_enabled() {
            return;
        }
        self.swarm
            .behaviour_mut()
            .floodsub
            .publish(topic, data);
    }

    fn listeners(&self) -> Vec<Multiaddr> {
        self.swarm.listeners().cloned().collect()
    }

    fn external_addresses(&self) -> Vec<Multiaddr> {
        self.swarm.external_addresses().cloned().collect()
    }
}

impl Lan {
    /// Initialize swarm for LAN connection module
    pub async fn init(state: &crate::QaulState, node_keys: &Keypair) -> Lan {
        log::trace!("Lan::init() start");

        // create ping configuration
        let mut ping_config = ping::Config::new();

        let config = Configuration::get(state);
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Lan::init() ping_config");

        // create MDNS behaviour
        let mdns_config = mdns::Config {
            ttl: Duration::from_secs(300),
            query_interval: Duration::from_secs(30),
            enable_ipv6: false,
        };
        let mdns = mdns::tokio::Behaviour::new(mdns_config, Node::get_id(state)).unwrap();

        // create behaviour
        let mut behaviour: QaulLanBehaviour = QaulLanBehaviour {
            floodsub: floodsub::Behaviour::new(Node::get_id(state)),
            mdns,
            ping: ping::Behaviour::new(ping_config),
            qaul_info: QaulInfo::new(Node::get_id(state)),
            qaul_messaging: QaulMessaging::new(Node::get_id(state)),
        };
        behaviour.floodsub.subscribe(Node::get_topic(state));

        let mut swarm = SwarmBuilder::with_existing_identity(node_keys.to_owned())
            .with_tokio()
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
        let config = Configuration::get(state);
        let active = config.lan.active;

        // only listen if transport is configured as active
        let mut listener_ids = Vec::new();
        if active {
            for listen in &config.lan.listen {
                let id =
                    Swarm::listen_on(&mut swarm, listen.parse().expect("can get a local socket"))
                        .expect("swarm can be started");
                listener_ids.push(id);
            }
        } else {
            log::info!("LAN transport disabled by configuration");
        }

        Lan {
            swarm,
            status: if active {
                TransportStatus::Running
            } else {
                TransportStatus::Disabled
            },
            listener_ids,
        }
    }
}
