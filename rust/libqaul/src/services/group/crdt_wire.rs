// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group CRDT wire codec: signing & verification
//!
//! Bridges the proto-free [`crdt`](super::crdt) core and the
//! `qaul.net.group.GroupOp` wire message. A `GroupOp` is signed by the
//! actor's identity (Ed25519) key over the deterministic protobuf
//! encoding of the message with its `signature` field empty; verifiers
//! resolve the actor's public key from its `actor_id` (a PeerId is a
//! hash of the key) and check both the signature and that `actor_id`
//! matches that key.
//!
//! The router-side public-key lookup is intentionally kept out of this
//! module: [`verify_and_decode`] takes the resolved `PublicKey`, so the
//! sign/verify logic is unit-testable without the wider stack. The
//! integration layer resolves the key (e.g. via `Users::get_pub_key`).

use libp2p::identity::{Keypair, PublicKey};
use libp2p::PeerId;
use prost::Message;

use super::crdt::{GroupOp, OpId, OpKind};

/// Import the generated group protobuf types.
pub use qaul_proto::qaul_net_group as proto_net;

/// Build and sign a `GroupOp` wire message from core fields.
///
/// `keys` is the actor's identity keypair; `actor_id` is derived from
/// it. The signature covers the encoded message with `signature` empty.
/// Returns `None` if signing fails.
pub fn sign_op(
    keys: &Keypair,
    group_id: &[u8],
    op_id: OpId,
    lamport: u64,
    created_at: u64,
    kind: &OpKind,
) -> Option<proto_net::GroupOp> {
    let actor_id = keys.public().to_peer_id().to_bytes();
    let mut msg = proto_net::GroupOp {
        group_id: group_id.to_vec(),
        op_id: op_id.to_vec(),
        actor_id,
        lamport,
        created_at,
        op: Some(kind_to_proto(kind)),
        signature: Vec::new(),
    };

    // sign the canonical (signature-empty) encoding
    let unsigned = msg.encode_to_vec();
    match keys.sign(&unsigned) {
        Ok(sig) => {
            msg.signature = sig;
            Some(msg)
        }
        Err(e) => {
            log::error!("group op signing failed: {}", e);
            None
        }
    }
}

/// Verify a wire `GroupOp` against `actor_pubkey` and, on success,
/// decode it into a core [`GroupOp`].
///
/// Checks: (1) the signature verifies over the signature-empty
/// encoding, (2) `actor_id` is a valid PeerId equal to the PeerId of
/// `actor_pubkey`, (3) `op_id` is exactly 16 bytes, (4) the `op` oneof
/// is present and well-formed. Returns `None` on any failure.
pub fn verify_and_decode(
    op: &proto_net::GroupOp,
    actor_pubkey: &PublicKey,
) -> Option<GroupOp> {
    // 1. actor_id must be the PeerId of the supplied key
    let actor_peer = PeerId::from_bytes(&op.actor_id).ok()?;
    if actor_peer != actor_pubkey.to_peer_id() {
        log::warn!("group op actor_id does not match supplied public key");
        return None;
    }

    // 2. re-encode with an empty signature and verify
    let mut unsigned = op.clone();
    unsigned.signature = Vec::new();
    let unsigned_bytes = unsigned.encode_to_vec();
    if !actor_pubkey.verify(&unsigned_bytes, &op.signature) {
        log::warn!("group op signature verification failed");
        return None;
    }

    // 3. op_id must be 16 bytes
    let op_id: OpId = op.op_id.as_slice().try_into().ok()?;

    // 4. decode the op kind
    let kind = proto_to_kind(op.op.as_ref()?)?;

    Some(GroupOp {
        op_id,
        actor_id: op.actor_id.clone(),
        lamport: op.lamport,
        created_at: op.created_at,
        kind,
    })
}

/// Decode a wire `GroupOp` into a core [`GroupOp`] **without** checking
/// the signature. Only for ops already verified before storage (see
/// `crdt_store::load_crdt`); never call this on freshly-received
/// network data — use [`verify_and_decode`] there.
pub fn decode_trusted(op: &proto_net::GroupOp) -> Option<GroupOp> {
    let op_id: OpId = op.op_id.as_slice().try_into().ok()?;
    let kind = proto_to_kind(op.op.as_ref()?)?;
    Some(GroupOp {
        op_id,
        actor_id: op.actor_id.clone(),
        lamport: op.lamport,
        created_at: op.created_at,
        kind,
    })
}

/// Convert a core [`OpKind`] into the proto oneof.
fn kind_to_proto(kind: &OpKind) -> proto_net::group_op::Op {
    match kind {
        OpKind::Add { member_id, role } => {
            proto_net::group_op::Op::Add(proto_net::AddMemberOp {
                member_id: member_id.clone(),
                role: *role,
            })
        }
        OpKind::Remove {
            member_id,
            observed_adds,
        } => proto_net::group_op::Op::Remove(proto_net::RemoveMemberOp {
            member_id: member_id.clone(),
            observed_adds: observed_adds.iter().map(|o| o.to_vec()).collect(),
        }),
        OpKind::UpdateMetadata { name, avatar } => {
            proto_net::group_op::Op::Meta(proto_net::UpdateMetadataOp {
                name: name.clone(),
                avatar: avatar.clone(),
            })
        }
        OpKind::Compact { epoch, below } => {
            proto_net::group_op::Op::Compact(proto_net::CompactOp {
                epoch: *epoch,
                below: *below,
            })
        }
    }
}

/// Convert a proto oneof into a core [`OpKind`], rejecting malformed
/// op_ids in a `Remove`'s `observed_adds`.
fn proto_to_kind(op: &proto_net::group_op::Op) -> Option<OpKind> {
    match op {
        proto_net::group_op::Op::Add(a) => Some(OpKind::Add {
            member_id: a.member_id.clone(),
            role: a.role,
        }),
        proto_net::group_op::Op::Remove(r) => {
            let mut observed = Vec::with_capacity(r.observed_adds.len());
            for b in &r.observed_adds {
                // skip malformed op_ids rather than fail the whole op:
                // a 16-byte tombstone reference is required.
                let id: OpId = b.as_slice().try_into().ok()?;
                observed.push(id);
            }
            Some(OpKind::Remove {
                member_id: r.member_id.clone(),
                observed_adds: observed,
            })
        }
        proto_net::group_op::Op::Meta(m) => Some(OpKind::UpdateMetadata {
            name: m.name.clone(),
            avatar: m.avatar.clone(),
        }),
        proto_net::group_op::Op::Compact(c) => Some(OpKind::Compact {
            epoch: c.epoch,
            below: c.below,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oid(b: u8) -> OpId {
        let mut o = [0u8; 16];
        o[0] = b;
        o
    }

    // sign → verify round-trips and reconstructs the core op exactly.
    #[test]
    fn sign_verify_round_trip() {
        let keys = Keypair::generate_ed25519();
        let pubkey = keys.public();
        let actor_id = pubkey.to_peer_id().to_bytes();

        let kind = OpKind::Add {
            member_id: vec![2; 4],
            role: 255,
        };
        let wire = sign_op(&keys, b"group-1", oid(7), 42, 1000, &kind).expect("sign");
        assert!(!wire.signature.is_empty());

        let core = verify_and_decode(&wire, &pubkey).expect("verify");
        assert_eq!(core.op_id, oid(7));
        assert_eq!(core.actor_id, actor_id);
        assert_eq!(core.lamport, 42);
        assert_eq!(core.created_at, 1000);
        assert_eq!(core.kind, kind);
    }

    // every op kind round-trips.
    #[test]
    fn all_kinds_round_trip() {
        let keys = Keypair::generate_ed25519();
        let pk = keys.public();
        let kinds = vec![
            OpKind::Add { member_id: vec![1; 4], role: 0 },
            OpKind::Remove { member_id: vec![1; 4], observed_adds: vec![oid(1), oid(2)] },
            OpKind::UpdateMetadata { name: Some("g".into()), avatar: Some(vec![9, 9]) },
            OpKind::UpdateMetadata { name: None, avatar: None },
            OpKind::Compact { epoch: 3, below: 100 },
        ];
        for (i, k) in kinds.iter().enumerate() {
            let w = sign_op(&keys, b"g", oid(i as u8), i as u64, 0, k).unwrap();
            let c = verify_and_decode(&w, &pk).unwrap();
            assert_eq!(&c.kind, k, "kind {} round-trip", i);
        }
    }

    // a tampered field invalidates the signature.
    #[test]
    fn tampered_op_rejected() {
        let keys = Keypair::generate_ed25519();
        let pk = keys.public();
        let mut wire = sign_op(
            &keys,
            b"g",
            oid(1),
            1,
            0,
            &OpKind::Add { member_id: vec![2; 4], role: 0 },
        )
        .unwrap();
        // flip the lamport after signing
        wire.lamport = 999;
        assert!(verify_and_decode(&wire, &pk).is_none(), "tampered op must fail");
    }

    // verifying against a different key fails.
    #[test]
    fn wrong_key_rejected() {
        let keys = Keypair::generate_ed25519();
        let other = Keypair::generate_ed25519().public();
        let wire = sign_op(
            &keys,
            b"g",
            oid(1),
            1,
            0,
            &OpKind::Add { member_id: vec![2; 4], role: 0 },
        )
        .unwrap();
        // actor_id (from `keys`) won't match `other`'s PeerId
        assert!(verify_and_decode(&wire, &other).is_none());
    }

    // an op_id that is not 16 bytes is rejected.
    #[test]
    fn bad_op_id_len_rejected() {
        let keys = Keypair::generate_ed25519();
        let pk = keys.public();
        let mut wire = sign_op(
            &keys,
            b"g",
            oid(1),
            1,
            0,
            &OpKind::Add { member_id: vec![2; 4], role: 0 },
        )
        .unwrap();
        // corrupt op_id length then re-sign so the signature is valid
        // but the op_id is structurally invalid
        wire.op_id = vec![1, 2, 3];
        let unsigned = {
            let mut u = wire.clone();
            u.signature = Vec::new();
            u.encode_to_vec()
        };
        wire.signature = keys.sign(&unsigned).unwrap();
        assert!(verify_and_decode(&wire, &pk).is_none(), "non-16-byte op_id rejected");
    }
}
