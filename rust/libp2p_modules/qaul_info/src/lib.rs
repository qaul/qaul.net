// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Routing Info Behaviour
//!
//! This module is a libp2p swarm-behaviour module.
//! It manages and defines the routing info exchange protocol.

//pub mod codec;
//pub mod length_codec;
//pub mod max_varint_codec;
pub mod protocol;
pub mod types;

use libp2p::{
    core::{transport::PortUse, Multiaddr},
    swarm::{NetworkBehaviour, NotifyHandler, OneShotHandler, ToSwarm},
    PeerId,
};
use std::{
    collections::VecDeque,
    task::{Context, Poll},
};

pub use crate::types::{QaulInfoData, QaulInfoReceived, QaulInfoSend};
use protocol::QaulInfoProtocol;

/// Network behaviour that handles the qaul_info protocol.
pub struct QaulInfo {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<ToSwarm<QaulInfoEvent, QaulInfoData>>,

    #[allow(dead_code)]
    config: QaulInfoConfig,
}

impl QaulInfo {
    /// Creates a `QaulInfo` with default configuration.
    pub fn new(local_peer_id: PeerId) -> Self {
        Self::from_config(QaulInfoConfig::new(local_peer_id))
    }

    /// Creates a `QaulInfo` with the given configuration.
    pub fn from_config(config: QaulInfoConfig) -> Self {
        QaulInfo {
            events: VecDeque::new(),
            config,
        }
    }

    /// Send a QaulInfoMessage to a specific node
    pub fn send_qaul_info_message(&mut self, node_id: PeerId, data: Vec<u8>) {
        // create event message
        let message = QaulInfoData { data };

        // Schedule message for sending
        self.events.push_back(ToSwarm::NotifyHandler {
            peer_id: node_id,
            handler: NotifyHandler::Any,
            event: message,
        });
    }
}

impl NetworkBehaviour for QaulInfo {
    type ConnectionHandler = OneShotHandler<QaulInfoProtocol, QaulInfoData, InnerMessage>;
    type ToSwarm = QaulInfoEvent;

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, QaulInfoData>> {
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(event);
        }

        Poll::Pending
    }

    /// This callback function is invoked for every established inbound connection.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.handle_established_inbound_connection
    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _peer: libp2p::PeerId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(Default::default())
    }

    /// This callback function is invoked for every established outbound connection.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.handle_established_outbound_connection
    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _peer: libp2p::PeerId,
        _addr: &Multiaddr,
        _role_override: libp2p::core::Endpoint,
        _port_use: PortUse,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(Default::default())
    }

    /// This callback function informs the behaviour about an event from Swarm.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.on_swarm_event
    fn on_swarm_event(&mut self, _event: libp2p::swarm::FromSwarm) {}

    /// This callback function informs the behaviour about an event generated.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.on_connection_handler_event
    fn on_connection_handler_event(
        &mut self,
        peer_id: libp2p::PeerId,
        _connection_id: libp2p::swarm::ConnectionId,
        event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
        // We received one of the following event notification
        let qaul_info_data = match event {
            // only process a received message
            Ok(InnerMessage::Received(event)) => event,
            // ignore the sent event
            Ok(InnerMessage::Sent) => return,
            Err(err) => {
                log::trace!("qaul_info_data failed: {}", err);
                return;
            }
        };

        // forward the message to the user
        self.events
            .push_back(ToSwarm::GenerateEvent(QaulInfoEvent::Message(
                QaulInfoReceived {
                    received_from: peer_id,
                    data: qaul_info_data.data,
                },
            )));
    }
}

/// Transmission between the `OneShotHandler` of the protocols handler
/// and the `QaulInfoHandler`.
#[derive(Debug)]
pub enum InnerMessage {
    /// We received an QaulRoutingInfoMessage from a remote.
    Received(QaulInfoData),
    /// We successfully sent an QaulRoutingInfoMessage request.
    Sent,
}

impl From<QaulInfoData> for InnerMessage {
    #[inline]
    fn from(remote: QaulInfoData) -> InnerMessage {
        InnerMessage::Received(remote)
    }
}

impl From<()> for InnerMessage {
    #[inline]
    fn from(_: ()) -> InnerMessage {
        InnerMessage::Sent
    }
}

/// Event that can happen on the qaul_info behaviour.
#[derive(Debug)]
pub enum QaulInfoEvent {
    /// A message has been received.
    Message(QaulInfoReceived),
}

impl From<QaulInfoEvent> for QaulInfoData {
    #[inline]
    fn from(event: QaulInfoEvent) -> QaulInfoData {
        match event {
            QaulInfoEvent::Message(data) => QaulInfoData { data: data.data },
        }
    }
}

/// Configuration options for the qaul info behaviour
#[derive(Debug, Clone)]
pub struct QaulInfoConfig {
    /// Peer id of the local node. Used for the source of the messages that we publish.
    pub local_peer_id: PeerId,
}

impl QaulInfoConfig {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self { local_peer_id }
    }
}
