/**
 * Event handling for connection modules
 */

use libp2p::{
    ping::{PingEvent, PingSuccess, PingFailure},
};
//use log::info;
use std::convert::TryFrom;
use crate::connections::ConnectionModule;
use crate::router::neighbours::Neighbours;

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
