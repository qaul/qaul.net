// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Multi-transport neighbours and teardown on the last transport loss (spec §4.2).

use crate::router_v2::test_utils::*;
use crate::router_v2::*;

/// The first transport for an unknown peer is new: the mirror is created
/// and the caller is told to run bootstrap work.
#[test]
fn first_transport_for_new_peer_reports_new() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    assert!(
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
        "first registration must report the pair as newly reachable"
    );

    let mirrors = state.mirrors.read().unwrap();
    let info = mirrors.get(&peer).expect("mirror created");
    assert_eq!(info.node_id, [77; 8]);
    assert!(info.transports.contains(&ConnectionModule::Lan));
}

/// Re-registering the same transport must report `false`. This is what
/// stops `on_neighbour_connect` re-sending a full INDEX_DUMP on every
/// ping, since ping_event fires continuously for a live neighbour.
#[test]
fn repeat_registration_of_same_transport_reports_not_new() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));

    for _ in 0..5 {
        assert!(
            !state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
            "repeat pings must not re-trigger bootstrap"
        );
    }

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(
        mirrors.get(&peer).unwrap().transports.len(),
        1,
        "transport set must not grow on repeats"
    );
}

/// A second, distinct transport to an already-known peer is also new —
/// §4.2 tracks reachability per (peer, transport) pair, and the new
/// transport needs its own INDEX_DUMP.
#[test]
fn second_distinct_transport_reports_new() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));
    assert!(
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet),
        "a distinct transport to a known peer is newly reachable"
    );

    let mirrors = state.mirrors.read().unwrap();
    let info = mirrors.get(&peer).unwrap();
    assert_eq!(info.transports.len(), 2);
    assert!(info.transports.contains(&ConnectionModule::Lan));
    assert!(info.transports.contains(&ConnectionModule::Internet));
}

/// Distinct peers are tracked independently.
#[test]
fn distinct_peers_are_independent() {
    let (state, _rx) = fresh_state();
    let peer_a = fresh_peer();
    let peer_b = fresh_peer();

    assert!(state.add_neighbour_transport(peer_a, [10; 8], ConnectionModule::Lan));
    assert!(
        state.add_neighbour_transport(peer_b, [20; 8], ConnectionModule::Lan),
        "a different peer on the same transport is still new"
    );

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer_a).unwrap().node_id, [10; 8]);
    assert_eq!(mirrors.get(&peer_b).unwrap().node_id, [20; 8]);
}

/// Dropping the last transport removes the mirror, so a later reconnect
/// reports `true` again and bootstrap re-runs. Without this the reconnect
/// path would silently stop sending INDEX_DUMP.
#[test]
fn reconnect_after_full_disconnect_reports_new_again() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));
    state.remove_neighbour_transport(peer, ConnectionModule::Lan);
    assert!(
        state.mirrors.read().unwrap().get(&peer).is_none(),
        "last transport removed → mirror dropped"
    );

    assert!(
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
        "reconnect must be reported as new so bootstrap re-runs"
    );
}

/// Dropping one of two transports keeps the mirror alive, and re-adding
/// only that transport is new — the surviving one is not.
#[test]
fn partial_disconnect_keeps_mirror_and_other_transport() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

    state.remove_neighbour_transport(peer, ConnectionModule::Lan);

    {
        let mirrors = state.mirrors.read().unwrap();
        let info = mirrors.get(&peer).expect("mirror survives partial drop");
        assert_eq!(info.transports.len(), 1);
        assert!(info.transports.contains(&ConnectionModule::Internet));
    }

    assert!(
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
        "the dropped transport is new again"
    );
    assert!(
        !state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet),
        "the surviving transport is not new"
    );
}
