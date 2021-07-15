/**
 * This module implements the qaul related behaviour into
 * rust-libp2p swarm
 * 
 * * send network tables
 * * send direct messages
 */

use libp2p::{
    core::{
        Multiaddr, 
        PeerId, 
        ConnectedPoint, 
        connection::ConnectionId,
    },
    swarm::{
        NetworkBehaviour, 
        NetworkBehaviourAction, 
        PollParameters
    },
};
use std::{
    collections::VecDeque, 
    error::Error,
    task::Context, 
    task::Poll
};
use void::Void;
use log::{info, error};

pub mod protocol;
pub mod handler;
pub use handler::{
    QaulRouterBehaviourConfig, 
    QaulRouterBehaviourResult, 
    QaulRouterBehaviourSuccess, 
    QaulRouterBehaviourFailure,
};
use handler::QaulRouterBehaviourHandler;


/** 
 * `QaulRouterBehaviour`, a libp2p `NetworkBehaviour`
 */
pub struct QaulRouterBehaviour {
    /// Configuration for outbound communication.
    config: QaulRouterBehaviourConfig,
    /// Queue of events to yield to the swarm.
    events: VecDeque<QaulRouterBehaviourEvent>,
}

/**
 * Event
 */
#[derive(Debug)]
pub struct QaulRouterBehaviourEvent {
    /// The peer ID of the remote.
    pub peer: PeerId,
    /// The result of an inbound or outbound communication.
    pub result: QaulRouterBehaviourResult,
}

impl QaulRouterBehaviour {
    /// Creates a new `QaulRouterBehaviour` with the given configuration.
    pub fn new(config: QaulRouterBehaviourConfig) -> Self {
        QaulRouterBehaviour {
            config,
            events: VecDeque::new(),
        }
    }
}

impl Default for QaulRouterBehaviour {
    fn default() -> Self {
        QaulRouterBehaviour::new(QaulRouterBehaviourConfig::new())
    }
}

impl NetworkBehaviour for QaulRouterBehaviour {
    type ProtocolsHandler = QaulRouterBehaviourHandler;
    type OutEvent = QaulRouterBehaviourEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        QaulRouterBehaviourHandler::new(self.config.clone())
    }

    fn addresses_of_peer(&mut self, _peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: &PeerId) {
        info!("inject_connected to {:?}", peer_id);
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        info!("inject_disconnected to {:?}", peer_id);
    }

    fn inject_connection_closed(&mut self, peer_id: &PeerId, _conn_id: &ConnectionId, _conn_point: &ConnectedPoint) {
        info!("inject_connection_closed to {:?}", peer_id);
    }

    fn inject_dial_failure(&mut self, peer_id: &PeerId) {
        error!("inject_dial_failure to {:?}", peer_id);
    }

    fn inject_addr_reach_failure(&mut self, peer_id: Option<&PeerId>, _addr: &Multiaddr, _error: &dyn Error) {
        error!("inject_addr_reach_failure to {:?}", peer_id);
    }

    fn inject_event(&mut self, peer: PeerId, _: ConnectionId, result: QaulRouterBehaviourResult) {
        self.events.push_front(QaulRouterBehaviourEvent { peer, result })
    }

    fn poll(&mut self, _: &mut Context<'_>, _: &mut impl PollParameters)
        -> Poll<NetworkBehaviourAction<Void, QaulRouterBehaviourEvent>>
    {
        if let Some(e) = self.events.pop_back() {
            Poll::Ready(NetworkBehaviourAction::GenerateEvent(e))
        } else {
            Poll::Pending
        }
    }
}
