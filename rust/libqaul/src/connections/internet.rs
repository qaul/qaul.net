// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
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
//!     name: qaul Community Node
//!     enabled: false
//!   do_listen: false
//!   listen:
//!   - /ip4/0.0.0.0/udp/9229/quic-v1
//!   - /ip4/0.0.0.0/tcp/9229
//!   - /ip6/::/udp/9229/quic-v1
//!   - /ip6/::/tcp/9229
//! ```

use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    identify,
    identity::Keypair,
    noise, ping,
    swarm::{NetworkBehaviour, Swarm},
    tcp, yamux, Multiaddr, PeerId, SwarmBuilder,
};
use prost::Message;
use state::InitCell;
use std::time::Duration;
use std::{
    collections::{BTreeMap, HashMap},
    sync::RwLock,
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
    pub floodsub: Floodsub,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub qaul_info: QaulInfo,
    pub qaul_messaging: QaulMessaging,
}

impl QaulInternetBehaviour {
    pub fn process_events(&mut self, event: QaulInternetEvent) {
        match event {
            QaulInternetEvent::QaulInfo(ev) => {
                self.qaul_info_event(ev);
            }
            QaulInternetEvent::QaulMessaging(ev) => {
                self.qaul_messaging_event(ev);
            }
            QaulInternetEvent::Ping(ev) => {
                self.ping_event(ev);
            }
            QaulInternetEvent::Identify(ev) => {
                self.identify_event(ev);
            }
            QaulInternetEvent::Floodsub(ev) => {
                self.floodsub_event(ev);
            }
        }
    }

    fn qaul_info_event(&mut self, event: QaulInfoEvent) {
        events::qaul_info_event(event, ConnectionModule::Internet);
    }
    fn qaul_messaging_event(&mut self, event: QaulMessagingEvent) {
        events::qaul_messaging_event(event, ConnectionModule::Internet);
    }
    fn ping_event(&mut self, event: ping::Event) {
        events::ping_event(event, ConnectionModule::Internet);
    }

    fn identify_event(&mut self, event: identify::Event) {
        match event {
            identify::Event::Received { peer_id, info } => {
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
            }
            identify::Event::Sent { peer_id } => {
                log::trace!("IdentifyEvent::Sent to {:?}", peer_id);
            }
            identify::Event::Pushed { peer_id, info: _ } => {
                log::trace!("IdentifyEvent::Pushed {:?}", peer_id);
            }
            identify::Event::Error { peer_id, error } => {
                log::trace!("IdentifyEvent::Error {:?} {:?}", peer_id, error);
            }
        }
    }

    fn floodsub_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                // feed Message
                if let Ok(resp) = proto_net::FeedContainer::decode(&msg.data[..]) {
                    Feed::received(ConnectionModule::Internet, msg.source, resp);
                }
            }
            _ => (),
        }
    }
}

pub struct InternetReConnection {
    pub address: Multiaddr,
    pub attempt: u32,
    pub last_try: u64,
}
pub struct InternetReConnections {
    peers: HashMap<Multiaddr, InternetReConnection>,
}
static INTERNETRECONNECTIONS: InitCell<RwLock<InternetReConnections>> = InitCell::new();
static INTERNETCONNECTIONS: InitCell<RwLock<BTreeMap<String, PeerId>>> = InitCell::new();

#[derive(Debug)]
pub enum QaulInternetEvent {
    Floodsub(FloodsubEvent),
    Identify(identify::Event),
    Ping(ping::Event),
    QaulInfo(QaulInfoEvent),
    QaulMessaging(QaulMessagingEvent),
}

impl From<FloodsubEvent> for QaulInternetEvent {
    fn from(event: FloodsubEvent) -> Self {
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
}

impl Internet {
    /// Initialize swarm for Internet overlay connection module
    pub async fn init(node_keys: &Keypair) -> Self {
        log::trace!("Internet.init() start");

        INTERNETRECONNECTIONS.set(RwLock::new(InternetReConnections {
            peers: HashMap::new(),
        }));
        INTERNETCONNECTIONS.set(RwLock::new(BTreeMap::<String, PeerId>::new()));

        // create ping configuration
        let mut ping_config = ping::Config::new();

        let config = Configuration::get();
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Internet.init() ping_config");

        // create behaviour
        let mut behaviour: QaulInternetBehaviour = QaulInternetBehaviour {
            floodsub: Floodsub::new(Node::get_id()),
            identify: identify::Behaviour::new(identify::Config::new(
                "/ipfs/0.1.0".into(),
                Node::get_keys().public(),
            )),
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
        let config = Configuration::get();

        for listen in &config.internet.listen {
            match Swarm::listen_on(&mut swarm, listen.parse().expect("can get a local socket")) {
                Ok(listener_id) => {
                    log::info!(
                        "INTERNET listening on `{}` with ID {:?}",
                        listen,
                        listener_id
                    );
                }
                Err(e) => {
                    log::error!("Error INTERNET start listening on `{}`: {}", listen, e);
                }
            }
        }

        // connect to remote peers that are specified in
        // the configuration config.internet.peers
        Self::peer_connect(&config, &mut swarm);

        log::trace!("Internet.init() peer_connect");

        // construct internet object
        let internet = Internet { swarm };

        internet
    }

    // check if connection is active
    pub fn is_active_connection(address: &Multiaddr) -> bool {
        let config = Configuration::get();
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
    pub fn set_redialed(addresse: &Multiaddr) {
        let mut reconnections = INTERNETRECONNECTIONS.get().write().unwrap();
        if let Some(peer) = reconnections.peers.get_mut(addresse) {
            peer.last_try = Timestamp::get_timestamp();
        }
    }

    /// redial a remote peer
    pub async fn peer_redial(addresse: &Multiaddr, swarm: &mut Swarm<QaulInternetBehaviour>) {
        Self::peer_dial(addresse.clone(), swarm);
    }

    /// add connection entry
    pub fn add_connection(address: String, peer_id: &PeerId) {
        let mut connections = INTERNETCONNECTIONS.get().write().unwrap();
        connections.insert(address.clone(), peer_id.clone());
    }

    /// peerid from mutiaddr uri
    pub fn peerid_from_address(address: String) -> Option<PeerId> {
        let connections = INTERNETCONNECTIONS.get().read().unwrap();
        if let Some(v) = connections.get(&address) {
            return Some(v.clone());
        }
        return None;
    }

    /// add reconnection
    pub fn add_reconnection(address: Multiaddr) {
        let mut reconnections = INTERNETRECONNECTIONS.get().write().unwrap();
        if let Some(peer) = reconnections.peers.get_mut(&address) {
            peer.last_try = Timestamp::get_timestamp();
        } else {
            reconnections.peers.insert(
                address.clone(),
                InternetReConnection {
                    address: address.clone(),
                    attempt: 0,
                    last_try: Timestamp::get_timestamp(),
                },
            );
        }
    }

    pub fn remove_reconnection(address: Multiaddr) {
        let mut reconnections = INTERNETRECONNECTIONS.get().write().unwrap();
        reconnections.peers.remove(&address);
    }

    /// check redial
    pub fn check_reconnection() -> Option<Multiaddr> {
        let reconnections = INTERNETRECONNECTIONS.get().read().unwrap();
        let now_ts = Timestamp::get_timestamp();
        for (addr, peer) in reconnections.peers.iter() {
            if (now_ts - peer.last_try) > 10000 {
                return Some(addr.clone());
            }
        }
        None
    }
}
