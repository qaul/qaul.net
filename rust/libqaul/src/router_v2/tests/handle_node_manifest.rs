// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Applying a full NODE_MANIFEST, including byte-exact stored discipline (spec §8.8 step 5).

use crate::router_v2::*;
use crate::router_v2::{
    codec::messages::{ManifestEntry, NodeManifest},
    identity::{delegation_signing_input, Multikey},
    manifest::Manifest,
    table::{Node, User},
    test_utils::*,
};
use libp2p::identity::Keypair;

fn keypair_and_multikey() -> (Keypair, Multikey) {
    let kp = Keypair::generate_ed25519();
    let mk = Multikey::from(kp.public());
    (kp, mk)
}

fn sign_entry(
    user_kp: &Keypair,
    host_mk: &Multikey,
    user_id: [u8; 8],
    timeout: u64,
) -> ManifestEntry {
    let signing_input = delegation_signing_input(&host_mk.encode(), timeout);
    let sig_bytes = user_kp.sign(&signing_input).unwrap();
    let entry_signature: [u8; 64] = sig_bytes.try_into().unwrap();
    ManifestEntry {
        user_id,
        timeout,
        entry_signature,
        profile_version: 0,
    }
}

/// Install a Node with a specific public key so we can sign
/// matching messages. Returns the origin's node_id.
fn install_origin_node(state: &RouterV2State, mk: &Multikey) -> [u8; 8] {
    let id = mk.to_id();
    let node = Node {
        id,
        public_key: Some(mk.clone()),
        manifest_version: 0,
        advertised_version: 0,
        is_gateway: false,
        delegated_users: Vec::new(),
        manifest_signature: None,
        retained_chunks: None,
        learn_sphere: None,
        manifest_log: crate::router_v2::manifest::ManifestLog::default(),
    };
    state.nodes.write().unwrap().insert(id, node);
    id
}

fn install_user_with_key(state: &RouterV2State, mk: &Multikey) -> [u8; 8] {
    let id = mk.to_id();
    let user = User {
        id,
        public_key: Some(mk.clone()),
        profile_version: 0,
        routing_entry: None,
        delegation_gateways: Vec::new(),
        is_hosted: false,
    };
    state.users.write().unwrap().insert(id, user);
    id
}

/// Wire a self-origin scenario: neighbour with origin's node_id,
/// origin bound at reserved idx 0 in the neighbour's node mirror,
/// origin's Node record installed with a real key.
fn setup_self_origin(state: &RouterV2State, host_mk: &Multikey) -> (libp2p::PeerId, [u8; 8]) {
    let host_id = install_origin_node(state, host_mk);
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);
    // Origin uses RESERVED_INDEX 0 in the sender's frame (§3.2).
    state
        .mirrors
        .write()
        .unwrap()
        .get_mut(&peer)
        .unwrap()
        .nodes
        .bind(0, host_id);
    (peer, host_id)
}

fn build_signed_manifest(
    host_kp: &Keypair,
    host_mk: &Multikey,
    version: u32,
    is_gateway: bool,
    entries: Vec<ManifestEntry>,
) -> Vec<NodeManifest> {
    let mut manifest = Manifest::new();
    manifest.manifest_version = version;
    manifest.set_gateway(is_gateway);
    manifest.set_entries(entries);
    manifest
        .build_chunks(host_mk.to_id(), host_kp, &host_mk.encode())
        .unwrap()
}

// ---------- happy path ----------

#[test]
fn happy_path_commits_manifest_to_node_record() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = install_user_with_key(&state, &user_mk);

    let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
    let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, entries);

    state
        .handle_node_manifest(
            chunks.into_iter().next().unwrap(),
            500,
            ConnectionModule::Lan,
        )
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    assert_eq!(node.manifest_version, 5);
    assert!(node.is_gateway);
    assert_eq!(node.delegated_users.len(), 1);
    assert_eq!(node.delegated_users[0].user_id, user_id);
    assert_eq!(node.delegated_users[0].delegation_timeout, 1_000_000);
}

// ---------- drop paths ----------
//
// Under the pull-based model (spec §8.5), NODE_MANIFEST is link-local
// and carries origin_node_id directly on the wire — `handle_node_manifest`
// takes no `neighbour` parameter. The old "unknown neighbour is noop"
// test is therefore obsolete; the equivalent drop path (origin_node_id
// maps to no Node record) is covered by `unknown_origin_node_id_is_noop`
// below.

/// Under the pull-based model (spec §8.5), NODE_MANIFEST carries
/// `origin_node_id` on the wire — no index translation via the
/// neighbour's mirror. If the id doesn't match any Node record we
/// hold, the handler drops the message.
#[test]
fn unknown_origin_node_id_is_noop() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let host_id = install_origin_node(&state, &host_mk);
    let peer = fresh_peer();
    state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);

    // Build a signed manifest, then rewrite origin_node_id to point
    // at a Node we have no record of.
    let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
    let mut msg = chunks.into_iter().next().unwrap();
    msg.origin_node_id = [99; 8];

    state
        .handle_node_manifest(msg, 0, ConnectionModule::Lan)
        .unwrap();

    // The real origin's Node record is untouched.
    assert_eq!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .manifest_version,
        0,
    );
    // No stub was created for the unknown id.
    assert!(state.nodes.read().unwrap().get(&[99; 8]).is_none());
}

#[test]
fn origin_with_no_public_key_is_noop() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let host_id = host_mk.to_id();

    // Install origin Node with NO public key.
    state.nodes.write().unwrap().insert(
        host_id,
        Node {
            id: host_id,
            public_key: None,
            manifest_version: 0,
            advertised_version: 0,
            is_gateway: false,
            delegated_users: Vec::new(),
            manifest_signature: None,
            retained_chunks: None,
            learn_sphere: None,
            manifest_log: crate::router_v2::manifest::ManifestLog::default(),
        },
    );

    let peer = fresh_peer();
    state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);
    state
        .mirrors
        .write()
        .unwrap()
        .get_mut(&peer)
        .unwrap()
        .nodes
        .bind(0, host_id);

    let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
    state
        .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
        .unwrap();

    assert_eq!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .manifest_version,
        0,
    );
}

#[test]
fn tampered_chunk_signature_dropped() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, vec![]);
    let mut msg = chunks.into_iter().next().unwrap();
    msg.manifest_signature[0] ^= 0xFF;

    state
        .handle_node_manifest(msg, 0, ConnectionModule::Lan)
        .unwrap();

    assert_eq!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .manifest_version,
        0,
    );
}

// ---------- per-entry filtering ----------

/// One bad entry sig + one good → only the bad one filtered; the
/// good one lands in the Node's delegated_users.
#[test]
fn bad_per_entry_signature_drops_only_that_entry() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (good_kp, good_mk) = keypair_and_multikey();
    let good_id = install_user_with_key(&state, &good_mk);
    let (bad_kp, bad_mk) = keypair_and_multikey();
    let bad_id = install_user_with_key(&state, &bad_mk);

    let good_entry = sign_entry(&good_kp, &host_mk, good_id, 1_000_000);
    let mut bad_entry = sign_entry(&bad_kp, &host_mk, bad_id, 1_000_000);
    bad_entry.entry_signature[0] ^= 0xFF;

    let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, vec![good_entry, bad_entry]);
    state
        .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    // §8.8 step 5: both entries are *stored* byte-exact — the manifest
    // signature covers the whole set, so dropping one would leave this node
    // unable to serve it or to apply a later delta against it.
    assert_eq!(node.delegated_users.len(), 2);

    // Verification gates *use*: only the well-signed entry earns a
    // delegation gateway.
    drop(node);
    drop(nodes);
    let users = state.users.read().unwrap();
    assert_eq!(
        users
            .get(&good_id)
            .unwrap()
            .read()
            .unwrap()
            .delegation_gateways
            .len(),
        1,
        "the correctly signed entry is trusted"
    );
    assert!(
        users
            .get(&bad_id)
            .unwrap()
            .read()
            .unwrap()
            .delegation_gateways
            .is_empty(),
        "a bad per-entry signature must not earn trust"
    );
}

#[test]
fn expired_entry_dropped_at_receive_time() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = install_user_with_key(&state, &user_mk);

    // timeout=500, now=1000 → expired.
    let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 500)];
    let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
    state
        .handle_node_manifest(
            chunks.into_iter().next().unwrap(),
            1_000,
            ConnectionModule::Lan,
        )
        .unwrap();

    // Stored regardless — expiry is a trust judgement, not a storage one,
    // and the stored set must stay byte-identical to what was signed.
    assert_eq!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .delegated_users
            .len(),
        1,
    );

    // §10.4: an expired delegation is never trusted.
    assert!(
        state
            .users
            .read()
            .unwrap()
            .get(&user_id)
            .unwrap()
            .read()
            .unwrap()
            .delegation_gateways
            .is_empty(),
        "an expired delegation must not earn a gateway"
    );
}

// ---------- flag propagation ----------

#[test]
fn is_gateway_flag_reflected_in_node_record() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let chunks = build_signed_manifest(&host_kp, &host_mk, 1, true, vec![]);
    state
        .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
        .unwrap();

    assert!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .is_gateway,
    );
}

/// An entry whose subject's key we do not hold is stored but not trusted.
///
/// This is exactly why storage and trust are separated: the entry survives,
/// so the manifest stays servable and a later delta still applies against
/// it, and the entry becomes trusted the moment §11.5 delivers the profile
/// — without re-fetching the manifest.
#[test]
fn entry_for_user_with_unknown_key_is_stored_but_untrusted() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = user_mk.to_id();
    // Do NOT install user — their key is unknown.

    let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
    let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
    state
        .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
        .unwrap();

    assert_eq!(
        state
            .nodes
            .read()
            .unwrap()
            .get(&host_id)
            .unwrap()
            .read()
            .unwrap()
            .delegated_users
            .len(),
        1,
        "stored byte-exact even though unverifiable"
    );

    // A stub User record exists, but with no gateway — nothing vouches for
    // it yet.
    assert!(state
        .users
        .read()
        .unwrap()
        .get(&user_id)
        .unwrap()
        .read()
        .unwrap()
        .delegation_gateways
        .is_empty());
}

// ---------- §3.8 trigger 3, node space ----------
//
// A node mapping carries `manifest_version` and, through the manifest it
// announces, the target's gateway status. Both are what the §2.3 membrane
// filters on, so a change to either has to re-introduce the index — otherwise
// a node that becomes a gateway after its mapping was already delivered to
// Local-sphere peers is never introduced to Internet peers, and its entries
// are dropped there as unknown mappings forever.

#[test]
fn committing_a_manifest_marks_the_origin_index_for_re_introduction() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);
    // our own index for the origin, which is what downstream peers see
    bind_own_dict(&state, index::Space::Node, 30, host_id);
    // start from a clean tracker: registration marks may already be pending
    let _ = state.pending_introductions(index::Space::Node);

    let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, Vec::new());
    state
        .handle_node_manifest(
            chunks.into_iter().next().unwrap(),
            500,
            ConnectionModule::Lan,
        )
        .unwrap();

    let pending: Vec<u16> = state
        .pending_introductions(index::Space::Node)
        .iter()
        .map(|t| t.0)
        .collect();
    assert!(
        pending.contains(&30),
        "a committed manifest changes what the mapping advertises (§3.8 trigger 3)"
    );
}

#[test]
fn re_committing_the_same_version_does_not_re_mark() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);
    bind_own_dict(&state, index::Space::Node, 30, host_id);

    let apply = |version: u32| {
        let chunks = build_signed_manifest(&host_kp, &host_mk, version, true, Vec::new());
        state
            .handle_node_manifest(
                chunks.into_iter().next().unwrap(),
                500,
                ConnectionModule::Lan,
            )
            .unwrap();
    };

    apply(5);
    let _ = state.pending_introductions(index::Space::Node);

    apply(5);
    let pending: Vec<u16> = state
        .pending_introductions(index::Space::Node)
        .iter()
        .map(|t| t.0)
        .collect();
    assert!(
        !pending.contains(&30),
        "nothing the mapping carries changed, so re-introducing would be pure noise"
    );
}

// ---------- §8.8 step 4: freshness of the completed manifest ----------
//
// The chunk signature proves the origin authored that version, not that it is
// the latest version the origin authored. A manifest the origin genuinely
// signed earlier therefore verifies perfectly and must still be refused, or it
// rolls the committed delegation set back to whatever it held at that version.

/// Commit v5, then replay a genuinely signed v3. The older manifest must not
/// overwrite the newer committed state.
#[test]
fn older_signed_manifest_does_not_roll_back_committed_state() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = install_user_with_key(&state, &user_mk);

    // v5: gateway, carrying one delegation.
    let v5 = build_signed_manifest(
        &host_kp,
        &host_mk,
        5,
        true,
        vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)],
    );
    state
        .handle_node_manifest(v5.into_iter().next().unwrap(), 500, ConnectionModule::Lan)
        .unwrap();

    // v3: the state before that delegation existed, and before the gateway bit.
    let v3 = build_signed_manifest(&host_kp, &host_mk, 3, false, vec![]);
    state
        .handle_node_manifest(v3.into_iter().next().unwrap(), 600, ConnectionModule::Lan)
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    assert_eq!(node.manifest_version, 5, "committed version rolled back");
    assert!(node.is_gateway, "gateway flag rolled back");
    assert_eq!(
        node.delegated_users.len(),
        1,
        "delegation set rolled back — a revoked entry would have been resurrected"
    );
    assert_eq!(node.delegated_users[0].user_id, user_id);
}

/// §8.8 says *strictly* fresher, so re-delivering the committed version is a
/// discard too.
#[test]
fn equal_version_manifest_is_discarded() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = install_user_with_key(&state, &user_mk);

    let v5 = build_signed_manifest(
        &host_kp,
        &host_mk,
        5,
        true,
        vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)],
    );
    state
        .handle_node_manifest(v5.into_iter().next().unwrap(), 500, ConnectionModule::Lan)
        .unwrap();

    // Same version, emptied entry set: a well-formed signature over v5 with
    // different contents is exactly what the freshness rule has to refuse.
    let v5_again = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
    state
        .handle_node_manifest(
            v5_again.into_iter().next().unwrap(),
            600,
            ConnectionModule::Lan,
        )
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    assert_eq!(node.manifest_version, 5);
    assert!(node.is_gateway);
    assert_eq!(node.delegated_users.len(), 1);
}

/// A fresher manifest still commits over an older one — the check must not be
/// so broad that it blocks ordinary §10.8 synchronization.
#[test]
fn fresher_manifest_still_commits() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    let v3 = build_signed_manifest(&host_kp, &host_mk, 3, false, vec![]);
    state
        .handle_node_manifest(v3.into_iter().next().unwrap(), 500, ConnectionModule::Lan)
        .unwrap();

    let (user_kp, user_mk) = keypair_and_multikey();
    let user_id = install_user_with_key(&state, &user_mk);
    let v4 = build_signed_manifest(
        &host_kp,
        &host_mk,
        4,
        true,
        vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)],
    );
    state
        .handle_node_manifest(v4.into_iter().next().unwrap(), 600, ConnectionModule::Lan)
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    assert_eq!(node.manifest_version, 4);
    assert!(node.is_gateway);
    assert_eq!(node.delegated_users.len(), 1);
}

/// The assembler keyed partials by origin alone and its fallback arm replaced a
/// partial at any other version, so one stale chunk could discard a fresher
/// multi-chunk assembly mid-flight and leave the pull unfinished.
#[test]
fn stale_chunk_does_not_discard_a_fresher_assembly_in_flight() {
    let (state, mut _rx) = fresh_state();
    let (host_kp, host_mk) = keypair_and_multikey();
    let (_, host_id) = setup_self_origin(&state, &host_mk);

    // Commit v5 so there is a committed version for the stale chunk to lose to.
    let v5 = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
    state
        .handle_node_manifest(v5.into_iter().next().unwrap(), 500, ConnectionModule::Lan)
        .unwrap();

    // 731 entries exceeds one chunk at 84 bytes per entry in a 60 KiB body.
    // Entry signatures do not need to verify here: the chunk signature is what
    // gates assembly, and this test is about the assembly surviving.
    let bulk: Vec<ManifestEntry> = (0..731u32)
        .map(|i| {
            let mut user_id = [0u8; 8];
            user_id[..4].copy_from_slice(&i.to_be_bytes());
            ManifestEntry {
                user_id,
                timeout: 1_000_000,
                entry_signature: [0u8; 64],
                profile_version: 0,
            }
        })
        .collect();
    let v6 = build_signed_manifest(&host_kp, &host_mk, 6, false, bulk);
    assert_eq!(v6.len(), 2, "fixture must produce a multi-chunk manifest");
    let mut v6 = v6.into_iter();
    let v6_first = v6.next().unwrap();
    let v6_second = v6.next().unwrap();

    // Chunk 0 of v6 arrives and is held as a partial.
    state
        .handle_node_manifest(v6_first, 600, ConnectionModule::Lan)
        .unwrap();

    // A stale v4 chunk lands between the two halves.
    let v4 = build_signed_manifest(&host_kp, &host_mk, 4, false, vec![]);
    state
        .handle_node_manifest(v4.into_iter().next().unwrap(), 650, ConnectionModule::Lan)
        .unwrap();

    // Chunk 1 of v6 completes the assembly, which must still be intact.
    state
        .handle_node_manifest(v6_second, 700, ConnectionModule::Lan)
        .unwrap();

    let nodes = state.nodes.read().unwrap();
    let node_arc = nodes.get(&host_id).unwrap();
    let node = node_arc.read().unwrap();
    assert_eq!(
        node.manifest_version, 6,
        "the stale chunk discarded the in-flight v6 assembly"
    );
}
