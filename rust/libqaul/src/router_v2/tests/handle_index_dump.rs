// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! INDEX_DUMP bulk mapping application (spec §8.4).

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::{IndexDump, Mapping},
    index::Space,
    seq::SeqNum,
    table::{RoutingEntry, TargetRef},
    test_utils::*,
};

fn mapping(abs_idx: u16, target_id: [u8; 8], version: u32) -> Mapping {
    Mapping {
        abs_idx,
        target_id,
        version,
    }
}

/// Both sections land: mirrors bound in each index space, and stubs
/// created in `users` / `nodes` carrying the advertised versions.
#[test]
fn both_sections_populate_mirrors_and_stubs() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    let dump = IndexDump {
        user_mappings: vec![mapping(5, [1; 8], 42)],
        node_mappings: vec![mapping(9, [2; 8], 99)],
    };

    state.handle_index_dump(peer, dump).unwrap();

    {
        let mirrors = state.mirrors.read().unwrap();
        let nm = mirrors.get(&peer).unwrap();
        assert_eq!(nm.users.id_of(5), Some([1; 8]));
        assert_eq!(nm.nodes.id_of(9), Some([2; 8]));
    }

    let users = state.users.read().unwrap();
    let user_arc = users.get(&[1; 8]).unwrap();
    let u = user_arc.read().unwrap();
    assert_eq!(u.profile_version, 42);
    assert!(u.public_key.is_none(), "stub must not fabricate a key");

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&[2; 8]).unwrap();
    let n = node_arc.read().unwrap();
    // Pull model (§10.8): a dump's node version is an *advertisement*,
    // not a committed manifest_version.
    assert_eq!(n.manifest_version, 0, "nothing committed yet");
    assert_eq!(n.advertised_version, 99);
}

/// Regression test for the accumulate-don't-clear decision. §8.4 lets a
/// sender split an oversized dictionary across several INDEX_DUMPs, and
/// the message carries no chunk framing, so the receiver cannot tell a
/// complete dump from chunk 1 of N. Clearing the mirror per §3.6's
/// literal "SHALL replace" would discard the earlier chunk; this test
/// fails the moment someone makes that change.
#[test]
fn second_dump_accumulates_and_does_not_clear_first() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    state
        .handle_index_dump(
            peer,
            IndexDump {
                user_mappings: vec![mapping(5, [1; 8], 1)],
                node_mappings: vec![mapping(5, [11; 8], 1)],
            },
        )
        .unwrap();

    state
        .handle_index_dump(
            peer,
            IndexDump {
                user_mappings: vec![mapping(9, [2; 8], 1)],
                node_mappings: vec![mapping(9, [22; 8], 1)],
            },
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    let nm = mirrors.get(&peer).unwrap();
    assert_eq!(nm.users.id_of(5), Some([1; 8]), "chunk 1 user survived");
    assert_eq!(nm.users.id_of(9), Some([2; 8]), "chunk 2 user landed");
    assert_eq!(nm.nodes.id_of(5), Some([11; 8]), "chunk 1 node survived");
    assert_eq!(nm.nodes.id_of(9), Some([22; 8]), "chunk 2 node landed");
}

/// A dump that rebinds a still-live index delegates to `apply_mapping`'s
/// teardown path: old routing entry cleared, own index released to
/// cooldown, own dict unbound, mirror repointed.
#[test]
fn dump_rebinding_live_index_clears_old_routing_state() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    let old_id = [10; 8];
    let new_id = [20; 8];
    let own_idx: u16 = 7;

    bind_mirror(&state, peer, Space::User, 5, old_id);
    let old_user = install_user(&state, old_id, 1);
    bind_own_dict(&state, Space::User, own_idx, old_id);

    let entry = Arc::new(RwLock::new(RoutingEntry {
        target_index: own_idx,
        target: TargetRef::User(old_user.clone()),
        seq_num: SeqNum::from(0u16),
        metric: 5,
        next_hop: 0,
        transport: ConnectionModule::Lan,
        last_update: 0,
        hop_count: 1,
        local_only: false,
    }));
    let entry_weak = Arc::downgrade(&entry);
    state
        .routing_table
        .write()
        .unwrap()
        .set(Space::User, own_idx, entry);
    old_user.write().unwrap().routing_entry = Some(entry_weak.clone());

    state
        .handle_index_dump(
            peer,
            IndexDump {
                user_mappings: vec![mapping(5, new_id, 1)],
                node_mappings: Vec::new(),
            },
        )
        .unwrap();

    assert!(state
        .routing_table
        .read()
        .unwrap()
        .get(Space::User, own_idx)
        .is_none());
    assert!(entry_weak.upgrade().is_none(), "old entry Arc must drop");
    assert_eq!(state.user_dict.read().unwrap().idx_of(&old_id), None);
    assert!(state
        .users_allocator
        .read()
        .unwrap()
        .idx_in_cooldown(own_idx));

    let mirrors = state.mirrors.read().unwrap();
    assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(new_id));
}

/// The user section is processed before the node section, because node
/// delegations may reference users the user section introduces.
#[test]
fn user_section_is_processed_before_node_section() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    // Same abs_idx in both spaces bound to different ids: if the two
    // sections were applied to the same dictionary, one would clobber
    // the other. The spaces are independent (§3.5).
    state
        .handle_index_dump(
            peer,
            IndexDump {
                user_mappings: vec![mapping(3, [7; 8], 5)],
                node_mappings: vec![mapping(3, [8; 8], 6)],
            },
        )
        .unwrap();

    let mirrors = state.mirrors.read().unwrap();
    let nm = mirrors.get(&peer).unwrap();
    assert_eq!(nm.users.id_of(3), Some([7; 8]));
    assert_eq!(nm.nodes.id_of(3), Some([8; 8]));
    drop(mirrors);

    assert_eq!(state.users.read().unwrap().len(), 1);
    assert_eq!(state.nodes.read().unwrap().len(), 1);
}

/// A dump from a peer with no mirror (never registered, or already
/// disconnected) is a silent no-op — `apply_mapping` bails per mapping.
#[test]
fn unknown_neighbour_is_noop() {
    let (state, _rx) = fresh_state();
    let peer = fresh_peer(); // never added to mirrors

    let dump = IndexDump {
        user_mappings: vec![mapping(5, [1; 8], 42)],
        node_mappings: vec![mapping(9, [2; 8], 99)],
    };

    assert!(state.handle_index_dump(peer, dump).is_ok());
    assert!(state.mirrors.read().unwrap().get(&peer).is_none());
    assert_eq!(state.users.read().unwrap().len(), 0);
    assert_eq!(state.nodes.read().unwrap().len(), 0);
}

/// An empty dump is legal — a node with an empty dictionary sends one on
/// connect — and must not panic or create phantom state.
#[test]
fn empty_dump_is_harmless() {
    let (state, _rx) = fresh_state();
    let peer = add_neighbour(&state);

    state
        .handle_index_dump(
            peer,
            IndexDump {
                user_mappings: Vec::new(),
                node_mappings: Vec::new(),
            },
        )
        .unwrap();

    assert_eq!(state.users.read().unwrap().len(), 0);
    assert_eq!(state.nodes.read().unwrap().len(), 0);
    assert!(
        state.mirrors.read().unwrap().get(&peer).is_some(),
        "mirror still registered"
    );
}
