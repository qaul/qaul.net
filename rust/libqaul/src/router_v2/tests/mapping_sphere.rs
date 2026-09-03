// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! The §2.3 gateway membrane applied to index mappings, in both the
//! `ROUTING_UPDATE` inline path (§8.3) and the `INDEX_DUMP` path (§8.4).

use crate::router_v2::*;
use crate::router_v2::{
    codec::{
        messages::{IndexDump, Mapping},
        Header, RoutingMessage,
    },
    index::Space,
    propagation::{self, on_neighbour_connect, should_introduce, tick_relay},
    test_utils::*,
};

fn decode_dump(bytes: &[u8]) -> IndexDump {
    let (header, body) = Header::decode(bytes).expect("frame header");
    assert_eq!(header.message_type, RoutingMessage::IndexDump);
    IndexDump::decode(&body[..header.payload_len as usize]).expect("IndexDump body")
}

// ---------------------------------------------------------------- predicate

/// The Local sphere is unfiltered: a village sees its own members.
#[test]
fn local_sphere_introduces_everything() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 0);
    install_node(&state, [2; 8], 0, false);

    assert!(should_introduce(
        &state,
        Space::User,
        &[1; 8],
        Sphere::Local
    ));
    assert!(should_introduce(
        &state,
        Space::Node,
        &[2; 8],
        Sphere::Local
    ));
}

/// §2.3: "User entries [...] SHALL be filtered out" over an Internet-sphere
/// transport, so the index binding they would reference is pointless there.
#[test]
fn internet_sphere_never_introduces_a_user() {
    let (state, _rx) = fresh_state();
    install_user(&state, [1; 8], 7);

    assert!(!should_introduce(
        &state,
        Space::User,
        &[1; 8],
        Sphere::Internet
    ));
}

/// §2.3: node entries cross the Internet sphere only when the target has
/// `is_gateway = 1`; the mapping follows the entry.
#[test]
fn internet_sphere_introduces_gateway_nodes_only() {
    let (state, _rx) = fresh_state();
    install_node(&state, [1; 8], 0, true);
    install_node(&state, [2; 8], 0, false);

    assert!(should_introduce(
        &state,
        Space::Node,
        &[1; 8],
        Sphere::Internet
    ));
    assert!(!should_introduce(
        &state,
        Space::Node,
        &[2; 8],
        Sphere::Internet
    ));
}

/// A node we hold no record for cannot be shown to be a gateway, so it is
/// withheld rather than assumed.
#[test]
fn an_unknown_node_is_not_introduced_over_the_internet() {
    let (state, _rx) = fresh_state();

    assert!(!should_introduce(
        &state,
        Space::Node,
        &[9; 8],
        Sphere::Internet
    ));
}

// ------------------------------------------------------------- INDEX_DUMP

/// §8.4 over LAN: the dump is the sender's whole dictionary.
#[test]
fn index_dump_over_lan_carries_both_spaces() {
    let (state, mut rx) = fresh_state();
    install_user(&state, [1; 8], 3);
    install_node(&state, [2; 8], 5, false);
    bind_own_dict(&state, Space::User, 10, [1; 8]);
    bind_own_dict(&state, Space::Node, 11, [2; 8]);

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Lan);

    let dump = decode_dump(&rx.try_recv().expect("bootstrap emits").bytes);
    assert_eq!(dump.user_mappings.len(), 1);
    assert_eq!(dump.node_mappings.len(), 1);
}

/// The whole point of 2-B: a bulk dump over an INTERNET transport must not
/// hand a foreign gateway a `User` record per user in this village.
#[test]
fn index_dump_over_internet_drops_user_mappings() {
    let (state, mut rx) = fresh_state();
    install_user(&state, [1; 8], 3);
    install_user(&state, [2; 8], 4);
    bind_own_dict(&state, Space::User, 10, [1; 8]);
    bind_own_dict(&state, Space::User, 11, [2; 8]);

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Internet);

    let dump = decode_dump(&rx.try_recv().expect("bootstrap emits").bytes);
    assert!(
        dump.user_mappings.is_empty(),
        "user indexes are meaningless in the Internet sphere"
    );
}

/// §2.3 again: only gateway node mappings cross the membrane.
#[test]
fn index_dump_over_internet_keeps_only_gateway_nodes() {
    let (state, mut rx) = fresh_state();
    install_node(&state, [1; 8], 5, true);
    install_node(&state, [2; 8], 6, false);
    bind_own_dict(&state, Space::Node, 10, [1; 8]);
    bind_own_dict(&state, Space::Node, 11, [2; 8]);

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Internet);

    let dump = decode_dump(&rx.try_recv().expect("bootstrap emits").bytes);
    assert_eq!(dump.node_mappings.len(), 1);
    assert_eq!(dump.node_mappings[0].target_id, [1; 8]);
    assert_eq!(dump.node_mappings[0].version, 5);
}

/// §8.4 says the dump is sent on *every* (re)connect. A dump emptied by the
/// sphere filter is still a dump — suppressing it would strand the
/// handshake.
#[test]
fn a_fully_filtered_index_dump_is_still_sent() {
    let (state, mut rx) = fresh_state();
    install_user(&state, [1; 8], 3);
    install_node(&state, [2; 8], 5, false);
    bind_own_dict(&state, Space::User, 10, [1; 8]);
    bind_own_dict(&state, Space::Node, 11, [2; 8]);

    on_neighbour_connect(&state, fresh_peer(), ConnectionModule::Internet);

    let dump = decode_dump(&rx.try_recv().expect("§8.4: sent on every connect").bytes);
    assert!(dump.user_mappings.is_empty());
    assert!(dump.node_mappings.is_empty());
}

// ------------------------------------------------------ inline mappings

/// One introduction, two neighbours in different spheres: the LAN peer sees
/// the user mapping, the INTERNET peer does not. This also pins that the
/// drain happens once and the filter is per-peer — suppressing for the
/// Internet peer must not withhold it from the LAN peer.
#[test]
fn inline_user_mappings_are_filtered_per_peer() {
    let (state, mut rx) = fresh_state();
    let lan = fresh_peer();
    let net = fresh_peer();
    state.add_neighbour_transport(lan, [10; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(net, [11; 8], ConnectionModule::Internet);

    install_user(&state, [1; 8], 3);
    bind_own_dict(&state, Space::User, 20, [1; 8]);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 20);

    tick_relay(&state, 1_000);

    let mut saw_lan = false;
    let mut saw_net = false;
    while let Ok(msg) = rx.try_recv() {
        let (header, body) = Header::decode(&msg.bytes).expect("frame header");
        if header.message_type != RoutingMessage::RoutingUpdate {
            continue;
        }
        let update = codec::messages::RoutingUpdate::decode(&body[..header.payload_len as usize])
            .expect("RoutingUpdate body");
        if msg.peer == lan {
            saw_lan = true;
            assert_eq!(update.user_mappings.len(), 1);
            assert_eq!(update.user_mappings[0].target_id, [1; 8]);
        } else if msg.peer == net {
            saw_net = true;
            assert!(
                update.user_mappings.is_empty(),
                "a user mapping must not cross the membrane"
            );
        }
    }

    assert!(saw_lan, "the LAN peer still receives the introduction");
    assert!(
        !saw_net,
        "with nothing left to say, no update is built for the Internet peer"
    );
}

/// A gateway introduction reaches both spheres; a non-gateway node reaches
/// only the Local one.
#[test]
fn inline_node_mappings_keep_gateways_on_both_spheres() {
    let (state, mut rx) = fresh_state();
    let lan = fresh_peer();
    let net = fresh_peer();
    state.add_neighbour_transport(lan, [10; 8], ConnectionModule::Lan);
    state.add_neighbour_transport(net, [11; 8], ConnectionModule::Internet);

    install_node(&state, [1; 8], 5, true);
    install_node(&state, [2; 8], 6, false);
    bind_own_dict(&state, Space::Node, 20, [1; 8]);
    bind_own_dict(&state, Space::Node, 21, [2; 8]);
    {
        let mut tracker = state.reintroduction_tracker.write().unwrap();
        tracker.mark_first_time(Space::Node, 20);
        tracker.mark_first_time(Space::Node, 21);
    }

    tick_relay(&state, 1_000);

    while let Ok(msg) = rx.try_recv() {
        let (header, body) = Header::decode(&msg.bytes).expect("frame header");
        if header.message_type != RoutingMessage::RoutingUpdate {
            continue;
        }
        let update = codec::messages::RoutingUpdate::decode(&body[..header.payload_len as usize])
            .expect("RoutingUpdate body");
        let ids: Vec<[u8; 8]> = update.node_mappings.iter().map(|m| m.target_id).collect();
        if msg.peer == lan {
            assert_eq!(ids, vec![[1; 8], [2; 8]]);
        } else if msg.peer == net {
            assert_eq!(ids, vec![[1; 8]], "only the gateway crosses the membrane");
        }
    }
}

// ------------------------------------------------------------- §8.4 chunking

/// A dictionary that fits is one chunk, framed as such.
#[test]
fn a_small_dictionary_is_a_single_chunk() {
    let chunks = propagation::split_index_dump(
        vec![Mapping {
            abs_idx: 1,
            target_id: [1; 8],
            version: 0,
        }],
        vec![Mapping {
            abs_idx: 2,
            target_id: [2; 8],
            version: 0,
        }],
    );

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].chunk_index, 0);
    assert_eq!(chunks[0].chunk_count, 1);
    assert_eq!(chunks[0].user_mappings.len(), 1);
    assert_eq!(chunks[0].node_mappings.len(), 1);
}

/// §8.4 sends a dump on every (re)connect, and the message itself is the cue
/// for the neighbour to introduce itself back — so an empty dictionary still
/// produces one chunk, not zero.
#[test]
fn an_empty_dictionary_still_produces_one_chunk() {
    let chunks = propagation::split_index_dump(Vec::new(), Vec::new());

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].chunk_count, 1);
    assert!(chunks[0].user_mappings.is_empty());
    assert!(chunks[0].node_mappings.is_empty());
}

/// The 3-E case: a dictionary past the payload bound is split rather than
/// dropped. Every mapping must appear exactly once, in order, with correct
/// framing on each chunk.
#[test]
fn an_oversized_dictionary_is_split_not_dropped() {
    let total = 10_000;
    let users: Vec<Mapping> = (0..total)
        .map(|i| Mapping {
            abs_idx: i as u16,
            target_id: [(i % 251) as u8; 8],
            version: i as u32,
        })
        .collect();

    let chunks = propagation::split_index_dump(users.clone(), Vec::new());

    assert!(chunks.len() > 1, "a dictionary this size must chunk");
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i as u8);
        assert_eq!(chunk.chunk_count, chunks.len() as u8);
    }

    let rejoined: Vec<u16> = chunks
        .iter()
        .flat_map(|c| c.user_mappings.iter().map(|m| m.abs_idx))
        .collect();
    let expected: Vec<u16> = users.iter().map(|m| m.abs_idx).collect();
    assert_eq!(rejoined, expected, "every mapping survives exactly once");
}

/// Each emitted chunk has to encode inside the payload bound — that is the
/// whole point of splitting.
#[test]
fn every_chunk_encodes_within_the_payload_bound() {
    let users: Vec<Mapping> = (0..9_000)
        .map(|i| Mapping {
            // Spaced so the delta encoding takes its escape path, the
            // worst case the split is sized against.
            abs_idx: (i * 7) as u16,
            target_id: [1; 8],
            version: 0,
        })
        .collect();
    let nodes: Vec<Mapping> = (0..9_000)
        .map(|i| Mapping {
            abs_idx: (i * 7) as u16,
            target_id: [2; 8],
            version: 0,
        })
        .collect();

    for chunk in propagation::split_index_dump(users, nodes) {
        let mut body = Vec::new();
        chunk.encode(&mut body).expect("encodes");
        assert!(
            body.len() <= 60 * 1024,
            "chunk {} encoded to {} bytes",
            chunk.chunk_index,
            body.len()
        );
        assert!(u16::try_from(body.len()).is_ok(), "fits payload_len");
    }
}

/// A split dump reaches the wire as several framed INDEX_DUMP messages to
/// the same peer, in ascending chunk order — §8.4 requires the receiver to
/// process them in order.
#[test]
fn on_connect_emits_every_chunk_in_order() {
    let (state, mut rx) = fresh_state();
    let peer = fresh_peer();
    // Distinct ids: `bind` is bidirectional, so reusing an id would unbind
    // the index it was previously held at and the dictionary would never
    // grow past the number of distinct ids.
    for i in 0..6_000u32 {
        let mut id = [0u8; 8];
        id[..4].copy_from_slice(&i.to_be_bytes());
        install_user(&state, id, 0);
        bind_own_dict(&state, Space::User, i as u16, id);
    }

    on_neighbour_connect(&state, peer, ConnectionModule::Lan);

    let mut seen = Vec::new();
    while let Ok(msg) = rx.try_recv() {
        assert_eq!(msg.peer, peer);
        seen.push(decode_dump(&msg.bytes));
    }

    assert!(seen.len() > 1, "6000 mappings must span several chunks");
    for (i, dump) in seen.iter().enumerate() {
        assert_eq!(dump.chunk_index, i as u8, "ascending chunk order");
        assert_eq!(dump.chunk_count, seen.len() as u8);
    }
}

/// Regression: `pending_introductions` drains, so an intro the §2.3 filter
/// withholds from *every* peer must be put back. Losing it means the index
/// is never introduced, and the receiver drops every entry referencing it
/// as an unknown mapping — permanently.
///
/// Found by a live three-node run, not by the per-peer tests above: those
/// always had a LAN peer that accepted the intro.
#[test]
fn an_intro_filtered_for_every_peer_is_re_marked() {
    let (state, _rx) = fresh_state();
    // Only an Internet peer, so a user intro can reach nobody.
    state.add_neighbour_transport(fresh_peer(), [10; 8], ConnectionModule::Internet);

    install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::User, 20, [1; 8]);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 20);

    tick_relay(&state, 1_000);

    assert_eq!(
        state.pending_introductions(Space::User),
        vec![(20, [1; 8], 0)],
        "an intro that reached no peer must still be pending"
    );
}

/// The same hazard for node space: a node not yet known to be a gateway is
/// filtered off the Internet sphere, and would otherwise never be
/// introduced even after it becomes one.
#[test]
fn a_non_gateway_node_intro_survives_to_be_introduced_later() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [10; 8], ConnectionModule::Internet);

    let node = install_node(&state, [5; 8], 0, false);
    bind_own_dict(&state, Space::Node, 21, [5; 8]);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::Node, 21);

    tick_relay(&state, 1_000);
    assert_eq!(
        state.pending_introductions(Space::Node),
        vec![(21, [5; 8], 0)],
        "withheld while it is not a gateway"
    );

    // It becomes a gateway; the mark must still be there to act on.
    node.write().unwrap().is_gateway = true;
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::Node, 21);
    tick_relay(&state, 2_000);
    assert!(
        state.pending_introductions(Space::Node).is_empty(),
        "once it is a gateway the intro goes out and the mark clears"
    );
}

/// A mark that did reach a peer must not be re-queued, or every index would
/// be re-introduced on every tick forever.
#[test]
fn a_delivered_intro_is_not_re_marked() {
    let (state, _rx) = fresh_state();
    state.add_neighbour_transport(fresh_peer(), [10; 8], ConnectionModule::Lan);

    install_user(&state, [1; 8], 0);
    bind_own_dict(&state, Space::User, 20, [1; 8]);
    state
        .reintroduction_tracker
        .write()
        .unwrap()
        .mark_first_time(Space::User, 20);

    tick_relay(&state, 1_000);

    assert!(
        state.pending_introductions(Space::User).is_empty(),
        "a delivered intro must clear"
    );
}
