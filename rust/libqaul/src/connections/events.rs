// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Event handling for connection modules

use libp2p::identity::PublicKey;
use libp2p::ping::{Event, Failure};
use std::convert::TryFrom;
//use std::time::Duration;

use qaul_info::QaulInfoEvent;
use qaul_management::QaulManagementEvent;
use qaul_messaging::QaulMessagingEvent;

use crate::connections::ConnectionModule;
use crate::router::router_net_proto;
use crate::router::users::Users;
use crate::router::{info::RouterInfo, neighbours::Neighbours};
use crate::router_v2::identity::Multikey;
use crate::router_v2::management::profile::ManagementOutcome;
use crate::router_v2::propagation;
use crate::services::messaging::Messaging;
use crate::utilities::timestamp::Timestamp;

/// Handle incoming QaulInfo behaviour events
pub fn qaul_info_event(state: &crate::QaulState, event: QaulInfoEvent, module: ConnectionModule) {
    match event {
        // received a RoutingInfo message
        QaulInfoEvent::Message(message) => {
            log::trace!(
                "QaulInfoEvent::Message(QaulInfoReceived) from {}",
                message.received_from
            );

            // forward to router
            if let Some(router_v2) = state.get_router_v2() {
                if let Err(e) = router_v2.received(
                    message.received_from,
                    module,
                    None,
                    &message.data,
                    Timestamp::get_timestamp(),
                ) {
                    log::error!("router_v2 received failed: {e}");
                }
            } else {
                let rs = state.get_router();
                RouterInfo::received(state, &rs, message);
            }
        }
    }
}

/// Handle incoming QaulManagement behaviour events (spec §11.4).
/// it only handles routing from v2 drops any v1 related
pub fn qaul_management_event(
    state: &crate::QaulState,
    event: QaulManagementEvent,
    _module: ConnectionModule,
) {
    match event {
        QaulManagementEvent::Message(message) => {
            let Some(router_v2) = state.get_router_v2() else {
                log::trace!("management message received while routing v1 is active, dropping");
                return;
            };

            let outcome = router_v2.on_management_received(
                message.received_from,
                &message.data,
                Timestamp::get_timestamp(),
            );

            apply_management_outcome(state, outcome);
        }
    }
}

fn apply_management_outcome(state: &crate::QaulState, outcome: ManagementOutcome) {
    match outcome {
        ManagementOutcome::None => {}
        ManagementOutcome::UserProfileLearned {
            user_id,
            multikey,
            name,
            profile_version,
            capabilities,
            signed,
        } => {
            let Ok(public_key) = PublicKey::try_decode_protobuf(&multikey.encode()) else {
                log::warn!("management: could not rebuild public key for {user_id:?}");
                return;
            };
            let peer_id = public_key.to_peer_id();
            let rs = state.get_router();

            if !signed.is_empty() {
                Users::add_signed_user_info_table(
                    state,
                    &rs,
                    &[router_net_proto::SignedUserProfile {
                        profile: signed.profile,
                        signature: signed.signature,
                    }],
                );
            }

            // name and key for user without extended profile
            Users::add_with_check_caps(state, &rs, peer_id, public_key, name, capabilities);

            log::debug!(
                "management: directory updated for {user_id:?} (version={profile_version})"
            );
        }
    }
}

/// Handle incoming QaulMessaging behaviour events
pub fn qaul_messaging_event(
    state: &crate::QaulState,
    event: QaulMessagingEvent,
    _module: ConnectionModule,
) {
    match event {
        // received a messaging message
        QaulMessagingEvent::Message(message) => {
            log::trace!(
                "QaulMessagingEvent::Message(QaulMessagingReceived) from {}",
                message.received_from
            );

            // forward to messaging module
            Messaging::received(state, message);
        }
    }
}

/// Handle incoming ping event
pub fn ping_event(state: &crate::QaulState, event: Event, module: ConnectionModule) {
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
            if let Some(router_v2) = state.get_router_v2() {
                match Multikey::try_from_peer_id(&peer) {
                    Ok(mk) => {
                        let node_id = mk.to_id();
                        if router_v2.add_neighbour_transport(peer, node_id, module) {
                            router_v2.register_neighbour_node(
                                node_id,
                                Some(mk),
                                Timestamp::get_timestamp(),
                            );
                            propagation::on_neighbour_connect(&router_v2, peer, module);
                        }
                    }
                    Err(e) => log::warn!("v2: cannot derive node_id for {peer}: {e}"),
                }
            } else {
                let rs = state.get_router();
                let micros = rtt_micros.unwrap_or(u32::MAX);
                Neighbours::update_node(state, &rs, module, peer, micros, None);
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
