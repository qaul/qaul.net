# CRDT for group membership and metadata

## Problem

`Group::members: BTreeMap<Vec<u8>, GroupMember>` and `Group::revision: u32`
(`rust/libqaul/src/services/group/mod.rs:84`) merge today via a monotonic
revision counter — effectively last-writer-wins on the whole
member-table. In a DTN mesh two members can concurrently invite or
remove members while partitioned; whoever lands with the higher
`revision` wins and the other operation is silently dropped. There is
no deterministic merge story.

The invite/reply path (`group::mod.rs:350` → `Member::on_be_invited`)
also bakes "the sender is authoritative" into the wire format — fine
for the happy path, brittle under partitions and admin contention.

## Proposal: signed-op OR-Set + LWW register

Replace the rev-counter merge with two CRDTs:

- **Membership** — Observed-Remove Set (OR-Set) keyed by `member_id`.
  Every `Add` carries a unique `op_id`; a `Remove` tombstones the set
  of `op_id`s it has observed. Re-adds work naturally (a fresh `Add`
  with a new `op_id`).
- **Metadata** (name, avatar, settings) — per-field LWW register
  keyed by `(lamport, actor_id)`, so concurrent edits resolve
  deterministically without surfacing as conflicts.

Each op is signed by the actor's identity key and carries a small
authorization claim (member-status or admin-status of the actor at
op time). Merge is unconditional; **authorization is checked at
apply time** — unauthorised ops merge but don't take effect, so a
later admin promotion can retroactively legitimise an op without a
rebroadcast.

## Wire format

```proto
message GroupOp {
    bytes group_id = 1;
    bytes op_id = 2;          // 16-byte random UUID
    bytes actor_id = 3;       // PeerId of the signer
    uint64 lamport = 4;       // group-local lamport clock
    uint64 created_at = 5;
    oneof op {
        AddMember add = 6;
        RemoveMember remove = 7;
        UpdateMetadata meta = 8;
    }
    bytes signature = 9;      // sign(actor's identity key, fields 1..8)
}

message AddMember {
    bytes member_id = 1;
    int32 role = 2;           // 0 = member, 255 = admin
}

message RemoveMember {
    bytes member_id = 1;
    repeated bytes observed_adds = 2;  // op_ids this remove tombstones
}

message UpdateMetadata {
    optional string name = 1;
    optional bytes avatar = 2;
}
```

Wire-compat with today: a new `GroupOp` variant on
`group_container::Message`, gated on
`Capabilities::GROUP_CRDT = 1 << 3`. Members without the bit keep
seeing today's invite/reply path; the CRDT layer translates between
the two as long as a mixed group exists.

## Authorisation policy (v1)

Kept deliberately small:

- Any member may `AddMember` (open invite). An invite-only mode flips
  this to "admins only" without changing the wire format — same op,
  different apply-time check.
- Only admins may `RemoveMember` or `UpdateMetadata`.
- The group creator is the bootstrap admin; admin promotions are
  `AddMember` ops with `role = 255` signed by an existing admin.

## Garbage collection

Tombstone bloat is the well-known CRDT failure mode and is genuinely
hard in a DTN system where a partitioned peer might resurface weeks
later. Pragmatic plan:

- Per-group **epoch counter**: admins can issue a `Compact(epoch, N)`
  op that all members must apply before accepting ops below epoch
  `N`. Ops from a peer that comes back beyond the compaction horizon
  are rejected, not merged — same trade as the rotation
  `rotation_max_stall_seconds` cap.
- Default horizon ~30 days, matching the DTN custody upper bound.
- Tombstones inside the live epoch are kept; only cross-epoch state
  is collapsed.

## Threat model

Signatures bind every op to its actor. Authorisation is enforced at
apply, so adversarial ops merge but are no-ops. The remaining
surface:

- A revoked admin can sign ops "before" their removal in lamport
  time and have them apply on peers that haven't seen the removal
  yet — unavoidable in a partition-tolerant system. Bounded by the
  compaction horizon.
- Concurrent add-vs-remove of the same member: OR-Set gives
  "remove only tombstones the adds it observed", so a concurrent
  fresh add wins. This is the right default for invites in a mesh
  ("I haven't seen them kicked, so I added them") but should be
  flagged for product review — the alternative is a one-way
  per-member kill bit.

## Out of scope

- Chat history as a CRDT. Messages already have natural causal
  ordering via sender + timestamp and don't need merge resolution.
- Multi-tier admin roles. Single admin bit for now.
- Re-keying the envelope encryption ([[group-file-envelope]]) on
  membership change. The envelope path reads the current membership
  view at send time; CRDT convergence happens underneath.

## Open questions

- Add-wins vs. remove-wins on concurrent add-vs-remove of the same
  member — product decision, not a crypto one.
- Should `Compact` ops require a quorum of admin signatures, or is
  a single admin enough? Quorum is safer; single is simpler.
- Lamport vs. hybrid logical clock. Lamport is simpler; HLC gives a
  better UX for "when was this user added" displays.
