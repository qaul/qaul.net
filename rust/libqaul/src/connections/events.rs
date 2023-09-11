// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Event handling for connection modules

use libp2p::ping::{Event, Failure};
use std::convert::TryFrom;
//use std::time::Duration;

use qaul_info::QaulInfoEvent;
use qaul_messaging::QaulMessagingEvent;

use crate::connections::ConnectionModule;
use crate::router::{info::RouterInfo, neighbours::Neighbours};
use crate::services::messaging::Messaging;

/// Handle incoming QaulInfo behaviour events
pub fn qaul_info_event(event: QaulInfoEvent, _module: ConnectionModule) {
    match event {
        // received a RoutingInfo message
        QaulInfoEvent::Message(message) => {
            log::trace!(
                "QaulInfoEvent::Message(QaulInfoReceived) from {}",
                message.received_from
            );

            // forward to router
            RouterInfo::received(message);
        }
    }
}

/// Handle incoming QaulMessaging behaviour events
pub fn qaul_messaging_event(event: QaulMessagingEvent, _module: ConnectionModule) {
    match event {
        // received a messaging message
        QaulMessagingEvent::Message(message) => {
            log::trace!(
                "QaulMessagingEvent::Message(QaulMessagingReceived) from {}",
                message.received_from
            );

            // forward to messaging module
            Messaging::received(message);
        }
    }
}

/// Handle incoming ping event
pub fn ping_event(event: Event, module: ConnectionModule) {
    match event {
        Event {
            peer,
            result: Result::Ok(duration),
            connection,
        } => {
            log::debug!(
                "PingSuccess::Ping: connection_id: {}, rtt to {} is {} ms",
                peer,
                connection,
                duration.as_secs() * 1000 + (duration.subsec_nanos() as u64 / 1_000_000 as u64)
            );

            let rtt_micros = u32::try_from(
                duration.as_secs() * 1_000_000 + (duration.subsec_nanos() / 1_000) as u64,
            );
            match rtt_micros {
                Ok(micros) => Neighbours::update_node(module, peer, micros),
                Err(_) => Neighbours::update_node(module, peer, 4294967295),
            }
        }
        // Event {
        //     peer,
        //     result: Result::Ok(Duration { .. }),
        //     connection: _,
        // } => {
        //     log::debug!("PingSuccess::Pong from {}", peer);
        // }
        Event {
            peer,
            result: Result::Err(Failure::Timeout),
            connection: _,
        } => {
            log::debug!("PingFailure::Timeout to {}", peer);
        }
        Event {
            peer,
            result: Result::Err(Failure::Other { error }),
            connection: _,
        } => {
            log::debug!("PingFailure::Other {} error: {}", peer, error);
        }
        Event {
            peer,
            result: Result::Err(Failure::Unsupported),
            connection: _,
        } => {
            log::debug!("PingFailure::Unsupported by peer {}", peer);
        }
    }
}
