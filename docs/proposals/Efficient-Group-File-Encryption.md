# Efficient File Encryption for Groups

## Overview

We should move group files to **per-file content encryption with a random
symmetric file key, and distribute that file key to each member via the
existing per-peer session**. This is the standard envelope / hybrid
encryption pattern and cuts the per-file cost from O(members × file_size)
to O(file_size) + O(members × 32 bytes).

Today, `services/chat/file.rs` treats a file like any other chat payload:
one ciphertext per recipient, each encrypted end-to-end under that
recipient's `CryptoState` session key. For a 100 MiB file in a 50-member
group that means ~5 GiB of ciphertext produced and pushed. Envelope
encryption produces the 100 MiB body once and sends a small
`file_key` envelope per member instead.

A group-wide long-lived key would be simpler still but carries real
downsides (removal of a member requires rotating everything, lost members
can read historical traffic if they kept the key). Per-file keys are a
middle ground that we can extend with group rekey later.

## Scope

In scope:

- one symmetric `file_key` per file, one ciphertext body for the whole group
- envelope delivery of `file_key` to each member via existing per-peer
  sessions
- integrity binding between the envelope and the body
- correct handling of partial membership, late joiners, and resends
- concept documentation (this file) and protobuf shape

Out of scope:

- a persistent group key / MLS-style group ratchet
- re-encrypting historical files on membership change
- transport-level chunking or resumable transfers — the body is still the
  file as stored today, only the encryption shape changes
- public-key broadcast encryption / attribute-based schemes

## Proposed Design

### Security goals

- **Confidentiality:** only current members at send time can derive
  `file_key` and decrypt the body.
- **Integrity:** body ciphertext is authenticated and bound to the envelope
  so a member cannot be tricked into decrypting one file body under a
  different file's key.
- **Forward secrecy at the session layer:** envelopes inherit the per-peer
  session's PFS properties; rotating a session (see `session_rotation.md`)
  invalidates future envelopes but not already-received files.
- **No downgrade:** a sender that advertises envelope support must not
  silently fall back to per-recipient body encryption.

### File-side encryption

On send:

1. Generate a random `file_key` (32 bytes, ChaCha20-Poly1305 key).
2. Generate a random `file_nonce` (12 bytes) and encrypt the file body:
   `body_ct = ChaCha20Poly1305(file_key, file_nonce, body, aad = file_id ‖ group_id)`.
3. Compute `body_digest = BLAKE3(body_ct)`.
4. Build `FileHeader { file_id, group_id, file_nonce, body_digest, mime,
   size, filename }`.
5. Store `body_ct` once in the file store and gossip `FileHeader` +
   `body_ct` through the group's file distribution (same path used today,
   minus the per-recipient crypto).

The `file_id` is a random 16 bytes chosen by the sender. `body_digest`
binds the envelope to exactly this body.

### Envelope delivery

For each current member `m` at send time:

1. Build `FileKeyEnvelope { file_id, group_id, file_key, body_digest,
   sent_at, sender_id }`.
2. Send it to `m` as a normal `CryptoService` message under the existing
   per-peer session, i.e. encrypted end-to-end with PFS already provided by
   the session.

Envelope messages are small (fixed-size) and DTN-friendly. They are
idempotent: receiving the same `file_id` envelope twice is a no-op.

### Receiver behavior

On envelope receipt:

- Look up or create a pending entry for `file_id`.
- Store `file_key` and `body_digest` in that entry.
- If `body_ct` is already available locally, verify `BLAKE3(body_ct) ==
  body_digest`, decrypt, and expose to the UI.

On body receipt:

- Store `body_ct` in the file store.
- If an envelope for `file_id` is already pending, verify and decrypt as
  above.

A file is shown to the user only after both envelope and verified body are
present. Missing envelopes after a configurable `envelope_ttl` surface a
"sender has not yet shared the key with you" UI state; this handles late
joiners and out-of-order DTN arrival.

### Membership edges

- **New member joins after send:** existing members relay the envelope
  (not the body) on request, protected under their own session with the
  joiner. Relay is opt-in per sender policy (`allow_key_relay`, default
  `true`) and is rate-limited.
- **Member removed before send:** removed ids are not included in the
  envelope fan-out. The body remains visible to them only if they already
  obtained `file_key` in an earlier send of the same `file_id`, which is
  irrelevant because `file_id` is random per send.
- **Group rekey / expulsion:** out of scope here; envelope-per-file limits
  the blast radius so no mass re-encrypt is needed.

### Protobuf shape

Add to the file service protos (the chat/file service container) without
changing existing messages:

```protobuf
message FileHeader {
    bytes file_id = 1;
    bytes group_id = 2;
    bytes file_nonce = 3;
    bytes body_digest = 4; // BLAKE3 of body_ct
    string filename = 5;
    string mime = 6;
    uint64 size = 7;
    uint64 sent_at = 8;
}

message FileBody {
    bytes file_id = 1;
    bytes body_ct = 2; // ChaCha20Poly1305 ciphertext
}

message FileKeyEnvelope {
    bytes file_id = 1;
    bytes group_id = 2;
    bytes file_key = 3;      // 32 bytes
    bytes body_digest = 4;   // must match FileHeader.body_digest
    uint64 sent_at = 5;
    bytes sender_id = 6;
}

message FileKeyRelayRequest {
    bytes file_id = 1;
    bytes group_id = 2;
}
```

`FileHeader` and `FileBody` travel over the group distribution path
unencrypted at the service layer (they are encrypted as a unit by the body
key). `FileKeyEnvelope` travels inside `CryptoserviceContainer` as a new
oneof variant.

### Storage changes

- File store keeps `body_ct` plus `FileHeader` metadata, same as today.
- A new `file_keys` sled tree keyed by `file_id` stores received envelopes
  until the matching body is fully decrypted, plus locally generated keys
  for sent files that might need relay.
- No per-recipient ciphertext duplication.

### Configuration

Extend `configuration.rs` with a `GroupFiles` section:

```rust
pub struct GroupFiles {
    pub envelope_enabled: bool,
    pub allow_key_relay: bool,
    pub envelope_ttl_seconds: u64, // default 30 days
    pub max_relay_requests_per_minute: u32,
}
```

`envelope_enabled = false` restores today's per-recipient body encryption
for compatibility during rollout.

### Capability negotiation

Peers advertise `file_envelope = true` in their capability set. Senders
only use envelope mode when all current group members advertise it; any
member without support causes that single send to fall back to
per-recipient body encryption for backward compatibility. This is logged
and surfaced so users can see why a send was heavy.

## What needs to be implemented

- `FileHeader`, `FileBody`, `FileKeyEnvelope`, `FileKeyRelayRequest`
  protobuf messages; new oneof variants in the chat/file container and
  `CryptoserviceContainer`.
- Sender path in `services/chat/file.rs`: generate `file_key` and
  `file_nonce`, encrypt body once, compute `body_digest`, fan out
  `FileKeyEnvelope` to each current group member over per-peer sessions,
  distribute `FileHeader` + `FileBody` via the existing group file path.
- Receiver path: pending-entry keyed by `file_id`, matching of envelope to
  body, digest verification before decrypt, idempotent handling of
  duplicate envelopes and duplicate bodies.
- `file_keys` sled tree for pending envelopes and locally held file keys;
  expiry by `envelope_ttl_seconds`.
- Key relay: respond to `FileKeyRelayRequest` under the session with the
  requesting peer, gated by `allow_key_relay` and
  `max_relay_requests_per_minute`.
- `GroupFiles` config section, RPC exposure, UI toggles.
- Capability advertisement bit `file_envelope`; sender-side downgrade only
  when any member lacks it, with a UI indicator.
- UI states: `WaitingForFileKey`, `FileKeyExpired`, `FallbackPerRecipient`.
- Integration coverage in `tests/integration/local_mesh.py`: 3-, 10-, and
  50-member groups; envelope arrives before body and vice versa; new
  member joins after send and fetches the key via relay; member removed
  before send does not receive envelope; capability fallback when one
  member is pre-envelope.

## Risks and Controls

- **Envelope-body mismatch attack:** mitigated by `body_digest` binding in
  the envelope and verified before decrypt.
- **Missing envelope → unreadable file:** surfaced as
  `WaitingForFileKey` with `envelope_ttl` expiry; relay mechanism covers
  late joiners.
- **Key relay abuse:** gated by `allow_key_relay` and rate limiting;
  relayers only respond for groups they are a current member of.
- **Downgrade attack via forged capability:** capability is advertised
  through authenticated identity channels; the sender UI shows which
  member caused a fallback so users can investigate.
- **Local key leakage:** `file_keys` tree is zeroized on deletion;
  entries expire by TTL even if the body never arrives.
- **Large groups, many envelopes:** envelopes are ~100 bytes each, sent
  over per-peer sessions; the saving versus per-recipient body encryption
  grows with file size and membership.

## Rollback

- set `GroupFiles.envelope_enabled = false`
- senders revert to per-recipient body encryption for new files
- already-sent envelope files remain decryptable because their keys are
  already in recipients' `file_keys` trees
- no on-disk migration; new tree and protos are additive

## Decision Gate

Proceed only if all of the following are true:

- per-file random keys are accepted over a long-lived group key, given
  membership-change semantics
- capability-based downgrade is acceptable for mixed-version groups
- local multi-node tests show no file loss across envelope/body reorder,
  late joiners, and rate-limited relays
- storage budget for `file_keys` TTL (default 30 d) is acceptable

## Conclusion

Envelope encryption makes group file sends roughly linear in body size
plus constant-per-member overhead, instead of quadratic. It preserves
per-peer PFS from the session layer, adds no new long-lived group secret,
and degrades cleanly to today's behavior for pre-envelope peers.
