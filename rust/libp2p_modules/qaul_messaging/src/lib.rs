// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Messaging Behaviour
//!
//! This module is a libp2p swarm-behaviour module.
//! It manages and defines the messaging exchange protocol.

pub mod protocol;
pub mod types;

pub use crate::types::{QaulMessagingData, QaulMessagingReceived, QaulMessagingSend};
use libp2p::{
    core::{transport::PortUse, Multiaddr},
    swarm::{
        ConnectionId, NetworkBehaviour, NotifyHandler, OneShotHandler, StreamUpgradeError, ToSwarm,
    },
    PeerId,
};
use protocol::QaulMessagingProtocol;
use std::{
    collections::VecDeque,
    task::{Context, Poll},
};

/// Network behaviour that handles the qaul_messaging protocol.
pub struct QaulMessaging {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<ToSwarm<QaulMessagingEvent, QaulMessagingData>>,
    #[allow(dead_code)]
    config: QaulMessagingConfig,
}

impl QaulMessaging {
    /// Creates a `QaulMessaging` with default configuration.
    pub fn new(local_peer_id: PeerId) -> Self {
        Self::from_config(QaulMessagingConfig::new(local_peer_id))
    }

    /// Creates a `QaulMessaging` with the given configuration.
    pub fn from_config(config: QaulMessagingConfig) -> Self {
        QaulMessaging {
            events: VecDeque::new(),
            config,
        }
    }

    /// Send a QaulMessagingMessage to a specific node
    pub fn send_qaul_messaging_message(&mut self, node_id: PeerId, data: Vec<u8>) {
        // create event message
        let message = QaulMessagingData { data };

        // Schedule message for sending
        self.events.push_back(ToSwarm::NotifyHandler {
            peer_id: node_id,
            handler: NotifyHandler::Any,
            event: message,
        });
    }
}

impl NetworkBehaviour for QaulMessaging {
    type ConnectionHandler = OneShotHandler<QaulMessagingProtocol, QaulMessagingData, InnerMessage>;
    type ToSwarm = QaulMessagingEvent;

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        _connection: ConnectionId,
        event: Result<InnerMessage, StreamUpgradeError<std::io::Error>>,
    ) {
        // We received one of the following event notification
        let qaul_messaging_data = match event {
            // only process a received message
            Ok(InnerMessage::Received(event)) => event,
            // ignore the sent event
            Ok(InnerMessage::Sent) => return,
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        };

        // forward the message to the user
        self.events
            .push_back(ToSwarm::GenerateEvent(QaulMessagingEvent::Message(
                QaulMessagingReceived {
                    received_from: peer_id,
                    data: qaul_messaging_data.data,
                },
            )));
    }

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, QaulMessagingData>> {
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
}

/// Transmission between the `OneShotHandler` of the protocols handler
/// and the `QaulMessagingHandler`.
#[derive(Debug)]
pub enum InnerMessage {
    /// We received an QaulMessagingMessage from a remote.
    Received(QaulMessagingData),
    /// We successfully sent an QaulMessagingMessage request.
    Sent,
}

impl From<QaulMessagingData> for InnerMessage {
    #[inline]
    fn from(remote: QaulMessagingData) -> InnerMessage {
        InnerMessage::Received(remote)
    }
}

impl From<()> for InnerMessage {
    #[inline]
    fn from(_: ()) -> InnerMessage {
        InnerMessage::Sent
    }
}

/// Event that can happen on the qaul_messaging behaviour.
#[derive(Debug)]
pub enum QaulMessagingEvent {
    /// A message has been received.
    Message(QaulMessagingReceived),
}

/// Configuration options for the qaul messaging behaviour
#[derive(Debug, Clone)]
pub struct QaulMessagingConfig {
    /// Peer id of the local node. Used for the source of the messages that we publish.
    pub local_peer_id: PeerId,
}

impl QaulMessagingConfig {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self { local_peer_id }
    }
}
