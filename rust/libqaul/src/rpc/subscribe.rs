// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Event subscription RPC
//!
//! A long-running RPC that lets external clients (qauld-ctl, the future
//! TUI, the Flutter UI) receive pushed events as they happen — chat
//! messages arriving, peers connecting/disconnecting, DTN deliveries,
//! etc. — without polling.
//!
//! ## Wire model
//!
//! qauld already routes any number of response frames per request_id
//! back to the originating client (see `clients/qauld/src/socket.rs`).
//! A subscribe call therefore needs no new transport: the client sends
//! one `SubscribeRequest`, libqaul records the subscriber under the
//! request_id, and every event fires one `Event` response per subscriber
//! tagged with that request_id. The stream stops naturally when the
//! client disconnects.
//!
//! ## Stale-subscriber handling
//!
//! In this initial implementation libqaul has no way to learn that a
//! qauld-ctl client has gone away — the subscriber list grows
//! monotonically and qauld silently drops events whose request_id is no
//! longer in its register. This is acceptable for the first cut (events
//! are tiny and infrequent for the dev/admin use case); a follow-up PR
//! will plumb a disconnect hook from qauld back into libqaul so the
//! list can be pruned.

use std::collections::HashMap;
use std::sync::RwLock;

use prost::Message;

use super::Rpc;
use crate::utilities::timestamp::Timestamp;

/// Import generated protobuf types for the subscribe wire format.
pub use qaul_proto::qaul_rpc_subscribe as proto;
/// Top-level Modules enum (we tag every emitted event with this module).
use crate::rpc::proto as rpc_proto;

/// Information we keep about each active subscriber.
#[derive(Clone, Debug)]
pub struct SubscriberInfo {
    /// The `request_id` from the originating SubscribeRequest. We tag
    /// every event we emit to this subscriber with this same id so qauld
    /// routes it back to the right socket.
    pub request_id: String,
    /// The `user_id` from the originating call. Empty for un-scoped
    /// subscriptions; carried through events for symmetry with other
    /// RPCs.
    pub user_id: Vec<u8>,
}

/// Instance-based subscription state owned by `QaulState`.
///
/// The map key is the subscriber's `request_id`; using it directly keeps
/// the bookkeeping simple at the cost of accepting a duplicate request_id
/// silently overwriting a prior subscription. That can't happen in
/// practice because qauld-ctl mints a fresh UUID per RPC.
pub struct SubscriptionState {
    pub subscribers: RwLock<HashMap<String, SubscriberInfo>>,
}

impl SubscriptionState {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(HashMap::new()),
        }
    }

    /// Number of active subscribers; useful for diagnostics.
    pub fn len(&self) -> usize {
        match self.subscribers.read() {
            Ok(g) => g.len(),
            Err(e) => {
                log::error!("SubscriptionState::len lock poisoned: {}", e);
                0
            }
        }
    }

    /// Drop the subscription identified by `request_id`. No-op if the
    /// id isn't currently subscribed.
    pub fn unsubscribe(&self, request_id: &str) {
        match self.subscribers.write() {
            Ok(mut g) => {
                if g.remove(request_id).is_some() {
                    log::info!(
                        "Subscribe: dropped subscriber {}; {} remaining",
                        request_id,
                        g.len()
                    );
                }
            }
            Err(e) => {
                log::error!("SubscriptionState::unsubscribe lock poisoned: {}", e);
            }
        }
    }
}

/// Subscribe RPC dispatch.
pub struct Subscribe {}

impl Subscribe {
    /// Handle an incoming `SubscribeRequest`. The subscription stays open
    /// indefinitely; events are pushed asynchronously by `fire`.
    pub fn rpc(
        state: &crate::QaulState,
        data: Vec<u8>,
        user_id: Vec<u8>,
        request_id: String,
    ) {
        // Decode the inner Subscribe envelope. We don't currently use any
        // of the fields, but we still parse so a malformed request is
        // logged rather than silently accepted.
        match proto::Subscribe::decode(&data[..]) {
            Ok(envelope) => match envelope.message {
                Some(proto::subscribe::Message::Request(_req)) => {
                    let info = SubscriberInfo {
                        request_id: request_id.clone(),
                        user_id,
                    };
                    match state.subscriptions.subscribers.write() {
                        Ok(mut subs) => {
                            subs.insert(request_id, info);
                            log::info!(
                                "Subscribe: {} active subscriber(s)",
                                subs.len()
                            );
                        }
                        Err(e) => {
                            log::error!("Subscribe: subscribers lock poisoned: {}", e);
                        }
                    }
                }
                Some(proto::subscribe::Message::Event(_)) => {
                    log::warn!(
                        "Subscribe: client sent an Event variant; ignoring"
                    );
                }
                None => {
                    log::warn!("Subscribe: empty oneof; ignoring");
                }
            },
            Err(e) => {
                log::error!("Subscribe: failed to decode request: {}", e);
            }
        }
    }

    /// Fan out an event to all active subscribers.
    ///
    /// Each subscriber receives one RPC response frame, encoded as a
    /// `Subscribe` envelope wrapping the `Event`, tagged with their
    /// original `request_id`. Topics this client doesn't recognise are
    /// expected to be ignored on the receiving side.
    pub fn fire(state: &crate::QaulState, topic: &str, payload: Vec<u8>) {
        let snapshot = match state.subscriptions.subscribers.read() {
            Ok(g) => {
                if g.is_empty() {
                    return;
                }
                g.values().cloned().collect::<Vec<_>>()
            }
            Err(e) => {
                log::error!("Subscribe::fire: subscribers lock poisoned: {}", e);
                return;
            }
        };

        let envelope = proto::Subscribe {
            message: Some(proto::subscribe::Message::Event(proto::Event {
                topic: topic.to_string(),
                payload,
                timestamp: Timestamp::get_timestamp(),
            })),
        };
        let mut buf = Vec::with_capacity(envelope.encoded_len());
        if let Err(e) = envelope.encode(&mut buf) {
            log::error!("Subscribe::fire: encode failed: {}", e);
            return;
        }

        for sub in snapshot {
            Rpc::send_message(
                state,
                buf.clone(),
                rpc_proto::Modules::Subscribe as i32,
                sub.request_id,
                sub.user_id,
            );
        }
    }
}

/// Topic name for incoming chat messages.
pub const TOPIC_CHAT_MESSAGE: &str = "chat.message";

/// Topic name for first-sighting of a peer on a transport.
pub const TOPIC_PEERS_CONNECTED: &str = "peers.connected";

/// Topic name for peer-disconnect / prune events.
///
/// Fired by the routing-table prune path when a previously-known
/// neighbour is dropped from a transport's neighbour set (e.g. its
/// `updated_at` aged past the staleness threshold, or the transport
/// reported a `ConnectionClosed`). Payload is the same `PeerEvent`
/// shape as `peers.connected` so subscribers can write one decoder.
///
/// Note: this is plumbing-only as of this commit — libqaul does not
/// yet call `emit_peer_disconnected` from any prune site. The
/// helper exists so the prune logic (when it lands as a separate
/// design decision around the staleness threshold) doesn't have to
/// touch the subscribe layer.
pub const TOPIC_PEERS_DISCONNECTED: &str = "peers.disconnected";

/// Topic name for DTN delivery responses (sender side: a storage node
/// has accepted or rejected one of our DTN-stored messages).
pub const TOPIC_DTN_DELIVERY_RESPONSE: &str = "dtn.delivery_response";

/// Emit a `DtnDeliveryResponseEvent` to all active subscribers.
///
/// Fired from the sender's side when a storage node returns a
/// `DtnResponse`. Subscribers filter on `response_type` to distinguish
/// accept (= delivered) from reject.
pub fn emit_dtn_delivery_response(
    state: &crate::QaulState,
    storage_node: &libp2p::PeerId,
    response: &crate::services::messaging::proto::DtnResponse,
) {
    if state.subscriptions.len() == 0 {
        return;
    }

    let event = proto::DtnDeliveryResponseEvent {
        signature: response.signature.clone(),
        storage_node: storage_node.to_bytes(),
        response_type: response.response_type as u32,
        reason: response.reason as u32,
    };
    let mut payload = Vec::with_capacity(event.encoded_len());
    if let Err(e) = event.encode(&mut payload) {
        log::error!("emit_dtn_delivery_response: encode failed: {}", e);
        return;
    }

    Subscribe::fire(state, TOPIC_DTN_DELIVERY_RESPONSE, payload);
}

/// Emit a `PeerEvent` to all active subscribers on the
/// `peers.connected` topic.
pub fn emit_peer_connected(
    state: &crate::QaulState,
    peer_id: &libp2p::PeerId,
    module: crate::connections::ConnectionModule,
) {
    emit_peer_event(state, TOPIC_PEERS_CONNECTED, peer_id, module);
}

/// Emit a `PeerEvent` on the `peers.disconnected` topic. Call this
/// from the routing-table prune path when a neighbour is removed.
///
/// No call sites yet — see [`TOPIC_PEERS_DISCONNECTED`] for context.
pub fn emit_peer_disconnected(
    state: &crate::QaulState,
    peer_id: &libp2p::PeerId,
    module: crate::connections::ConnectionModule,
) {
    emit_peer_event(state, TOPIC_PEERS_DISCONNECTED, peer_id, module);
}

fn emit_peer_event(
    state: &crate::QaulState,
    topic: &'static str,
    peer_id: &libp2p::PeerId,
    module: crate::connections::ConnectionModule,
) {
    if state.subscriptions.len() == 0 {
        return;
    }

    let event = proto::PeerEvent {
        peer_id: peer_id.to_bytes(),
        module: module.as_int() as u32,
    };
    let mut payload = Vec::with_capacity(event.encoded_len());
    if let Err(e) = event.encode(&mut payload) {
        log::error!("emit_peer_event({topic}): encode failed: {e}");
        return;
    }

    Subscribe::fire(state, topic, payload);
}

/// Emit a `ChatMessageEvent` to all active subscribers.
///
/// Convenience wrapper around `Subscribe::fire` so the event-source call
/// site stays a one-liner. New event sources should follow the same shape
/// (one helper per topic) to keep their boilerplate out of `process.rs`.
pub fn emit_chat_message(
    state: &crate::QaulState,
    receiver_id: &libp2p::PeerId,
    sender_id: &libp2p::PeerId,
    group_id: &crate::services::group::GroupId,
    message_id: &[u8],
    sent_at: u64,
    content: &str,
) {
    // Skip the encoding cost when nobody is listening.
    if state.subscriptions.len() == 0 {
        return;
    }

    let event = proto::ChatMessageEvent {
        receiver_id: receiver_id.to_bytes(),
        sender_id: sender_id.to_bytes(),
        group_id: group_id.to_bytes(),
        message_id: message_id.to_vec(),
        sent_at,
        received_at: Timestamp::get_timestamp(),
        content: content.to_string(),
    };
    let mut payload = Vec::with_capacity(event.encoded_len());
    if let Err(e) = event.encode(&mut payload) {
        log::error!("emit_chat_message: encode failed: {}", e);
        return;
    }

    Subscribe::fire(state, TOPIC_CHAT_MESSAGE, payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Subscribe::rpc` records the caller in `SubscriptionState`.
    /// `unsubscribe` removes them. Both lookups go through the public
    /// instance API so this exercises the wire path the `client_disconnected`
    /// hook relies on.
    #[test]
    fn subscribe_then_unsubscribe_round_trip() {
        let state = crate::QaulState::new_for_simulation();

        // Build a minimal SubscribeRequest envelope.
        let req = proto::Subscribe {
            message: Some(proto::subscribe::Message::Request(
                proto::SubscribeRequest { topics: Vec::new() },
            )),
        };
        let mut data = Vec::new();
        req.encode(&mut data).unwrap();

        Subscribe::rpc(&state, data, Vec::new(), "rid-1".to_string());
        assert_eq!(state.subscriptions.len(), 1);

        state.subscriptions.unsubscribe("rid-1");
        assert_eq!(state.subscriptions.len(), 0);
    }

    /// `client_disconnected` is the hook qauld calls on socket close;
    /// it must drop every request_id it's given.
    #[test]
    fn client_disconnected_removes_all_listed_subscribers() {
        let state = crate::QaulState::new_for_simulation();

        for rid in ["a", "b", "c"] {
            let req = proto::Subscribe {
                message: Some(proto::subscribe::Message::Request(
                    proto::SubscribeRequest { topics: Vec::new() },
                )),
            };
            let mut data = Vec::new();
            req.encode(&mut data).unwrap();
            Subscribe::rpc(&state, data, Vec::new(), rid.to_string());
        }
        assert_eq!(state.subscriptions.len(), 3);

        super::Rpc::client_disconnected(
            &state,
            &["a".to_string(), "b".to_string()],
        );
        assert_eq!(state.subscriptions.len(), 1);
    }

    /// `emit_peer_connected` produces a frame on the libqaul→client RPC
    /// channel for every active subscriber, encoded as a Subscribe
    /// envelope wrapping an Event with topic `peers.connected`.
    #[test]
    fn peer_connected_event_is_delivered_to_subscribers() {
        use crate::connections::ConnectionModule;

        let state = crate::QaulState::new_for_simulation();

        // Register one subscriber.
        let req = proto::Subscribe {
            message: Some(proto::subscribe::Message::Request(
                proto::SubscribeRequest { topics: Vec::new() },
            )),
        };
        let mut data = Vec::new();
        req.encode(&mut data).unwrap();
        Subscribe::rpc(&state, data, Vec::new(), "sub-1".to_string());

        let peer = libp2p::PeerId::from(libp2p::identity::Keypair::generate_ed25519().public());
        emit_peer_connected(&state, &peer, ConnectionModule::Lan);

        // Drain the libqaul→extern channel; we expect exactly one frame.
        let frame = match Rpc::receive_from_libqaul(&state) {
            Ok(f) => f,
            Err(e) => panic!("expected one frame, got {e:?}"),
        };
        let qrpc = rpc_proto::QaulRpc::decode(&frame[..]).expect("QaulRpc decodes");
        assert_eq!(qrpc.module, rpc_proto::Modules::Subscribe as i32);
        assert_eq!(qrpc.request_id, "sub-1");

        let envelope = proto::Subscribe::decode(&qrpc.data[..]).expect("Subscribe decodes");
        let event = match envelope.message {
            Some(proto::subscribe::Message::Event(ev)) => ev,
            other => panic!("expected Event, got {other:?}"),
        };
        assert_eq!(event.topic, TOPIC_PEERS_CONNECTED);

        let payload = proto::PeerEvent::decode(&event.payload[..]).expect("PeerEvent decodes");
        assert_eq!(payload.peer_id, peer.to_bytes());
        assert_eq!(payload.module, ConnectionModule::Lan.as_int() as u32);
    }

    /// `emit_peer_disconnected` shares the same wire shape as
    /// `emit_peer_connected` (a `PeerEvent` payload) but fires on the
    /// `peers.disconnected` topic. The plumbing must be in place even
    /// though no routing-table prune call site exists yet — clients
    /// (including qauld-tui) already subscribe to the topic.
    #[test]
    fn peer_disconnected_event_is_delivered_to_subscribers() {
        use crate::connections::ConnectionModule;

        let state = crate::QaulState::new_for_simulation();

        let req = proto::Subscribe {
            message: Some(proto::subscribe::Message::Request(
                proto::SubscribeRequest { topics: Vec::new() },
            )),
        };
        let mut data = Vec::new();
        req.encode(&mut data).unwrap();
        Subscribe::rpc(&state, data, Vec::new(), "sub-1".to_string());

        let peer = libp2p::PeerId::from(libp2p::identity::Keypair::generate_ed25519().public());
        emit_peer_disconnected(&state, &peer, ConnectionModule::Internet);

        let frame = match Rpc::receive_from_libqaul(&state) {
            Ok(f) => f,
            Err(e) => panic!("expected one frame, got {e:?}"),
        };
        let qrpc = rpc_proto::QaulRpc::decode(&frame[..]).expect("QaulRpc decodes");
        assert_eq!(qrpc.module, rpc_proto::Modules::Subscribe as i32);
        assert_eq!(qrpc.request_id, "sub-1");

        let envelope = proto::Subscribe::decode(&qrpc.data[..]).expect("Subscribe decodes");
        let event = match envelope.message {
            Some(proto::subscribe::Message::Event(ev)) => ev,
            other => panic!("expected Event, got {other:?}"),
        };
        assert_eq!(event.topic, TOPIC_PEERS_DISCONNECTED);

        let payload = proto::PeerEvent::decode(&event.payload[..]).expect("PeerEvent decodes");
        assert_eq!(payload.peer_id, peer.to_bytes());
        assert_eq!(payload.module, ConnectionModule::Internet.as_int() as u32);
    }

    /// `emit_dtn_delivery_response` produces a frame on the libqaul→client
    /// RPC channel for every active subscriber, encoded as a Subscribe
    /// envelope wrapping an Event with topic `dtn.delivery_response`.
    #[test]
    fn dtn_delivery_response_event_is_delivered_to_subscribers() {
        use crate::services::messaging::proto as messaging_proto;

        let state = crate::QaulState::new_for_simulation();

        // Register one subscriber.
        let req = proto::Subscribe {
            message: Some(proto::subscribe::Message::Request(
                proto::SubscribeRequest { topics: Vec::new() },
            )),
        };
        let mut data = Vec::new();
        req.encode(&mut data).unwrap();
        Subscribe::rpc(&state, data, Vec::new(), "sub-1".to_string());

        let storage_node =
            libp2p::PeerId::from(libp2p::identity::Keypair::generate_ed25519().public());
        let response = messaging_proto::DtnResponse {
            signature: vec![1, 2, 3, 4],
            response_type: messaging_proto::dtn_response::ResponseType::Accepted as i32,
            reason: messaging_proto::dtn_response::Reason::None as i32,
        };
        emit_dtn_delivery_response(&state, &storage_node, &response);

        let frame = match Rpc::receive_from_libqaul(&state) {
            Ok(f) => f,
            Err(e) => panic!("expected one frame, got {e:?}"),
        };
        let qrpc = rpc_proto::QaulRpc::decode(&frame[..]).expect("QaulRpc decodes");
        assert_eq!(qrpc.module, rpc_proto::Modules::Subscribe as i32);
        assert_eq!(qrpc.request_id, "sub-1");

        let envelope = proto::Subscribe::decode(&qrpc.data[..]).expect("Subscribe decodes");
        let event = match envelope.message {
            Some(proto::subscribe::Message::Event(ev)) => ev,
            other => panic!("expected Event, got {other:?}"),
        };
        assert_eq!(event.topic, TOPIC_DTN_DELIVERY_RESPONSE);

        let payload = proto::DtnDeliveryResponseEvent::decode(&event.payload[..])
            .expect("DtnDeliveryResponseEvent decodes");
        assert_eq!(payload.signature, vec![1, 2, 3, 4]);
        assert_eq!(payload.storage_node, storage_node.to_bytes());
        assert_eq!(
            payload.response_type,
            messaging_proto::dtn_response::ResponseType::Accepted as u32
        );
    }

    /// With no active subscribers, `emit_peer_connected` is a no-op:
    /// nothing gets pushed into the libqaul→extern channel.
    #[test]
    fn no_subscribers_produces_no_event() {
        use crate::connections::ConnectionModule;

        let state = crate::QaulState::new_for_simulation();
        let peer = libp2p::PeerId::from(libp2p::identity::Keypair::generate_ed25519().public());
        emit_peer_connected(&state, &peer, ConnectionModule::Internet);

        // The channel should be empty.
        match Rpc::receive_from_libqaul(&state) {
            Err(_) => {} // expected (TryRecvError::Empty)
            Ok(_) => panic!("event was emitted with no subscribers"),
        }
    }
}
