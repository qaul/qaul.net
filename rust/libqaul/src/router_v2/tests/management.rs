// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Network management sub-protocol: envelope, profile fetch scheduling
//! (spec §11.3, §11.5, §14).

use crate::connections::ConnectionModule;
use crate::router_v2::{
    identity::{Multikey, Profile},
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
    RouterV2State,
};
use libp2p::identity::Keypair;
use prost::Message;
use std::sync::{Arc, RwLock};

use proto::{
    management_message::Body, ManagementMessage, Profile as ProtoProfile, ProfileRequest,
    ProfileResponse,
};
use qaul_proto::qaul_net_router_management as proto;

// ---------- §11.3 envelope ----------

mod envelope {
    use super::*;

    fn request_envelope() -> ManagementMessage {
        ManagementMessage {
            version: 1,
            destination: vec![1, 2, 3, 4, 5, 6, 7, 8],
            destination_is_node: false,
            source: vec![9, 10, 11, 12, 13, 14, 15, 16],
            source_is_node: true,
            request_id: 42,
            body: Some(Body::ProfileRequest(ProfileRequest { cached_version: 7 })),
        }
    }

    #[test]
    fn a_profile_request_round_trips() {
        let msg = request_envelope();
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.destination, msg.destination);
        assert!(!decoded.destination_is_node);
        assert!(decoded.source_is_node);
        assert_eq!(decoded.request_id, 42);
        match decoded.body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 7),
            other => panic!("expected a ProfileRequest body, got {other:?}"),
        }
    }

    #[test]
    fn a_profile_response_round_trips() {
        let msg = ManagementMessage {
            version: 1,
            destination: vec![9, 10, 11, 12, 13, 14, 15, 16],
            destination_is_node: true,
            source: vec![1, 2, 3, 4, 5, 6, 7, 8],
            source_is_node: false,
            request_id: 42,
            body: Some(Body::ProfileResponse(ProfileResponse {
                found: true,
                profile: Some(ProtoProfile {
                    multikey: vec![0xAA; 36],
                    profile_version: 3,
                    name: "alice".into(),
                    self_signature: vec![0xBB; 64],
                    capabilities: 1,
                    signed_profile: Vec::new(),
                    signed_profile_signature: Vec::new(),
                }),
            })),
        };
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        match decoded.body {
            Some(Body::ProfileResponse(r)) => {
                assert!(r.found);
                let p = r.profile.expect("profile present");
                assert_eq!(p.profile_version, 3);
                assert_eq!(p.name, "alice");
                assert_eq!(p.capabilities, 1);
            }
            other => panic!("expected a ProfileResponse body, got {other:?}"),
        }
    }

    /// §11.5 permits `found: false`; the profile is then absent rather than
    /// empty, and a receiver must not treat a default-constructed profile as
    /// a real one.
    #[test]
    fn a_not_found_response_carries_no_profile() {
        let msg = ManagementMessage {
            version: 1,
            destination: vec![0; 8],
            destination_is_node: false,
            source: vec![1; 8],
            source_is_node: true,
            request_id: 1,
            body: Some(Body::ProfileResponse(ProfileResponse {
                found: false,
                profile: None,
            })),
        };
        let decoded = ManagementMessage::decode(&msg.encode_to_vec()[..]).unwrap();

        match decoded.body {
            Some(Body::ProfileResponse(r)) => {
                assert!(!r.found);
                assert!(r.profile.is_none());
            }
            other => panic!("expected a ProfileResponse body, got {other:?}"),
        }
    }

    /// Phase 13 will add field numbers 9-12 (subscribe/revoke). A message
    /// carrying one must decode without error today, with an unrecognised
    /// body, so a newer peer cannot break an older one.
    #[test]
    fn an_unknown_body_variant_decodes_without_error() {
        let mut bytes = request_envelope().encode_to_vec();
        // field 9, wire type 2 (length-delimited), empty payload
        bytes.extend_from_slice(&[(9 << 3) | 2, 0]);

        let decoded = ManagementMessage::decode(&bytes[..])
            .expect("a reserved-field body must not break decoding");
        assert_eq!(decoded.request_id, 42);
    }

    /// The wire `Profile` and the internal one must agree on what is signed:
    /// §11.5 fixes the input at `multikey || profile_version || name`, and
    /// `capabilities` is deliberately outside it.
    #[test]
    fn the_wire_profile_and_the_signed_input_agree() {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());

        let internal = Profile {
            multikey: mk.clone(),
            version: 5,
            name: "alice".into(),
            self_signature: [0u8; 64],
        };
        let signing_input = internal.sign_input();
        let signature: [u8; 64] = kp.sign(&signing_input).unwrap().try_into().unwrap();

        let wire = ProtoProfile {
            multikey: mk.encode(),
            profile_version: 5,
            name: "alice".into(),
            self_signature: signature.to_vec(),
            capabilities: 0xFF, // must not affect verification
            signed_profile: Vec::new(),
            signed_profile_signature: Vec::new(),
        };

        let rebuilt = Profile {
            multikey: Multikey::decode(&wire.multikey).unwrap(),
            version: wire.profile_version,
            name: wire.name.clone(),
            self_signature: signature,
        };
        assert_eq!(rebuilt.sign_input(), signing_input);
        assert!(
            mk.verify(&rebuilt.sign_input(), &signature),
            "a profile rebuilt from the wire must verify"
        );
    }

    /// Changing a signed field breaks verification; changing `capabilities`
    /// does not. This is the whole reason capabilities sit outside the
    /// signature — and the known exposure it carries.
    #[test]
    fn capabilities_are_outside_the_signature() {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());

        let profile = Profile {
            multikey: mk.clone(),
            version: 5,
            name: "alice".into(),
            self_signature: [0u8; 64],
        };
        let sig: [u8; 64] = kp.sign(&profile.sign_input()).unwrap().try_into().unwrap();

        // capabilities is not an input, so the same signature verifies
        // whatever a forwarder sets it to.
        assert!(mk.verify(&profile.sign_input(), &sig));

        // a signed field, by contrast, invalidates it
        let tampered = Profile {
            name: "mallory".into(),
            ..profile
        };
        assert!(!mk.verify(&tampered.sign_input(), &sig));
    }
}

// ---------- §11.5 fetch scheduling ----------
//
// Per decision 12 there are no futures: a trigger schedules a fetch and
// returns, and the response handler caches and re-runs trust evaluation. So
// the observable behaviour is "a message went out" plus the in-flight
// bookkeeping that deduplicates and expires.

mod request_profile {
    use super::*;

    fn entry(
        target: TargetRef,
        next_hop: u16,
        transport: ConnectionModule,
    ) -> Arc<RwLock<RoutingEntry>> {
        Arc::new(RwLock::new(RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 10,
            next_hop,
            transport,
            last_update: 0,
            hop_count: 0,
            local_only: false,
        }))
    }

    /// A subject reachable in user space, with a neighbour to send through.
    fn reachable_user(state: &RouterV2State) -> [u8; 8] {
        let peer = fresh_peer();
        let neighbour_id = [9u8; 8];
        state.add_neighbour_transport(peer, neighbour_id, ConnectionModule::Lan);
        bind_own_dict(state, Space::Node, 100, neighbour_id);

        let subject = [3u8; 8];
        let user = install_user(state, subject, 0);
        let e = entry(TargetRef::User(user.clone()), 100, ConnectionModule::Lan);
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
        state.routing_table.write().unwrap().set(Space::User, 40, e);
        subject
    }

    #[test]
    fn a_fetch_for_a_reachable_subject_is_sent() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);

        let out = rx.try_recv().expect("a request should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, subject.to_vec());
        assert!(!decoded.destination_is_node);
        assert_eq!(decoded.source, state.host_mk.to_id().to_vec());
        assert!(decoded.source_is_node);
        match decoded.body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 0),
            other => panic!("expected a ProfileRequest, got {other:?}"),
        }
    }

    /// The cached version is what the responder uses to decide whether it has
    /// anything fresher to send (§11.5).
    #[test]
    fn the_cached_profile_version_is_carried() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);
        state
            .users
            .read()
            .unwrap()
            .get(&subject)
            .unwrap()
            .write()
            .unwrap()
            .profile_version = 4;

        state.request_profile(subject, false, 1_000);

        let out = rx.try_recv().unwrap();
        match ManagementMessage::decode(&out.bytes[..]).unwrap().body {
            Some(Body::ProfileRequest(r)) => assert_eq!(r.cached_version, 4),
            other => panic!("expected a ProfileRequest, got {other:?}"),
        }
    }

    /// Several triggers can fire for one subject at once — the unverifiable
    /// branch of trust evaluation, a manifest with an unknown origin, and a
    /// fresher mapping version. Only one request should go out.
    #[test]
    fn repeated_triggers_produce_one_in_flight_request() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);
        state.request_profile(subject, false, 1_050);
        state.request_profile(subject, false, 1_100);

        assert!(rx.try_recv().is_ok(), "the first request goes out");
        assert!(
            rx.try_recv().is_err(),
            "the second and third must be deduplicated"
        );
    }

    /// The two index spaces are distinct subjects (§3.5), so an in-flight
    /// user fetch must not suppress a node fetch for the same 8 bytes.
    #[test]
    fn the_index_space_is_part_of_the_dedup_key() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        // make the same id reachable as a node too
        let node = install_node(&state, subject, 1, false);
        bind_own_dict(&state, Space::Node, 41, subject);
        let e = entry(TargetRef::Node(node), 100, ConnectionModule::Lan);
        state.routing_table.write().unwrap().set(Space::Node, 41, e);

        state.request_profile(subject, false, 1_000);
        state.request_profile(subject, true, 1_000);

        assert!(rx.try_recv().is_ok(), "user-space fetch");
        assert!(
            rx.try_recv().is_ok(),
            "node-space fetch is a separate subject"
        );
    }

    /// No route means nothing is sent — and nothing is recorded, so a later
    /// trigger can retry once the subject becomes reachable. Recording an
    /// unsent request would strand the subject until the sweeper ran.
    #[test]
    fn an_unreachable_subject_is_not_recorded_as_in_flight() {
        let (state, mut rx) = fresh_state();

        state.request_profile([42u8; 8], false, 1_000);
        assert!(rx.try_recv().is_err(), "nothing to send it through");

        // now make it reachable; the retry must go out
        let subject = reachable_user(&state);
        state.request_profile(subject, false, 1_100);
        assert!(rx.try_recv().is_ok(), "a later trigger must not be blocked");
    }

    /// §11.4: a fetch from a user-form node is sourced from the hosted user,
    /// not the node. Sourcing from the node is what made the response
    /// unroutable beyond a direct adjacency.
    #[test]
    fn a_fetch_from_a_user_form_node_is_sourced_from_the_hosted_user() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        let host_kp = Keypair::generate_ed25519();
        let host_user = Multikey::from(host_kp.public());
        let host_user_id = host_user.to_id();
        state.register_hosted_user(host_user_id, 1, host_user);

        state.request_profile(subject, false, 1_000);

        let out = rx.try_recv().expect("a request should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, subject.to_vec());
        assert_eq!(
            decoded.source,
            host_user_id.to_vec(),
            "user form must source from the hosted user"
        );
        assert!(!decoded.source_is_node);
    }

    /// §11.2 is best-effort with no retransmission at this layer, so a lost
    /// response must not pin the subject forever.
    #[test]
    fn a_stale_in_flight_entry_is_swept_and_the_subject_retried() {
        let (state, mut rx) = fresh_state();
        let subject = reachable_user(&state);

        state.request_profile(subject, false, 1_000);
        assert!(rx.try_recv().is_ok());

        // still in flight: suppressed
        state.request_profile(subject, false, 2_000);
        assert!(rx.try_recv().is_err());

        // past the timeout, the sweep clears it
        let timeout_ms = state.options.manifest_request_timeout * 1_000;
        state.clear_management_msgs(1_000 + timeout_ms + 1);

        state.request_profile(subject, false, 1_000 + timeout_ms + 2);
        assert!(rx.try_recv().is_ok(), "a swept subject can be retried");
    }
}

// ---------- §11.4 source addressing ----------
//
// A response is routed to the request's `source` by the same next-hop lookup
// as the request itself. §3.2 leaves a user-form node with no node entry
// anywhere in the mesh, so an envelope sourced from its node id is
// answerable only by a neighbour that happens to be adjacent — which is why
// profile fetch succeeded at one hop and failed at two.

mod source_addressing {
    use super::*;
    use crate::router_v2::{
        management::profile::{HostedProfile, SignedProfileBlob},
        OutboundMsg, PropagationForm,
    };

    /// Registers a hosted account and publishes its profile, in the order
    /// `lib.rs` uses at startup so the write-through actually lands.
    fn hosted_account(state: &RouterV2State, name: &str) -> [u8; 8] {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());
        let id = mk.to_id();
        state.register_hosted_user(id, 1, mk.clone());

        let mut profile = Profile {
            multikey: mk,
            version: 1,
            name: name.into(),
            self_signature: [0u8; 64],
        };
        profile.self_signature = kp.sign(&profile.sign_input()).unwrap().try_into().unwrap();
        state.register_hosted_profile(
            id,
            HostedProfile {
                profile,
                signed: SignedProfileBlob::default(),
            },
        );
        id
    }

    #[test]
    fn user_form_sources_from_the_user_at_the_reserved_index() {
        let (state, _rx) = fresh_state();
        let user_id = hosted_account(&state, "alice");

        assert_eq!(state.propagated_identity(), (user_id, false));
    }

    #[test]
    fn node_form_sources_from_the_node() {
        let (state, _rx) = fresh_state();
        hosted_account(&state, "alice");
        *state.propagation_form.write().unwrap() = PropagationForm::Node;

        assert_eq!(
            state.propagated_identity(),
            (state.host_mk.to_id(), true),
            "node form propagates the node, so that is what a reply can reach"
        );
    }

    /// The form is read from the *synced* value, not the desired one: a form
    /// the origin tick has not emitted yet is not something the mesh can
    /// route to.
    #[test]
    fn an_unsynced_desired_form_does_not_change_the_source() {
        let (state, _rx) = fresh_state();
        let user_id = hosted_account(&state, "alice");
        // A second hosted user makes node form *desired* (§3.2), but nothing
        // has synced it, so the mesh still only carries the user entry.
        let second = Multikey::from(Keypair::generate_ed25519().public());
        state.register_hosted_user(second.to_id(), 1, second);

        assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
        assert_eq!(state.propagated_identity(), (user_id, false));
    }

    /// Degenerate, and outside §3.2's model: such a node propagates a user
    /// entry at an index bound to nothing. The node id is no worse than what
    /// it had, and keeps the adjacent case working.
    #[test]
    fn a_node_hosting_no_users_falls_back_to_the_node_id() {
        let (state, _rx) = fresh_state();

        assert_eq!(state.propagated_identity(), (state.host_mk.to_id(), true));
    }

    const REQUESTER_USER: [u8; 8] = [7; 8];
    const REQUESTER_NODE: [u8; 8] = [8; 8];
    const NEIGHBOUR_NODE: [u8; 8] = [9; 8];

    /// Builds the responder from the 11-B line: it hosts the subject, has one
    /// neighbour, and reaches the requester's *user* two hops away through
    /// that neighbour — while holding no node entry for the requester, which
    /// is not adjacent to it either. Delivers a `ProfileRequest` sourced as
    /// given and returns the reply, if one could be routed.
    fn reply_to_request_sourced_as(source: [u8; 8], source_is_node: bool) -> Option<OutboundMsg> {
        let (state, mut rx) = fresh_state();
        let subject = hosted_account(&state, "subject");

        let neighbour = fresh_peer();
        state.add_neighbour_transport(neighbour, NEIGHBOUR_NODE, ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 100, NEIGHBOUR_NODE);

        // The requester's user entry, two hops out through the neighbour.
        let user = install_user(&state, REQUESTER_USER, 0);
        let entry = Arc::new(RwLock::new(RoutingEntry {
            target_index: 40,
            target: TargetRef::User(user.clone()),
            seq_num: SeqNum::from(0u16),
            metric: 20,
            next_hop: 100,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 2,
            local_only: true,
        }));
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, 40, entry);

        // REQUESTER_NODE deliberately gets no node-space entry and no
        // adjacency — §3.2 gives a user-form node neither.

        let envelope = ManagementMessage {
            version: 1,
            destination: subject.to_vec(),
            destination_is_node: false,
            source: source.to_vec(),
            source_is_node,
            request_id: 77,
            body: Some(Body::ProfileRequest(ProfileRequest { cached_version: 0 })),
        };
        state.on_management_received(neighbour, &envelope.encode_to_vec(), 1_000);

        rx.try_recv().ok()
    }

    /// The 11-B regression. Two hops matters: at one hop the adjacency
    /// shortcut in `next_hop_for_node` answers regardless of the source, and
    /// that is exactly what hid this.
    #[test]
    fn a_node_sourced_request_cannot_be_answered_two_hops_out() {
        assert!(
            reply_to_request_sourced_as(REQUESTER_NODE, true).is_none(),
            "no node entry and no adjacency leaves nothing to route the reply to"
        );
    }

    #[test]
    fn a_user_sourced_request_is_answered_two_hops_out() {
        let out = reply_to_request_sourced_as(REQUESTER_USER, false)
            .expect("the reply must route back along the requester's user entry");

        assert_eq!(out.transport, ConnectionModule::Lan);
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, REQUESTER_USER.to_vec());
        assert!(!decoded.destination_is_node);
        assert_eq!(decoded.request_id, 77);
        match decoded.body {
            Some(Body::ProfileResponse(r)) => {
                assert!(r.found);
                assert_eq!(r.profile.expect("profile present").name, "subject");
            }
            other => panic!("expected a ProfileResponse, got {other:?}"),
        }
    }
}

// ---------- §11.4 forward-loop suppression ----------
//
// The dedup key must separate a request from a reply that reuses its
// request_id. Under §11.4 source addressing a user-form node stamps its own
// requests with the same identity that `Addressing::reply` puts on replies it
// sources as the subject, so `(source, request_id)` alone names two unrelated
// messages — which cost the 3-hop pair on the 4-node lab line.

mod forward_dedup {
    use super::*;
    use crate::router_v2::OutboundMsg;
    use std::sync::atomic::Ordering;

    const SRC: [u8; 8] = [1; 8];
    const DST_A: [u8; 8] = [2; 8];
    const DST_B: [u8; 8] = [3; 8];

    /// A pure forwarder: no identity below is local, and both destinations
    /// are reachable through one neighbour, so every envelope is forwarded
    /// rather than dispatched.
    fn forwarding_state() -> (
        RouterV2State,
        tokio::sync::mpsc::UnboundedReceiver<OutboundMsg>,
    ) {
        let (state, rx) = fresh_state();
        state.add_neighbour_transport(fresh_peer(), [9; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 100, [9; 8]);

        for (idx, id) in [(41u16, DST_A), (42u16, DST_B)] {
            let user = install_user(&state, id, 0);
            let entry = Arc::new(RwLock::new(RoutingEntry {
                target_index: idx,
                target: TargetRef::User(user.clone()),
                seq_num: SeqNum::from(0u16),
                metric: 10,
                next_hop: 100,
                transport: ConnectionModule::Lan,
                last_update: 0,
                hop_count: 1,
                local_only: true,
            }));
            user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));
            state
                .routing_table
                .write()
                .unwrap()
                .set(Space::User, idx, entry);
        }
        (state, rx)
    }

    fn envelope(source: [u8; 8], destination: [u8; 8], request_id: u32, response: bool) -> Vec<u8> {
        let body = if response {
            Body::ProfileResponse(ProfileResponse {
                found: false,
                profile: None,
            })
        } else {
            Body::ProfileRequest(ProfileRequest { cached_version: 0 })
        };
        ManagementMessage {
            version: 1,
            destination: destination.to_vec(),
            destination_is_node: false,
            source: source.to_vec(),
            source_is_node: false,
            request_id,
            body: Some(body),
        }
        .encode_to_vec()
    }

    /// The lab regression, in the shape it actually occurred: one node's own
    /// request and a reply it sourced as the subject, same id, different
    /// destinations.
    #[test]
    fn a_reply_is_not_mistaken_for_a_request_sharing_its_id() {
        let (state, mut rx) = forwarding_state();

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_000);
        assert!(rx.try_recv().is_ok(), "the request is forwarded");

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_B, 7, true), 1_100);
        assert!(
            rx.try_recv().is_ok(),
            "a reply sharing the id must still be forwarded"
        );
    }

    /// The case that rules out keying on `(source, destination, request_id)`:
    /// when two nodes query each other at once, a reply and an unrelated
    /// request agree on source *and* destination. Only the direction bit
    /// separates them.
    #[test]
    fn a_reply_is_separated_even_when_source_and_destination_match() {
        let (state, mut rx) = forwarding_state();

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_000);
        assert!(rx.try_recv().is_ok());

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, true), 1_100);
        assert!(
            rx.try_recv().is_ok(),
            "identical but for direction — the key must still tell them apart"
        );
    }

    /// The suppression it exists for has to keep working.
    #[test]
    fn an_identical_message_is_still_dropped_as_a_loop() {
        let (state, mut rx) = forwarding_state();

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_000);
        assert!(rx.try_recv().is_ok());

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_100);
        assert!(
            rx.try_recv().is_err(),
            "a genuine re-delivery must still be suppressed"
        );
    }

    #[test]
    fn the_forward_memory_expires() {
        let (state, mut rx) = forwarding_state();

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_000);
        assert!(rx.try_recv().is_ok());

        state.on_management_received(fresh_peer(), &envelope(SRC, DST_A, 7, false), 1_000 + 5_001);
        assert!(
            rx.try_recv().is_ok(),
            "past FORWARD_MEMORY_MS the key is forgotten"
        );
    }

    /// §6.1 seeds sequence numbers randomly so independent nodes do not agree
    /// by construction. Request ids need it for the same reason: a fixed start
    /// puts symmetrically-placed nodes' counters in lockstep.
    #[test]
    fn request_ids_do_not_start_from_a_fixed_seed() {
        let first = fresh_state().0.next_request_id.load(Ordering::Relaxed);
        let any_different =
            (0..100).any(|_| fresh_state().0.next_request_id.load(Ordering::Relaxed) != first);
        assert!(any_different, "next_request_id seed appears to be constant");
    }
}
