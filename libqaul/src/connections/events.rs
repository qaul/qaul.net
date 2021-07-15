/**
 * Event handling for connection modules
 */

use libp2p::{
    ping::{PingEvent, PingSuccess, PingFailure},
};
use std::convert::TryFrom;
use log::{info, error};

use crate::connections::ConnectionModule;
use crate::router::neighbours::Neighbours;
use crate::router_behaviour::{
    QaulRouterBehaviourEvent,
    QaulRouterBehaviourSuccess,
    QaulRouterBehaviourFailure,
};

/**
 * Handle QaulRouter Communication
 */
pub fn qaul_router_event( event: QaulRouterBehaviourEvent, module: ConnectionModule ) {
    match event {
        // received a RoutingInfo message
        QaulRouterBehaviourEvent {
            peer,
            result: Result::Ok(QaulRouterBehaviourSuccess::RoutingInfo { routing, users }),
        } => {
            info!("QaulRouterBehaviourSuccess::RoutingInfo");
        }
        // received a InfoRequest
        QaulRouterBehaviourEvent {
            peer,
            result: Result::Ok(QaulRouterBehaviourSuccess::InfoRequest { users }),
        } => {
            info!("QaulRouterBehaviourSuccess::InfoRequest");
        }
        // received an InfoResponse as an answer to my request
        QaulRouterBehaviourEvent {
            peer,
            result: Result::Ok(QaulRouterBehaviourSuccess::InfoResponse { users }),
        } => {
            info!("QaulRouterBehaviourSuccess::InfoRequest");
        }
        // a time occurred
        QaulRouterBehaviourEvent {
            peer,
            result: Result::Err(QaulRouterBehaviourFailure::Timeout),
        } => {
            error!("QaulRouterBehaviourFailure::Timeout to {}", peer);
        }
        // another error occurred
        QaulRouterBehaviourEvent {
            peer,
            result: Result::Err(QaulRouterBehaviourFailure::Other { error }),
        } => {
            error!("QaulRouterBehaviourFailure::Other {} error: {}", peer, error);
        }
    }
}


/**
 * Handle incoming ping event
 */
pub fn ping_event( event: PingEvent, module: ConnectionModule ) {
    match event {
        PingEvent {
            peer,
            result: Result::Ok(PingSuccess::Ping { rtt }),
        } => {
            tracing::trace!("PingSuccess::Ping: rtt to {} is {} ms", peer, rtt.as_millis());
            let rtt_micros = u32::try_from(rtt.as_micros());
            match rtt_micros {
                Ok( micros ) => Neighbours::update_node( module, peer, micros ),
                Err(_) => Neighbours::update_node( module, peer, 4294967295 ),
            }
        }
        PingEvent {
            peer,
            result: Result::Ok(PingSuccess::Pong),
        } => {
            tracing::trace!("PingSuccess::Pong from {}", peer);
        }
        PingEvent {
            peer,
            result: Result::Err(PingFailure::Timeout),
        } => {
            tracing::trace!("PingFailure::Timeout to {}", peer);
        }
        PingEvent {
            peer,
            result: Result::Err(PingFailure::Other { error }),
        } => {
            tracing::trace!("PingFailure::Other {} error: {}", peer, error);
        }
    }
}
