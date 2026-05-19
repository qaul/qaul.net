// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Internet Overlay Connection Module
//!
//! **Statically connect to nodes in the Internet.**
//!
//! This module connects to predefined nodes in the internet.
//! The addresses of the peers to connect to are read from
//! the config file:
//!
//! ```yaml
//! internet:
//!   active: true
//!   peers:
//!   - address: /ip4/144.91.74.192/udp/9229/quic-v1
//!     name: qaul Community Node [IPv4]
//!     enabled: false
//!   - address: /ip6/2a02:c207:3004:3887::1/udp/9229/quic-v1
//!     name: qaul Community Node [IPv6]
//!     enabled: false
//!   do_listen: false
//!   listen:
//!   - /ip4/0.0.0.0/udp/9229/quic-v1
//!   - /ip4/0.0.0.0/tcp/9229
//!   - /ip6/::/udp/9229/quic-v1
//!   - /ip6/::/tcp/9229
//! ```

use libp2p::{
    core::transport::ListenerId,
    floodsub, identify,
    identity::Keypair,
    noise, ping,
    swarm::{NetworkBehaviour, Swarm},
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use prost::Message;
use std::time::Duration;
use std::{
    collections::{BTreeMap, HashMap},
    sync::RwLock,
};

use crate::connections::transport::{
    Transport, TransportCapabilities, TransportError, TransportStatus,
};
use crate::connections::{events, ConnectionModule};
use crate::node::Node;
use crate::services::feed::proto_net;
use crate::services::feed::Feed;
use crate::storage::configuration::Configuration;
use crate::utilities::timestamp::Timestamp;
use qaul_info::{QaulInfo, QaulInfoEvent};
use qaul_messaging::{QaulMessaging, QaulMessagingEvent};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "QaulInternetEvent")]
pub struct QaulInternetBehaviour {
    pub floodsub: floodsub::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub qaul_info: QaulInfo,
    pub qaul_messaging: QaulMessaging,
}

impl QaulInternetBehaviour {
    pub fn process_events(&mut self, state: &crate::QaulState, event: QaulInternetEvent) {
        match event {
            QaulInternetEvent::QaulInfo(ev) => {
                self.qaul_info_event(state, ev);
            }
            QaulInternetEvent::QaulMessaging(ev) => {
                self.qaul_messaging_event(state, ev);
            }
            QaulInternetEvent::Ping(ev) => {
                self.ping_event(state, ev);
            }
            QaulInternetEvent::Identify(ev) => {
                self.identify_event(ev);
            }
            QaulInternetEvent::Floodsub(ev) => {
                self.floodsub_event(state, ev);
            }
        }
    }

    fn qaul_info_event(&mut self, state: &crate::QaulState, event: QaulInfoEvent) {
        events::qaul_info_event(state, event, ConnectionModule::Internet);
    }
    fn qaul_messaging_event(&mut self, state: &crate::QaulState, event: QaulMessagingEvent) {
        events::qaul_messaging_event(state, event, ConnectionModule::Internet);
    }
    fn ping_event(&mut self, state: &crate::QaulState, event: ping::Event) {
        events::ping_event(state, event, ConnectionModule::Internet);
    }

    fn identify_event(&mut self, event: identify::Event) {
        match event {
            identify::Event::Received {
                peer_id,
                info,
                connection_id,
            } => {
                // add node to floodsub
                self.floodsub.add_node_to_partial_view(peer_id);

                // print received information
                log::trace!("IdentifyEvent::Received from {:?}", peer_id);
                log::trace!("  added peer_id {:?} to floodsub", peer_id);
                log::trace!("  public key: {:?}", info.public_key);
                log::trace!("  protocol version: {:?}", info.protocol_version);
                log::trace!("  agent version: {:?}", info.agent_version);
                log::trace!("  listen addresses: {:?}", info.listen_addrs);
                log::trace!("  protocols: {:?}", info.protocols);
                log::trace!("  observed address: {:?}", info.observed_addr);
                log::trace!("  connection ID: {:?}", connection_id);
            }
            identify::Event::Sent {
                peer_id,
                connection_id,
            } => {
                log::trace!(
                    "IdentifyEvent::Sent to {:?} via connection ID {:?}",
                    peer_id,
                    connection_id
                );
            }
            identify::Event::Pushed {
                peer_id,
                info: _,
                connection_id,
            } => {
                log::trace!(
                    "IdentifyEvent::Pushed {:?} via connection ID {:?}",
                    peer_id,
                    connection_id
                );
            }
            identify::Event::Error {
                peer_id,
                error,
                connection_id,
            } => {
                log::trace!(
                    "IdentifyEvent::Error {:?} {:?} {:?}",
                    peer_id,
                    connection_id,
                    error
                );
            }
        }
    }

    fn floodsub_event(&mut self, state: &crate::QaulState, event: floodsub::Event) {
        match event {
            floodsub::Event::Message(msg) => {
                // feed Message
                if let Ok(resp) = proto_net::FeedContainer::decode(&msg.data[..]) {
                    Feed::received(state, ConnectionModule::Internet, msg.source, resp);
                }
            }
            _ => (),
        }
    }
}

pub struct InternetReConnection {
    #[allow(dead_code)]
    pub address: Multiaddr,
    #[allow(dead_code)]
    pub attempt: u32,
    pub last_try: u64,
}
pub struct InternetReConnections {
    peers: HashMap<Multiaddr, InternetReConnection>,
}

impl InternetReConnections {
    /// Add or update a reconnection entry (shared inner logic).
    fn add_reconnection_inner(&mut self, address: Multiaddr) {
        if let Some(peer) = self.peers.get_mut(&address) {
            peer.last_try = Timestamp::get_timestamp();
        } else {
            self.peers.insert(
                address.clone(),
                InternetReConnection {
                    address: address.clone(),
                    attempt: 0,
                    last_try: Timestamp::get_timestamp(),
                },
            );
        }
    }

    /// Remove a reconnection entry (shared inner logic).
    fn remove_reconnection_inner(&mut self, address: Multiaddr) {
        self.peers.remove(&address);
    }

    /// Update last_try timestamp for a reconnection entry (shared inner logic).
    fn set_redialed_inner(&mut self, addresse: &Multiaddr) {
        if let Some(peer) = self.peers.get_mut(addresse) {
            peer.last_try = Timestamp::get_timestamp();
        }
    }

    /// Check if any reconnection is due (shared inner logic).
    fn check_reconnection_inner(&self) -> Option<Multiaddr> {
        let now_ts = Timestamp::get_timestamp();
        for (addr, peer) in self.peers.iter() {
            if (now_ts - peer.last_try) > 10000 {
                return Some(addr.clone());
            }
        }
        None
    }
}

/// Add a connection entry to the connections map (shared inner logic).
fn add_connection_impl(connections: &mut BTreeMap<String, PeerId>, address: String, peer_id: &PeerId) {
    connections.insert(address, peer_id.clone());
}

/// Get PeerId from address in the connections map (shared inner logic).
fn peerid_from_address_impl(connections: &BTreeMap<String, PeerId>, address: String) -> Option<PeerId> {
    connections.get(&address).cloned()
}

/// Instance-based internet connections state.
/// Replaces the global INTERNETRECONNECTIONS and INTERNETCONNECTIONS statics
/// for multi-instance use.
pub struct InternetState {
    /// Reconnection tracking.
    pub reconnections: RwLock<InternetReConnections>,
    /// Active connections mapping (address string -> PeerId).
    pub connections: RwLock<BTreeMap<String, PeerId>>,
}

impl InternetState {
    /// Create a new empty InternetState.
    pub fn new() -> Self {
        Self {
            reconnections: RwLock::new(InternetReConnections {
                peers: HashMap::new(),
            }),
            connections: RwLock::new(BTreeMap::new()),
        }
    }

    /// Add a connection entry (instance method).
    pub fn add_connection(&self, address: String, peer_id: &PeerId) {
        let mut connections = self.connections.write().unwrap();
        add_connection_impl(&mut connections, address, peer_id);
    }

    /// Get PeerId from address (instance method).
    pub fn peerid_from_address(&self, address: String) -> Option<PeerId> {
        let connections = self.connections.read().unwrap();
        peerid_from_address_impl(&connections, address)
    }

    /// Add reconnection entry (instance method).
    pub fn add_reconnection(&self, address: Multiaddr) {
        let mut reconnections = self.reconnections.write().unwrap();
        reconnections.add_reconnection_inner(address);
    }

    /// Remove reconnection entry (instance method).
    pub fn remove_reconnection(&self, address: Multiaddr) {
        let mut reconnections = self.reconnections.write().unwrap();
        reconnections.remove_reconnection_inner(address);
    }

    /// Set redialed timestamp (instance method).
    pub fn set_redialed(&self, addresse: &Multiaddr) {
        let mut reconnections = self.reconnections.write().unwrap();
        reconnections.set_redialed_inner(addresse);
    }

    /// Check reconnection (instance method).
    pub fn check_reconnection(&self) -> Option<Multiaddr> {
        let reconnections = self.reconnections.read().unwrap();
        reconnections.check_reconnection_inner()
    }
}

#[derive(Debug)]
pub enum QaulInternetEvent {
    Floodsub(floodsub::Event),
    Identify(identify::Event),
    Ping(ping::Event),
    QaulInfo(QaulInfoEvent),
    QaulMessaging(QaulMessagingEvent),
}

impl From<floodsub::Event> for QaulInternetEvent {
    fn from(event: floodsub::Event) -> Self {
        Self::Floodsub(event)
    }
}

impl From<identify::Event> for QaulInternetEvent {
    fn from(event: identify::Event) -> Self {
        Self::Identify(event)
    }
}

impl From<ping::Event> for QaulInternetEvent {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}

impl From<QaulInfoEvent> for QaulInternetEvent {
    fn from(event: QaulInfoEvent) -> Self {
        Self::QaulInfo(event)
    }
}

impl From<QaulMessagingEvent> for QaulInternetEvent {
    fn from(event: QaulMessagingEvent) -> Self {
        Self::QaulMessaging(event)
    }
}

/// Internet Connection Module of libqaul
///
/// it creates a libp2p swarm
pub struct Internet {
    pub swarm: Swarm<QaulInternetBehaviour>,
    status: TransportStatus,
    listener_ids: Vec<ListenerId>,
}

impl Transport for Internet {
    fn id(&self) -> &'static str {
        "internet"
    }

    fn label(&self) -> &'static str {
        "Internet"
    }

    fn module(&self) -> ConnectionModule {
        ConnectionModule::Internet
    }

    fn capabilities(&self) -> TransportCapabilities {
        TransportCapabilities {
            supports_runtime_toggle: true,
            supports_peer_list: true,
            is_local_only: false,
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

        {
            let mut config = Configuration::get_mut(state);
            config.internet.active = false;
        }
        Configuration::save(state);

        self.status = TransportStatus::Disabled;
        log::info!("Internet transport stopped");
        Ok(())
    }

    fn start(&mut self, state: &crate::QaulState) -> Result<(), TransportError> {
        if self.status == TransportStatus::Running {
            return Ok(());
        }

        let config = Configuration::get(state);

        if config.internet.do_listen {
            for listen in &config.internet.listen {
                match listen.parse() {
                    Ok(addr) => match Swarm::listen_on(&mut self.swarm, addr) {
                        Ok(id) => {
                            self.listener_ids.push(id);
                        }
                        Err(e) => {
                            log::error!("Internet start: failed to listen on {}: {}", listen, e);
                        }
                    },
                    Err(e) => {
                        log::error!("Internet start: invalid address {}: {}", listen, e);
                    }
                }
            }
        }

        Self::peer_connect(&config, &mut self.swarm);
        drop(config);

        {
            let mut config = Configuration::get_mut(state);
            config.internet.active = true;
        }
        Configuration::save(state);

        self.status = TransportStatus::Running;
        log::info!("Internet transport started");
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

impl Internet {
    /// Initialize swarm for Internet overlay connection module
    pub async fn init(state: &crate::QaulState, node_keys: &Keypair) -> Self {
        log::trace!("Internet.init() start");

        // Internet state is managed by state.connections.internet.

        // create ping configuration
        let mut ping_config = ping::Config::new();

        let config = Configuration::get(state);
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Internet.init() ping_config");

        // create behaviour
        let mut behaviour: QaulInternetBehaviour = QaulInternetBehaviour {
            floodsub: floodsub::Behaviour::new(Node::get_id(state)),
            identify: identify::Behaviour::new(identify::Config::new(
                "/ipfs/0.1.0".into(),
                Node::get_keys(state).public(),
            )),
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
                log::trace!("internal INTERNET node ID: {:?}", key.public().to_peer_id());
                Ok(behaviour)
            })
            .unwrap()
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        log::trace!("Internet.init() swarm created");

        // connect swarm to the listening interfaces defined in
        // the configuration array config.internet.listen
        let config = Configuration::get(state);
        let active = config.internet.active;

        let mut listener_ids = Vec::new();
        if active {
            // listen on configured addresses
            for listen in &config.internet.listen {
                match Swarm::listen_on(
                    &mut swarm,
                    listen.parse().expect("can get a local socket"),
                ) {
                    Ok(listener_id) => {
                        log::info!(
                            "INTERNET listening on `{}` with ID {:?}",
                            listen,
                            listener_id
                        );
                        listener_ids.push(listener_id);
                    }
                    Err(e) => {
                        log::error!("Error INTERNET start listening on `{}`: {}", listen, e);
                    }
                }
            }

            // connect to remote peers
            Self::peer_connect(&config, &mut swarm);
            log::trace!("Internet.init() peer_connect");
        } else {
            log::info!("Internet transport disabled by configuration");
        }

        Internet {
            swarm,
            status: if active {
                TransportStatus::Running
            } else {
                TransportStatus::Disabled
            },
            listener_ids,
        }
    }

    // check if connection is active
    pub fn is_active_connection(state: &crate::QaulState, address: &Multiaddr) -> bool {
        let config = Configuration::get(state);
        let address_str = address.to_string();
        for peer in &config.internet.peers {
            if address_str == peer.address {
                return peer.enabled;
            }
        }
        return false;
    }

    /// connect to remote peers that are specified in
    /// the configuration config.internet.peers
    pub fn peer_connect(config: &Configuration, swarm: &mut Swarm<QaulInternetBehaviour>) {
        for peer in &config.internet.peers {
            if peer.enabled {
                match peer.address.clone().parse() {
                    Ok(addresse) => Self::peer_dial(addresse, swarm),
                    Err(error) => log::trace!(
                        "peer address {} parse error: {:?}",
                        peer.address.clone(),
                        error
                    ),
                }
            }
        }
    }

    /// dial a remote peer
    pub fn peer_dial(addresse: Multiaddr, swarm: &mut Swarm<QaulInternetBehaviour>) {
        match swarm.dial(addresse.clone()) {
            Ok(_) => log::trace!("peer {:?} dialed", addresse),
            Err(error) => log::trace!("peer {} swarm dial error: {:?}", addresse, error),
        }
    }

    /// set tried time
    pub fn set_redialed(state: &crate::QaulState, addresse: &Multiaddr) {
        state.connections.internet.set_redialed(addresse);
    }

    /// redial a remote peer
    pub async fn peer_redial(addresse: &Multiaddr, swarm: &mut Swarm<QaulInternetBehaviour>) {
        Self::peer_dial(addresse.clone(), swarm);
    }

    /// add connection entry
    pub fn add_connection(state: &crate::QaulState, address: String, peer_id: &PeerId) {
        state.connections.internet.add_connection(address, peer_id);
    }

    /// peerid from multi-address uri
    pub fn peerid_from_address(state: &crate::QaulState, address: String) -> Option<PeerId> {
        state.connections.internet.peerid_from_address(address)
    }

    /// add reconnection
    pub fn add_reconnection(state: &crate::QaulState, address: Multiaddr) {
        state.connections.internet.add_reconnection(address);
    }

    pub fn remove_reconnection(state: &crate::QaulState, address: Multiaddr) {
        state.connections.internet.remove_reconnection(address);
    }

    /// check redial
    pub fn check_reconnection(state: &crate::QaulState) -> Option<Multiaddr> {
        state.connections.internet.check_reconnection()
    }
}
