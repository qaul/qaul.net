// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! §5.1's BLE quality penalty, end to end: the radio's RSSI has to reach the
//! receive path or every BLE hop prices as the best-quality link.
//!
//! `metric::rssi_penalty` and its bucket boundaries were already implemented
//! and tested. What was missing was a value: the BLE ingress passed `None`, so
//! `rssi_penalty` always returned 0 and route selection could not tell a strong
//! link from a marginal one. These tests pin the stored metric, which is the
//! only place the difference is observable.

use crate::router_v2::*;
use crate::router_v2::{
    codec::{
        messages::{Mapping, RoutingUpdate, UserEntry},
        Header, RoutingMessage, PROTOCOL_VERSION,
    },
    index::Space,
    test_utils::*,
};
use libp2p::PeerId;

const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;
const TARGET_ID: [u8; 8] = [9; 8];

fn setup_ble_neighbour(state: &RouterV2State) -> PeerId {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Ble1m);
    // The entry's next hop resolves through our own node dict, so the
    // neighbour has to be bound there or the entry is dropped unstored.
    bind_own_dict(
        state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    peer
}

/// An entry arriving over BLE 1M carrying metric 10.
fn ble_update() -> Vec<u8> {
    let msg = RoutingUpdate {
        user_mappings: vec![Mapping {
            abs_idx: 5,
            target_id: TARGET_ID,
            version: 1,
        }],
        node_mappings: Vec::new(),
        user_entries: vec![UserEntry {
            abs_idx: 5,
            seq: 1,
            metric: 10,
            hop_count: 1,
            local_only: false,
        }],
        node_entries: Vec::new(),
    };
    let mut body = Vec::new();
    msg.encode(&mut body).unwrap();
    let header = Header {
        version: PROTOCOL_VERSION,
        message_type: RoutingMessage::RoutingUpdate,
        payload_len: body.len() as u16,
    };
    let mut out = Vec::new();
    header.encode(&mut out);
    out.extend_from_slice(&body);
    out
}

/// Drive one BLE routing update at `rssi` and return the stored metric.
fn stored_metric_for(rssi: Option<i8>) -> u16 {
    let (state, _rx) = fresh_state();
    let peer = setup_ble_neighbour(&state);
    state.set_neighbour_rssi(peer, rssi);

    state
        .received(
            peer,
            ConnectionModule::Ble1m,
            state.neighbour_rssi(&peer),
            &ble_update(),
            1_000,
        )
        .unwrap();

    let own_idx = state.user_dict.read().unwrap().idx_of(&TARGET_ID).unwrap();
    let rt = state.routing_table.read().unwrap();
    let stored = rt.get(Space::User, own_idx).expect("entry must be stored");
    let metric = stored.read().unwrap().metric;
    metric
}

// ---------- the penalty actually reaches the metric ----------

/// BLE 1M weighs 50. With a strong signal the penalty is zero, so the stored
/// metric is the carried 10 plus the transport weight only.
#[test]
fn strong_signal_costs_transport_weight_only() {
    assert_eq!(stored_metric_for(Some(-50)), 60);
}

/// Each §5.1 bucket has to be visible in the stored metric, or route selection
/// cannot distinguish link quality. This is the assertion that fails when the
/// ingress passes None.
#[test]
fn each_bucket_changes_the_stored_metric() {
    assert_eq!(stored_metric_for(Some(-60)), 60, "-60 dBm: penalty 0");
    assert_eq!(stored_metric_for(Some(-61)), 65, "-61 dBm: penalty 5");
    assert_eq!(stored_metric_for(Some(-75)), 65, "-75 dBm: penalty 5");
    assert_eq!(stored_metric_for(Some(-76)), 70, "-76 dBm: penalty 10");
    assert_eq!(stored_metric_for(Some(-85)), 70, "-85 dBm: penalty 10");
    assert_eq!(stored_metric_for(Some(-86)), 80, "-86 dBm: penalty 20");
}

/// A marginal link must cost strictly more than a strong one. Without a
/// supplied RSSI these are equal, which is the whole of 4-B.
#[test]
fn a_marginal_link_costs_more_than_a_strong_one() {
    let strong = stored_metric_for(Some(-40));
    let marginal = stored_metric_for(Some(-95));
    assert!(
        marginal > strong,
        "a marginal BLE link must price above a strong one, got {marginal} vs {strong}"
    );
    assert_eq!(marginal - strong, 20, "the §5.1 worst-bucket penalty");
}

/// No reading means no penalty — the pre-existing behaviour, kept deliberately
/// rather than inventing a value for a link we could not measure.
#[test]
fn an_unknown_reading_adds_no_penalty() {
    assert_eq!(stored_metric_for(None), 60);
}

// ---------- the store itself ----------

#[test]
fn rssi_round_trips_through_neighbour_state() {
    let (state, _rx) = fresh_state();
    let peer = setup_ble_neighbour(&state);

    assert_eq!(state.neighbour_rssi(&peer), None, "defaults to no reading");

    state.set_neighbour_rssi(peer, Some(-70));
    assert_eq!(state.neighbour_rssi(&peer), Some(-70));

    // Every advertisement carries a fresh reading, so later ones must win.
    state.set_neighbour_rssi(peer, Some(-90));
    assert_eq!(state.neighbour_rssi(&peer), Some(-90));

    state.set_neighbour_rssi(peer, None);
    assert_eq!(state.neighbour_rssi(&peer), None, "clearable");
}

#[test]
fn rssi_for_an_unknown_peer_is_none() {
    let (state, _rx) = fresh_state();
    assert_eq!(state.neighbour_rssi(&fresh_peer()), None);
}
