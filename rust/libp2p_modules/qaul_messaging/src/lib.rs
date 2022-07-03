// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Messaging Behaviour
//!
//! This module is a libp2p swarm-behaviour module.
//! It manages and defines the messaging exchange protocol.

pub mod protocol;
pub mod types;

use libp2p::{
    core::{connection::ConnectionId, Multiaddr, PeerId},
    swarm::{
        NetworkBehaviour, NetworkBehaviourAction, NotifyHandler, OneShotHandler, PollParameters,
    },
};
use std::{
    collections::VecDeque,
    task::{Context, Poll},
};

pub use crate::types::{QaulMessagingData, QaulMessagingReceived, QaulMessagingSend};
use protocol::QaulMessagingProtocol;

/// Network behaviour that handles the qaul_messaging protocol.
pub struct QaulMessaging {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<
        NetworkBehaviourAction<
            QaulMessagingEvent,
            <QaulMessaging as NetworkBehaviour>::ConnectionHandler,
        >,
    >,
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
        self.events
            .push_back(NetworkBehaviourAction::NotifyHandler {
                peer_id: node_id,
                handler: NotifyHandler::Any,
                event: message,
            });
    }
}

impl NetworkBehaviour for QaulMessaging {
    type ConnectionHandler = OneShotHandler<QaulMessagingProtocol, QaulMessagingData, InnerMessage>;
    type OutEvent = QaulMessagingEvent;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        Default::default()
    }

    fn addresses_of_peer(&mut self, _id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connection_established(
        &mut self,
        _peer_id: &PeerId,
        _connection_id: &ConnectionId,
        _endpoint: &libp2p::core::ConnectedPoint,
        _failed_addresses: Option<&Vec<Multiaddr>>,
        _other_established: usize,
    ) {
        // should we inform qaul messaging?
    }

    fn inject_connection_closed(
        &mut self,
        _: &PeerId,
        _: &ConnectionId,
        _: &libp2p::core::ConnectedPoint,
        _: <Self::ConnectionHandler as libp2p::swarm::IntoConnectionHandler>::Handler,
        _remaining_established: usize,
    ) {
        // should we inform qaul messaging?
    }

    fn inject_event(
        &mut self,
        received_from: PeerId,
        _connection: ConnectionId,
        event: InnerMessage,
    ) {
        // We received one of the following event notification
        let qaul_messaging_data = match event {
            // only process a received message
            InnerMessage::Received(event) => event,
            // ignore the sent event
            InnerMessage::Sent => return,
        };

        // forward the message to the user
        self.events.push_back(NetworkBehaviourAction::GenerateEvent(
            QaulMessagingEvent::Message(QaulMessagingReceived {
                received_from,
                data: qaul_messaging_data.data,
            }),
        ));
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        _: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<Self::OutEvent, Self::ConnectionHandler>> {
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(event);
        }

        Poll::Pending
    }
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
pub struct QaulMessagingConfig {
    /// Peer id of the local node. Used for the source of the messages that we publish.
    pub local_peer_id: PeerId,
}

impl QaulMessagingConfig {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self { local_peer_id }
    }
}
