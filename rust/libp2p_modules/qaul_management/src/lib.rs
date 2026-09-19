// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Management Behaviour
//!
//! libp2p swarm behaviour for the network management sub-protocol (spec
//! §11): a unicast pipe carrying opaque `ManagementMessage` bytes between
//! neighbours.

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

pub use crate::types::{QaulManagementData, QaulManagementReceived, QaulManagementSend};
use protocol::QaulManagementProtocol;

/// Network behaviour that handles the qaul_management protocol.
pub struct QaulManagement {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<ToSwarm<QaulManagementEvent, QaulManagementData>>,

    #[allow(dead_code)]
    config: QaulManagementConfig,
}

impl QaulManagement {
    /// Creates a `QaulManagement` with default configuration.
    pub fn new(local_peer_id: PeerId) -> Self {
        Self::from_config(QaulManagementConfig::new(local_peer_id))
    }

    /// Creates a `QaulManagement` with the given configuration.
    pub fn from_config(config: QaulManagementConfig) -> Self {
        QaulManagement {
            events: VecDeque::new(),
            config,
        }
    }

    /// Hand an encoded `ManagementMessage` to one neighbour.
    /// node_id here is the next hop, not the dest
    pub fn send_qaul_management_message(&mut self, node_id: PeerId, data: Vec<u8>) {
        let message = QaulManagementData { data };

        self.events.push_back(ToSwarm::NotifyHandler {
            peer_id: node_id,
            handler: NotifyHandler::Any,
            event: message,
        });
    }
}

impl NetworkBehaviour for QaulManagement {
    type ConnectionHandler =
        OneShotHandler<QaulManagementProtocol, QaulManagementData, InnerMessage>;
    type ToSwarm = QaulManagementEvent;

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, QaulManagementData>> {
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
        let qaul_management_data = match event {
            // only process a received message
            Ok(InnerMessage::Received(event)) => event,
            // ignore the sent event
            Ok(InnerMessage::Sent) => return,
            Err(err) => {
                // §11.2 is best-effort: a failed exchange is dropped, and
                // the using layer re-issues when it notices the outcome
                // never arrived.
                log::trace!("qaul_management_data failed: {}", err);
                return;
            }
        };

        self.events
            .push_back(ToSwarm::GenerateEvent(QaulManagementEvent::Message(
                QaulManagementReceived {
                    received_from: peer_id,
                    data: qaul_management_data.data,
                },
            )));
    }
}

/// Transmission between the `OneShotHandler` of the protocols handler
/// and the `QaulManagementHandler`.
#[derive(Debug)]
pub enum InnerMessage {
    /// We received a ManagementMessage from a remote.
    Received(QaulManagementData),
    /// We successfully sent a ManagementMessage request.
    Sent,
}

impl From<QaulManagementData> for InnerMessage {
    #[inline]
    fn from(remote: QaulManagementData) -> InnerMessage {
        InnerMessage::Received(remote)
    }
}

impl From<()> for InnerMessage {
    #[inline]
    fn from(_: ()) -> InnerMessage {
        InnerMessage::Sent
    }
}

/// Event that can happen on the qaul_management behaviour.
#[derive(Debug)]
pub enum QaulManagementEvent {
    /// A message has been received.
    Message(QaulManagementReceived),
}

impl From<QaulManagementEvent> for QaulManagementData {
    #[inline]
    fn from(event: QaulManagementEvent) -> QaulManagementData {
        match event {
            QaulManagementEvent::Message(data) => QaulManagementData { data: data.data },
        }
    }
}

/// Configuration options for the qaul management behaviour
#[derive(Debug, Clone)]
pub struct QaulManagementConfig {
    /// Peer id of the local node.
    pub local_peer_id: PeerId,
}

impl QaulManagementConfig {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self { local_peer_id }
    }
}
