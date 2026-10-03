// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Assuming and relinquishing the gateway role (spec §2.3), and the
//! rate-limited manifest bump that carries the flag out (spec §10.8).

use crate::connections::ConnectionModule;
use crate::router_v2::{test_utils::*, BumpTrigger};

/// `last_manifest_bump_ms` starts at 0, so a bump at t=0 is inside the
/// rate-limit window. Real callers pass epoch milliseconds; these fixtures
/// only need a timestamp past one window.
const NOW: u64 = 120_000;

/// A node with no INTERNET transport is not a gateway.
#[test]
fn lan_only_host_is_not_a_gateway() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Lan);

    assert!(!state.sync_gateway_role());
    assert!(!state.host_is_gateway());
}

/// §2.3: "A node SHALL act as a gateway whenever it has an active INTERNET
/// transport connection."
#[test]
fn internet_transport_assumes_the_gateway_role() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

    assert!(state.sync_gateway_role());
    assert!(state.host_is_gateway());
}

/// §2.3: the role "SHALL be relinquished when no INTERNET transport
/// remains active".
#[test]
fn losing_the_last_internet_transport_relinquishes_the_role() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();
    assert!(state.host_is_gateway());

    state.remove_neighbour_transport(peer, ConnectionModule::Internet);

    assert!(state.sync_gateway_role());
    assert!(!state.host_is_gateway());
}

/// One INTERNET peer among several LAN peers still makes this a gateway,
/// and the role survives losing an unrelated LAN link.
#[test]
fn the_role_tracks_the_last_remaining_internet_peer() {
    let (state, _rx) = fresh_state();
    let lan = fresh_peer();
    let net = fresh_peer();
    state.add_neighbour_transport(lan, [1; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(net, [2; 8], ConnectionModule::Internet);
    state.sync_gateway_role();
    assert!(state.host_is_gateway());

    state.remove_neighbour_transport(lan, ConnectionModule::Lan);
    assert!(!state.sync_gateway_role());
    assert!(state.host_is_gateway());

    state.remove_neighbour_transport(net, ConnectionModule::Internet);
    assert!(state.sync_gateway_role());
    assert!(!state.host_is_gateway());
}

/// Re-running the sync with no connectivity change is a no-op: it must not
/// keep dirtying the flags and bumping the version every tick.
#[test]
fn a_steady_role_does_not_dirty_the_manifest() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

    assert!(state.sync_gateway_role());
    assert!(!state.sync_gateway_role());
    assert!(!state.sync_gateway_role());

    // The first sync's dirty flag is still pending here; clear it through a
    // bump and confirm nothing re-dirties.
    state
        .try_bump_manifest_version(NOW, BumpTrigger::Accumulated)
        .unwrap();
    assert!(!state.sync_gateway_role());
    assert!(state
        .try_bump_manifest_version(u64::MAX, BumpTrigger::Accumulated)
        .is_none());
}

/// §10.8 lists an `is_gateway` transition as a bump trigger. It dirties no
/// delegation entry, so the bump has to fire on the flags change alone.
#[test]
fn a_flags_only_change_bumps_the_manifest_version() {
    let (state, _rx) = fresh_state();
    let before = state.manifest.read().unwrap().manifest_version;

    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();

    assert!(state.dirty_delegations.read().unwrap().is_empty());
    let bumped = state
        .try_bump_manifest_version(NOW, BumpTrigger::Accumulated)
        .expect("a flags-only change is a §10.8 bump trigger");
    assert_eq!(bumped, before.wrapping_add(1));
    assert_eq!(state.manifest.read().unwrap().manifest_version, bumped);
}

/// §10.8: "An `is_gateway` flag change is subject to the rate limit, so a
/// host with a flapping INTERNET connection re-advertises at most once per
/// minute."
#[test]
fn the_flag_change_obeys_the_rate_limit() {
    let (state, _rx) = fresh_state();
    let window_ms = state.options.manifest_rate_limit.saturating_mul(1000);
    let peer = fresh_peer();

    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();
    let first = state
        .try_bump_manifest_version(window_ms, BumpTrigger::Accumulated)
        .expect("first bump");

    // Flap back inside the window: dirtied, but held.
    state.remove_neighbour_transport(peer, ConnectionModule::Internet);
    assert!(state.sync_gateway_role());
    assert!(state
        .try_bump_manifest_version(window_ms + 1, BumpTrigger::Accumulated)
        .is_none());
    assert_eq!(state.manifest.read().unwrap().manifest_version, first);

    // Once the window elapses the accumulated change goes out as one bump.
    let bumped = state
        .try_bump_manifest_version(window_ms * 2, BumpTrigger::Accumulated)
        .expect("the window has elapsed");
    assert_eq!(bumped, first.wrapping_add(1));
}

/// The bump re-signs, and the signature covers the `flags` byte, so the
/// retained chunks a requester is served must carry the new bit.
#[test]
fn the_bump_re_signs_the_manifest_with_the_new_flag() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();
    state
        .try_bump_manifest_version(NOW, BumpTrigger::Accumulated)
        .unwrap();

    let manifest = state.manifest.read().unwrap();
    let chunks = manifest
        .retained_chunks
        .as_ref()
        .expect("a bumped manifest is signed");
    assert!(chunks.iter().all(|c| c.flags & 0x01 != 0));
}

/// A flags-only bump appends no delta-log records: §8.6 propagates the
/// change through the delta header's `flags`, not through a record.
#[test]
fn a_flags_only_bump_appends_no_log_records() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);
    state.sync_gateway_role();

    let from = state.manifest.read().unwrap().manifest_version;
    state
        .try_bump_manifest_version(NOW, BumpTrigger::Accumulated)
        .unwrap();

    assert!(state
        .own_manifest_log
        .read()
        .unwrap()
        .records_after(from)
        .is_empty());
}

/// The self `Node` record mirrors the manifest, so local reads of
/// `Node.is_gateway` agree with what we advertise.
#[test]
fn the_self_node_record_mirrors_the_role() {
    let (state, _rx) = fresh_state();
    let host_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

    // Node form installs the self record; §3.2 and §2.3 share the trigger.
    state.sync_propagation_form(0);
    state.sync_gateway_role();

    let nodes = state.nodes.read().unwrap();
    let node = nodes.get(&host_id).expect("self node record");
    assert!(node.read().unwrap().is_gateway);
}

/// The self record is seeded from the manifest, not hard-coded false, so a
/// host that is already a gateway when it enters node form starts correct.
#[test]
fn the_self_node_record_is_seeded_from_the_manifest() {
    let (state, _rx) = fresh_state();
    let host_id = state.host_mk.to_id();
    state.register_hosted_user([1; 8], 0, fresh_multikey());
    state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

    state.sync_gateway_role();
    state.sync_propagation_form(0);

    let nodes = state.nodes.read().unwrap();
    assert!(nodes.get(&host_id).unwrap().read().unwrap().is_gateway);
}
