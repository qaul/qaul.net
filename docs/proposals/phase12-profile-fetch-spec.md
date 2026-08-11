# Phase 12 — Management sub-protocol (§11.5 profile fetch): design spec

Design detail for the Phase 12 subtasks in `implementation.md`. Written
before any code. Spec references are to
`docs/protocols/Qaul-Routing-Protocol.md`.

## 0. What this unblocks, and what it does not

Unblocks: manifest entry trust (`refresh_delegation_trust` currently
reports `unverifiable=N`), manifests from non-neighbour origins, node-form
propagation (a second hosted user), cross-village routing, and — via §9 —
the user directory that chat and `users list` read.

Does not touch: §11.6 subscribe and §11.7 revoke. Field numbers are
reserved for them; Phase 13 fills them in.

---

## 1. Wire schema

New file `protobuf/proto_definitions/router/router_management.proto`.

```proto
syntax = "proto3";
package qaul.net.router_management;

// §11.3. Every management message is this envelope.
message ManagementMessage {
  uint32 version             = 1;  // sub-protocol version, 1 for now
  bytes  destination         = 2;  // 8-byte ID (§3.3)
  bool   destination_is_node = 3;  // selects the index space (§3.5)
  bytes  source              = 4;  // 8-byte ID of the requester
  bool   source_is_node      = 5;
  uint32 request_id          = 6;  // echoed by the response

  oneof body {
    ProfileRequest  profile_request  = 7;
    ProfileResponse profile_response = 8;
    // 9-12 reserved for Phase 13: delegation_subscribe,
    // delegation_subscribe_ack, delegation_revoke,
    // delegation_revoke_ack (§11.6, §11.7).
  }
  reserved 9, 10, 11, 12;
}

// §11.5
message ProfileRequest  { uint32 cached_version = 1; }

message ProfileResponse { bool found = 1; Profile profile = 2; }

message Profile {
  bytes  multikey        = 1;  // full multikey public key (§3.3)
  uint32 profile_version = 2;
  string name            = 3;  // may be empty
  bytes  self_signature  = 4;  // subject's ed25519 signature over
                               // (multikey || profile_version || name)

  // qaul extension, not in §11.5. Feature-capability bitset, matching
  // v1's UserInfo.capabilities (router_net_info.proto:95). Deliberately
  // OUTSIDE self_signature: §11.5 fixes the signing input at 1||2||3 and
  // Phase 1 has tests pinning those bytes. Absent decodes to 0, which is
  // the conservative default the crypto gate already assumes.
  uint32 capabilities    = 5;
}
```

`self_signature`'s input is already implemented: `Profile::sign_input()` in
`router_v2/identity.rs`. Do not re-derive it — it has tests pinning the
byte layout.

**Build wiring**, two lines in `qaul-proto/build.rs`:

- add `"router/router_management.proto"` to the source list (~line 111)
- add `"qaul.net.router_management.rs"` to the generated-module list (~line 155)

Bindings then land at `qaul_proto::qaul_net_router_management`.

---

## 2. `next_hop_for_node`

`router_v2/state/lookup.rs`, mirroring `next_hop_for_user`:

```rust
pub fn next_hop_for_node(&self, target: [u8; 8])
    -> Option<([u8; 8], ConnectionModule)>
```

Resolve `target` through `node_dict` → `routing_table.get(Space::Node, idx)`
→ `next_hop_node_id(entry.next_hop)`.

**No delegation-gateway fallback and no nearest-gateway default.** Nodes are
routed by their own entries (§11.4 step 2 selects the index space, nothing
more). A miss means "cannot resolve", which §11.4 step 3 turns into a drop.

This differs deliberately from `resolve_forwarding`, whose default route
would send a management message to a gateway that is not its destination.

---

## 3. Module layout

```
rust/libp2p_modules/qaul_management/     # new behaviour, mirrors qaul_info
  src/lib.rs, protocol.rs, types.rs

rust/libqaul/src/router_v2/management/
  mod.rs        ManagementState, send_management_message
  forwarding.rs on_message_received: §11.4 dispatch-or-forward
  profile.rs    the two §11.5 handlers
```

The behaviour shuttles opaque bytes. All decoding and dispatch live in
`router_v2/management/`, matching how `qaul_info` relates to `receive/`.

Protocol identifier carries a `/v2` suffix from the first commit (Phase 15
subtask 3) so v1 and v2 meshes never mix on the wire.

---

## 4. `ManagementState`

Per decision 12, there are no futures and no per-request channels.

```rust
pub struct ManagementState {
    /// One outstanding fetch per (subject, is_node). Deduplicates the
    /// several triggers that can fire for the same subject at once.
    in_flight: RwLock<HashMap<([u8; 8], bool), u64>>,  // value = sent_at ms
    next_request_id: AtomicU32,
}
```

`in_flight` doubles as the timeout record: a sweeper drops entries older
than the request timeout so a lost response cannot pin a subject forever.
Same shape as `outstanding_manifest_requests`; reuse that sweeper's cadence
(the 1-second relay tick).

`request_id` is allocated and echoed for §11.3 conformance and for Phase 13,
but nothing correlates on it — see §7 for why that is safe.

---

## 5. Send path

```rust
pub fn request_profile(&self, subject: [u8; 8], is_node: bool, now: u64)
```

1. If `in_flight` already holds `(subject, is_node)`, return.
2. Resolve `cached_version` from the `User`/`Node` record, or 0 if none.
3. Build the envelope: `destination = subject`,
   `destination_is_node = is_node`, `source = host_mk.to_id()`,
   `source_is_node = true`, fresh `request_id`,
   body `ProfileRequest { cached_version }`.
4. Resolve the first hop — `next_hop_for_node` or `next_hop_for_user` by
   `is_node` — then `peer_of_node`. On a miss, drop and log; do **not**
   record in-flight, so a later trigger can retry once a route exists.
5. Record `in_flight`, send.

Fire-and-forget. The caller does not learn the outcome; it re-runs its own
check when the response lands (§8).

---

## 6. Receive path (§11.4)

`on_message_received(state, from_peer, bytes)`:

1. Decode. Reject unknown `version`.
2. If `destination` matches a local identity — `host_mk.to_id()` when
   `destination_is_node`, else any hosted user id — dispatch on body kind.
3. Otherwise forward: resolve next hop by `destination_is_node`, re-encode
   unchanged, send. **The envelope is not rewritten** — no index
   translation applies, because the IDs are global (§8.5's rationale for
   `origin_node_id` being a full ID applies here too).
4. If unresolvable, drop and log (§11.2 best-effort).

**Loop risk, worth a ruling.** The envelope has no TTL or hop count, and
§11.4 defines no bound. Forwarding is loop-free while the routing table is
consistent, but a transient inconsistency could bounce a message between
two nodes until the table settles. Options: (a) accept it, relying on
routing loop-freedom and the fact that management traffic is rare;
(b) track recently-forwarded `(source, request_id)` for a few seconds and
drop repeats; (c) propose a hop-limit field into the spec. **(b) is cheap
and local; my recommendation.** Flagging because it is a genuine gap in
§11.4, not an implementation oversight.

---

## 7. The §11.5 handlers

### ProfileRequest

1. Is the subject a local identity? If yes, build and sign our own
   `Profile` and answer. `Users::create_signed_profile`
   (`node/user_accounts.rs`) already does this for v1 — reuse it rather
   than writing a second signing path.
2. If `cached_version >= our profile_version`, answer
   `ProfileResponse { found: true }` with the profile anyway (cheap) or
   skip; either is spec-legal. Prefer answering — it costs one small
   message and removes a retry.
3. If the subject is not local: **forward per §11.4**. §11.5 permits
   answering from cache on the subject's behalf, but the first cut should
   not — serving a cached profile risks handing out a stale key, and the
   authoritative answer is one more hop away. Revisit if load justifies it.

The response is addressed to the original `source` with the same
`request_id`, `destination_is_node = source_is_node`.

### ProfileResponse

Both checks below are `SHALL` in §11.5. Order matters: verify before
caching, never the reverse.

1. Decode `multikey`. Reject if malformed.
2. `Multikey::to_id(multikey) == destination-of-the-original-request`.
   This is what makes an unsolicited or misrouted response harmless, and
   it is why `request_id` correlation is not load-bearing: a response
   whose key does not hash to the ID we hold is rejected regardless of
   which request it claims to answer.
3. `self_signature` verifies against `multikey` over `Profile::sign_input()`.
4. Cache onto the `User`/`Node` record: `public_key`, `profile_version`,
   and name.
5. Clear `in_flight`.
6. Re-run `refresh_delegation_trust` for any origin whose manifest holds
   an entry for this subject. Simplest correct approach: re-run it for all
   nodes holding an entry with this `user_id`. Manifest sets are small;
   optimise only if it shows up.

A `found: false` response clears `in_flight` and caches nothing.

---

## 8. Trigger points

Replace three existing TODOs. Each schedules a fetch and returns — none
blocks.

| site | subject | is_node |
|---|---|---|
| `refresh_delegation_trust`, the `unverifiable` branch (`receive/manifest_apply.rs`) | `delegated.user_id` | false |
| `handle_node_manifest` / `handle_manifest_delta`, origin key unknown | `origin_node_id` | true |
| `apply_mapping`, user mapping with fresher `profile_version` (§8.8 step 2) | `mapping.target_id` | false |

Do not hold any lock across `request_profile` — it takes `nodes`/`users`
read locks internally. This module has produced three lock-order bugs
already; keep the fetch call outside every guard.

---

## 9. Directory write

Per decision 13, v2 writes to the existing directory rather than building a
second one.

On a verified `ProfileResponse` for a **user** (not a node), call
`router::users::Users::add` with the multikey-derived `PeerId` and the
name. That populates the table read by `get_pub_key` (Noise handshake, BLE
crypto, messaging, feed), `get_user_id_by_q8id` (chat), `get_name_by_q8id`
(group search) and `get_user_snapshot`.

Consequence: `users list` starts returning remote users on a v2-only mesh,
and `chat send` resolves a direct-chat group without a v1 bootstrap.

**Capabilities ride the Profile as an unsigned extension** (field 5, §1).
Pass the decoded value through to `Users::add` alongside the key and name.

Rationale: v1 already carries capabilities unsigned, on `UserInfo`
(`router_net_info.proto:95`), *not* on the signed `UserProfile`. Carrying
them unsigned here is therefore parity with v1, not a regression, and it
costs one proto field rather than a Phase 15 blocker.

Left at 0, `crypto/mod.rs:476` refuses key rotation — its own comment says
the gate is deliberately conservative. So the failure mode is safe
(messages still encrypt; sessions never rotate), but on a v2-default
release it would silently disable rotation mesh-wide, which is why the
field is worth carrying now.

**Known exposure, inherited from v1:** an unsigned capabilities field is
forgeable by a forwarder, who could zero it to prevent two peers ever
rotating — a downgrade attack on forward secrecy. v1 has the identical
exposure via its flooded, unsigned table. The real fix is capabilities
inside the signed profile, which requires a §11.5 change to the signing
input; raise as a spec proposal, tracked in §6.2, and do not block Phase 12
on it.

---

## 10. Test plan

Unit:

- Envelope round-trips; reserved fields 9-12 decode as unknown, no error.
- `next_hop_for_node` resolves a node entry, and does **not** fall back to
  a gateway when the node is unknown.
- `ProfileResponse` with `hash(multikey) != requested_id` → rejected,
  nothing cached.
- `ProfileResponse` with a bad `self_signature` → rejected, nothing cached.
- Dedup: two triggers for one subject produce one in-flight entry.
- Sweeper: a stale in-flight entry is cleared, allowing a later retry.

Integration (two nodes, in-process):

- A holds an unverifiable manifest entry for B's user → fetch → the entry
  becomes trusted and `User.delegation_gateways` is populated.
- Forwarding: a three-node line where the subject is two hops away; the
  middle node forwards rather than consuming.
- Unresolvable destination is dropped without a loop.
- After a fetch, `Users::get_user_id_by_q8id` resolves the subject, i.e.
  the directory write landed.

---

## 11. Rulings (settled)

1. **Loop bound in §11.4** — dedup on `(source, request_id)` for a few
   seconds and drop repeats. Local, cheap, no spec deviation. The absence
   of a TTL in the envelope remains a genuine §11.4 gap worth raising
   upstream.
2. **Answer-from-cache** — no. A node forwards unless it is the subject.
   Serving a cached profile risks handing out a stale key for a saving of
   one hop; revisit only if load justifies it.
3. **Capabilities** — carried as unsigned `Profile` field 5, implemented in
   this phase (§9). The signed-capabilities fix is a spec proposal, not a
   Phase 12 or Phase 15 blocker.
