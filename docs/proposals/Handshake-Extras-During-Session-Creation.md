# Multiple Encrypted Chat Messages during Session Creation

## Overview

We should let the initiator send **multiple encrypted payloads on the first
handshake leg of Noise KK**, queued as additional pre-completion frames under
the partial handshake cipher, and drained once the responder completes the
handshake.

The current qaul crypto only produces one ciphertext from
`encrypt_noise_kk_handshake_1` in `rust/libqaul/src/services/crypto/noise.rs`
before it needs the responder's ephemeral. Under DTN, the responder may be
offline for hours or days, and the sender is forced to either block or drop
additional chat messages. Noise KK explicitly allows extra pre-handshake
payloads encrypted with the handshake `CipherState`, which is what we want.

A per-message ratchet or a full 0-RTT pattern switch (e.g. KK1, IK) would be
heavier and would reopen handshake semantics we don't need to reopen. This
concept keeps KK intact and only widens what the initiator may put on the
wire during message 1's extended period.

## Scope

In scope:

- initiator sending `N > 1` encrypted chat messages before receiving the
  handshake response
- on-disk queuing of pre-completion messages for DTN delivery
- bounded count and size limits on pre-completion messages
- correct ordering, replay protection, and decryption on the responder side
  once msg 2 is sent
- concept documentation (this file) and protobuf shape

Dependencies:

- The capability advertisement bit and the `Modules::Crypto` RPC envelope
  (used to expose `HandshakeExtras` config) both build on plumbing added by
  the `feat/crypto-session-rotation` branch:
  - `UserInfo.capabilities` (`router_net_info.proto`) and the
    `router::users::Capabilities` API (`ROTATION`, `LOCAL`, `supports`).
  - `services/crypto/crypto_rpc.proto` (`Crypto` RPC oneof, dispatched via
    `Modules::Crypto` in `rust/libqaul/src/rpc/mod.rs`).
- The cleanest path is to land this feature **on top of**
  `feat/crypto-session-rotation`, adding a new bit (e.g.
  `Capabilities::HANDSHAKE_EXTRAS = 0x2`) to the existing `Capabilities`
  bitset and new request/response variants (e.g.
  `GetExtrasConfigRequest`, `SetExtrasConfigRequest`) to the existing
  `Crypto` oneof. The gate logic in `Crypto::perform_rotation`
  (`mod.rs`, post-rebase) is the template for the
  `Capabilities::supports(caps, HANDSHAKE_EXTRAS)` check.
- If this feature must ship before rotation, both pieces of plumbing
  need to be duplicated here and reconciled at merge time.

Out of scope:

- switching handshake pattern away from KK
- 0-RTT resumption across sessions
- per-message double ratchet
- backward secrecy for pre-completion payloads (handled by rotation; see
  `docs/protocols/Noise-Session-Rotation.md`, currently on the
  `feat/crypto-session-rotation` branch)

## Proposed Design

### Integration with instance-based state

libqaul's global statics (`CONFIG`, `USERACCOUNTS`, `USERS`,
`CRYPTOSTORAGE`, …) have been replaced by an instance-based
`crate::QaulState` that owns each subsystem's `*State` struct. New code
introduced by this feature must follow the same pattern:

- Every new function on `Crypto` and `CryptoNoise` takes
  `state: &crate::QaulState` as the first argument, mirroring the
  existing `Crypto::encrypt(state, …)`, `Crypto::decrypt(state, …)`,
  `CryptoNoise::encrypt_noise_kk_handshake_1(state, …)` signatures.
- Storage access uses
  `CryptoStorage::get_db_ref(state, account_id)` rather than reaching
  into a global; the returned `CryptoAccount` already exposes
  per-session sled trees.
- Configuration access goes through `Configuration::get(state)` /
  `Configuration::get_mut(state)` / `Configuration::save(state)`. The
  new `HandshakeExtras` block lives inside the existing
  `Configuration` struct, so adding it requires no new state plumbing
  beyond the field itself.
- The periodic expiry task (sessions past `pre_completion_deadline`,
  orphans past `orphan_ttl`) plugs into the existing
  `Libqaul::run` / `event_loop` ticker pattern in
  `rust/libqaul/src/lib.rs`: a new `Ticker::new(Duration::from_millis(...))`
  alongside `routing_table_ticker` / `messaging_ticker`, a new
  `EventType::HandshakeExtras` variant, and a handler that reads
  every account's `CryptoAccount` via `&*self.state`. Free-standing
  tokio tasks holding global statics are no longer permitted.
- The RPC for the `HandshakeExtras` config follows the
  `Modules::Crypto` dispatch added by the rotation work
  (`rust/libqaul/src/rpc/mod.rs` forwards `state` into
  `Crypto::rpc(state, …)`); each handler receives `state`, calls
  `Configuration::get_mut(state)` / `save(state)`, and emits its
  response via `Rpc::send_message(state, …)`.

### Security properties of pre-completion payloads

In Noise KK the handshake is:

```
-> e, es, ss
<- e, ee, se
```

After message 1 the initiator holds a `CipherState` keyed from a chaining key
mixed with `es` and `ss`. Payloads encrypted under this state have:

- **Confidentiality:** yes — attacker without initiator's or responder's
  static private key cannot decrypt.
- **Sender authentication:** yes — mixed with `ss`, so only the legitimate
  initiator's static key could have produced it.
- **Responder forward secrecy:** partial — responder's ephemeral is not yet
  mixed; compromise of the responder's static key allows decryption of
  pre-completion frames. This is weaker than post-handshake transport
  messages.
- **Replay resistance inside a session:** yes — each frame carries a strictly
  increasing `pre_index` used as the Noise nonce.

We accept the weaker responder-side forward secrecy as the price of DTN UX.
It is bounded: as soon as the responder sends msg 2, all further messages use
full KK transport keys with full PFS. The tradeoff is documented and surfaced
to the UI as "queued while peer offline".

### Sender behavior

When the user sends a chat message and the session is `HalfOutgoing` (msg 1
already produced, msg 2 not yet received):

1. Reuse the existing handshake `CipherState` kept in `CryptoState` instead of
   discarding it after msg 1.
2. Encrypt the new payload as a pre-completion frame with
   `pre_index = state.pre_index_out` and then increment.
3. Wrap it in `HandshakeExtraPayload { session_id, pre_index, ciphertext }`.
4. Hand to the messaging layer for normal DTN queuing.

The handshake `CipherState` and its running nonce must be persisted across
restarts, otherwise the queued messages become undecryptable. We extend
`CryptoState` rather than adding a sibling tree, since the data is strictly
tied to a single session.

### Receiver behavior

On msg 2 receipt the responder completes the handshake as today and
transitions to `Transport`. Any `HandshakeExtraPayload` frames with
`session_id` matching a known `HalfIncoming` or freshly completed session are
decrypted using the handshake `CipherState` snapshot captured at the point
of msg 1 processing, advancing through `pre_index`.

Ordering rules:

- `pre_index` must be strictly increasing per session on the responder side.
- Duplicates (same `pre_index`) are dropped and logged.
- Gaps are tolerated for DTN arrival order; the responder keeps a bitmap of
  seen `pre_index` values up to `max_pre_messages`.
- A `HandshakeExtraPayload` arriving for a `session_id` the responder has
  never seen a msg 1 for is buffered for `orphan_ttl`, then dropped.

### Limits

Defaults (configurable in `configuration.rs`):

- `max_pre_messages`: 64 pre-completion messages per session
- `max_pre_bytes`: 1 MiB aggregate ciphertext per session
- `orphan_ttl`: 24 h for `HandshakeExtraPayload` with no matching msg 1
- `pre_completion_deadline`: 7 d — if msg 2 never arrives within this, the
  session and queued payloads are discarded

Once a limit is hit, new user messages fall back to one of:

- block send with a "waiting for peer" UI state, or
- force a fresh session (new `session_id`, new msg 1) carrying the next
  message, with the previous pre-queue invalidated.

We prefer the second: it self-heals if the responder's key material rotated.

### Rotation and interaction with the existing session lifecycle

Pre-completion frames are tied to one `session_id`. If the initiator decides
to abandon the session (e.g. pre-completion deadline exceeded) it opens a new
session with a new `session_id` and new `e`. Previously queued frames under
the old `session_id` are dropped locally; the DTN layer should treat them as
cancelled.

This interacts cleanly with the rotation design (`docs/protocols/Noise-Session-Rotation.md`,
on the `feat/crypto-session-rotation` branch): rotation only fires on
sessions in `Transport` state. A session stuck in `HalfOutgoing` never
rotates — it is either completed or discarded.

### Protobuf shape

Extend `crypto_net.proto` without altering existing messages:

```protobuf
message HandshakeExtraPayload {
    uint32 session_id = 1;
    uint64 pre_index = 2;
    bytes ciphertext = 3;
    uint64 created_at = 4;
}

message CryptoserviceContainer {
    oneof message {
        SecondHandshake second_handshake = 1;
        HandshakeExtraPayload handshake_extra = 2;
    }
}
```

The first handshake message itself keeps its current framing; only the
extras get a new container variant.

### State changes

Extend `CryptoState` with:

- `pre_cipher_out: Option<Vec<u8>>` — serialized handshake `CipherState` on
  initiator side after msg 1
- `pre_index_out: u64`
- `pre_cipher_in: Option<Vec<u8>>` — serialized handshake `CipherState`
  snapshot on responder side after msg 1 processing
- `pre_index_in_highest: u64`
- `pre_index_in_seen: Vec<u8>` — bitmap, length bounded by `max_pre_messages`
- `pre_bytes_accounted: u64`

All new fields **must** carry `#[serde(default)]`. `CryptoState` is
bincode-serialized into the existing `crypto_state` sled tree;
deserialization of pre-existing rows would fail without per-field
defaults. This is the same on-disk-compatibility pattern used elsewhere
in the crypto module — see how the rotation branch added
`established_at` to `CryptoState` with `#[serde(default)]` so existing
rows decode to `0` and are handled by a "trigger never fires until
re-handshake" guard.

All fields are cleared and zeroized when the session transitions to
`Transport` and the handshake-cipher snapshot is no longer needed, except we
keep them until all queued extras are drained from the messaging layer.
"Drained" is observable per session: `pre_index_out` (initiator) and
`pre_index_in_highest` minus the popcount of `pre_index_in_seen`
(responder) reach zero outstanding when the messaging layer has
acknowledged delivery of every queued frame. Until then the cipher
material stays.

### Configuration

Extend `configuration.rs`:

```rust
pub struct HandshakeExtras {
    pub enabled: bool,
    pub max_pre_messages: u32,
    pub max_pre_bytes: u64,
    pub orphan_ttl_seconds: u64,
    pub pre_completion_deadline_seconds: u64,
}
```

Peers with `enabled = false` continue to produce and accept exactly one
message per handshake leg. Extras frames from such peers are dropped without
error; the sender's queue falls back to "waiting for peer" UX.

## What needs to be implemented

- `encrypt_noise_kk_handshake_extra` and `decrypt_noise_kk_handshake_extra`
  on `CryptoNoise`, both taking `state: &crate::QaulState` as the first
  argument (consistent with the existing `encrypt_noise_kk_handshake_1`
  / `decrypt_noise_kk_handshake_1` signatures), reusing the post-msg-1
  handshake `CipherState`.
- Serialize and persist the handshake `CipherState` (key + running nonce) on
  both initiator and responder inside `CryptoState`.
- New `CryptoState` fields: `pre_cipher_out`, `pre_index_out`,
  `pre_cipher_in`, `pre_index_in_highest`, `pre_index_in_seen` (bitmap),
  `pre_bytes_accounted`. Clear and zeroize on transition to `Transport` once
  extras drain.
- `HandshakeExtraPayload` protobuf message and a new oneof variant in
  `CryptoserviceContainer`.
- Messaging send path: when session is `HalfOutgoing`, route chat messages
  through the extras encoder instead of blocking or opening a new session.
  This replaces the current `HalfOutgoing` early-return at
  `Crypto::encrypt_with_state` (`mod.rs`); `encrypt_with_state` itself
  needs to be lifted to take `state: &crate::QaulState` so it can call
  the extras helpers.
- Messaging receive path: decrypt extras against the responder's handshake
  cipher snapshot; buffer orphans (extras arriving before matching msg 1)
  for `orphan_ttl` and drain when msg 1 arrives.
- Per-session `pre_index` enforcement: strictly increasing on sender,
  duplicate-drop and gap-tolerant bitmap on receiver.
- Limits from `HandshakeHandshakeExtras` config: `max_pre_messages`,
  `max_pre_bytes`, `orphan_ttl`, `pre_completion_deadline`. On limit hit,
  abandon current session and open a new one carrying the next message.
- Periodic task to expire sessions past `pre_completion_deadline` and
  orphans past `orphan_ttl`, zeroizing their material. Implemented as a
  new `Ticker` in `Libqaul::run` (e.g. `Duration::from_millis(60_000)`
  alongside `routing_table_ticker`), a new `EventType::HandshakeExtras`
  variant, and a handler that iterates `UserAccounts::get_all_users(&*self.state)`
  and calls into the extras-expiry routine with the per-account
  `CryptoAccount`. Gated on `HandshakeExtras::enabled`.
- `HandshakeExtras` struct in `configuration.rs` with `enabled` flag default
  `false`, plus RPC/UI exposure. The RPC handlers reuse the
  `Modules::Crypto` envelope from the rotation work (each handler takes
  `state: &crate::QaulState`, mutates via `Configuration::get_mut(state)`,
  persists via `Configuration::save(state)`, replies via
  `Rpc::send_message(state, …)`).
- UI states: `Queued`, `QueuedLimitReached`, `PreCompletionDeadlineExceeded`.
- Capability advertisement bit so senders only emit extras to peers that
  support them; fall back to single-message behavior otherwise. Adds
  `Capabilities::HANDSHAKE_EXTRAS = 0x2` to the bitset introduced by
  the rotation branch (`router::users::Capabilities`), updates
  `Capabilities::LOCAL` to OR it in, and gates the extras send path
  with `Capabilities::supports(caps, HANDSHAKE_EXTRAS)` after a
  `Users::get_capabilities(router, &remote_id)` lookup.
- Integration coverage as new pytest scenarios under
  `tests/integration/`, driven by the existing helpers
  (`tests/integration/lib/node.py` for `qauld-ctl` access,
  `tests/integration/lib/network.py` for meshnet-lab topology control,
  `tests/integration/conftest.py` for the `converged_network`
  fixture). Suggested files:
  `test_handshake_extras_offline_responder.py` (many extras buffered
  while responder is offline, single drain on reconnect),
  `test_handshake_extras_responder_restart.py` (responder restart
  between msg 1 and msg 2, queued extras must still decrypt after
  reload),
  `test_handshake_extras_orphan_first.py` (extras arriving before
  matching msg 1, then msg 1 lands and orphans drain),
  `test_handshake_extras_orphan_expiry.py` (orphans past `orphan_ttl`
  are dropped and zeroised), and
  `test_handshake_extras_limit_reset.py` (`max_pre_messages` /
  `max_pre_bytes` exceeded → fresh session opened, previous queue
  invalidated).

## Risks and Controls

- **Weaker PFS for queued messages:** documented; bounded by
  `pre_completion_deadline`; surfaced to UI so users understand the property
  of a "queued" message.
- **Responder static key compromise leaks queue:** mitigated by short
  `pre_completion_deadline` and by operator rotation of identity keys; same
  threat model as any pre-handshake 0-RTT-style data.
- **Replay of pre-completion frames:** prevented by per-session `pre_index`
  and the responder's seen bitmap.
- **Amplification / spam:** bounded by `max_pre_messages` and
  `max_pre_bytes` per session; orphans capped by `orphan_ttl` and by a
  global cap per sender peer id.
- **State bloat:** pre-cipher state is freed the moment msg 2 completes and
  all extras drain.
- **Mixed-version peers:** extras are a new oneof variant; old peers ignore
  unknown variants and sender falls back to single-message behavior.

## Rollback

- set `HandshakeExtras.enabled = false`
- existing sessions continue to work with single-message handshake semantics
- no on-disk migration required; new `CryptoState` fields are optional and
  default to `None` / `0`

## Decision Gate

Proceed only if all of the following are true:

- partial responder-side forward secrecy for queued frames is acceptable,
  given the bounded deadline
- mixed-version peers can coexist without dropping transport messages
- local multi-node tests show correct ordering and no message loss across
  restarts on both sides
- UI can clearly distinguish "queued" from "delivered" for the user

## Conclusion

Allowing multiple encrypted payloads on the initiator's first handshake leg
gives qaul a concrete UX win under DTN, without changing the Noise KK
pattern or the existing `CryptoState` storage model. The added state is
strictly additive and is discarded as soon as the handshake completes, so
the steady-state behavior of the crypto layer is unchanged.
