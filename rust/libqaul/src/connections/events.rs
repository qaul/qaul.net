// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Event handling for connection modules

use libp2p::{
    ping::{PingEvent, PingSuccess, PingFailure},
};
use std::convert::TryFrom;

use qaul_info::QaulInfoEvent;
use qaul_messaging::QaulMessagingEvent;

use crate::connections::ConnectionModule;
use crate::router::{
    neighbours::Neighbours,
    info::RouterInfo,
};
use crate::services::messaging::Messaging;


/// Handle incoming QaulInfo behaviour events
pub fn qaul_info_event( event: QaulInfoEvent, _module: ConnectionModule ) {
    match event {
        // received a RoutingInfo message
        QaulInfoEvent::Message(message) => {
            log::info!("QaulInfoEvent::Message(QaulInfoReceived) from {}", message.received_from);

            // forward to router
            RouterInfo::received(message);
        }
    }
}

/// Handle incoming QaulMessaging behaviour events
pub fn qaul_messaging_event( event: QaulMessagingEvent, _module: ConnectionModule ) {
    match event {
        // received a RoutingInfo message
        QaulMessagingEvent::Message(message) => {
            log::info!("QaulMessagingEvent::Message(QaulMessagingReceived) from {}", message.received_from);

            // forward to router
            Messaging::received(message);
        }
    }
}

/// Handle incoming ping event
pub fn ping_event( event: PingEvent, module: ConnectionModule ) {
    match event {
        PingEvent {
            peer,
            result: Result::Ok(PingSuccess::Ping { rtt }),
        } => {
            log::debug!("PingSuccess::Ping: rtt to {} is {} ms", peer, rtt.as_millis());
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
            log::debug!("PingSuccess::Pong from {}", peer);
        }
        PingEvent {
            peer,
            result: Result::Err(PingFailure::Timeout),
        } => {
            log::debug!("PingFailure::Timeout to {}", peer);
        }
        PingEvent {
            peer,
            result: Result::Err(PingFailure::Other { error }),
        } => {
            log::debug!("PingFailure::Other {} error: {}", peer, error);
        }
        PingEvent {
            peer,
            result: Result::Err(PingFailure::Unsupported),
        } => {
            log::debug!("PingFailure::Unsupported by peer {}", peer);
        }
    }
}
