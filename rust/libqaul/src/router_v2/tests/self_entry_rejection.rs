// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §7.1: a node originates its own routing entry and never learns one.
//! Split horizon (§7.3) blocks the simple topologies; a ring does not, so
//! the receive path rejects entries naming an identity we host.

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::{NodeEntry, UserEntry},
    index::{Space, RESERVED_INDEX},
    receive::ReceiveCtx,
    test_utils::*,
};
use libp2p::PeerId;

const UPSTREAM_NODE_ID: [u8; 8] = [77; 8];
const UPSTREAM_IDX: u16 = 500;
const INCOMING_IDX: u16 = 9;

fn ctx(peer: PeerId) -> ReceiveCtx {
    ReceiveCtx {
        neighbour: peer,
        transport: ConnectionModule::Lan,
        rssi_dbm: None,
        now: 1_000,
    }
}

/// A neighbour that advertises `target_id` at `INCOMING_IDX`, with its own
/// node id bound so next-hop resolution would succeed if the entry were
/// accepted.
fn neighbour_advertising(state: &RouterV2State, space: Space, target_id: [u8; 8]) -> PeerId {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, UPSTREAM_NODE_ID, ConnectionModule::Lan);
    bind_mirror(state, peer, space, INCOMING_IDX, target_id);
    bind_own_dict(state, Space::Node, UPSTREAM_IDX, UPSTREAM_NODE_ID);
    peer
}

fn user_entry(seq: u16, metric: u16) -> UserEntry {
    UserEntry {
        abs_idx: INCOMING_IDX,
        seq,
        metric,
        hop_count: 2,
        local_only: true,
    }
}

fn node_entry(seq: u16, metric: u16) -> NodeEntry {
    NodeEntry {
        abs_idx: INCOMING_IDX,
        seq,
        metric,
        hop_count: 2,
        local_only: true,
        manifest_version: 0,
    }
}

// ------------------------------------------------------------ user space

/// The ring case from the audit: A—B—C—A, A hosts U. C's next hop for U is
/// B, so split horizon lets C relay U back to A. A's own slot is empty, so
/// §7.2 would accept it unconditionally.
#[test]
fn an_entry_for_our_own_hosted_user_is_rejected() {
    let (state, _rx) = fresh_state();
    let hosted = [1; 8];
    state.register_hosted_user(hosted, 0, fresh_multikey());
    let peer = neighbour_advertising(&state, Space::User, hosted);

    state
        .apply_user_entry(&ctx(peer), user_entry(42, 100))
        .expect("processed");

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, RESERVED_INDEX)
            .is_none(),
        "a route to our own user must never be stored"
    );
}

/// The committed entry is what `next_hop_for_user` prefers over any
/// delegation gateway, so a stored self-entry would misroute locally
/// addressed traffic out to a neighbour.
#[test]
fn our_own_user_keeps_no_routing_entry_back_reference() {
    let (state, _rx) = fresh_state();
    let hosted = [1; 8];
    state.register_hosted_user(hosted, 0, fresh_multikey());
    let peer = neighbour_advertising(&state, Space::User, hosted);

    state
        .apply_user_entry(&ctx(peer), user_entry(42, 100))
        .expect("processed");

    let users = state.users.read().unwrap();
    let user = users.get(&hosted).expect("hosted user");
    assert!(
        user.read().unwrap().routing_entry.is_none(),
        "commit_routing_entry must not run for our own user"
    );
    assert!(state.next_hop_for_user(hosted).is_none());
}

/// The rejection runs before `translate_incoming`, which allocates and binds
/// an own index as a side effect. Under node form a hosted user holds no
/// index of its own (§3.2), so translating first would both pollute the
/// dictionary and mark the index for re-introduction — making us advertise
/// our own user at an index a neighbour's mistake produced.
#[test]
fn a_rejected_self_entry_allocates_no_index() {
    let (state, _rx) = fresh_state();
    // Two hosted users puts this node in node form, where neither holds a
    // user index.
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    state.sync_propagation_form(1_000);
    assert!(state.user_dict.read().unwrap().idx_of(&[1; 8]).is_none());

    let peer = neighbour_advertising(&state, Space::User, [1; 8]);
    state
        .apply_user_entry(&ctx(peer), user_entry(42, 100))
        .expect("processed");

    assert!(
        state.user_dict.read().unwrap().idx_of(&[1; 8]).is_none(),
        "a rejected entry must not bind an index for our own user"
    );
    assert!(
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User)
            .is_empty(),
        "and must not queue our own user for re-introduction"
    );
}

/// A user we merely know about is not ours — the check keys on `is_hosted`,
/// not on presence in the users map.
#[test]
fn an_entry_for_a_remote_user_is_still_accepted() {
    let (state, _rx) = fresh_state();
    let remote = [5; 8];
    install_user(&state, remote, 0);
    bind_own_dict(&state, Space::User, 20, remote);
    let peer = neighbour_advertising(&state, Space::User, remote);

    state
        .apply_user_entry(&ctx(peer), user_entry(42, 100))
        .expect("processed");

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 20)
            .is_some(),
        "the fix must not reject ordinary foreign entries"
    );
}

// ------------------------------------------------------------ node space

/// The node-space half of the ring: our own node entry coming back around
/// under node form, where we are bound at the reserved node index.
#[test]
fn an_entry_for_our_own_node_is_rejected() {
    let (state, _rx) = fresh_state();
    let host_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    state.sync_propagation_form(1_000);
    assert_eq!(
        state.node_dict.read().unwrap().id_of(RESERVED_INDEX),
        Some(host_id)
    );

    let peer = neighbour_advertising(&state, Space::Node, host_id);
    state
        .apply_node_entry(&ctx(peer), node_entry(42, 100))
        .expect("processed");

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::Node, RESERVED_INDEX)
            .is_none(),
        "a route to ourselves must never be stored"
    );
}

/// §8.8 step 3e records a node entry's `manifest_version` even when relay
/// inclusion rejects it. That must not happen for our own node: our
/// committed version is authoritative and a neighbour's echo is not an
/// advertisement to track.
#[test]
fn our_own_node_records_no_advertised_version() {
    let (state, _rx) = fresh_state();
    let host_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.register_hosted_user([2; 8], 0, fresh_multikey());
    state.sync_propagation_form(1_000);

    let peer = neighbour_advertising(&state, Space::Node, host_id);
    let mut entry = node_entry(42, 100);
    entry.manifest_version = 999;
    state
        .apply_node_entry(&ctx(peer), entry)
        .expect("processed");

    let nodes = state.nodes.read().unwrap();
    let node = nodes.get(&host_id).expect("self node record");
    assert_eq!(
        node.read().unwrap().advertised_version,
        0,
        "our own committed version is authoritative"
    );
}

/// A neighbour's node entry is ordinary traffic and must still land.
#[test]
fn an_entry_for_a_remote_node_is_still_accepted() {
    let (state, _rx) = fresh_state();
    let remote = [5; 8];
    install_node(&state, remote, 0, false);
    bind_own_dict(&state, Space::Node, 20, remote);
    let peer = neighbour_advertising(&state, Space::Node, remote);

    state
        .apply_node_entry(&ctx(peer), node_entry(42, 100))
        .expect("processed");

    assert!(
        state
            .routing_table
            .read()
            .unwrap()
            .get(Space::Node, 20)
            .is_some(),
        "the fix must not reject ordinary foreign entries"
    );
}

// ------------------------------------------------------------- predicate

#[test]
fn is_local_identity_distinguishes_hosted_from_remote() {
    let (state, _rx) = fresh_state();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    install_user(&state, [5; 8], 0);

    assert!(state.is_local_identity([1; 8], false));
    assert!(!state.is_local_identity([5; 8], false));
    assert!(!state.is_local_identity([9; 8], false), "unknown user");

    assert!(state.is_local_identity(state.host_mk.to_id(), true));
    assert!(!state.is_local_identity([5; 8], true));
}

/// The two spaces are independent: a user id that happens to equal our node
/// id is still a user, and vice versa.
#[test]
fn is_local_identity_does_not_cross_spaces() {
    let (state, _rx) = fresh_state();
    let host_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());

    assert!(!state.is_local_identity(host_id, false));
    assert!(!state.is_local_identity([1; 8], true));
}
