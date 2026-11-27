# Crypto — Session Lifecycle

Node-to-node encryption uses Noise KK (`Noise_KK_X25519_ChaChaPoly_SHA256`).
This covers two features around the session lifecycle. Session **rotation** has
its own document: `Noise-Session-Rotation.md`.

Code: `rust/libqaul/src/services/crypto/`.

## Handshake extras

Noise KK normally lets the initiator send nothing useful until the responder
completes the handshake — a problem under DTN, where the responder may be
offline for hours. Handshake extras let the initiator send **multiple encrypted
chat messages during session creation**, before the handshake completes.

The extra payloads ride under the partial-handshake cipher (which Noise KK
allows) and are queued on disk for DTN delivery. The responder drains them once
it finishes the handshake, with correct ordering and replay protection. Counts
and sizes are bounded so the pre-completion queue can't grow without limit.

This keeps KK intact — no per-message ratchet, no pattern switch — and only
widens what the initiator may put on the wire during message 1's extended
window.

## Cold re-key

Recovers a session after one side loses its crypto state (e.g. a wiped
database) while the peer is still reachable.

When a node receives undecryptable traffic for an unknown session, and it has
**no** existing session with that peer, it re-initiates a fresh KK handshake.
KK is mutually identity-authenticated, so this is safe: the new session is
provably with the real peer. The trigger is rate-limited per peer, and stale
traffic for a peer we *do* still have a session with is dropped rather than
re-keyed — that's the guard against a re-key storm.

When the fresh handshake completes, it supersedes any prior transport session
for that peer: the old row is deleted and the new session becomes primary, so
both sides converge on one live session.

## Rationale

- **Full session rotation over a per-message ratchet** — a Signal-style ratchet
  gives strong forward secrecy but is hostile to delayed/out-of-order delivery,
  which qaul depends on. Rotating the whole KK session preserves per-session
  out-of-order tolerance. (See the rotation doc.)
- **Cold re-key is separate from rotation** — different threat: rotation is
  "keys are fine, refresh them"; cold re-key is "one side's state is gone." They
  share no trigger and shouldn't be conflated.
