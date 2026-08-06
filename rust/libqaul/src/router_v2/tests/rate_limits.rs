// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Per-neighbour request and serve rate limiting (spec §14).

use crate::connections::ConnectionModule;
use crate::router_v2::{index::Space, test_utils::*};

/// Defaults from §14, asserted so the tests below stay meaningful if the
/// config changes underneath them.
#[test]
fn the_configured_defaults_match_the_spec() {
    let (state, _rx) = fresh_state();
    assert_eq!(state.options.manifest_request_rate, 4);
    assert_eq!(state.options.manifest_serve_rate, 4);
}

#[test]
fn the_serve_window_admits_exactly_the_limit_then_refuses() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let limit = state.options.manifest_serve_rate;

    for i in 0..limit {
        assert!(
            state.allow_manifest_serve(peer, 1_000_000),
            "sample {i} should be admitted, limit is {limit}"
        );
    }
    assert!(
        !state.allow_manifest_serve(peer, 1_000_000),
        "the sample after the limit must be refused"
    );
}

#[test]
fn a_refused_sample_is_not_recorded() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let limit = state.options.manifest_serve_rate;

    for _ in 0..limit {
        assert!(state.allow_manifest_serve(peer, 1_000_000));
    }
    // Hammer the limiter while it is closed.
    for _ in 0..50 {
        assert!(!state.allow_manifest_serve(peer, 1_000_000));
    }

    // Those refusals must not have extended the window: one second
    // after the *admitted* samples, the budget is fully back.
    for _ in 0..limit {
        assert!(
            state.allow_manifest_serve(peer, 1_001_000),
            "refused attempts must not push the window forward"
        );
    }
}

#[test]
fn the_window_slides_rather_than_resetting() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    // Four samples spread across the second: t=0, 300, 600, 900.
    for offset in [0u64, 300, 600, 900] {
        assert!(state.allow_manifest_serve(peer, 1_000_000 + offset));
    }
    assert!(!state.allow_manifest_serve(peer, 1_000_950));

    // At t=1000 the first sample has aged out, so exactly one slot frees.
    assert!(state.allow_manifest_serve(peer, 1_001_000));
    assert!(
        !state.allow_manifest_serve(peer, 1_001_000),
        "only the aged-out sample should free a slot, not the whole window"
    );

    // By t=1900 the samples at 300, 600 and 900 have also aged out.
    assert!(state.allow_manifest_serve(peer, 1_001_900));
    assert!(state.allow_manifest_serve(peer, 1_001_900));
}

#[test]
fn a_sample_exactly_one_second_old_has_aged_out() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let limit = state.options.manifest_serve_rate;

    for _ in 0..limit {
        assert!(state.allow_manifest_serve(peer, 1_000_000));
    }
    // The boundary is `now - sample >= 1000`, so 1000ms later is out.
    assert!(state.allow_manifest_serve(peer, 1_001_000));
}

#[test]
fn the_windows_are_per_neighbour() {
    let (state, _rx) = fresh_state();
    let noisy = fresh_peer();
    let quiet = fresh_peer();

    for _ in 0..state.options.manifest_serve_rate {
        assert!(state.allow_manifest_serve(noisy, 1_000_000));
    }
    assert!(!state.allow_manifest_serve(noisy, 1_000_000));

    assert!(
        state.allow_manifest_serve(quiet, 1_000_000),
        "one neighbour exhausting its budget must not starve another"
    );
}

#[test]
fn the_request_and_serve_windows_are_independent() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();

    for _ in 0..state.options.manifest_serve_rate {
        assert!(state.allow_manifest_serve(peer, 1_000_000));
    }
    assert!(!state.allow_manifest_serve(peer, 1_000_000));

    assert!(
        state.allow_manifest_request(peer, 1_000_000),
        "serving a lot must not stop us from asking"
    );
}

#[test]
fn losing_the_last_transport_drops_the_windows() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [1u8; 8], ConnectionModule::Lan);

    assert!(state.allow_manifest_serve(peer, 1_000_000));
    assert!(state.allow_manifest_request(peer, 1_000_000));
    assert_eq!(state.manifest_serve_window.read().unwrap().len(), 1);
    assert_eq!(state.manifest_request_window.read().unwrap().len(), 1);

    state.remove_neighbour_transport(peer, ConnectionModule::Lan);

    assert!(
        state.manifest_serve_window.read().unwrap().is_empty(),
        "serve window leaks once the peer is gone"
    );
    assert!(
        state.manifest_request_window.read().unwrap().is_empty(),
        "request window leaks once the peer is gone"
    );
}

#[test]
fn a_surviving_transport_keeps_the_windows() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [1u8; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(peer, [1u8; 8], ConnectionModule::Internet);

    for _ in 0..state.options.manifest_serve_rate {
        assert!(state.allow_manifest_serve(peer, 1_000_000));
    }

    // The neighbour is still reachable over Internet, so its budget
    // must not be silently reset by dropping one transport.
    state.remove_neighbour_transport(peer, ConnectionModule::Lan);
    assert!(
        !state.allow_manifest_serve(peer, 1_000_000),
        "dropping one of two transports must not clear the window"
    );
}

/// §10.8: a request batch refused by the rate limit goes back on the
/// queue. Those origins are still stale — dropping them would strand the
/// node until an unrelated advertisement re-fired the pull trigger.
#[test]
fn a_rate_limited_batch_is_requeued_not_dropped() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let origin = [42u8; 8];

    state
        .pending_manifest_requests
        .write()
        .unwrap()
        .entry(peer)
        .or_default()
        .insert(origin);

    // Exhaust the request budget for this neighbour first.
    for _ in 0..state.options.manifest_request_rate {
        assert!(state.allow_manifest_request(peer, 1_000_000));
    }

    let drained = state.drain_manifest_reqs(1_000_000);
    assert!(drained.is_empty(), "the batch must not go out");

    let pending = state.pending_manifest_requests.read().unwrap();
    assert!(
        pending.get(&peer).is_some_and(|o| o.contains(&origin)),
        "the origin must still be queued for the next tick"
    );
}

#[test]
fn an_unthrottled_batch_drains_and_is_marked_outstanding() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer();
    let origin = [42u8; 8];

    state
        .pending_manifest_requests
        .write()
        .unwrap()
        .entry(peer)
        .or_default()
        .insert(origin);

    let drained = state.drain_manifest_reqs(1_000_000);
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].0, peer);
    assert_eq!(drained[0].1.items.len(), 1);
    assert_eq!(drained[0].1.items[0].origin_node_id, origin);
    // No state held for this origin, so it is a bootstrap request.
    assert!(drained[0].1.items[0].have_none());

    assert!(state
        .outstanding_manifest_requests
        .read()
        .unwrap()
        .contains_key(&(origin, peer)));
    assert!(state.pending_manifest_requests.read().unwrap().is_empty());
}

/// The serve limit is charged per answered item, and only for decisions
/// that actually send. A neighbour that is already current must not be
/// able to burn another's budget.
#[test]
fn nothing_and_sealed_decisions_are_not_charged() {
    use crate::router_v2::codec::messages::{ManifestRequest, ManifestRequestItem};

    use crate::router_v2::identity::SelfDelegation;
    use crate::router_v2::BumpTrigger;

    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, [1u8; 8], ConnectionModule::Lan);
    bind_own_dict(&state, Space::Node, 1, state.host_mk.to_id());

    // Commit two versions, so `have_version == committed` lands on the
    // Nothing branch rather than the log_base one.
    state.add_self_delegation(
        [1u8; 8],
        0,
        SelfDelegation {
            timeout: 9_000,
            entry_signature: [0xA1; 64],
        },
    );
    state
        .try_bump_manifest_version(1_000_000, BumpTrigger::Accumulated)
        .unwrap();
    state.add_self_delegation(
        [2u8; 8],
        0,
        SelfDelegation {
            timeout: 9_000,
            entry_signature: [0xB2; 64],
        },
    );
    let committed = state
        .try_bump_manifest_version(1_070_000, BumpTrigger::Accumulated)
        .unwrap();

    // Ten items from a neighbour that is already current.
    let req = ManifestRequest {
        items: (0..10)
            .map(|_| ManifestRequestItem {
                origin_node_id: state.host_mk.to_id(),
                have_version: committed,
                item_flags: 0x00,
            })
            .collect(),
    };
    state
        .handle_manifest_request(peer, ConnectionModule::Lan, req, 1_070_500)
        .unwrap();

    assert!(rx.try_recv().is_err(), "no responses should be emitted");
    assert!(
        state
            .manifest_serve_window
            .read()
            .unwrap()
            .get(&peer)
            .is_none_or(|w| w.is_empty()),
        "decisions that send nothing must not consume serve budget"
    );
}
