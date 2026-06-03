# Crypto Session Rotation with Forward and Backward Secrecy

## Overview

We should implement this feature as **full Noise session rotation with an
overlap window**, not as a per-message symmetric ratchet.

The current qaul crypto uses `Noise_KK_X25519_ChaChaPoly_SHA256` with one
long-lived session per peer, stored in `CryptoState` and indexed by a 4 byte
`session_id`. A full re-handshake already exists conceptually (session id is
picked at handshake time), so the cheapest path to PFS and backward security is
to re-run the KK handshake on a schedule and keep the previous session alive
for a short, bounded grace period.

A per-message ratchet would add strong properties but would be hostile to our
DTN and out-of-order model (`out_of_order_indexes`, delayed delivery, messages
sitting on custodians for days). Full session rotation is compatible with how
libqaul already treats session state.

## Scope

In scope:

- periodic rotation of the Noise KK session between two users
- time-based and volume-based rotation triggers, configurable per node
- grace period during which the old session can still decrypt in-flight
  messages
- good UX for delayed, duplicated, and out-of-order messages across a rotation
  boundary
- concept documentation (this file) and protobuf shape

Out of scope:

- replacing Noise KK with a different handshake pattern
- a per-message double ratchet
- group session rekeying (separate concern)
- key transparency or out-of-band identity verification

## Proposed Design

### Security goals

- **Forward secrecy (PFS):** compromise of current session keys must not
  decrypt messages from previous sessions.
- **Backward secrecy (post-compromise security):** compromise of a session must
  not decrypt messages sent after a successful rotation, assuming the attacker
  does not stay active during the rotation handshake.
- **Delay tolerance:** a legitimately delayed or reordered message must still
  decrypt within the grace window.

PFS comes from each rotation deriving fresh ephemerals (`e`, `re`) and
discarding `cipher_in` / `cipher_out` of the previous session after the grace
period. Backward secrecy comes from the new session not being derivable from
the old transport keys.

### Rotation triggers

Each node enforces rotation when any of the following becomes true for a
session:

- **time:** `now - session.established_at >= rotation_period`
- **volume outbound:** `session.index_nonce_out >= rotation_volume`
- **volume inbound:** `session.highest_index_nonce_in >= rotation_volume`
- **manual:** user or admin requests rekey

Defaults (configurable in `configuration.rs`):

- `rotation_period`: 7 days
- `rotation_volume`: 2^20 messages
- `grace_period`: 1 RTT estimate, clamped to `[1 min, 24 h]`
- `grace_volume`: 256 messages accepted on the old session after rotation
  start

Either side may initiate. The initiator is whichever peer trips a trigger
first; ties are resolved by lower `PeerId`.

### Rotation protocol

Rotation re-runs the existing Noise KK handshake under a **new** `session_id`,
while the old session remains usable for receiving only.

Sequence:

1. Initiator generates a new random `session_id'` and sends
   `RotateHandshakeFirst { session_id': u32, noise_e: bytes, nonce: bytes }`
   encrypted under the current session as a `CryptoService` message.
2. Responder validates, runs KK second step, sends
   `RotateHandshakeSecond { session_id': u32, noise_e: bytes, nonce: bytes,
   signature: bytes }` back under the current session.
3. Both sides mark the old session as **draining** and the new session as
   **primary**. All new outbound messages use `session_id'`.
4. The draining session is kept for `grace_period` or until `grace_volume`
   messages have been accepted on it, whichever comes first.
5. On expiry, the draining session's `cipher_in`, `cipher_out`, and ephemerals
   are zeroized and removed from the sled tree.

The `nonce` carried in both handshake frames is the "nonce based approach"
suggested in the issue: it binds the rotation to a specific challenge so a
replayed old handshake cannot resurrect a retired session.

### Message routing across rotation

Every encrypted payload already carries `session_id`. Receiver logic:

- `session_id == primary.id` → decrypt with primary session, normal path.
- `session_id == draining.id` and draining not expired → decrypt with draining
  session, normal out-of-order handling via
  `out_of_order_indexes`.
- `session_id == draining.id` and draining expired → drop, log as
  `RotationGraceExpired`, surface to UI as "message expired, ask sender to
  resend".
- unknown `session_id` from a peer we have a session with → treat as a new
  rotation attempt; run the first-handshake path.

This keeps the UX good for the realistic failure mode: a message was sent
under session N, crossed a rotation, and arrived during N's grace window.

### Protobuf shape

Add to `crypto_net.proto` without changing existing messages:

```protobuf
message RotateHandshakeFirst {
    uint32 new_session_id = 1;
    bytes noise_e = 2;
    bytes nonce = 3;
    uint64 initiated_at = 4;
}

message RotateHandshakeSecond {
    uint32 new_session_id = 1;
    bytes noise_e = 2;
    bytes nonce = 3;
    bytes signature = 4;
    uint64 received_at = 5;
}

message CryptoserviceContainer {
    oneof message {
        SecondHandshake second_handshake = 1;
        RotateHandshakeFirst rotate_first = 2;
        RotateHandshakeSecond rotate_second = 3;
    }
}
```

### State changes

`CryptoState` stays as-is; we store two `CryptoState` rows per peer during
rotation, discriminated by `session_id`. Add a sibling tree `rotation_meta`
keyed by `remote_id` holding:

- `primary_session_id: u32`
- `draining_session_id: Option<u32>`
- `draining_until: Option<u64>`
- `draining_remaining_volume: Option<u64>`

### Configuration

Extend `configuration.rs` with a `CryptoRotation` section:

```rust
pub struct CryptoRotation {
    pub enabled: bool,
    pub period_seconds: u64,
    pub volume_messages: u64,
    pub grace_period_seconds: u64,
    pub grace_volume_messages: u64,
}
```

Values are per-node, not negotiated. Peers with stricter policies rotate
more aggressively; both sides still converge because rotation is
symmetric once triggered.

## Delivery Plan

### Phase 1: Rotation primitives

Add `rotate()` on `CryptoNoise`, the new protobuf messages, and the
`rotation_meta` tree. No trigger wiring yet; rotation is callable only from
tests.

### Phase 2: Trigger and grace window

Wire time-based and volume-based triggers into the messaging send and receive
paths. Implement grace window expiry on a periodic task. Draining session is
read-only.

### Phase 3: Configuration and UX

Expose `CryptoRotation` config via RPC and UI. Surface rotation events
(`Rotated`, `GraceExpired`, `MessageDroppedPastGrace`) to the client so the UI
can explain "message from peer X could not be decrypted because their session
was rotated".

### Phase 4: Local multi-node validation

Use `tests/integration/local_mesh.py` to cover:

- clean rotation under load
- rotation during offline peer (DTN interaction)
- message arriving after grace expiry
- both peers tripping rotation simultaneously
- rotation across a qauld restart (state must survive)

### Phase 5: Controlled rollout

1. ship rotation code with `enabled = false` by default
2. enable on internal test nodes
3. enable for peers whose capability advertisement includes rotation
4. flip default to `enabled = true`
5. keep a feature flag for emergency disable

## Risks and Controls

- **Split brain on session id:** two simultaneous rotations picking different
  ids. Resolved by lower `PeerId` wins; loser adopts winner's `new_session_id`.
- **Grace too short → lost messages:** configurable, default tuned for qaul's
  DTN latency; UI surfaces the cause.
- **Grace too long → weakened PFS:** hard upper bound of 24 h on
  `grace_period`.
- **Replay of old rotation handshake:** prevented by the `nonce` field and by
  requiring the rotation frame to be authenticated under the current session.
- **State bloat:** at most two sessions per peer retained; expiry task
  zeroizes and removes draining state.

## Rollback

- set `CryptoRotation.enabled = false`
- existing long-lived sessions continue to work unchanged
- no on-disk migration required; the `rotation_meta` tree is additive

## Decision Gate

Proceed only if all of the following are true:

- full-session rotation is accepted as the approach over a per-message ratchet
- grace window behavior is acceptable for the DTN delay profile
- mixed-version peers can coexist without dropping messages
- local multi-node tests show no message loss under clean and interrupted
  rotations

## Conclusion

Full Noise KK session rotation with a bounded grace window gives us PFS,
backward security, and configurable time- and volume-based rotation, while
preserving qaul's tolerance for delayed and out-of-order messages. It reuses
the existing handshake machinery and adds state that is strictly additive to
the current `CryptoState` storage.
