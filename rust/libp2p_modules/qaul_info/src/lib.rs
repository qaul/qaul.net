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
    swarm::{
        ConnectionId, FromSwarm, NetworkBehaviour, NotifyHandler, OneShotHandler,
        OneShotHandlerConfig, SubstreamProtocol, ToSwarm,
    },
    PeerId,
};
use std::{
    collections::{HashMap, VecDeque},
    task::{Context, Poll},
};

pub use crate::types::{QaulInfoData, QaulInfoReceived, QaulInfoSend};
use protocol::QaulInfoProtocol;

/// Network behaviour that handles the qaul_info protocol.
pub struct QaulInfo {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<ToSwarm<QaulInfoEvent, QaulInfoData>>,
    outbound: VecDeque<(PeerId, QaulInfoData)>,
    /// Each peer's open connections, oldest first
    connections: HashMap<PeerId, Vec<ConnectionId>>,
    /// The connection each peer's frame is being sent on
    in_flight: HashMap<PeerId, ConnectionId>,

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
            outbound: VecDeque::new(),
            connections: HashMap::new(),
            in_flight: HashMap::new(),
            config,
        }
    }

    /// Send a QaulInfoMessage to a specific node
    pub fn send_qaul_info_message(&mut self, node_id: PeerId, data: Vec<u8>) {
        self.outbound.push_back((node_id, QaulInfoData { data }));
    }

    /// The peer's oldest connection, or any while none is known yet.
    fn handler_for(&self, peer_id: &PeerId) -> NotifyHandler {
        match self.connections.get(peer_id).and_then(|ids| ids.first()) {
            Some(id) => NotifyHandler::One(*id),
            None => NotifyHandler::Any,
        }
    }

    /// Lets the peer's next frame go once the one on `connection_id` is done.
    fn release(&mut self, peer_id: &PeerId, connection_id: ConnectionId) {
        if self.in_flight.get(peer_id) == Some(&connection_id) {
            self.in_flight.remove(peer_id);
        }
    }
}

/// A handler whose own substream limit never binds.
fn ordered_handler() -> OneShotHandler<QaulInfoProtocol, QaulInfoData, InnerMessage> {
    OneShotHandler::new(
        SubstreamProtocol::new(QaulInfoProtocol::new(), ()),
        OneShotHandlerConfig {
            max_dial_negotiated: u32::MAX,
            ..Default::default()
        },
    )
}

impl NetworkBehaviour for QaulInfo {
    type ConnectionHandler = OneShotHandler<QaulInfoProtocol, QaulInfoData, InnerMessage>;
    type ToSwarm = QaulInfoEvent;

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, QaulInfoData>> {
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(event);
        }

        // the oldest frame whose peer has nothing in flight
        let next = self
            .outbound
            .iter()
            .position(|(peer_id, _)| !self.in_flight.contains_key(peer_id));
        if let Some((peer_id, data)) = next.and_then(|i| self.outbound.remove(i)) {
            let handler = self.handler_for(&peer_id);
            if let NotifyHandler::One(id) = handler {
                self.in_flight.insert(peer_id, id);
            }
            return Poll::Ready(ToSwarm::NotifyHandler {
                handler,
                peer_id,
                event: data,
            });
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
        Ok(ordered_handler())
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
        Ok(ordered_handler())
    }

    /// This callback function informs the behaviour about an event from Swarm.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.on_swarm_event
    fn on_swarm_event(&mut self, event: FromSwarm) {
        match event {
            FromSwarm::ConnectionEstablished(established) => {
                self.connections
                    .entry(established.peer_id)
                    .or_default()
                    .push(established.connection_id);
            }
            FromSwarm::ConnectionClosed(closed) => {
                self.release(&closed.peer_id, closed.connection_id);
                if let Some(ids) = self.connections.get_mut(&closed.peer_id) {
                    ids.retain(|id| *id != closed.connection_id);
                    if ids.is_empty() {
                        self.connections.remove(&closed.peer_id);
                    }
                }
            }
            _ => {}
        }
    }

    /// This callback function informs the behaviour about an event generated.
    /// For documentation please see:
    /// https://docs.rs/libp2p/latest/libp2p/swarm/trait.NetworkBehaviour.html#tymethod.on_connection_handler_event
    fn on_connection_handler_event(
        &mut self,
        peer_id: libp2p::PeerId,
        connection_id: libp2p::swarm::ConnectionId,
        event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
        // We received one of the following event notification
        let qaul_info_data = match event {
            // only process a received message
            Ok(InnerMessage::Received(event)) => event,
            Ok(InnerMessage::Sent) => {
                self.release(&peer_id, connection_id);
                return;
            }
            Err(err) => {
                log::trace!("qaul_info_data failed: {}", err);
                self.release(&peer_id, connection_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::{
        core::{ConnectedPoint, Endpoint},
        swarm::{
            behaviour::{ConnectionClosed, ConnectionEstablished},
            StreamUpgradeError,
        },
    };

    fn endpoint() -> ConnectedPoint {
        ConnectedPoint::Dialer {
            address: "/ip4/127.0.0.1/tcp/1".parse().unwrap(),
            role_override: Endpoint::Dialer,
            port_use: PortUse::Reuse,
        }
    }

    fn establish(behaviour: &mut QaulInfo, peer_id: PeerId, id: usize) {
        let endpoint = endpoint();
        behaviour.on_swarm_event(FromSwarm::ConnectionEstablished(ConnectionEstablished {
            peer_id,
            connection_id: ConnectionId::new_unchecked(id),
            endpoint: &endpoint,
            failed_addresses: &[],
            other_established: 0,
        }));
    }

    fn close(behaviour: &mut QaulInfo, peer_id: PeerId, id: usize) {
        let endpoint = endpoint();
        behaviour.on_swarm_event(FromSwarm::ConnectionClosed(ConnectionClosed {
            peer_id,
            connection_id: ConnectionId::new_unchecked(id),
            endpoint: &endpoint,
            cause: None,
            remaining_established: 0,
        }));
    }

    /// The handler and payload of the next frame handed to the swarm.
    fn next_send(behaviour: &mut QaulInfo) -> (NotifyHandler, Vec<u8>) {
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        match behaviour.poll(&mut cx) {
            Poll::Ready(ToSwarm::NotifyHandler { handler, event, .. }) => (handler, event.data),
            _ => panic!("expected a frame to be handed to a connection"),
        }
    }

    fn nothing_to_send(behaviour: &mut QaulInfo) -> bool {
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        behaviour.poll(&mut cx).is_pending()
    }

    /// The connection's handler reports how its frame went.
    fn report(
        behaviour: &mut QaulInfo,
        peer_id: PeerId,
        id: usize,
        outcome: Result<InnerMessage, StreamUpgradeError<std::io::Error>>,
    ) {
        behaviour.on_connection_handler_event(peer_id, ConnectionId::new_unchecked(id), outcome);
    }

    fn is_connection(handler: &NotifyHandler, id: usize) -> bool {
        matches!(handler, NotifyHandler::One(c) if *c == ConnectionId::new_unchecked(id))
    }

    #[test]
    fn frames_to_a_peer_use_its_oldest_connection() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        establish(&mut behaviour, peer, 2);

        behaviour.send_qaul_info_message(peer, vec![1]);
        assert!(is_connection(&next_send(&mut behaviour).0, 1));

        close(&mut behaviour, peer, 1);
        behaviour.send_qaul_info_message(peer, vec![2]);
        assert!(is_connection(&next_send(&mut behaviour).0, 2));

        close(&mut behaviour, peer, 2);
        behaviour.send_qaul_info_message(peer, vec![3]);
        assert!(matches!(next_send(&mut behaviour).0, NotifyHandler::Any));
    }

    /// The connection is chosen at hand-over, not at send, so a frame queued
    /// while its connection was open is not lost when that connection closes.
    #[test]
    fn a_frame_queued_before_its_connection_closes_goes_to_the_next() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        establish(&mut behaviour, peer, 2);

        behaviour.send_qaul_info_message(peer, vec![1]);
        close(&mut behaviour, peer, 1);
        assert!(is_connection(&next_send(&mut behaviour).0, 2));
    }

    #[test]
    fn frames_are_handed_over_in_send_order() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        for byte in 0..5u8 {
            behaviour.send_qaul_info_message(peer, vec![byte]);
        }

        let sent: Vec<u8> = (0..5)
            .map(|_| {
                let byte = next_send(&mut behaviour).1[0];
                report(&mut behaviour, peer, 1, Ok(InnerMessage::Sent));
                byte
            })
            .collect();
        assert_eq!(sent, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn a_peers_next_frame_waits_until_the_last_is_sent() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        behaviour.send_qaul_info_message(peer, vec![1]);
        behaviour.send_qaul_info_message(peer, vec![2]);

        assert_eq!(next_send(&mut behaviour).1, vec![1]);
        assert!(nothing_to_send(&mut behaviour));

        report(&mut behaviour, peer, 1, Ok(InnerMessage::Sent));
        assert_eq!(next_send(&mut behaviour).1, vec![2]);
    }

    /// libp2p's handler never frees the slot of a failed send, so the
    /// behaviour must, or one failure would block the peer for good.
    #[test]
    fn a_failed_send_lets_the_next_frame_go() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        behaviour.send_qaul_info_message(peer, vec![1]);
        behaviour.send_qaul_info_message(peer, vec![2]);

        next_send(&mut behaviour);
        report(&mut behaviour, peer, 1, Err(StreamUpgradeError::Timeout));
        assert_eq!(next_send(&mut behaviour).1, vec![2]);
    }

    #[test]
    fn closing_the_connection_lets_the_next_frame_go() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        establish(&mut behaviour, peer, 2);
        behaviour.send_qaul_info_message(peer, vec![1]);
        behaviour.send_qaul_info_message(peer, vec![2]);

        next_send(&mut behaviour);
        close(&mut behaviour, peer, 1);
        let (handler, data) = next_send(&mut behaviour);
        assert!(is_connection(&handler, 2));
        assert_eq!(data, vec![2]);
    }

    #[test]
    fn a_report_from_another_connection_does_not_release_the_peer() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        establish(&mut behaviour, peer, 1);
        establish(&mut behaviour, peer, 2);
        behaviour.send_qaul_info_message(peer, vec![1]);
        behaviour.send_qaul_info_message(peer, vec![2]);

        next_send(&mut behaviour);
        report(&mut behaviour, peer, 2, Ok(InnerMessage::Sent));
        assert!(nothing_to_send(&mut behaviour));
    }

    #[test]
    fn a_peer_with_a_frame_in_flight_does_not_hold_up_others() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let (slow, other) = (PeerId::random(), PeerId::random());
        establish(&mut behaviour, slow, 1);
        establish(&mut behaviour, other, 2);
        behaviour.send_qaul_info_message(slow, vec![1]);
        behaviour.send_qaul_info_message(slow, vec![2]);
        behaviour.send_qaul_info_message(other, vec![3]);

        assert_eq!(next_send(&mut behaviour).1, vec![1]);
        let (handler, data) = next_send(&mut behaviour);
        assert!(is_connection(&handler, 2));
        assert_eq!(data, vec![3]);
    }

    /// The swarm drops a frame for a peer it has no connection to, so no
    /// report would ever come back for it.
    #[test]
    fn a_frame_with_no_known_connection_does_not_block_the_peer() {
        let mut behaviour = QaulInfo::new(PeerId::random());
        let peer = PeerId::random();
        behaviour.send_qaul_info_message(peer, vec![1]);
        behaviour.send_qaul_info_message(peer, vec![2]);

        assert!(matches!(next_send(&mut behaviour).0, NotifyHandler::Any));
        assert_eq!(next_send(&mut behaviour).1, vec![2]);
    }
}
