# Noise KK Session Rotation

This document describes the Noise KK session rotation protocol used by qaul to provide forward secrecy (PFS), backward (post-compromise) security, and bounded resistance to long-running session compromise — without breaking qaul's tolerance for delayed and out-of-order messages.

The implementation lives in `rust/libqaul/src/services/crypto/`; the network-format messages are defined in `protobuf/proto_definitions/services/crypto/crypto_net.proto`.

## Goals

A node-to-node Noise KK session is normally long-lived in qaul: it is established on first contact and kept until one peer goes away. That gives an attacker who compromises the static keys an open-ended window in which to read traffic. Session rotation closes that window on a configurable schedule:

- **Forward secrecy (PFS).** Compromise of the current session's transport keys must not let an attacker decrypt messages from previous sessions.
- **Backward / post-compromise security.** Compromise of a session must not let an attacker decrypt messages sent after a successful rotation, assuming the attacker is not active during the rotation handshake itself.
- **Configurable rotation period.** Both *time-based* (rotate when the session has been alive longer than N seconds) and *volume-based* (rotate when the session has carried more than M messages) triggers, configurable per node.
- **Delay tolerance, bounded by nonce.** A legitimately delayed or reordered message sent under the previous session must still decrypt after the rotation completes — but only up to the sender's declared final nonce on that session. The cut-over ACK fixes the last legitimate old-session nonce; anything above it is refused. This keeps in-flight traffic alive across the rotation boundary without any wall-clock window.

## Approach: full session rotation, not a per-message ratchet

A per-message symmetric ratchet (Signal-style) gives strong PFS but is hostile to qaul's delay-tolerant model. qaul's `CryptoState` already tracks `out_of_order_indexes` so that messages arriving out of nonce order — or arriving days later from a custodian — still decrypt against a single transport cipher. A per-message ratchet would either drop those messages or require keeping every intermediate ratchet state indefinitely.

The chosen approach is **full session rotation**: re-run the existing `Noise_KK_X25519_ChaChaPoly_SHA256` handshake on a schedule, keeping the previous session alive to *receive* (never send) exactly the in-flight tail up to each side's declared final nonce. This:

- reuses the existing handshake machinery,
- requires no change to message routing for in-flight traffic (every encrypted payload already carries a `session_id`),
- preserves the existing out-of-order tolerance within each session,
- and adds storage that is strictly additive to the current `CryptoState` rows.

## Trigger model

A node fires a rotation for a peer when any of the following becomes true:

- **time:** `now − session.established_at ≥ rotation_period`
- **outbound volume:** `session.index_nonce_out ≥ rotation_volume`
- **inbound volume:** `session.highest_index_nonce_in ≥ rotation_volume`
- **manual:** an operator forces a rotation via the `TriggerRotationRequest` RPC (e.g. `qauld-ctl crypto rotate --user-id …`).

Either side may initiate. If both sides trip a trigger close enough in time to start handshakes concurrently, the **lower numeric `new_session_id` wins**: the loser drops its `HalfOutgoing` row and processes the winner's incoming `RotateHandshakeFirst` instead. Ties are not possible in practice (32-bit random session ids), but if they did occur the implementation defensively prefers the local rotation.

Defaults — defined in `libqaul::storage::configuration::CryptoRotation`:

| Field | Default | Meaning |
|---|---|---|
| `enabled` | `false` | Master switch; ships dormant for backward-compatible rollout. |
| `period_seconds` | `7 * 24 * 3600` | Rotate when the session is older than 7 days. |
| `volume_messages` | `2^20` | Rotate after ~1 M messages on the session. |

There are no grace-period settings: the previous session's lifetime is bounded by its own traffic (retired once every nonce up to the declared final has arrived), not by a timer.

Per-node policy is **not** negotiated. Peers with stricter policies rotate more aggressively; both sides converge because rotation is symmetric once triggered (either side's rotation produces a valid new shared session).

## Protocol

Rotation re-runs Noise KK under a fresh `session_id'`, while the old session remains usable for receiving only. Three frames carry the rotation, all encapsulated in a `CryptoserviceContainer` (see `protobuf/proto_definitions/services/crypto/crypto_net.proto`) and encrypted under the existing transport session:

```protobuf
message RotateHandshakeFirst {
    uint32 new_session_id = 1;
    bytes  noise_e        = 2;
    bytes  nonce          = 3;   // 16 random bytes, see "Anti-replay"
    uint64 initiated_at   = 4;
}

message RotateHandshakeSecond {
    uint32 new_session_id  = 1;
    bytes  noise_e         = 2;
    bytes  nonce           = 3;  // echoed from RotateHandshakeFirst
    uint64 final_nonce_out = 4;  // responder's last old-session nonce (B->A)
    uint64 received_at     = 5;
}

message RotateHandshakeFinal {   // cut-over ACK, sent under the old session
    uint32 new_session_id  = 1;
    bytes  nonce           = 2;  // echoed from RotateHandshakeFirst
    uint64 final_nonce_out = 3;  // initiator's last old-session nonce (A->B)
}
```

### Sequence

1. **Initiator** picks a random 32-bit `session_id'`, runs Noise KK step 1 with fresh ephemerals (a brand-new `CryptoState` saved as `HalfOutgoing` under `{remote_id, session_id'}`), and sends `RotateHandshakeFirst` encrypted under the current (still-primary) session.
2. **Responder** validates, runs KK step 2, persists the new `CryptoState` as `Transport`, **moves the previous primary into the draining slot**, and sends `RotateHandshakeSecond` back — also encrypted under the *previous* session, since the initiator has not promoted yet. This frame's `final_nonce_out` is its own nonce: the responder's last message on the old session.
3. **Initiator** verifies the echoed nonce, finalises KK step 2, flips its own meta (new session primary, old session draining), and sends `RotateHandshakeFinal` — the cut-over ACK — under the old session. The ACK's `final_nonce_out` is its own nonce: the initiator's last message on the old session. From here on, both sides use only the new session for sending.
4. Each side drains the old session's inbound direction: it keeps accepting frames with nonces **at or below** the peer's declared final, and **refuses** any frame above it (`MessageDroppedPostDrain`). The declared final is the boundary the spec fixes at ACK time.
5. Once every nonce up to the declared final has been received (and this node's own outbound on the old session is confirmed), the old `CryptoState` row's transport keys are zeroized and the row is removed. A `DrainCompleted` event is emitted. No timer is involved anywhere.

The `nonce` field carries the "nonce-based" anti-replay binding suggested in the original concept: a fresh 16-byte random value stamped on each rotation. The responder echoes it in `RotateHandshakeSecond`; the initiator verifies the echo. A replayed `RotateHandshakeFirst` cannot resurrect an already-retired session because the initiator side has no pending state matching the replayed nonce.

### Receiver routing

Every encrypted payload carries `session_id`, so receiver logic is straightforward:

| Incoming `session_id` | Behaviour |
|---|---|
| matches `primary` | decrypt with primary, normal path. |
| matches `draining`, nonce ≤ declared final (or final not yet known) | decrypt with draining, normal out-of-order handling; record the nonce in the drain bitmap. |
| matches `draining`, nonce > declared final | refuse, emit `MessageDroppedPostDrain`. |
| matches `last_retired_session_id` | drop, emit `MessageDroppedPostDrain` — UI surfaces "message expired, ask sender to resend." |
| unknown, from a peer with an active session | treat as a fresh first-handshake attempt. |

This keeps the UX clean for the common failure mode: a message sent under session N, crossed a rotation boundary, and arrived while N was draining or after it was retired.

## State

Per-peer rotation state lives in a sled tree `rotation_meta` (one row per `remote_id`), separately from the existing `CryptoState` rows:

```rust
pub struct RotationMeta {
    primary_session_id:           u32,
    pending_initiated_session_id: Option<u32>,
    draining_session_id:          Option<u32>,
    draining_recv_target:         Option<u64>,  // peer's declared final nonce
    draining_recv_base:           u64,          // inbound high-water at cut-over
    draining_recv_seen:           Vec<u8>,      // bitmap of (base, target] receipts
    last_retired_session_id:      Option<u32>,
}
```

During a rotation a peer may have **two `CryptoState` rows** (primary + draining) for a brief window. The `primary_session_id` field on `rotation_meta` disambiguates which row is authoritative for new outbound traffic — important on the responder side, where there is a moment between "we completed step 2" and "the initiator sent its first message under the new session" when both rows are in `Transport` state.

`last_retired_session_id` is the bookkeeping that lets the decrypt path detect "message arrived after the old session was retired" (and emit `MessageDroppedPostDrain`) instead of treating an old-session frame as an unknown new-handshake attempt.

## Capability negotiation

A node will not initiate a rotation with a peer that has not advertised support. `UserInfo.capabilities` (a `uint32` bitset on the routing layer's `UserInfo`) carries this advertisement; bit 0 (`Capabilities::ROTATION = 0x1`) is set by every binary that contains rotation code. A peer running an older binary sends `capabilities = 0` (or omits the field entirely, which decodes to 0), and the gate in `Crypto::perform_rotation` refuses to rotate with them. Without this gate, a `RotateHandshakeFirst` to a legacy peer would be silently dropped, leaving the initiator with a dangling `HalfOutgoing` row.

## Events

The crypto module emits a small in-memory ring buffer (cap 256) of rotation events for client tooling and the UI:

- `Rotated` — a rotation finalised on this node; `primary_session_id` and `draining_session_id` are populated.
- `DrainCompleted` — a draining session finished its nonce drain and its keys were zeroized.
- `MessageDroppedPostDrain` — an inbound message arrived on a retired session, or on the draining session with a nonce above the peer's declared final.

Clients query the log via `GetRotationEventsRequest`. The buffer is intentionally not persisted: rotation history is already observable on-wire to an adversary, and an on-disk log would add a forensic artefact with no clear defender benefit.

## Configuration and operator surface

The `CryptoRotation` config is read and written via the `Modules::Crypto` RPC (`crypto_rpc.proto`):

- `GetConfigRequest` / `GetConfigResponse` — read current config.
- `SetConfigRequest` / `SetConfigResponse` — partial update, validated server-side; zero-valued numeric fields are rejected to prevent foot-guns.
- `GetRotationEventsRequest` / `GetRotationEventsResponse` — query the event log with optional `since_ms` lower bound and `limit` cap.
- `TriggerRotationRequest` / `TriggerRotationResponse` — operator-level manual rotate, bypasses time/volume triggers but still respects the capability gate and "rotation already in flight" guard.

Both `qaul-cli` and `qauld-ctl` expose these via a `crypto` subcommand:

```sh
qauld-ctl crypto config                 # show current config
qauld-ctl crypto enable                 # flip master switch on
qauld-ctl crypto set --period-seconds 86400 --volume-messages 100000
qauld-ctl crypto rotate --user-id <peer>
qauld-ctl crypto events --limit 20
```

## Threat model and limits

- **Replay of an old `RotateHandshakeFirst`** — prevented by the per-rotation `nonce` and by the requirement that the rotation frame is authenticated under the *current* session. An attacker who replays an older frame produces a duplicate that the initiator's `pending_initiated_session_id` check rejects.
- **Sender keeps using the old session past its declared final** — refused by construction: the final nonce in the cut-over frames is a hard upper bound, so a compromised or buggy peer cannot stretch the old session's lifetime by continuing to send on it. The UI surfaces `MessageDroppedPostDrain` so the user is told why a delivery failed.
- **Old session lingering → weakened PFS** — bounded by traffic, not time: the old session dies as soon as its declared in-flight tail has arrived, and nothing above the declared final is ever accepted on it.
- **State bloat** — at most two `CryptoState` rows per peer are retained at any time; the drain bitmap only spans the in-flight tail `(base, target]`, so it stays small.
- **Identity-key compromise (out of scope).** Rotation derives forward secrecy from new ephemerals, not from new static keys. An attacker who has stolen a peer's static identity key can decrypt a fresh rotation by impersonating the peer at handshake time. Identity-key compromise is recovered only by reissuing the user's qaul ID, which is a separate concern.

## Rollback

Rotation is gated by `CryptoRotation::enabled` (defaults to `false`). To disable in the field:

1. Set `enabled = false` via `SetConfigRequest` (or edit `config.yaml`).
2. Existing long-lived sessions continue to work unchanged.
3. No on-disk migration is required: the `rotation_meta` tree is additive and ignored when rotation is off.

In-flight rotations at the moment of disable complete naturally (the daemon is not interrupted mid-handshake); subsequent triggers are simply not fired.

## Related files

- `rust/libqaul/src/services/crypto/mod.rs` — `Crypto::encrypt`, `Crypto::decrypt`, trigger plumbing, RPC handlers.
- `rust/libqaul/src/services/crypto/noise.rs` — `rotate_initiate`, `rotate_complete_responder`, `rotate_finalize_initiator`, `record_drain_received`, `retire_drain`.
- `rust/libqaul/src/services/crypto/sessionmanager.rs` — `process_rotate_first`, `process_rotate_second`, payload framing.
- `rust/libqaul/src/services/crypto/storage.rs` — `RotationMeta`, `CryptoAccount` sled trees.
- `rust/libqaul/src/services/crypto/events.rs` — rotation event ring buffer.
- `rust/libqaul/src/router/users.rs` — `Capabilities` bitset, advertisement, gate look-up.
- `protobuf/proto_definitions/services/crypto/crypto_net.proto` — wire format.
- `protobuf/proto_definitions/services/crypto/crypto_rpc.proto` — RPC surface.
- `tests/integration/test_crypto_rotation_*.py` — multi-node integration tests.
