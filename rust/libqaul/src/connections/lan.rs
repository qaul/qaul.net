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
//!   - /ip4/0.0.0.0/tcp/0
//!   - /ip6/::/tcp/0
//! ```

use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent},
    mdns,
    mdns::{async_io::Behaviour as Mdns, Config},
    noise::Config as NoiseConfig,
    ping,
    swarm::{keep_alive, NetworkBehaviour, Swarm},
    tcp::{async_io::Transport as TcpTransport, Config as GenTcpConfig},
    yamux,
};
use prost::Message;
use std::time::Duration;

// DNS is excluded on mobile, as it is not working there
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use libp2p::dns::DnsConfig;
use libp2p::Transport;

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
    pub mdns: Mdns,
    pub keep_alive: keep_alive::Behaviour,
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
            QaulLanEvent::KeepAlive(ev) => {
                self.keep_alive_event(ev);
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
    fn keep_alive_event(&mut self, event: void::Void) {
        log::trace!("QaulLanBehaviour::keep_alive_event: {:?}", event);
    }
    fn ping_event(&mut self, event: ping::Event) {
        events::ping_event(event, ConnectionModule::Lan);
    }
    fn mdsn_event(&mut self, event: mdns::Event) {
        match event {
            mdns::Event::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    log::trace!("MdnsEvent::Discovered, peer {:?} to floodsub added", peer);
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            mdns::Event::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
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
    KeepAlive(void::Void),
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

impl From<void::Void> for QaulLanEvent {
    fn from(event: void::Void) -> Self {
        Self::KeepAlive(event)
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
    pub async fn init(auth_keys: NoiseConfig) -> Lan {
        log::trace!("Lan::init() start");

        // TCP transport without DNS resolution on android
        // as the DNS module crashes on android due to a file system access
        #[cfg(any(target_os = "android", target_os = "ios"))]
        let transport = {
            let tcp = TcpTransport::new(GenTcpConfig::new().nodelay(true));
            tcp
        };
        // create tcp transport with DNS for all other devices
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        let transport = async {
            let tcp = TcpTransport::new(GenTcpConfig::new().nodelay(true));
            let dns_tcp = DnsConfig::system(tcp).await.unwrap();
            dns_tcp
        }
        .await;

        log::trace!("Lan::init() transport created");

        let transport_upgraded = transport
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(upgrade::ReadyUpgrade::new(yamux::Config::default()))
            //.timeout(std::time::Duration::from_secs(100 * 365 * 24 * 3600)) // 100 years
            .boxed();

        log::trace!("Lan::init() transport_upgraded");

        // create ping configuration
        // with customized parameters
        //
        // * keep connection alive
        let mut ping_config = ping::Config::new();

        let config = Configuration::get();
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Lan::init() ping_config");

        let mut swarm = {
            log::trace!("Lan::init() swarm creation started");

            // create MDNS behaviour
            // TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
            let mdns = Mdns::new(Config::default(), Node::get_id()).unwrap();

            log::trace!("Lan::init() swarm mdns module created");

            // TODO: set shorter re-advertisement time
            //       see here: libp2p-mdns/src/behaviour.rs
            let mut behaviour = QaulLanBehaviour {
                floodsub: Floodsub::new(Node::get_id()),
                mdns,
                keep_alive: libp2p::swarm::keep_alive::Behaviour::default(),
                ping: ping::Behaviour::new(ping_config),
                qaul_info: QaulInfo::new(Node::get_id()),
                qaul_messaging: QaulMessaging::new(Node::get_id()),
            };

            log::trace!("Lan::init() swarm behaviour defined");

            behaviour.floodsub.subscribe(Node::get_topic());

            log::trace!("Lan::init() swarm behaviour floodsub subscribed");

            Swarm::with_threadpool_executor(transport_upgraded, behaviour, Node::get_id())
        };

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
