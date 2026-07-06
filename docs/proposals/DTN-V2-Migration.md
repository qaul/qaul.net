# DTN V2 Migration Plan

## Overview

DTN V2 (custody-routed store-and-forward, `DtnRoutedV2`) is functionally
complete and hardened for release. This document proposes how the network
migrates from DTN V1 (single storage-node store-and-forward) to V2, and
what happens to V1 along the way.

The core claims:

- **No stored-data conversion is needed.** V1 entries are repacked per
  storage node (`dtn-messages` / `dtn-messages-ids`) and cannot be
  meaningfully rewritten into custody-routed V2 messages. They drain
  naturally: each entry is either delivered and confirmed, or expires.
- **Interop is solved by a capability bit, not a protocol version.** The
  wire already distinguishes V1/V2 by the `EnvelopPayload` oneof variant.
  An old node receiving `dtn_routed_v2` silently drops the unknown
  variant — the real risk in a mixed-version network is black-holing
  messages through custodians that don't speak V2. Senders must therefore
  only route custody through nodes that advertise support.
- **The rollout is three phases**, each shippable independently, with V1
  fully removable only in the last one.

## Current state (for reference)

| | V1 | V2 |
|---|---|---|
| Wire | `EnvelopPayload.dtn` (bytes) | `EnvelopPayload.dtn_routed_v2` (`DtnRoutedV2`) |
| Storage | `dtn-messages`, `dtn-messages-ids` | `dtn-routed-v2`, `dtn-sender-quotas` |
| Routing | single configured storage node | ordered custody route, forwarded hop by hop |
| Quota | node total only | node total + per-sender (priority-scaled) |
| Expiry | none explicit | `expires_at` + 7-day local retention cap + handoff budget |
| Opt-in | `storage.users` allow-list | `dtn_v2_custody_enabled` (default **false**) |

Prerequisites:

1. **merged** — DTN V2 release hardening (retention cap, quota
   self-heal, blocked-sender rejection, recipient-first delivery,
   V2 state RPC) and the tier-0 delivery fixes.
2. PR #907 `feat/identity-priority` — local priority levels,
   priority-scaled per-sender quota (base configurable via
   `dtn_v2_sender_quota_mb`).
3. `feat/sender-routes` (stacked on #907) — sender-defined custody
   routes with the resolution order: sender route → receiver's
   published route → configured V1 storage node as single-custodian
   route.

## The DTN_V2 capability bit

`User.capabilities` already carries a feature bitset advertised via
`UserInfo` (bit 0 = `ROTATION`). We add:

```rust
/// Accepts and forwards DtnRoutedV2 custody messages.
pub const DTN_V2: u32 = 1 << 1;
```

- Advertised in `Capabilities::LOCAL` by every binary that ships V2.
- `select_custody_target()` and `resolve_custody_route()` skip custodians
  that do not advertise `DTN_V2` (the final recipient is exempt from the
  check when directly reachable — direct delivery doesn't need custody).
- Note the existing limitation: capabilities are relearned from
  `UserInfo` and default to 0 for peers we haven't heard from since
  restart. Skipping a capable-but-not-yet-relearned custodian is safe
  (the message stays queued and retries); routing through an incapable
  one is not. Defaulting to "not capable" is therefore correct.

**Open question for review:** should `dtn_v2_custody_enabled` also gate
the advertised bit (capability = "I *will* take custody") or only code
support (capability = "I *can* parse V2")? Proposal: advertise only when
custody is enabled, so senders don't route through nodes that will
reject with `USER_NOT_ACCEPTED`. This makes the bit dynamic but requires
re-signing/re-advertising the profile when the setting flips.

## Phase A — ship V2 alongside V1 (this release)

Everything already implemented plus the capability bit:

- V2 custody stays **opt-in** (`dtn custody enable`).
- New sends: `dtn send-routed` (explicit or resolved route). The normal
  chat send path still uses V1 fallback when the recipient is offline.
- V1 send/receive fully intact.
- Upgrade step: none required (config fields have serde defaults).

Exit criteria: V2 deployed in the community network, custody enabled on
a few well-connected nodes, delivery confirmed via `dtn state` V2
counters.

## Phase B — V2 becomes the default (release N+1)

- `dtn_v2_custody_enabled` defaults to **true** for new accounts; an
  upgrade step (`utilities/upgrade/`, precedent: `v2_0_0_rc_5`) flips it
  for existing accounts unless explicitly disabled.
- The messaging DTN fallback (recipient offline) switches from
  `send_dtn_message()` (V1) to building a `DtnRoutedV2` with
  `resolve_custody_route()`. The V1 storage node keeps working through
  resolution step ③ — as a single-custodian V2 route — so operators
  don't need to reconfigure anything.
- V1 *sending* is removed; V1 *receiving* stays (nodes still drain
  entries deposited by not-yet-upgraded peers).
- `dtn add/remove/size` remain meaningful: the storage-node list feeds
  resolution step ③, and `size_total` caps both stores.

Exit criteria: V1 trees empty on upgraded nodes (`dtn state` shows
0 V1 messages), no V1 sends observed in the network.

## Phase C — V1 removal (release N+2)

- V1 receive path (`Dtn::net`, `EnvelopPayload.dtn` handling) removed;
  the oneof field number stays **reserved** in the protobuf.
- Startup cleanup: if `dtn-messages` / `dtn-messages-ids` are empty,
  drop the trees; if non-empty (node skipped Phase B), log a warning,
  drop after the entries' natural 1-hour retransmit expiry, or
  immediately — the entries are undeliverable by then anyway
  (**reviewer input wanted**: silent drop vs. warn-and-keep one release).
- `DtnMessageEntry`, `DtnStorageState` (V1 halves) deleted.

## What explicitly does NOT happen

- No conversion of stored V1 entries into V2 messages (impossible to
  reconstruct custody semantics from repacked per-node blobs; the
  1-hour unconfirmed-retransmit window makes them short-lived anyway).
- No protobuf version field — the oneof variant plus the capability bit
  is sufficient and already backward compatible.
- No forced custody: a node can keep `dtn_v2_custody_enabled = false`
  through all phases and still send, receive, and be a final recipient.

## Risks

| Risk | Mitigation |
|---|---|
| Black-holing via non-V2 custodian | capability bit gating (Phase A) |
| Mixed network: V2-only sender, V1-only receiver's storage node | resolution step ③ requires the storage node itself to advertise `DTN_V2`; until it does, sender keeps the plain queued/retransmit path |
| Capability spoofing (node advertises but drops) | same trust model as V1 storage nodes: custody responses are the delivery signal; unconfirmed entries retry other custodians and expire bounded by retention |
| Config flip surprise in Phase B | upgrade step logs the change; `dtn custody disable` remains a one-liner |
