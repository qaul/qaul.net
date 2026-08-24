// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Cross-host delegation, target side: accepting a user into this node's
//! manifest (spec §10.1, §10.3, §11.6).

use crate::connections::ConnectionModule;
use crate::node::user_accounts::UserAccount;
use crate::router_v2::{
    delegation::DelegationRequest,
    identity::Multikey,
    index::Space,
    management::Addressing,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef, User},
    test_utils::*,
    RouterV2State,
};
use libp2p::{identity::Keypair, PeerId};
use std::sync::{Arc, RwLock};

use prost::Message;
use proto::{management_message::Body, DelegationSubscribe, ManagementMessage};
use qaul_proto::qaul_net_router_management as proto;

const NOW: u64 = 1_000_000_000;
const TTL_MS: u64 = 6 * 60 * 60 * 1000;

fn fresh_account() -> UserAccount {
    let keys = Keypair::generate_ed25519();
    UserAccount {
        id: PeerId::from(keys.public()),
        keys,
        name: "subscriber".into(),
        password_hash: None,
        password_salt: None,
    }
}

/// Puts the delegating user in the users map with its *real* key, so a
/// signature it produced actually verifies. `install_user` mints a random
/// key, which is the wrong fixture here.
fn install_user_with_key(state: &RouterV2State, id: [u8; 8], key: Multikey) {
    state.users.write().unwrap().insert(
        id,
        User {
            id,
            public_key: Some(key),
            profile_version: 3,
            routing_entry: None,
            delegation_gateways: Vec::new(),
            is_hosted: false,
        },
    );
}

/// A subscribe as the user would build it: signed over the *target's*
/// multikey, which is the node being asked to carry them (§10.1).
fn subscribe_from(
    state: &RouterV2State,
    account: &UserAccount,
    timeout: u64,
) -> DelegationSubscribe {
    let delegation = account.issue_self_delegation(&state.host_mk, timeout);
    DelegationSubscribe {
        user_id: account.routing_user_id().to_vec(),
        timeout,
        entry_signature: delegation.entry_signature.to_vec(),
    }
}

fn addressed_to_us(state: &RouterV2State, source: [u8; 8]) -> Addressing {
    Addressing {
        destination: state.host_mk.to_id(),
        destination_is_node: true,
        source,
        source_is_node: false,
        request_id: 7,
    }
}

fn manifest_holds(state: &RouterV2State, user_id: &[u8; 8]) -> bool {
    state.has_self_delegation(user_id)
}

#[test]
fn a_verified_subscribe_joins_the_manifest() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(manifest_holds(&state, &user_id));
    assert!(
        state.dirty_delegations.read().unwrap().contains(&user_id),
        "an accepted subscribe must ride the next accumulated bump"
    );
}

/// The entry carries the profile_version we hold for the user, not zero —
/// `upsert_entry` replaces the whole record.
#[test]
fn an_accepted_entry_carries_the_cached_profile_version() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    let manifest = state.manifest.read().unwrap();
    let entry = manifest
        .entries()
        .iter()
        .find(|e| e.user_id == user_id)
        .expect("entry recorded");
    assert_eq!(entry.profile_version, 3);
}

/// A signature made for a *different* target must not verify here — that
/// is the whole point of signing over the target's multikey.
#[test]
fn a_subscribe_signed_for_another_target_is_rejected() {
    let (state, _rx) = fresh_state();
    let (other, _rx2) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let req = subscribe_from(&other, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(!manifest_holds(&state, &user_id));
}

#[test]
fn a_subscribe_with_a_corrupt_signature_is_rejected() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let mut req = subscribe_from(&state, &account, NOW + TTL_MS);
    req.entry_signature[0] ^= 0xff;
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(!manifest_holds(&state, &user_id));
}

#[test]
fn an_already_expired_subscribe_is_rejected() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let req = subscribe_from(&state, &account, NOW - 1);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(!manifest_holds(&state, &user_id));
}

/// §10.1 signs over the target's multikey, so a subscribe naming someone
/// else is not ours to verify — §11.4 should have forwarded it.
#[test]
fn a_subscribe_addressed_elsewhere_is_ignored() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_user_with_key(&state, user_id, account.multikey());

    let mut addressing = addressed_to_us(&state, user_id);
    addressing.destination = [42; 8];

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressing, req, NOW);

    assert!(!manifest_holds(&state, &user_id));
    assert!(state.pending_subscribes.read().unwrap().is_empty());
}

// ----- parking behind a §11.5 fetch -----

/// The realistic parking case: the user is known and routable — learned
/// from a routing entry — but we hold no key for them yet.
fn install_keyless_reachable_user(state: &RouterV2State, id: [u8; 8]) {
    let peer = fresh_peer();
    let neighbour_id = [9u8; 8];
    state.add_neighbour_transport(peer, neighbour_id, ConnectionModule::Lan);
    bind_own_dict(state, Space::Node, 100, neighbour_id);

    state.users.write().unwrap().insert(
        id,
        User {
            id,
            public_key: None,
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
            is_hosted: false,
        },
    );
    let user = state.users.read().unwrap().get(&id).unwrap();

    let e = Arc::new(RwLock::new(RoutingEntry {
        target_index: 0,
        target: TargetRef::User(user.clone()),
        seq_num: SeqNum::from(0u16),
        metric: 10,
        next_hop: 100,
        transport: ConnectionModule::Lan,
        last_update: 0,
        hop_count: 0,
        local_only: false,
    }));
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
    state.routing_table.write().unwrap().set(Space::User, 40, e);
}

#[test]
fn a_subscribe_for_an_unknown_key_parks_and_fetches() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();
    install_keyless_reachable_user(&state, user_id);

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(!manifest_holds(&state, &user_id));
    assert_eq!(
        state
            .pending_subscribes
            .read()
            .unwrap()
            .get(&user_id)
            .map(|v| v.len()),
        Some(1),
    );
    assert!(
        state
            .management_in_flight
            .read()
            .unwrap()
            .contains_key(&(user_id, false)),
        "parking must be paired with a §11.5 fetch, or it never resolves"
    );
}

/// A subject we cannot reach at all still parks — `request_profile` is
/// best-effort (§11.2) and records nothing when it cannot send — and the
/// park is reclaimed by the timeout sweep rather than lingering.
#[test]
fn a_subscribe_for_an_unreachable_subject_parks_without_a_fetch() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    assert!(state.management_in_flight.read().unwrap().is_empty());
    assert!(!state.pending_subscribes.read().unwrap().is_empty());

    let window_ms = state.options.manifest_request_timeout * 1000;
    state.clear_pending_subscribes(NOW + window_ms + 1);
    assert!(state.pending_subscribes.read().unwrap().is_empty());
}

#[test]
fn a_parked_subscribe_settles_once_the_key_arrives() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);
    assert!(!manifest_holds(&state, &user_id));

    // the §11.5 response landing
    install_user_with_key(&state, user_id, account.multikey());
    state.resume_pending_subscribes(&user_id, NOW + 500);

    assert!(manifest_holds(&state, &user_id));
    assert!(state.pending_subscribes.read().unwrap().is_empty());
}

/// The key arriving is what makes a verdict possible, not what makes it
/// positive.
#[test]
fn a_parked_subscribe_can_still_be_rejected_on_resume() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();

    let mut req = subscribe_from(&state, &account, NOW + TTL_MS);
    req.entry_signature[10] ^= 0xff;
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    install_user_with_key(&state, user_id, account.multikey());
    state.resume_pending_subscribes(&user_id, NOW + 500);

    assert!(!manifest_holds(&state, &user_id));
    assert!(state.pending_subscribes.read().unwrap().is_empty());
}

#[test]
fn a_parked_subscribe_expires_when_the_fetch_never_returns() {
    let (state, _rx) = fresh_state();
    let account = fresh_account();
    let user_id = account.routing_user_id();

    let req = subscribe_from(&state, &account, NOW + TTL_MS);
    state.handle_delegation_subscribe(addressed_to_us(&state, user_id), req, NOW);

    let window_ms = state.options.manifest_request_timeout * 1000;
    state.clear_pending_subscribes(NOW + window_ms - 1);
    assert!(!state.pending_subscribes.read().unwrap().is_empty());

    state.clear_pending_subscribes(NOW + window_ms + 1);
    assert!(state.pending_subscribes.read().unwrap().is_empty());
}

/// Two users parking at once must not clear each other.
#[test]
fn parked_subscribes_are_tracked_per_user() {
    let (state, _rx) = fresh_state();
    let a = fresh_account();
    let b = fresh_account();

    state.handle_delegation_subscribe(
        addressed_to_us(&state, a.routing_user_id()),
        subscribe_from(&state, &a, NOW + TTL_MS),
        NOW,
    );
    state.handle_delegation_subscribe(
        addressed_to_us(&state, b.routing_user_id()),
        subscribe_from(&state, &b, NOW + TTL_MS),
        NOW,
    );

    install_user_with_key(&state, a.routing_user_id(), a.multikey());
    state.resume_pending_subscribes(&a.routing_user_id(), NOW + 500);

    assert!(manifest_holds(&state, &a.routing_user_id()));
    assert_eq!(
        state
            .pending_subscribes
            .read()
            .unwrap()
            .get(&b.routing_user_id())
            .map(|v| v.len()),
        Some(1),
    );
}

/// Installs a node in the given gateway state with a node-space routing
/// entry carrying `local_only` and `metric`.
fn install_gateway(
    state: &RouterV2State,
    id: [u8; 8],
    idx: u16,
    is_gateway: bool,
    local_only: bool,
    metric: u16,
) {
    let node = install_node(state, id, 1, is_gateway);
    bind_own_dict(state, Space::Node, idx, id);

    let e = Arc::new(RwLock::new(RoutingEntry {
        target_index: idx,
        target: TargetRef::Node(node),
        seq_num: SeqNum::from(0u16),
        metric,
        next_hop: idx,
        transport: ConnectionModule::Lan,
        last_update: 0,
        hop_count: 1,
        local_only,
    }));
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, idx, e);
}

/// An eligible gateway that is also a direct neighbour, so a subscribe
/// addressed to it can actually leave the node.
fn install_reachable_gateway(state: &RouterV2State, id: [u8; 8], idx: u16, metric: u16) {
    state.add_neighbour_transport(fresh_peer(), id, ConnectionModule::Lan);
    install_gateway(state, id, idx, true, true, metric);
}

// ----- §10.3 gateway selection, issuing side -----

mod selection {
    use super::*;
    use crate::router_v2::delegation::GatewayCandidate;

    fn ids(candidates: &[GatewayCandidate]) -> Vec<[u8; 8]> {
        candidates.iter().map(|c| c.node_id).collect()
    }

    #[test]
    fn a_local_only_gateway_is_eligible() {
        let (state, _rx) = fresh_state();
        install_gateway(&state, [1; 8], 10, true, true, 20);

        assert_eq!(ids(&state.eligible_gateways()), vec![[1u8; 8]]);
    }

    /// Criterion 2: a non-gateway would carry the user but its manifest
    /// would never cross the Internet sphere.
    #[test]
    fn a_non_gateway_is_not_eligible() {
        let (state, _rx) = fresh_state();
        install_gateway(&state, [1; 8], 10, false, true, 20);

        assert!(state.eligible_gateways().is_empty());
    }

    /// Criterion 1: over a path that leaves the Local sphere the user can
    /// neither reliably convey the delegation nor monitor the target.
    #[test]
    fn a_gateway_reached_across_spheres_is_not_eligible() {
        let (state, _rx) = fresh_state();
        install_gateway(&state, [1; 8], 10, true, false, 20);

        assert!(state.eligible_gateways().is_empty());
    }

    /// This is exactly where §10.3 selection and forwarding's anycast part
    /// ways: `nearest_gateway` would take this node, `eligible_gateways`
    /// must not.
    #[test]
    fn selection_is_stricter_than_the_forwarding_anycast() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [1; 8], ConnectionModule::Internet);
        install_gateway(&state, [1; 8], 10, true, false, 20);

        assert!(state.nearest_gateway().is_some());
        assert!(state.eligible_gateways().is_empty());
    }

    #[test]
    fn candidates_come_back_best_metric_first() {
        let (state, _rx) = fresh_state();
        install_gateway(&state, [1; 8], 10, true, true, 50);
        install_gateway(&state, [2; 8], 11, true, true, 10);
        install_gateway(&state, [3; 8], 12, true, true, 30);

        assert_eq!(
            ids(&state.eligible_gateways()),
            vec![[2u8; 8], [3u8; 8], [1u8; 8]]
        );
    }

    /// A gateway with no node-space routing entry has no `local_only` to
    /// test, so criterion 1 cannot be satisfied.
    #[test]
    fn a_gateway_with_no_routing_entry_is_not_eligible() {
        let (state, _rx) = fresh_state();
        install_node(&state, [1; 8], 1, true);

        assert!(state.eligible_gateways().is_empty());
    }

    /// §10.1 signs over the target's full key, so selection has to report
    /// whether we hold one — a candidate without it needs a §11.5 fetch
    /// before it can be delegated to.
    #[test]
    fn a_candidate_reports_whether_its_key_is_held() {
        let (state, _rx) = fresh_state();
        install_gateway(&state, [1; 8], 10, true, true, 20);
        state
            .nodes
            .read()
            .unwrap()
            .get(&[1; 8])
            .unwrap()
            .write()
            .unwrap()
            .public_key = None;

        let candidates = state.eligible_gateways();
        assert_eq!(candidates.len(), 1);
        assert!(candidates[0].multikey.is_none());
    }
}

// ----- §11.6 issuing and ack handling -----

mod issuing {
    use super::*;
    use proto::DelegationSubscribeAck;

    /// `issued_at` matters: a refresh re-signs with a *new* timeout, and
    /// that is what takes the user back out of the refresh window.
    pub(super) fn request_to(
        state: &RouterV2State,
        account: &UserAccount,
        target: [u8; 8],
        issued_at: u64,
    ) -> DelegationRequest {
        let target_mk = state
            .nodes
            .read()
            .unwrap()
            .get(&target)
            .unwrap()
            .read()
            .unwrap()
            .public_key
            .clone()
            .unwrap();
        DelegationRequest {
            user_id: account.routing_user_id(),
            target_node_id: target,
            delegation: account.issue_self_delegation(&target_mk, issued_at + TTL_MS),
        }
    }

    pub(super) fn ack_from(target: [u8; 8], user_id: [u8; 8], request_id: u32) -> Addressing {
        Addressing {
            destination: user_id,
            destination_is_node: false,
            source: target,
            source_is_node: true,
            request_id,
        }
    }

    /// The only request_id in flight after a send.
    pub(super) fn sole_request_id(state: &RouterV2State) -> u32 {
        let outstanding = state.outstanding_subscribes.read().unwrap();
        assert_eq!(outstanding.len(), 1);
        *outstanding.keys().next().unwrap()
    }

    #[test]
    fn a_subscribe_goes_out_and_is_remembered() {
        let (state, mut rx) = fresh_state();
        let account = fresh_account();
        install_reachable_gateway(&state, [1; 8], 10, 20);

        assert!(state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW));

        let out = rx.try_recv().expect("a subscribe should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, [1u8; 8].to_vec());
        assert!(decoded.destination_is_node);
        // §11.6 is an exchange between a user and a gateway, so the user is
        // the source — that is what routes the ack back to the signer.
        assert_eq!(decoded.source, account.routing_user_id().to_vec());
        assert!(!decoded.source_is_node);
        assert!(matches!(decoded.body, Some(Body::DelegationSubscribe(_))));
        assert_eq!(state.outstanding_subscribes.read().unwrap().len(), 1);
    }

    /// §11.2 is best-effort: with no route there is nothing to wait for, so
    /// nothing is recorded either.
    #[test]
    fn an_unsendable_subscribe_records_nothing() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        install_node(&state, [1; 8], 1, true);

        let target_mk = state
            .nodes
            .read()
            .unwrap()
            .get(&[1; 8])
            .unwrap()
            .read()
            .unwrap()
            .public_key
            .clone()
            .unwrap();
        let request = DelegationRequest {
            user_id: account.routing_user_id(),
            target_node_id: [1; 8],
            delegation: account.issue_self_delegation(&target_mk, NOW + TTL_MS),
        };

        assert!(!state.send_delegation_subscribe(request, NOW));
        assert!(state.outstanding_subscribes.read().unwrap().is_empty());
    }

    #[test]
    fn an_accepting_ack_settles_the_user() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 20);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);
        let request_id = sole_request_id(&state);

        state.handle_delegation_subscribe_ack(
            ack_from([1; 8], user_id, request_id),
            DelegationSubscribeAck {
                accepted: true,
                reason: 0,
            },
            NOW + 100,
        );

        assert!(state.has_live_subscription(&user_id, NOW + 100));
        assert!(state.outstanding_subscribes.read().unwrap().is_empty());
        assert!(
            state
                .select_delegation_target(&user_id, NOW + 100)
                .is_none(),
            "a settled user must not be re-subscribed on the next tick"
        );
    }

    /// §10.3: on rejection the user tries a different acceptable gateway.
    #[test]
    fn a_refusal_moves_the_user_to_the_next_gateway() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 20);

        let best = state.select_delegation_target(&user_id, NOW).unwrap();
        assert_eq!(best.node_id, [1; 8]);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);
        let request_id = sole_request_id(&state);

        state.handle_delegation_subscribe_ack(
            ack_from([1; 8], user_id, request_id),
            DelegationSubscribeAck {
                accepted: false,
                reason: 1,
            },
            NOW + 100,
        );

        assert!(!state.has_live_subscription(&user_id, NOW + 100));
        let next = state.select_delegation_target(&user_id, NOW + 100).unwrap();
        assert_eq!(next.node_id, [2; 8], "the refusing gateway must be skipped");
    }

    /// A silent gateway is as unusable as one that refused.
    #[test]
    fn a_timed_out_subscribe_frees_the_user_and_skips_the_target() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 20);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);

        assert!(
            state.select_delegation_target(&user_id, NOW).is_none(),
            "a subscribe in flight blocks a second one"
        );

        let window = state.options.manifest_request_timeout * 1000;
        state.clear_delegation_state(NOW + window + 1);

        assert!(state.outstanding_subscribes.read().unwrap().is_empty());
        let next = state
            .select_delegation_target(&user_id, NOW + window + 1)
            .unwrap();
        assert_eq!(next.node_id, [2; 8]);
    }

    /// AT_CAPACITY is transient, so a refusal has to fade or the gateway is
    /// lost for the lifetime of the process.
    #[test]
    fn a_refusal_fades_and_the_gateway_is_reconsidered() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);
        let request_id = sole_request_id(&state);
        state.handle_delegation_subscribe_ack(
            ack_from([1; 8], user_id, request_id),
            DelegationSubscribeAck {
                accepted: false,
                reason: 1,
            },
            NOW,
        );

        assert!(state.select_delegation_target(&user_id, NOW).is_none());

        let later = NOW + 5 * 60 * 1000 + 1;
        state.clear_delegation_state(later);
        assert_eq!(
            state
                .select_delegation_target(&user_id, later)
                .unwrap()
                .node_id,
            [1; 8]
        );
    }

    #[test]
    fn an_ack_from_the_wrong_node_is_discarded() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);
        let request_id = sole_request_id(&state);

        state.handle_delegation_subscribe_ack(
            ack_from([9; 8], user_id, request_id),
            DelegationSubscribeAck {
                accepted: true,
                reason: 0,
            },
            NOW + 100,
        );

        assert!(!state.has_live_subscription(&user_id, NOW + 100));
        assert_eq!(
            state.outstanding_subscribes.read().unwrap().len(),
            1,
            "a mismatched ack must not consume the outstanding request"
        );
    }

    /// §10.1 needs the target's full key before anything can be signed, so
    /// selection asks for it rather than falling to a worse candidate.
    #[test]
    fn a_keyless_best_candidate_is_fetched_not_skipped() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 20);
        state
            .nodes
            .read()
            .unwrap()
            .get(&[1; 8])
            .unwrap()
            .write()
            .unwrap()
            .public_key = None;

        assert!(state
            .select_delegation_target(&account.routing_user_id(), NOW)
            .is_none());
        assert!(
            state
                .management_in_flight
                .read()
                .unwrap()
                .contains_key(&([1u8; 8], true)),
            "the missing key must be fetched as a node profile (§11.5)"
        );
    }
}

// ----- §10.3 break monitor and §10.4 refresh -----

mod lifecycle {
    use super::issuing::*;
    use super::*;
    use proto::DelegationSubscribeAck;

    /// Drives a user to a settled subscription with `target`.
    fn settle(state: &RouterV2State, account: &UserAccount, target: [u8; 8], now: u64) {
        state.send_delegation_subscribe(request_to(state, account, target, now), now);
        let request_id = sole_request_id(state);
        state.handle_delegation_subscribe_ack(
            ack_from(target, account.routing_user_id(), request_id),
            DelegationSubscribeAck {
                accepted: true,
                reason: 0,
            },
            now,
        );
    }

    /// Rewrites the target's node-space routing entry.
    fn set_entry(state: &RouterV2State, id: [u8; 8], idx: u16, local_only: bool) {
        install_gateway(state, id, idx, true, local_only, 10);
    }

    #[test]
    fn a_target_that_stops_being_a_gateway_breaks_the_subscription() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        settle(&state, &account, [1; 8], NOW);
        assert!(state.has_live_subscription(&user_id, NOW));

        state
            .nodes
            .read()
            .unwrap()
            .get(&[1; 8])
            .unwrap()
            .write()
            .unwrap()
            .is_gateway = false;
        state.clear_delegation_state(NOW + 1);

        assert!(!state.has_live_subscription(&user_id, NOW + 1));
    }

    /// §10.3: `local_only` 1 → 0 means the user can no longer reliably
    /// reach or monitor the target, so the delegation is broken.
    #[test]
    fn a_target_leaving_the_local_sphere_breaks_the_subscription() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        settle(&state, &account, [1; 8], NOW);

        set_entry(&state, [1; 8], 10, false);
        state.clear_delegation_state(NOW + 1);

        assert!(!state.has_live_subscription(&user_id, NOW + 1));
    }

    #[test]
    fn a_target_whose_route_expired_breaks_the_subscription() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        settle(&state, &account, [1; 8], NOW);

        state.routing_table.write().unwrap().clear(Space::Node, 10);
        state.clear_delegation_state(NOW + 1);

        assert!(!state.has_live_subscription(&user_id, NOW + 1));
    }

    #[test]
    fn a_broken_subscription_re_selects_another_gateway() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 20);
        settle(&state, &account, [1; 8], NOW);

        set_entry(&state, [1; 8], 10, false);
        state.clear_delegation_state(NOW + 1);

        assert_eq!(
            state
                .select_delegation_target(&user_id, NOW + 1)
                .unwrap()
                .node_id,
            [2; 8]
        );
    }

    /// A healthy subscription is left alone until the refresh window opens.
    #[test]
    fn a_settled_user_is_not_touched_before_the_refresh_window() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        settle(&state, &account, [1; 8], NOW);

        state.clear_delegation_state(NOW + 1);
        assert!(state.select_delegation_target(&user_id, NOW + 1).is_none());
    }

    /// §10.4: inside TTL/2, re-issue — and to the *same* gateway, since a
    /// refresh is the original conveyed again.
    #[test]
    fn the_refresh_window_re_targets_the_same_gateway() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 5); // better metric
        settle(&state, &account, [1; 8], NOW);

        let refresh_ms = state.options.delegation_referesh * 1000;
        let due = NOW + TTL_MS - refresh_ms;

        let target = state.select_delegation_target(&user_id, due).unwrap();
        assert_eq!(
            target.node_id, [1; 8],
            "a refresh must not silently migrate the user to a better gateway"
        );
    }

    /// If the current target has stopped qualifying by the time the refresh
    /// is due, the refresh becomes a re-selection.
    #[test]
    fn a_refresh_against_a_broken_target_picks_a_new_one() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        install_reachable_gateway(&state, [2; 8], 11, 20);
        settle(&state, &account, [1; 8], NOW);

        set_entry(&state, [1; 8], 10, false);
        let refresh_ms = state.options.delegation_referesh * 1000;
        let due = NOW + TTL_MS - refresh_ms;

        assert_eq!(
            state
                .select_delegation_target(&user_id, due)
                .unwrap()
                .node_id,
            [2; 8]
        );
    }

    /// A refreshed subscription carries the new timeout.
    #[test]
    fn a_refresh_ack_extends_the_subscription() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        settle(&state, &account, [1; 8], NOW);

        let refresh_ms = state.options.delegation_referesh * 1000;
        let due = NOW + TTL_MS - refresh_ms;
        settle(&state, &account, [1; 8], due);

        let subscriptions = state.subscriptions.read().unwrap();
        let sub = subscriptions.get(&user_id).unwrap();
        assert_eq!(sub.acked_at_ms, due);
        assert!(state.select_delegation_target(&user_id, due).is_none());
    }
}

// ----- §10.7 gateway-side reachability sweep -----

mod reachability {
    use super::*;
    use crate::router_v2::BumpTrigger;

    const GRACE_MS: u64 = 60_000;

    /// Puts `user` in our manifest the way an accepted subscribe would.
    fn carry(state: &RouterV2State, account: &UserAccount) -> [u8; 8] {
        let user_id = account.routing_user_id();
        state.add_self_delegation(
            user_id,
            0,
            account.issue_self_delegation(&state.host_mk, NOW + TTL_MS),
        );
        user_id
    }

    /// A foreign user with a live routing entry, i.e. deliverable.
    fn make_reachable(state: &RouterV2State, user_id: [u8; 8], idx: u16) {
        install_user_with_key(state, user_id, fresh_multikey());
        let user = state.users.read().unwrap().get(&user_id).unwrap();
        let e = Arc::new(RwLock::new(RoutingEntry {
            target_index: idx,
            target: TargetRef::User(user.clone()),
            seq_num: SeqNum::from(0u16),
            metric: 10,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: NOW,
            hop_count: 1,
            local_only: false,
        }));
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&e));
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, idx, e);
    }

    /// The trap: a hosted user never has a routing entry, and under node
    /// form (§3.2) has no user index either. Testing it like a foreign
    /// entry would evict our own users on the first tick.
    #[test]
    fn a_hosted_user_is_never_swept() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        state.register_hosted_user(user_id, 0, account.multikey());

        assert!(!state.sweep_unreachable_delegations(NOW));
        assert!(!state.sweep_unreachable_delegations(NOW + GRACE_MS * 10));
        assert!(state.has_self_delegation(&user_id));
    }

    #[test]
    fn a_reachable_delegated_user_is_kept() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        make_reachable(&state, user_id, 30);

        assert!(!state.sweep_unreachable_delegations(NOW + GRACE_MS * 10));
        assert!(state.has_self_delegation(&user_id));
    }

    /// A single unreachable observation starts the clock; it does not evict.
    #[test]
    fn one_unreachable_tick_does_not_evict() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        make_reachable(&state, user_id, 30);
        state.sweep_unreachable_delegations(NOW);

        state.routing_table.write().unwrap().clear(Space::User, 30);

        assert!(!state.sweep_unreachable_delegations(NOW + 1));
        assert!(state.has_self_delegation(&user_id));
    }

    #[test]
    fn an_unreachable_user_is_dropped_past_the_grace() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        make_reachable(&state, user_id, 30);
        state.sweep_unreachable_delegations(NOW);

        state.routing_table.write().unwrap().clear(Space::User, 30);
        state.sweep_unreachable_delegations(NOW + 1);

        assert!(state.sweep_unreachable_delegations(NOW + GRACE_MS + 2));
        assert!(!state.has_self_delegation(&user_id));
    }

    /// §10.8 exempts the removal from the 60 s window, so the black hole
    /// clears without waiting for the next accumulated bump.
    #[test]
    fn the_removal_bumps_inside_the_rate_limit_window() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        make_reachable(&state, user_id, 30);
        state.try_bump_manifest_version(NOW, BumpTrigger::Accumulated);
        let before = state.manifest.read().unwrap().manifest_version;

        state.routing_table.write().unwrap().clear(Space::User, 30);
        state.sweep_unreachable_delegations(NOW + 1);
        assert!(state.sweep_unreachable_delegations(NOW + GRACE_MS + 2));

        let after = state.manifest.read().unwrap().manifest_version;
        assert_eq!(
            after,
            before + 1,
            "the forced removal must not be rate-limited"
        );
    }

    /// Reachability returning before the grace elapses resets the clock.
    #[test]
    fn recovering_before_the_grace_resets_the_clock() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        make_reachable(&state, user_id, 30);
        state.sweep_unreachable_delegations(NOW);

        state.routing_table.write().unwrap().clear(Space::User, 30);
        state.sweep_unreachable_delegations(NOW + 1);

        make_reachable(&state, user_id, 30);
        state.sweep_unreachable_delegations(NOW + GRACE_MS / 2);

        state.routing_table.write().unwrap().clear(Space::User, 30);
        assert!(!state.sweep_unreachable_delegations(NOW + GRACE_MS));
        assert!(state.has_self_delegation(&user_id));
    }

    /// A user we have never routed to is unreachable, but the clock still
    /// starts on first sight rather than evicting immediately.
    #[test]
    fn a_never_seen_user_starts_the_clock_then_drops() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);

        assert!(!state.sweep_unreachable_delegations(NOW));
        assert!(state.sweep_unreachable_delegations(NOW + GRACE_MS + 1));
        assert!(!state.has_self_delegation(&user_id));
    }

    #[test]
    fn liveness_does_not_accumulate_for_entries_that_left() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carry(&state, &account);
        state.sweep_unreachable_delegations(NOW);
        assert_eq!(state.delegation_liveness.read().unwrap().len(), 1);

        state.remove_self_delegation(&user_id);
        state.sweep_unreachable_delegations(NOW + 1);

        assert!(state.delegation_liveness.read().unwrap().is_empty());
    }
}

// ----- §10.5 / §11.7 revocation -----

mod revoke {
    use super::issuing::*;
    use super::*;
    use proto::{DelegationRevoke, DelegationSubscribeAck};

    /// §10.5: the revocation is a signature over the same content as the
    /// entry it cancels, so the delegation signature *is* the revocation.
    fn revoke_for(state: &RouterV2State, account: &UserAccount, timeout: u64) -> DelegationRevoke {
        let delegation = account.issue_self_delegation(&state.host_mk, timeout);
        DelegationRevoke {
            user_id: account.routing_user_id().to_vec(),
            timeout,
            revoke_signature: delegation.entry_signature.to_vec(),
        }
    }

    /// Gets the user carried in our manifest, as an accepted subscribe would.
    fn carried(state: &RouterV2State, account: &UserAccount) -> [u8; 8] {
        let user_id = account.routing_user_id();
        install_user_with_key(state, user_id, account.multikey());
        state.handle_delegation_subscribe(
            addressed_to_us(state, user_id),
            subscribe_from(state, account, NOW + TTL_MS),
            NOW,
        );
        assert!(state.has_self_delegation(&user_id));
        user_id
    }

    #[test]
    fn a_valid_revoke_removes_the_entry() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carried(&state, &account);

        state.handle_delegation_revoke(
            addressed_to_us(&state, user_id),
            revoke_for(&state, &account, NOW + TTL_MS),
            NOW + 1,
        );

        assert!(!state.has_self_delegation(&user_id));
    }

    #[test]
    fn a_revoke_with_a_bad_signature_leaves_the_entry() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carried(&state, &account);

        let mut req = revoke_for(&state, &account, NOW + TTL_MS);
        req.revoke_signature[0] ^= 0xff;
        state.handle_delegation_revoke(addressed_to_us(&state, user_id), req, NOW + 1);

        assert!(state.has_self_delegation(&user_id));
    }

    /// §10.5's replay protection: a revocation only ever cancels the one
    /// delegation its timeout names.
    #[test]
    fn a_revoke_naming_another_timeout_is_a_no_op() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carried(&state, &account);

        state.handle_delegation_revoke(
            addressed_to_us(&state, user_id),
            revoke_for(&state, &account, NOW + TTL_MS + 1),
            NOW + 1,
        );

        assert!(
            state.has_self_delegation(&user_id),
            "a stale revoke must not cancel a later delegation"
        );
    }

    /// §11.7: removal of an already-absent entry is a successful no-op.
    #[test]
    fn revoking_an_absent_entry_is_a_successful_no_op() {
        let (state, mut rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_keyless_reachable_user(&state, user_id);
        state
            .users
            .read()
            .unwrap()
            .get(&user_id)
            .unwrap()
            .write()
            .unwrap()
            .public_key = Some(account.multikey());

        state.handle_delegation_revoke(
            addressed_to_us(&state, user_id),
            revoke_for(&state, &account, NOW + TTL_MS),
            NOW,
        );

        let out = rx.try_recv().expect("an ack should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        match decoded.body {
            Some(Body::DelegationRevokeAck(ack)) => assert!(ack.done),
            other => panic!("expected a revoke ack, got {other:?}"),
        }
    }

    /// A revoke for a user whose key we lack parks behind the same §11.5
    /// fetch as a subscribe, and completes when it lands.
    #[test]
    fn a_revoke_parks_when_the_key_is_missing() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = carried(&state, &account);
        state
            .users
            .read()
            .unwrap()
            .get(&user_id)
            .unwrap()
            .write()
            .unwrap()
            .public_key = None;

        state.handle_delegation_revoke(
            addressed_to_us(&state, user_id),
            revoke_for(&state, &account, NOW + TTL_MS),
            NOW + 1,
        );
        assert!(state.has_self_delegation(&user_id));
        assert!(!state.pending_subscribes.read().unwrap().is_empty());

        install_user_with_key(&state, user_id, account.multikey());
        state.resume_pending_subscribes(&user_id, NOW + 2);

        assert!(!state.has_self_delegation(&user_id));
    }

    // ----- issuing side -----

    /// §10.3: a broken delegation is queued for §10.5's fast path rather
    /// than being left purely to the TTL lapse.
    #[test]
    fn a_broken_delegation_queues_a_revocation() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);
        state.send_delegation_subscribe(request_to(&state, &account, [1; 8], NOW), NOW);
        let request_id = sole_request_id(&state);
        state.handle_delegation_subscribe_ack(
            ack_from([1; 8], user_id, request_id),
            DelegationSubscribeAck {
                accepted: true,
                reason: 0,
            },
            NOW,
        );

        install_gateway(&state, [1; 8], 10, true, false, 10); // local_only → 0
        state.clear_delegation_state(NOW + 1);

        let queued = state.drain_pending_revocations();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].user_id, user_id);
        assert_eq!(queued[0].target_node_id, [1; 8]);
        assert_eq!(queued[0].timeout, NOW + TTL_MS);
        assert!(
            state.drain_pending_revocations().is_empty(),
            "draining must not hand the same revocation out twice"
        );
    }

    #[test]
    fn a_revoke_goes_out_addressed_to_the_old_gateway() {
        let (state, mut rx) = fresh_state();
        let account = fresh_account();
        let user_id = account.routing_user_id();
        install_reachable_gateway(&state, [1; 8], 10, 10);

        let target_mk = state.node_public_key(&[1; 8]).unwrap();
        assert!(state.send_delegation_revoke(DelegationRequest {
            user_id,
            target_node_id: [1; 8],
            delegation: account.issue_self_delegation(&target_mk, NOW + TTL_MS),
        }));

        let out = rx.try_recv().expect("a revoke should have been sent");
        let decoded = ManagementMessage::decode(&out.bytes[..]).unwrap();
        assert_eq!(decoded.destination, [1u8; 8].to_vec());
        assert!(decoded.destination_is_node);
        assert_eq!(decoded.source, user_id.to_vec());
        match decoded.body {
            Some(Body::DelegationRevoke(r)) => assert_eq!(r.timeout, NOW + TTL_MS),
            other => panic!("expected a revoke, got {other:?}"),
        }
    }
}
