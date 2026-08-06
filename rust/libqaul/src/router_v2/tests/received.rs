// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Wire dispatch: header decode, version skip, per-type routing (spec §8.8 steps 1-3).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{
        messages::{IndexDump, Mapping, RoutingUpdate, UserEntry},
        Header, RoutingMessage, PROTOCOL_VERSION,
    },
    index::Space,
    test_utils::*,
};
use libp2p::PeerId;

const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

fn setup_neighbour(state: &RouterV2State) -> PeerId {
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
    bind_own_dict(
        state,
        Space::Node,
        NEIGHBOUR_IDX_IN_NODE_DICT,
        NEIGHBOUR_NODE_ID,
    );
    peer
}

/// Encode a message with the given type + body bytes into a full wire
/// frame (4-byte header + body).
fn frame(msg_type: RoutingMessage, body: &[u8]) -> Vec<u8> {
    let header = Header {
        version: PROTOCOL_VERSION,
        message_type: msg_type,
        payload_len: body.len() as u16,
    };
    let mut out = Vec::new();
    header.encode(&mut out);
    out.extend_from_slice(body);
    out
}

/// Encode a full ROUTING_UPDATE message ready for `received()`.
fn frame_routing_update(msg: &RoutingUpdate) -> Vec<u8> {
    let mut body = Vec::new();
    msg.encode(&mut body).unwrap();
    frame(RoutingMessage::RoutingUpdate, &body)
}

fn small_valid_update(target_id: [u8; 8]) -> RoutingUpdate {
    RoutingUpdate {
        user_mappings: vec![Mapping {
            abs_idx: 5,
            target_id,
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
    }
}

#[test]
fn empty_buf_is_noop() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);

    state
        .received(peer, ConnectionModule::Lan, None, &[], 1_000)
        .unwrap();

    assert_eq!(state.users.read().unwrap().len(), 0);
}

#[test]
fn valid_routing_update_dispatches_to_orchestrator() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let target_id = [1; 8];

    let msg = small_valid_update(target_id);
    let bytes = frame_routing_update(&msg);

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    // Mirror + stub + routing entry all landed via the orchestrator.
    assert!(state.users.read().unwrap().get(&target_id).is_some());
    let own_idx = state.user_dict.read().unwrap().idx_of(&target_id).unwrap();
    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, own_idx)
        .is_some());
}

/// Two messages back-to-back must both be processed. This pins the
/// frame-advancement math (advance `buf` by `4 + payload_len`, not
/// just `payload_len`) — the bug that would silently corrupt the
/// next header.
#[test]
fn multiple_valid_messages_in_batch_are_all_processed() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let target_a = [1; 8];
    let target_b = [2; 8];

    let mut bytes = frame_routing_update(&small_valid_update(target_a));
    bytes.extend(frame_routing_update(&small_valid_update(target_b)));

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    let users = state.users.read().unwrap();
    assert!(users.get(&target_a).is_some(), "first message applied");
    assert!(users.get(&target_b).is_some(), "second message applied");
}

/// Forward-compat behaviour (§8.2): a message with an unknown version
/// must be skipped past (using payload_len) so that a subsequent
/// valid message is still processed.
#[test]
fn bad_version_skips_and_processes_subsequent_valid_message() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let target_id = [3; 8];

    // Fake message with unknown version 0xFE and payload_len 8.
    let bad_body = [0xAAu8; 8];
    let mut bytes = vec![0xFE, 0x01, 0x00, 0x08];
    bytes.extend_from_slice(&bad_body);

    // Then a valid RoutingUpdate.
    bytes.extend(frame_routing_update(&small_valid_update(target_id)));

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    assert!(
        state.users.read().unwrap().get(&target_id).is_some(),
        "valid message following a BadVersion must still be processed",
    );
}

/// Header says payload_len=100, but only 4 bytes of body follow.
/// The receive loop should log-and-return without applying anything.
#[test]
fn truncated_body_returns_without_partial_state() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);

    let mut bytes = Vec::new();
    // Header: version=1, type=RoutingUpdate=1, payload_len=100.
    bytes.extend_from_slice(&[PROTOCOL_VERSION, 0x01, 0x00, 0x64]);
    // Only 4 bytes of body, not 100.
    bytes.extend_from_slice(&[0x00; 4]);

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    assert_eq!(state.users.read().unwrap().len(), 0);
}

/// A body that fails to decode must not desync the frame loop: alignment
/// comes from the header's `payload_len`, which is consumed before the body
/// is parsed, so the next message still processes.
#[test]
fn undecodable_body_is_skipped_and_next_processed() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let target_id = [4; 8];

    // A ManifestDelta too short to be a valid body, then a good update.
    let delta_body = [0x00u8; 2];
    let mut bytes = frame(RoutingMessage::ManifestDelta, &delta_body);
    bytes.extend(frame_routing_update(&small_valid_update(target_id)));

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    // The malformed delta was dropped, then the RoutingUpdate applied.
    assert!(state.users.read().unwrap().get(&target_id).is_some());
}

/// INDEX_DUMP now has a handler, so it must actually populate mirrors
/// rather than falling into the catch-all — and the message after it
/// must still be processed.
#[test]
fn index_dump_is_dispatched_to_handler() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let dumped_id = [9; 8];
    let target_id = [4; 8];

    let dump = IndexDump {
        user_mappings: vec![Mapping {
            abs_idx: 12,
            target_id: dumped_id,
            version: 77,
        }],
        node_mappings: Vec::new(),
    };
    let mut body = Vec::new();
    dump.encode(&mut body).unwrap();

    let mut bytes = frame(RoutingMessage::IndexDump, &body);
    bytes.extend(frame_routing_update(&small_valid_update(target_id)));

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    // The dump was handled, not skipped.
    {
        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(
            mirrors.get(&peer).unwrap().users.id_of(12),
            Some(dumped_id),
            "INDEX_DUMP must reach handle_index_dump"
        );
    }
    let users = state.users.read().unwrap();
    let dumped_arc = users.get(&dumped_id).unwrap();
    assert_eq!(dumped_arc.read().unwrap().profile_version, 77);

    // Frame alignment preserved: the following update still applied.
    assert!(users.get(&target_id).is_some());
}

#[test]
fn malformed_routing_update_body_does_not_corrupt_frame_alignment() {
    let (state, _rx) = fresh_state();
    let peer = setup_neighbour(&state);
    let target_id = [5; 8];

    // Header claims payload_len=4, but 4 bytes of garbage isn't a
    // valid RoutingUpdate body — decoder fails. Frame alignment is
    // preserved because buf was advanced before the decode attempt,
    // so the next message still processes.
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[PROTOCOL_VERSION, 0x01, 0x00, 0x04]);
    bytes.extend_from_slice(&[0xFF; 4]); // garbage body
    bytes.extend(frame_routing_update(&small_valid_update(target_id)));

    state
        .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
        .unwrap();

    assert!(
        state.users.read().unwrap().get(&target_id).is_some(),
        "valid message after a body-decode failure must still be processed",
    );
}
