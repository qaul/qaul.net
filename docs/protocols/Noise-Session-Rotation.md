# Noise KK Session Rotation

This document describes the Noise KK session rotation protocol used by qaul to provide forward secrecy (PFS), backward (post-compromise) security, and bounded resistance to long-running session compromise — without breaking qaul's tolerance for delayed and out-of-order messages.

The implementation lives in `rust/libqaul/src/services/crypto/`; the network-format messages are defined in `protobuf/proto_definitions/services/crypto/crypto_net.proto`.

## Goals

A node-to-node Noise KK session is normally long-lived in qaul: it is established on first contact and kept until one peer goes away. That gives an attacker who compromises the static keys an open-ended window in which to read traffic. Session rotation closes that window on a configurable schedule:

- **Forward secrecy (PFS).** Compromise of the current session's transport keys must not let an attacker decrypt messages from previous sessions.
- **Backward / post-compromise security.** Compromise of a session must not let an attacker decrypt messages sent after a successful rotation, assuming the attacker is not active during the rotation handshake itself.
- **Configurable rotation period.** Both *time-based* (rotate when the session has been alive longer than N seconds) and *volume-based* (rotate when the session has carried more than M messages) triggers, configurable per node.
- **Delay tolerance.** A legitimately delayed or reordered message sent under the previous session must still decrypt for a bounded grace window after the rotation completes. qaul nodes routinely buffer messages on custodian nodes for hours or days, so any rotation strategy that drops in-flight traffic at the rotation boundary would visibly break direct chat.

## Approach: full session rotation, not a per-message ratchet

A per-message symmetric ratchet (Signal-style) gives strong PFS but is hostile to qaul's delay-tolerant model. qaul's `CryptoState` already tracks `out_of_order_indexes` so that messages arriving out of nonce order — or arriving days later from a custodian — still decrypt against a single transport cipher. A per-message ratchet would either drop those messages or require keeping every intermediate ratchet state indefinitely.

The chosen approach is **full session rotation**: re-run the existing `Noise_KK_X25519_ChaChaPoly_SHA256` handshake on a schedule, keeping the previous session alive for a short, bounded grace period during which it can still *receive* (but not send) messages. This:

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
| `grace_period_seconds` | `3600` | How long the previous session stays decryptable after rotation. |
| `grace_volume_messages` | `256` | How many extra messages the previous session may still receive. |

The grace period is intended to cover one round-trip-time estimate for the worst legitimate delay we want to tolerate. It is clamped in design at `[1 min, 24 h]`; the upper bound is what trades us PFS strength against UX, and the lower bound prevents pathological configurations that would retire the previous session before the responder has had a chance to confirm.

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
    uint32 new_session_id = 1;
    bytes  noise_e        = 2;
    bytes  nonce          = 3;   // echoed from RotateHandshakeFirst
    bytes  signature      = 4;
    uint64 received_at    = 5;
}
```

### Sequence

1. **Initiator** picks a random 32-bit `session_id'`, runs Noise KK step 1 with fresh ephemerals (a brand-new `CryptoState` saved as `HalfOutgoing` under `{remote_id, session_id'}`), and sends `RotateHandshakeFirst` encrypted under the current (still-primary) session.
2. **Responder** validates, runs KK step 2, persists the new `CryptoState` as `Transport`, **moves the previous primary into the draining slot**, and sends `RotateHandshakeSecond` back — also encrypted under the *previous* session, since the initiator has not promoted yet.
3. **Initiator** verifies the echoed nonce, finalises KK step 2, and flips its own meta: new session is now primary, old session is draining.
4. The draining session is kept for `grace_period_seconds` or until `grace_volume_messages` messages have been accepted on it, whichever comes first.
5. On expiry, the draining `CryptoState` row's transport keys (`cipher_in`, `cipher_out`) are zeroized and the row is removed. A `GraceExpired` event is emitted.

The `nonce` field carries the "nonce-based" anti-replay binding suggested in the original concept: a fresh 16-byte random value stamped on each rotation. The responder echoes it in `RotateHandshakeSecond`; the initiator verifies the echo. A replayed `RotateHandshakeFirst` cannot resurrect an already-retired session because the initiator side has no pending state matching the replayed nonce.

### Receiver routing

Every encrypted payload carries `session_id`, so receiver logic is straightforward:

| Incoming `session_id` | Behaviour |
|---|---|
| matches `primary` | decrypt with primary, normal path. |
| matches `draining` (and not yet expired) | decrypt with draining, normal out-of-order handling. Decrement `draining_remaining_volume`. |
| matches `draining` (already expired) | drop, emit `MessageDroppedPastGrace`. |
| matches `last_retired_session_id` | drop, emit `MessageDroppedPastGrace` — UI surfaces "message expired, ask sender to resend." |
| unknown, from a peer with an active session | treat as a fresh first-handshake attempt. |

This keeps the UX clean for the common failure mode: a message sent under session N, crossed a rotation boundary, and arrived during N's grace window or shortly after.

## State

Per-peer rotation state lives in a sled tree `rotation_meta` (one row per `remote_id`), separately from the existing `CryptoState` rows:

```rust
pub struct RotationMeta {
    primary_session_id:           u32,
    pending_initiated_session_id: Option<u32>,
    draining_session_id:          Option<u32>,
    draining_until:               Option<u64>,
    draining_remaining_volume:    Option<u64>,
    last_retired_session_id:      Option<u32>,
    last_retired_at:              Option<u64>,
}
```

During a rotation a peer may have **two `CryptoState` rows** (primary + draining) for a brief window. The `primary_session_id` field on `rotation_meta` disambiguates which row is authoritative for new outbound traffic — important on the responder side, where there is a moment between "we completed step 2" and "the initiator sent its first message under the new session" when both rows are in `Transport` state.

`last_retired_*` is the bookkeeping that lets the decrypt path detect "message arrived after grace" (and emit `MessageDroppedPastGrace`) instead of treating an old-session frame as an unknown new-handshake attempt.

## Capability negotiation

A node will not initiate a rotation with a peer that has not advertised support. `UserInfo.capabilities` (a `uint32` bitset on the routing layer's `UserInfo`) carries this advertisement; bit 0 (`Capabilities::ROTATION = 0x1`) is set by every binary that contains rotation code. A peer running an older binary sends `capabilities = 0` (or omits the field entirely, which decodes to 0), and the gate in `Crypto::perform_rotation` refuses to rotate with them. Without this gate, a `RotateHandshakeFirst` to a legacy peer would be silently dropped, leaving the initiator with a dangling `HalfOutgoing` row.

## Events

The crypto module emits a small in-memory ring buffer (cap 256) of rotation events for client tooling and the UI:

- `Rotated` — a rotation finalised on this node; `primary_session_id` and `draining_session_id` are populated.
- `GraceExpired` — a draining session's grace window elapsed and its keys were zeroized.
- `MessageDroppedPastGrace` — an inbound message arrived under a session id whose grace window had already been retired.

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
- **Grace too short → lost messages** — configurable; default tuned for qaul's DTN latency. The UI surfaces `MessageDroppedPastGrace` so the user is told why a delivery failed.
- **Grace too long → weakened PFS** — design hard upper bound of 24 h on `grace_period_seconds`; operators tune the trade-off downward as their threat model demands.
- **State bloat** — at most two `CryptoState` rows per peer are retained at any time; the drain ticker (60 s interval) zeroizes and removes draining state on expiry.
- **Identity-key compromise (out of scope).** Rotation derives forward secrecy from new ephemerals, not from new static keys. An attacker who has stolen a peer's static identity key can decrypt a fresh rotation by impersonating the peer at handshake time. Identity-key compromise is recovered only by reissuing the user's qaul ID, which is a separate concern.

## Rollback

Rotation is gated by `CryptoRotation::enabled` (defaults to `false`). To disable in the field:

1. Set `enabled = false` via `SetConfigRequest` (or edit `config.yaml`).
2. Existing long-lived sessions continue to work unchanged.
3. No on-disk migration is required: the `rotation_meta` tree is additive and ignored when rotation is off.

In-flight rotations at the moment of disable complete naturally (the daemon is not interrupted mid-handshake); subsequent triggers are simply not fired.

## Proposed evolution: receiver-confirmation grace

> **Status:** design proposal, not yet implemented. The current implementation
> uses the wall-clock + volume grace described above. This section is the
> agreed direction after discussion of the DTN problem; see the open issue
> on `feat/crypto-session-rotation` for the conversation.

### Motivation

The shipped grace model trades two parameters against each other:
`grace_period_seconds` (default 1 h, hard cap 24 h) and
`grace_volume_messages` (default 256). Both are wall-clock or counter
bound. That fits a synchronous transport. qaul is not synchronous — DTN
custody routing can stash a message at a custodian for days or weeks
before delivery. With the current grace, a legitimately delayed message
that crossed a rotation boundary gets dropped past the 24 h cap and the
user sees `MessageDroppedPastGrace` for traffic the protocol *did*
deliver correctly end-to-end. The signal is correct ("the cipher key is
gone") but the situation is not an attack — it is a normal DTN delay.

The right primitive in a delay-tolerant system is **delivery
confirmation**, not wall clock. Drop the wall clock as the *primary*
trigger for retirement; keep it only as a final upper bound that catches
peers who have permanently gone dark.

### Summary of the change

- Both sides exchange their last sent and last received nonces under the
  old session during the rotation handshake.
- Each side retires its own inbound cipher for the old session **when it
  has decrypted every nonce up to the peer's stated max-sent** —
  delivery-bound, not time-bound.
- A configurable hard upper bound (`rotation_max_stall_seconds`,
  default ~30 days, much longer than today's 24 h cap) catches dark
  peers; if it fires, the side gives up and zeroises the old cipher
  anyway, emitting a `RotationStalled` event.
- Outbound cipher for the old session is discarded at the moment of
  switchover, exactly as today. Forward secrecy on the send path is
  unchanged; the new mechanism only governs how long the *receive*
  path keeps the old cipher available.

### Wire changes

The rotation grows from two frames to three. All three remain
encapsulated in a `CryptoserviceContainer` and encrypted under the old
transport session — unchanged from today's stealth and anti-MITM-drop
properties.

```protobuf
message RotateHandshakeFirst {
    uint32 new_session_id        = 1;
    bytes  noise_e               = 2;
    bytes  nonce                 = 3;   // 16-byte anti-replay, as today
    uint64 initiated_at          = 4;
    bytes  rotation_uuid         = 5;   // NEW: 16-byte, identifies this rotation
}

message RotateHandshakeSecond {
    uint32 new_session_id              = 1;
    bytes  noise_e                     = 2;
    bytes  nonce                       = 3;   // echoed from RHF
    bytes  signature                   = 4;
    uint64 received_at                 = 5;
    bytes  rotation_uuid               = 6;   // NEW: echoed from RHF
    uint64 responder_last_sent_nonce   = 7;   // NEW: B's final outbound count under old session
    uint64 responder_last_recv_nonce   = 8;   // NEW: highest A→B nonce B has decrypted
}

message RotateHandshakeFinal {                // NEW message
    bytes  rotation_uuid               = 1;
    uint64 initiator_last_sent_nonce   = 2;   // A's final outbound count under old session
    uint64 initiator_last_recv_nonce   = 3;   // highest B→A nonce A has decrypted
}
```

Per-side accounting:

- **initiator (A)** stops sending under the old session at frame-2-receive
  time. Its `initiator_last_sent_nonce` is the final outbound counter at
  that instant. It transmits `RotateHandshakeFinal` (under the *new*
  session — wrapping cost is tiny and it's the first message on the new
  cipher) and from then onwards retires its own outbound cipher for old.
- **responder (B)** stops sending under the old session at frame-2-send
  time. Its `responder_last_sent_nonce` is final at that instant. It is
  reported back to A inside `RotateHandshakeSecond`.

### Retirement rules

For each direction independently:

- **A→B direction.** B retires its `cipher_in` for the old session once
  it has decrypted every nonce in `(B.local_inbound_max + 1) ..=
  A.initiator_last_sent_nonce`. A's outbound cipher for the old session
  is discarded at frame-2-receive time and not needed again.
- **B→A direction.** Symmetric: A retires its `cipher_in` for the old
  session once it has decrypted every nonce in
  `(A.local_inbound_max + 1) ..= B.responder_last_sent_nonce`. B's
  outbound cipher for the old session is discarded at frame-2-send time.

Either side may retire its old `cipher_in` immediately if the peer's
stated max-sent equals its own max-received at handshake time (no
in-flight gap).

A `rotation_max_stall_seconds` deadline overrides both rules: if the gap
hasn't closed by `now + rotation_max_stall_seconds` measured from
handshake completion, the side zeroises its old `cipher_in` anyway. The
intended default is on the order of 30 days — long enough to absorb a
DTN journey, short enough to bound key material lifetime.

### Replay and retransmits

Frames RHF and RHFinal can be lost (DTN-stored, never arrive, or
custodian retries them). Each frame carries `rotation_uuid` so:

- B caches `(rotation_uuid → RHS bytes)` for the duration of the rotation
  plus a short tail (e.g., one minute). A retransmitted RHF with a
  known UUID triggers a re-send of the cached RHS rather than a fresh
  rotation.
- A caches its own outbound `rotation_uuid` until RHFinal has been
  acknowledged (the first encrypted-under-new-session message from B is
  acknowledgement enough — no explicit ack needed).
- An adversary replaying an RHF observed earlier produces nothing
  useful: B's cache no longer holds the rotation, B starts a fresh one;
  the replayer cannot make the existing peers think a stale rotation
  succeeded because the new session keys are bound to fresh ephemerals.

### Simultaneous rotation

Both sides may trigger at once and emit RHF before seeing each other's.
The existing tie-break stays the same in spirit but moves from
"lower `new_session_id` wins" to **"lower `min(local_peer_id,
remote_peer_id)` wins"** — the session id is random and tells you
nothing about which side issued it first, whereas peer ids are stable
inputs both sides can compute. Loser drops its `HalfOutgoing` row,
processes the winner's RHF normally, and the Phase 4 simultaneous
rotation integration test stays valid with the new framing.

### Concurrent rotations per peer

A rotation in progress, plus a trigger to start another (e.g. volume
trigger refires while waiting on a slow RHS), is **queued**, not
chained. `rotation_meta` may hold at most one `pending_initiated`
slot plus one `draining` slot at a time. Triggers received while the
slot is occupied are dropped (the rotation already in flight will
satisfy them once it lands). This keeps the state machine to two
sessions per peer, matching today's bound.

### State changes

`RotationMeta` evolves:

```rust
pub struct RotationMeta {
    primary_session_id:           u32,
    pending_initiated_session_id: Option<u32>,
    draining_session_id:          Option<u32>,
    // Replaces draining_until + draining_remaining_volume:
    peer_last_sent_under_drain:   Option<u64>,   // peer's stated final outbound count
    my_inbound_max_under_drain:   Option<u64>,   // my own decrypted count under draining
    rotation_uuid:                Option<Vec<u8>>,// current rotation, or last completed
    stall_deadline_ms:            Option<u64>,   // wall-clock fallback for dark-peer case
    last_retired_session_id:      Option<u32>,
    last_retired_at:              Option<u64>,
}
```

`draining_until` and `draining_remaining_volume` are removed; the
delivery-bound check replaces them as the primary retirement signal.

### Configuration changes

| Field | Replaces | Default | Meaning |
|---|---|---|---|
| `rotation_max_stall_seconds` | `grace_period_seconds` (as a cap) | `30 * 24 * 3600` (30 d) | Hard upper bound; retire `cipher_in` for old session even if peer hasn't caught up. |
| `rotation_min_drain_seconds` | (new) | `60` | Minimum draining window even when the nonce gap closes immediately, to avoid rotation churn under bursty traffic. |
| `grace_volume_messages` | (deprecated) | — | Removed; nonce-based accounting replaces volume-based grace. |

`enabled`, `period_seconds`, `volume_messages` stay as today.

### Events

- New `RotationStalled` event when `rotation_max_stall_seconds` fires —
  the dark-peer case. UI should surface as "rotation with peer X did
  not complete; the old session has been retired by timeout and any
  further messages from them will fail to decrypt."
- `MessageDroppedPastGrace` keeps the same shape but its semantics
  become "sender's rotation stalled past the hard upper bound" rather
  than "30-minute grace expired", which is a stronger signal of a
  real problem.

### Capability negotiation

Receiver-confirmation rotation is opt-in via a new capability bit:

```rust
impl Capabilities {
    pub const ROTATION: u32              = 1 << 0;   // current shipped rotation
    pub const ROTATION_RECEIPT_BOUND: u32 = 1 << 1;   // new mechanism
    pub const LOCAL: u32 = Self::ROTATION | Self::ROTATION_RECEIPT_BOUND;
}
```

A peer that does not advertise `ROTATION_RECEIPT_BOUND` falls back to
the wall-clock grace described in the main "Protocol" section above.
Both code paths must coexist during rollout; the gate is checked at
`perform_rotation` start.

### Threat model deltas vs the shipped design

- **MITM dropping rotation frames.** Same as the shipped design: frames
  are encrypted under the old session, so an attacker who drops them
  selectively must drop the surrounding regular traffic too, which is
  itself a visible "peer went silent" signal. The receipt-bound model
  does not change this property.
- **Replay of `RotateHandshakeFirst`.** Mitigated by `rotation_uuid` +
  the existing nonce echo. An old RHF with a UUID B has already
  forgotten triggers a fresh rotation; a stale RHF with a UUID B still
  caches triggers a cached RHS re-send. Either way, no successful
  resurrection of a retired session.
- **Indefinite key retention from a never-completing rotation.** Capped
  by `rotation_max_stall_seconds`. The dark-peer case is bounded, just
  more loosely than today's 24 h.
- **State-loss recovery (peer wiped local key material).** Out of scope
  for this section. Tracked separately. The candidate direction is
  "fall back to a fresh KK session-zero handshake when the receiver
  drops messages under an unknown `session_id`" — implicit, no
  explicit reason token, no metadata leak about whose state was lost.

### Out of scope

- **State-loss recovery** — separate design follow-up. The trigger here
  is "peer lost all keys and is sending under a new identity-signed
  handshake-init"; the question is how the receiver decides whether to
  accept that as a legitimate fresh handshake vs an active impersonation
  attempt. Different threat surface from rotation grace; should not be
  blended.
- **Identity-key compromise** — already out of scope per the existing
  threat-model section.
- **Cross-version downgrade attacks** — assumed prevented by the
  capability bit being read from authenticated `UserInfo`. Will be
  formalised in the implementation PR.

## Related files

- `rust/libqaul/src/services/crypto/mod.rs` — `Crypto::encrypt`, `Crypto::decrypt`, trigger plumbing, RPC handlers.
- `rust/libqaul/src/services/crypto/noise.rs` — `rotate_initiate`, `rotate_complete_responder`, `rotate_finalize_initiator`, `drain_expired_rotations`.
- `rust/libqaul/src/services/crypto/sessionmanager.rs` — `process_rotate_first`, `process_rotate_second`, payload framing.
- `rust/libqaul/src/services/crypto/storage.rs` — `RotationMeta`, `CryptoAccount` sled trees.
- `rust/libqaul/src/services/crypto/events.rs` — rotation event ring buffer.
- `rust/libqaul/src/router/users.rs` — `Capabilities` bitset, advertisement, gate look-up.
- `protobuf/proto_definitions/services/crypto/crypto_net.proto` — wire format.
- `protobuf/proto_definitions/services/crypto/crypto_rpc.proto` — RPC surface.
- `tests/integration/test_crypto_rotation_*.py` — multi-node integration tests.
