// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §2.2: the hop counter is sphere-local. A relayer restarts the count when
//! it carries a Local-sphere-learned entry across the gateway membrane, so
//! each sphere spends its own 63-hop budget.

use crate::router_v2::*;
use crate::router_v2::{
    codec::{messages::RoutingUpdate, Header, RoutingMessage},
    index::Space,
    propagation::{compute_outgoing_hop_count, tick_relay},
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};
use std::sync::{Arc, RwLock};

// ------------------------------------------------------------- the rule

/// Staying inside the Local sphere accumulates as usual.
#[test]
fn local_to_local_keeps_the_count() {
    assert_eq!(
        compute_outgoing_hop_count(20, ConnectionModule::Lan, Sphere::Local),
        20
    );
}

/// The membrane crossing: a village-learned entry restarts at zero on the
/// far side.
#[test]
fn local_learned_entry_resets_crossing_into_the_internet_sphere() {
    assert_eq!(
        compute_outgoing_hop_count(20, ConnectionModule::Lan, Sphere::Internet),
        0
    );
    assert_eq!(
        compute_outgoing_hop_count(63, ConnectionModule::Ble1m, Sphere::Internet),
        0
    );
}

/// Already inside the Internet sphere, so no membrane is being crossed and
/// the Internet sphere's own budget keeps accumulating.
#[test]
fn internet_to_internet_keeps_the_count() {
    assert_eq!(
        compute_outgoing_hop_count(7, ConnectionModule::Internet, Sphere::Internet),
        7
    );
}

/// The reset is one-directional. `should_propagate` seals Internet → Local,
/// so this combination should never be reached — but if it ever is, it must
/// not reset, or the cap would stop bounding anything.
#[test]
fn internet_learned_entry_is_not_reset_towards_the_local_sphere() {
    assert_eq!(
        compute_outgoing_hop_count(7, ConnectionModule::Internet, Sphere::Local),
        7
    );
}

// ------------------------------------------------------- through tick_relay

fn install_entry(
    state: &RouterV2State,
    idx: u16,
    node_id: [u8; 8],
    hop_count: u8,
    metric: u16,
    transport: ConnectionModule,
    next_hop: u16,
) {
    let node = install_node(state, node_id, 0, true);
    bind_own_dict(state, Space::Node, idx, node_id);
    let entry = RoutingEntry {
        target_index: idx,
        target: TargetRef::Node(node),
        seq_num: SeqNum::from(42),
        metric,
        next_hop,
        transport,
        hop_count,
        local_only: transport != ConnectionModule::Internet,
        last_update: 1_000,
    };
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::Node, idx, Arc::new(RwLock::new(entry)));
    state
        .relay_queue
        .write()
        .unwrap()
        .insert((Space::Node, idx));
}

fn node_entries_for(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<OutboundMsg>,
    peer: PeerId,
) -> Vec<codec::messages::NodeEntry> {
    let mut out = Vec::new();
    while let Ok(msg) = rx.try_recv() {
        if msg.peer != peer {
            continue;
        }
        let (header, body) = Header::decode(&msg.bytes).expect("frame header");
        if header.message_type != RoutingMessage::RoutingUpdate {
            continue;
        }
        let update = RoutingUpdate::decode(&body[..header.payload_len as usize])
            .expect("RoutingUpdate body");
        out.extend(update.node_entries);
    }
    out
}

/// A gateway relaying another village gateway onto the highway: the hop
/// count restarts, and the metric — which §9.2 and §10.6 select on — does
/// not.
#[test]
fn relaying_a_village_route_onto_the_internet_resets_only_the_hop_count() {
    let (state, mut rx) = fresh_state();
    let net = fresh_peer();
    state.add_neighbour_transport(net, [90; 8], ConnectionModule::Internet);
    // The next hop must be a bound node other than the outgoing neighbour,
    // or split horizon suppresses the relay.
    install_node(&state, [80; 8], 0, false);
    bind_own_dict(&state, Space::Node, 5, [80; 8]);

    install_entry(&state, 30, [3; 8], 20, 200, ConnectionModule::Lan, 5);

    tick_relay(&state, 1_000);

    let entries = node_entries_for(&mut rx, net);
    let relayed = entries
        .iter()
        .find(|e| e.abs_idx == 30)
        .expect("the gateway route crosses the membrane");
    assert_eq!(relayed.hop_count, 0, "§2.2: the counter is sphere-local");
    assert_eq!(
        relayed.metric, 200,
        "the metric stays truthful for §10.6 gateway selection"
    );
    assert_eq!(relayed.seq, 42, "the origin's sequence number is preserved");
}

/// The same entry relayed inside its own sphere is untouched.
#[test]
fn relaying_a_village_route_within_the_village_keeps_the_hop_count() {
    let (state, mut rx) = fresh_state();
    let lan = fresh_peer();
    state.add_neighbour_transport(lan, [90; 8], ConnectionModule::Lan);
    install_node(&state, [80; 8], 0, false);
    bind_own_dict(&state, Space::Node, 5, [80; 8]);

    install_entry(&state, 30, [3; 8], 20, 200, ConnectionModule::Lan, 5);

    tick_relay(&state, 1_000);

    let entries = node_entries_for(&mut rx, lan);
    let relayed = entries.iter().find(|e| e.abs_idx == 30).expect("relayed");
    assert_eq!(relayed.hop_count, 20);
    assert_eq!(relayed.metric, 200);
}

/// An entry already learned over the Internet sphere keeps accumulating
/// against the Internet sphere's own budget — the reset happens at the
/// membrane, not on every Internet-bound send.
#[test]
fn an_internet_learned_route_keeps_accumulating_on_the_internet() {
    let (state, mut rx) = fresh_state();
    let net = fresh_peer();
    state.add_neighbour_transport(net, [90; 8], ConnectionModule::Internet);
    install_node(&state, [80; 8], 0, false);
    bind_own_dict(&state, Space::Node, 5, [80; 8]);

    install_entry(&state, 30, [3; 8], 7, 45, ConnectionModule::Internet, 5);

    tick_relay(&state, 1_000);

    let entries = node_entries_for(&mut rx, net);
    let relayed = entries.iter().find(|e| e.abs_idx == 30).expect("relayed");
    assert_eq!(
        relayed.hop_count, 7,
        "no membrane crossed, so no reset — the cap still bounds the sphere"
    );
}

/// The point of the whole change: a route that has nearly exhausted the
/// village budget arrives on the highway with a full one, instead of the
/// three hops a shared budget would have left it.
#[test]
fn a_nearly_exhausted_village_route_gets_a_full_budget_on_the_highway() {
    let (state, mut rx) = fresh_state();
    let net = fresh_peer();
    state.add_neighbour_transport(net, [90; 8], ConnectionModule::Internet);
    install_node(&state, [80; 8], 0, false);
    bind_own_dict(&state, Space::Node, 5, [80; 8]);

    install_entry(&state, 30, [3; 8], 60, 600, ConnectionModule::Lan, 5);

    tick_relay(&state, 1_000);

    let entries = node_entries_for(&mut rx, net);
    let relayed = entries.iter().find(|e| e.abs_idx == 30).expect("relayed");
    assert_eq!(relayed.hop_count, 0);
    assert_eq!(relayed.metric, 600, "the true cost still travels");
}
