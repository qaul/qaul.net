# Groups — Membership and File Encryption

Two changes to how group chats manage state and encrypt files.

Code: `rust/libqaul/src/services/group/`, `rust/libqaul/src/services/chat/file.rs`.

## Membership CRDT

Group membership and metadata converge through a CRDT instead of a
revision-counter merge, so concurrent changes on different nodes reconcile
without a coordinator.

- **Membership** is an OR-Set: adds and removes are independent operations, so a
  member added on one node and removed on another converge deterministically
  rather than depending on which revision number won.
- **Metadata** (name, etc.) is last-writer-wins.
- Every operation is **signed**, so membership changes are attributable and
  can't be forged.
- **Epoch compaction** collapses old tombstone history periodically, bounding
  how large the op set grows.

The CRDT is the source of truth for membership; the live group state is derived
from it.

```sh
qauld-ctl group crdt-view      # inspect the derived membership/metadata state
qauld-ctl group crdt-compact   # collapse tombstone history (admin)
```

## File envelope encryption

Reduces the cost of sending a file to a group. Instead of encrypting the whole
file body once per recipient (N× the work for N members), the body is encrypted
**once** under a per-file content key, and only that small key is wrapped per
recipient. Each member unwraps the key and decrypts the shared body.

> **Status:** in progress — the primitives, wire format, and key store are in
> place, but end-to-end send/receive is not yet working. Not ready for use.
