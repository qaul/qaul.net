// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group CRDT op-set persistence
//!
//! Stores the grow-only set of signed `GroupOp`s per group in the
//! per-account `group_ops` sled tree and rebuilds a [`GroupCrdt`] from
//! it on demand.
//!
//! Ops are stored as their encoded `qaul.net.group.GroupOp` bytes,
//! keyed by `group_id ++ op_id`. **Signature verification happens once,
//! on receive, before [`save_op`]** (the network handler resolves the
//! actor's public key and calls [`crdt_wire::verify_and_decode`]); ops
//! loaded back from the local store are trusted and only structurally
//! decoded.

use prost::Message;

use super::crdt::{GroupCrdt, OpId};
use super::crdt_wire::{self, proto_net};
use super::storage::GroupAccountDb;

/// Storage key for one op: `group_id ++ op_id`.
fn op_key(group_id: &[u8], op_id: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(group_id.len() + op_id.len());
    k.extend_from_slice(group_id);
    k.extend_from_slice(op_id);
    k
}

/// Key range covering every op of one group.
fn group_range(group_id: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let first = op_key(group_id, &[0u8; 16]);
    let last = op_key(group_id, &[0xffu8; 16]);
    (first, last)
}

/// Persist an already-verified `GroupOp` into the op set. Idempotent:
/// re-storing the same `(group_id, op_id)` overwrites identical bytes.
pub fn save_op(db: &GroupAccountDb, group_id: &[u8], op: &proto_net::GroupOp) {
    if op.op_id.len() != 16 {
        log::warn!("refusing to store group op with non-16-byte op_id");
        return;
    }
    let key = op_key(group_id, &op.op_id);
    let bytes = op.encode_to_vec();
    if let Err(e) = db.ops.insert(key, bytes) {
        log::error!("group op store insert: {}", e);
        return;
    }
    if let Err(e) = db.ops.flush() {
        log::error!("group op store flush: {}", e);
    }
}

/// Rebuild the [`GroupCrdt`] for a group from its persisted op set.
///
/// `founder` is the group creator (bootstrap admin). Stored ops are
/// decoded structurally and folded in; they were signature-verified
/// before storage, so no public-key lookup is needed here. Ops that no
/// longer decode (corruption / forward-incompatible) are skipped.
pub fn load_crdt(db: &GroupAccountDb, group_id: &[u8], founder: Vec<u8>) -> GroupCrdt {
    let mut crdt = GroupCrdt::new(founder);
    let (first, last) = group_range(group_id);
    for item in db.ops.range(first..last) {
        let (_k, bytes) = match item {
            Ok(kv) => kv,
            Err(e) => {
                log::error!("group op store iter: {}", e);
                continue;
            }
        };
        let proto = match proto_net::GroupOp::decode(&bytes[..]) {
            Ok(p) => p,
            Err(e) => {
                log::error!("group op decode (stored): {}", e);
                continue;
            }
        };
        if let Some(core) = crdt_wire::decode_trusted(&proto) {
            crdt.merge_op(core);
        }
    }
    crdt
}

/// Whether an op with `op_id` is already stored for `group_id`.
pub fn has_op(db: &GroupAccountDb, group_id: &[u8], op_id: &OpId) -> bool {
    db.ops
        .get(op_key(group_id, op_id))
        .map(|v| v.is_some())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::crdt::{OpKind, ROLE_ADMIN, ROLE_MEMBER};
    use libp2p::identity::Keypair;

    fn test_db() -> GroupAccountDb {
        let db = sled::Config::new().temporary(true).open().unwrap();
        GroupAccountDb {
            groups: db.open_tree("groups").unwrap(),
            invited: db.open_tree("invited").unwrap(),
            ops: db.open_tree("group_ops").unwrap(),
        }
    }

    fn oid(b: u8) -> OpId {
        let mut o = [0u8; 16];
        o[0] = b;
        o
    }

    // ops saved for a group are reloaded and folded into the same view.
    #[test]
    fn save_load_round_trip() {
        let db = test_db();
        let founder_keys = Keypair::generate_ed25519();
        let founder = founder_keys.public().to_peer_id().to_bytes();
        let group = b"group-1".to_vec();
        let member = vec![2u8; 4];

        // founder signs an Add(member) op and we persist it
        let op = crdt_wire::sign_op(
            &founder_keys,
            &group,
            oid(1),
            1,
            0,
            &OpKind::Add { member_id: member.clone(), role: ROLE_MEMBER },
        )
        .unwrap();
        save_op(&db, &group, &op);
        assert!(has_op(&db, &group, &oid(1)));

        // reload and derive the view
        let crdt = load_crdt(&db, &group, founder.clone());
        let view = crdt.view();
        assert!(view.is_member(&member), "saved add survives reload");
        assert!(view.is_admin(&founder));
        assert_eq!(crdt.op_count(), 1);
    }

    // ops are isolated per group.
    #[test]
    fn per_group_isolation() {
        let db = test_db();
        let keys = Keypair::generate_ed25519();
        let founder = keys.public().to_peer_id().to_bytes();
        let g1 = b"g1".to_vec();
        let g2 = b"g2".to_vec();

        let op = crdt_wire::sign_op(
            &keys,
            &g1,
            oid(1),
            1,
            0,
            &OpKind::Add { member_id: vec![9; 4], role: ROLE_ADMIN },
        )
        .unwrap();
        save_op(&db, &g1, &op);

        // g1 has the op; g2 does not
        assert_eq!(load_crdt(&db, &g1, founder.clone()).op_count(), 1);
        assert_eq!(load_crdt(&db, &g2, founder).op_count(), 0);
    }

    // a full add → remove sequence reloads to the converged view.
    #[test]
    fn sequence_reloads_converged() {
        let db = test_db();
        let keys = Keypair::generate_ed25519();
        let founder = keys.public().to_peer_id().to_bytes();
        let group = b"g".to_vec();
        let member = vec![3u8; 4];

        let add = crdt_wire::sign_op(&keys, &group, oid(1), 1, 0,
            &OpKind::Add { member_id: member.clone(), role: ROLE_MEMBER }).unwrap();
        save_op(&db, &group, &add);
        let remove = crdt_wire::sign_op(&keys, &group, oid(2), 2, 0,
            &OpKind::Remove { member_id: member.clone(), observed_adds: vec![oid(1)] }).unwrap();
        save_op(&db, &group, &remove);

        let view = load_crdt(&db, &group, founder).view();
        assert!(!view.is_member(&member), "remove applied after reload");
    }
}
