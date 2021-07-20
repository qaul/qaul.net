//! # Qaul Routing Info Behaviour
//! 
//! This module is a libp2p swarm-behaviour module.
//! It manages and defines the routing info exchange protocol.

pub mod protocol;
pub mod types;

use libp2p::{
    core::{
        Multiaddr, 
        PeerId, 
        connection::ConnectionId
    },
    swarm::{
        NetworkBehaviour,
        NetworkBehaviourAction,
        PollParameters,
        ProtocolsHandler,
        OneShotHandler,
        NotifyHandler,
        DialPeerCondition,
    }
};
use cuckoofilter::{CuckooError, CuckooFilter};
use fnv::FnvHashSet;
use log::warn;
use smallvec::SmallVec;
use std::{
    collections::{
        VecDeque,
        hash_map::{DefaultHasher, HashMap},
    },
    task::{Context, Poll}
};

use protocol::QaulInfoProtocol;
pub use crate::types::{
    QaulInfoData,
    QaulInfoReceived,
    QaulInfoSend,
};

/// Network behaviour that handles the qaul_info protocol.
pub struct QaulInfo {
    /// Events that need to be handed to the outside when polling.
    events: VecDeque<NetworkBehaviourAction<QaulInfoData, QaulInfoEvent>>,

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
        let message = QaulInfoData {
            data,
        };

        // Schedule message for sending
        self.events.push_back(NetworkBehaviourAction::NotifyHandler {
            peer_id: node_id,
            handler: NotifyHandler::Any,
            event: message,
        });
    }
}

impl NetworkBehaviour for QaulInfo {
    //type ProtocolsHandler = OneShotHandler<QaulInfoProtocol, QaulInfoRpc, InnerMessage>;
    type ProtocolsHandler = OneShotHandler<QaulInfoProtocol, QaulInfoData, InnerMessage>;
    type OutEvent = QaulInfoEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        Default::default()
    }

    fn addresses_of_peer(&mut self, _id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, _id: &PeerId) {
        // should we inform qaul router?
    }

    fn inject_disconnected(&mut self, _id: &PeerId) {
        // should we inform qaul router?
    }

    fn inject_event(
        &mut self,
        received_from: PeerId,
        _connection: ConnectionId,
        event: InnerMessage,
    ) {
        // We received one of the following event notification
        let qaul_info_data = match event {
            // only process a received message
            InnerMessage::Received(event) => event,
            // ignore the sent event
            InnerMessage::Sent => return,
        };

        // forward the message to the user
        self.events.push_back(
            NetworkBehaviourAction::GenerateEvent(
                QaulInfoEvent::Message(QaulInfoReceived{
                    received_from,
                    data: qaul_info_data.data,
                })
            )
        );
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        _: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Some(event) = self.events.pop_front() {
            return Poll::Ready(event);
        }

        Poll::Pending
    }
}

/// Transmission between the `OneShotHandler` of the protocols handler
/// and the `QaulInfoHandler`.
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


/// Configuration options for the qaul info behaviour
pub struct QaulInfoConfig {
    /// Peer id of the local node. Used for the source of the messages that we publish.
    pub local_peer_id: PeerId,
}

impl QaulInfoConfig {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            local_peer_id,
        }
    }
}




