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
//! [lan]
//! active = true
//! listen = "/ip4/0.0.0.0/tcp/0"
//! ```

use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    mplex,
    noise::{AuthenticKeypair, NoiseConfig, X25519Spec},
    ping::{Ping, PingConfig, PingEvent},
    swarm::Swarm,
    tcp::{GenTcpConfig, TcpTransport},
    yamux, NetworkBehaviour, Transport,
};
// DNS is excluded on mobile, as it is not working there
use async_std::task;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use libp2p::{dns::DnsConfig, websocket::WsConfig};
use prost::Message;

use crate::node::Node;
use crate::services::feed::Feed;
use crate::storage::configuration::Configuration;
use std::time::Duration;

use crate::connections::{events, ConnectionModule};
use qaul_info::{QaulInfo, QaulInfoEvent};
use qaul_messaging::{QaulMessaging, QaulMessagingEvent};

use crate::services::feed::proto_net;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "QaulLanEvent")]
pub struct QaulLanBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    pub ping: Ping,
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
    fn ping_event(&mut self, event: PingEvent) {
        events::ping_event(event, ConnectionModule::Lan);
    }
    fn mdsn_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    log::trace!("MdnsEvent::Discovered, peer {:?} to floodsub added", peer);
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
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
    Mdns(MdnsEvent),
    Ping(PingEvent),
    QaulInfo(QaulInfoEvent),
    QaulMessaging(QaulMessagingEvent),
}

impl From<FloodsubEvent> for QaulLanEvent {
    fn from(event: FloodsubEvent) -> Self {
        Self::Floodsub(event)
    }
}

impl From<MdnsEvent> for QaulLanEvent {
    fn from(event: MdnsEvent) -> Self {
        Self::Mdns(event)
    }
}

impl From<PingEvent> for QaulLanEvent {
    fn from(event: PingEvent) -> Self {
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
    pub async fn init(auth_keys: AuthenticKeypair<X25519Spec>) -> Lan {
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
            let ws_dns_tcp = WsConfig::new(
                DnsConfig::system(TcpTransport::new(GenTcpConfig::new().nodelay(true)))
                    .await
                    .unwrap(),
            );

            dns_tcp.or_transport(ws_dns_tcp)
        }
        .await;

        log::trace!("Lan::init() transport created");

        let transport_upgraded = transport
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(upgrade::SelectUpgrade::new(
                yamux::YamuxConfig::default(),
                mplex::MplexConfig::default(),
            ))
            //.timeout(std::time::Duration::from_secs(100 * 365 * 24 * 3600)) // 100 years
            .boxed();

        log::trace!("Lan::init() transport_upgraded");

        // create ping configuration
        // with customized parameters
        //
        // * keep connection alive
        let mut ping_config = PingConfig::new();
        ping_config = ping_config.with_keep_alive(true);
        let config = Configuration::get();
        ping_config =
            ping_config.with_interval(Duration::from_secs(config.routing.ping_neighbour_period));

        log::trace!("Lan::init() ping_config");

        let mut swarm = {
            log::trace!("Lan::init() swarm creation started");

            // create MDNS behaviour
            // TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
            let mdns = task::block_on(Mdns::new(MdnsConfig::default())).unwrap();

            log::trace!("Lan::init() swarm mdns module created");

            // TODO: set shorter re-advertisement time
            //       see here: libp2p-mdns/src/behaviour.rs
            let mut behaviour = QaulLanBehaviour {
                floodsub: Floodsub::new(Node::get_id()),
                mdns,
                ping: Ping::new(ping_config),
                qaul_info: QaulInfo::new(Node::get_id()),
                qaul_messaging: QaulMessaging::new(Node::get_id()),
            };

            log::trace!("Lan::init() swarm behaviour defined");

            behaviour.floodsub.subscribe(Node::get_topic());

            log::trace!("Lan::init() swarm behaviour floodsub subscribed");

            Swarm::new(transport_upgraded, behaviour, Node::get_id())
        };

        log::trace!("Lan::init() swarm created");

        // connect swarm to the listening interface in
        // the configuration config.lan.listen
        let config = Configuration::get();
        Swarm::listen_on(
            &mut swarm,
            config.lan.listen.parse().expect("can get a local socket"),
        )
        .expect("swarm can be started");

        log::trace!("Lan::init() swarm connected");

        let lan = Lan { swarm };

        lan
    }
}
