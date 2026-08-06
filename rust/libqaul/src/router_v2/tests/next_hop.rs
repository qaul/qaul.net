// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Next-hop resolution: direct routing entry first, then best-metric delegation gateway (spec §9.2).

use crate::router_v2::*;
use crate::router_v2::{
    index::Space,
    seq::SeqNum,
    table::{Node, RoutingEntry, TargetRef},
    test_utils::*,
};
use std::sync::Weak;

fn make_entry(
    target: TargetRef,
    next_hop: u16,
    metric: u16,
    transport: ConnectionModule,
) -> Arc<RwLock<RoutingEntry>> {
    Arc::new(RwLock::new(RoutingEntry {
        target_index: 0,
        target,
        seq_num: SeqNum::from(0u16),
        metric,
        next_hop,
        transport,
        last_update: 0,
        hop_count: 0,
        local_only: false,
    }))
}

#[test]
fn unknown_user_returns_none() {
    let (state, _rx) = fresh_state();
    assert_eq!(state.next_hop_for_user([99; 8]), None);
}

#[test]
fn known_user_with_no_routing_data_returns_none() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 0);
    assert_eq!(state.next_hop_for_user([1; 8]), None);
}

/// Step 2: a direct routing entry whose next_hop resolves through the
/// dictionary should produce that hop's node id and the entry's transport.
#[test]
fn direct_routing_entry_resolves_next_hop_and_transport() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let neighbour_id = [9; 8];
    let neighbour_idx = 100;
    bind_own_dict(&state, Space::Node, neighbour_idx, neighbour_id);

    let entry = make_entry(
        TargetRef::User(user.clone()),
        neighbour_idx,
        42,
        ConnectionModule::Lan,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, 5, entry.clone());
    user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((neighbour_id, ConnectionModule::Lan)),
    );
}

/// Step 3: no direct entry; two delegation gateways; the gateway with
/// the lowest metric wins, and its routing entry's next_hop / transport
/// determine the result.
#[test]
fn gateway_fallback_picks_lowest_metric() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let g_hi = install_node(&state, [10; 8], 0, true);
    let g_lo = install_node(&state, [20; 8], 0, true);

    bind_own_dict(&state, Space::Node, 50, [10; 8]);
    bind_own_dict(&state, Space::Node, 60, [20; 8]);

    let nbr_hi = [11; 8];
    let nbr_lo = [21; 8];
    bind_own_dict(&state, Space::Node, 101, nbr_hi);
    bind_own_dict(&state, Space::Node, 102, nbr_lo);

    let e_hi = make_entry(
        TargetRef::Node(g_hi.clone()),
        101,
        30,
        ConnectionModule::Lan,
    );
    let e_lo = make_entry(
        TargetRef::Node(g_lo.clone()),
        102,
        10,
        ConnectionModule::Internet,
    );
    {
        let mut rt = state.routing_table.write().unwrap();
        rt.set(Space::Node, 50, e_hi);
        rt.set(Space::Node, 60, e_lo);
    }

    user.write().unwrap().delegation_gateways = vec![Arc::downgrade(&g_hi), Arc::downgrade(&g_lo)];

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((nbr_lo, ConnectionModule::Internet)),
    );
}

/// A direct routing entry must be preferred over a delegation gateway,
/// even when the gateway has a lower metric.
#[test]
fn direct_entry_preferred_over_lower_metric_gateway() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let direct_nbr = [50; 8];
    bind_own_dict(&state, Space::Node, 200, direct_nbr);
    let direct = make_entry(
        TargetRef::User(user.clone()),
        200,
        100,
        ConnectionModule::Lan,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, 5, direct.clone());

    let gw = install_node(&state, [10; 8], 0, true);
    bind_own_dict(&state, Space::Node, 50, [10; 8]);
    let gw_nbr = [11; 8];
    bind_own_dict(&state, Space::Node, 101, gw_nbr);
    let gw_entry = make_entry(
        TargetRef::Node(gw.clone()),
        101,
        5,
        ConnectionModule::Internet,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 50, gw_entry);

    {
        let mut u = user.write().unwrap();
        u.routing_entry = Some(Arc::downgrade(&direct));
        u.delegation_gateways.push(Arc::downgrade(&gw));
    }

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((direct_nbr, ConnectionModule::Lan)),
    );
}

#[test]
fn dangling_direct_entry_falls_through_to_gateway() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let orphan = make_entry(TargetRef::User(user.clone()), 0, 100, ConnectionModule::Lan);
    let dangling: Weak<RwLock<RoutingEntry>> = Arc::downgrade(&orphan);
    drop(orphan);

    let gw = install_node(&state, [10; 8], 0, true);
    bind_own_dict(&state, Space::Node, 50, [10; 8]);
    let gw_nbr = [11; 8];
    bind_own_dict(&state, Space::Node, 101, gw_nbr);
    let gw_entry = make_entry(
        TargetRef::Node(gw.clone()),
        101,
        5,
        ConnectionModule::Internet,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 50, gw_entry);

    {
        let mut u = user.write().unwrap();
        u.routing_entry = Some(dangling);
        u.delegation_gateways.push(Arc::downgrade(&gw));
    }

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((gw_nbr, ConnectionModule::Internet)),
    );
}

#[test]
fn dangling_gateway_is_skipped() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let live = install_node(&state, [10; 8], 0, true);
    bind_own_dict(&state, Space::Node, 50, [10; 8]);
    let live_nbr = [11; 8];
    bind_own_dict(&state, Space::Node, 101, live_nbr);
    let live_entry = make_entry(
        TargetRef::Node(live.clone()),
        101,
        30,
        ConnectionModule::Lan,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 50, live_entry);

    // Dangling gateway: build a Node, take a Weak, drop the strong.
    let orphan = Arc::new(RwLock::new(Node {
        id: [20; 8],
        public_key: Some(fresh_multikey()),
        manifest_version: 0,
        advertised_version: 0,
        is_gateway: true,
        delegated_users: Vec::new(),
        manifest_signature: None,
        retained_chunks: None,
        learn_sphere: None,
        manifest_log: crate::router_v2::manifest::ManifestLog::default(),
    }));
    let dangling = Arc::downgrade(&orphan);
    drop(orphan);

    {
        let mut u = user.write().unwrap();
        u.delegation_gateways = vec![dangling, Arc::downgrade(&live)];
    }

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((live_nbr, ConnectionModule::Lan)),
    );
}

#[test]
fn gateway_with_no_routing_entry_is_skipped() {
    let (state, _rx) = fresh_state();
    let user = install_user(&state, [1; 8], 0);

    let unreachable = install_node(&state, [20; 8], 0, true);
    bind_own_dict(&state, Space::Node, 60, [20; 8]);

    let reachable = install_node(&state, [10; 8], 0, true);
    bind_own_dict(&state, Space::Node, 50, [10; 8]);
    let r_nbr = [11; 8];
    bind_own_dict(&state, Space::Node, 101, r_nbr);
    let r_entry = make_entry(
        TargetRef::Node(reachable.clone()),
        101,
        5,
        ConnectionModule::Lan,
    );
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, 50, r_entry);

    {
        let mut u = user.write().unwrap();
        u.delegation_gateways = vec![Arc::downgrade(&unreachable), Arc::downgrade(&reachable)];
    }

    assert_eq!(
        state.next_hop_for_user([1; 8]),
        Some((r_nbr, ConnectionModule::Lan)),
    );
}

#[test]
fn next_hop_node_id_resolves_bound_indices_and_misses_unbound() {
    let (state, _rx) = fresh_state();
    bind_own_dict(&state, Space::Node, 77, [7; 8]);
    assert_eq!(state.next_hop_node_id(77), Some([7; 8]));
    assert_eq!(state.next_hop_node_id(78), None);
}
