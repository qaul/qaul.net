# Cross-Community Messaging Failure

**Status:** open · **Found:** 2026-10-27 · **Updated:** 2026-10-27 (code review and
log forensics, see [Related defects](#related-defects-found-in-the-same-review) and
[Fix plan](#fix-plan)) · **Component:** router_v2 (#880)

A leaf node cannot deliver a message to a user in another community, measured on a
3×10 community/hub topology.

## What the bug is

Under router_v2, an ordinary community member cannot send a message to a user in
another community. The attempt fails on the sender, before anything reaches the
network.

The sender logs two errors, in the same millisecond (`000a`, 09:02:20.919Z):

```
ERROR libqaul::services::crypto::noise > No key found for user PeerId("12D3KooWMpt95434FdGFKomxJjxCSCgvct6qu6HFAu9fKo8YEhyC")
ERROR libqaul::services::group > group notify message sending failed Encryption error occurred
```

The first is the cause. Opening a Noise KK session
(`services/crypto/noise.rs:51`) looks the recipient's public key up with
`Users::get_pub_key` (`router/users.rs:521`), which reads only the local user
directory. A member has no directory entry for a user in another community,
because §2.3 deliberately keeps foreign user entries out of its Local sphere — the
member has never seen that user's profile. `Crypto::encrypt` returns `None`, and
`services/messaging/mod.rs:682` abandons the send with the second error.

**The key is not actually missing.** qaul user IDs are ed25519 `PeerId`s, and
libp2p inlines an ed25519 public key into the `PeerId` itself (the identity
multihash; every `12D3KooW…` ID is one). The sender was handed the recipient's
`PeerId`, so it held the recipient's full public key all along — only the lookup
ignores it. router_v2 already depends on this: `id_from_peer_id`
(`router_v2/identity.rs`) recovers the key from the `PeerId` in order to route.
This is [defect 1](#1-the-crypto-key-lookup-ignores-the-key-inside-the-peerid)
and fix 1a.

Because the send is abandoned at the encryption step, the message never reaches the
routing layer at all. The sender logged **zero** `v2 forwarding` decisions across
the whole run, so §9.2 route resolution was never consulted.

The failure is silent from the user's point of view: the group is created locally,
the invite is accepted by the CLI without error, and nothing is ever delivered.

## Impact and scope

This blocks the community-network use case entirely: separate communities joined
through a community node, which is the deployment shape router_v2's sphere model
exists to serve. Any member-to-member message across a community boundary fails.

What still works, measured in the same runs:

- **Same-community delivery.** 1.1 s end to end, including group create, invite and
  message.
- **Containment.** Members hold 0 of 20 foreign user entries, exactly as §2.3
  requires.
- **The cross-sphere directory.** All three gateways assemble a complete directory —
  21 delegated users each (the hub's 1 plus 10 from each foreign gateway) — in about
  3 minutes.

So the protocol's isolation guarantee holds. It is cross-boundary reachability that
does not.

The failure is on the member, not the gateway. A gateway holds the directory and
resolves foreign users into real user entries, so the keys it needs are present. The
affected senders are ordinary leaf nodes, which is to say almost every node in a real
deployment.

v1 is unaffected, but only because it has no membrane: it floods the full table
through the hub regardless of the hub's LAN being switched off, so every node learns
every user and encryption always has a key.

## How we found it

It surfaced sideways, out of protocol-overhead measurement rather than a bug report.

The original task was measuring router_v2's byte overhead on community topologies.
That produced a harness, now in `tests/integration/communities/` — `gen_communities.py` for the topologies,
`phases.py` (then `setup_communities.py`) to wire the two spheres, `overhead.py` to measure — and a
first result on a 3×10 topology. But every number was a v2 absolute with no v1 arm
beside it, so none of it could answer whether v2 was actually better. That gap is
what redirected the work toward a like-for-like comparison.

Building that comparison exposed the real problem. `crossing.py` was written to test
something both protocols could satisfy: does a message addressed to a user in another
community arrive, and how long does it take? It addresses the recipient through
`group invite --user-id`, because a foreign user never appears in `users list` under
v2 and the usual discovery-by-listing path does not exist.

The first run showed containment holding and the crossing failing. That looked like a
clean result, but it was not trustworthy yet — it took four runs to separate the real
defect from artefacts of how it was being measured:

1. **Run 1** reported the crossing failed. The directory had not assembled yet, so
   the failure could not be attributed.
2. **Run 2** compared against a reversed dial order to test whether the first gateway
   to peer was the victim. The comparison was invalid: one sample was taken 29
   minutes after setup, the other 20 seconds after.
3. **Run 3** added a 180-second settle. Both dial orders then came back completely
   healthy — the directory complete on every gateway — which refuted the dial-order
   theory and showed earlier readings had been of a system still converging.
4. **Run 4** ran the crossing against that settled, complete directory. It still
   failed, at the invite leg. That was the first run where every precondition was
   verified present, which is what makes the result meaningful.

The sender's log then gave the cause directly: a missing-key error, the encryption
error it produces, and no forwarding decisions at all.

## Evidence from the lab

All figures from `communities-3x10` — 3 communities of 10 members, each a degree-≤4
lattice behind its own gateway, all three gateways peered to a hub whose LAN is
switched off. 31 nodes, 42 links.

The decisive run, taken after a 180-second settle with every precondition verified:

| Measurement | Result |
| --- | --- |
| Containment, members only | held — 0 of 20 foreign users |
| Directory, per gateway | 3 of 3 complete — 20 of 20 foreign users each |
| Gateway convergence | v4, 10 manifest entries, 3 node entries, 0 pending intros |
| Hub node space | 3 of 3 gateways present |
| Same-community delivery | 1.1 s |
| Cross-community delivery | failed at the invite leg, at both 60 s and 120 s |

Containment has to be scored on non-gateway members. A gateway is supposed to know
every foreign user, so pooling gateways with members makes a working directory read
as a 100% membrane breach — which is what the harness reported until it was
corrected. Split properly: gateways 20, 20, 20; members 0 across all 27.

On the sender, member `000a`:

```
sender knows 10 user(s); target is NOT among them
v2 forwarding decisions: 0
group notify message sending failed Encryption error occurred
```

The v1 arm, same topology, same commands:

| Measurement | v2 | v1 |
| --- | --- | --- |
| Cross-community delivery | failed | 2.2 s |
| Foreign users known per member | 0 of 20 | 19 of 20 |
| Directory settled | 10.3 s | still changing at 90 s |

v1's success is not a baseline to aim at. It delivers because it has bridged the
communities at layer 3, which is precisely what switching off the hub's LAN was meant
to prevent.

## Steps to reproduce

Needs meshnet-lab and a `qauld` built from the branch, run as root. Reproduced 4
times out of 4.

**One command.** From `tests/integration/communities`:

```sh
sudo -v
sudo QAUL_ROUTING_V2=1 RUST_LOG='libqaul::router_v2=debug,libqaul=info' \
  MESHNET_LAB=$HOME/meshnet-lab ./run.sh -c
```

`run.sh` runs the whole sequence in one invocation, which matters because a sudo
timestamp expiring mid-sequence has already corrupted one measurement. It archives
every node's log to `tests/integration/runs/`. The stage-by-stage version, the
options and the v1 arm are in the suite's
[README](../../tests/integration/communities/README.md#running-it). The 180-second
settle it describes is what invalidated two of the runs above when skipped.

**Expected output from `crossing.py`:**

```
containment   held (0/20 foreign users, 0%)
reachability  FAILED at invite
-> UNREACHABLE: a message addressed to another community never arrived
```

**Confirm the cause** on the sender, which is community 0's lattice corner:

```sh
grep -c 'v2 forwarding' /tmp/qaul-000a/qauld.err      # expect 0
grep 'No key found' /tmp/qaul-000a/qauld.err          # expect the recipient's PeerId
grep -i 'group notify' /tmp/qaul-000a/qauld.err       # expect the encryption error
```

On an archived run, the same lines are in `runs/<run>/000a/qauld.err.gz`; read them
with `zgrep`.

**For the v1 comparison**, run with `-1` and `QAUL_ROUTING_V2` **erased**; the README
explains why erasing it is not optional.

## What has been ruled out

Each of these was suspected during investigation and cleared by measurement. They are
listed because three of them cost a run each, and because the list narrows where the
defect can be.

| Subsystem | Status | Evidence that cleared it |
| --- | --- | --- |
| §2.3 containment | working | Members hold 0 of 20 foreign users; the membrane does what it is specified to do |
| Gateway node-entry propagation | working | Each gateway holds node entries for the hub and both foreign gateways, metric 30 at 2 hops over Internet |
| Manifest propagation across the membrane | working | The hub sends 31 `NODE_MANIFEST` frames over Internet; gateway `0001` received 29, totalling 19,517 B |
| Manifest relay into the community | working | Members receive `NODE_MANIFEST` over LAN; the sender received 5 frames, 2,273 B |
| Delegation assembly | working | `0001` delegates 9 cross-host users plus 1 self, manifest v4, expiring in ~5h35m |
| Cross-sphere directory | working | 3 of 3 gateways complete, 21 delegated users each, 0 pending intros |
| §9.2 route resolution | working | `resolve_forwarding` implements local-identity, direct entry, gateway-authoritative DTN, then nearest-gateway anycast; wired at `services/messaging/mod.rs:52`, 16 test call sites |
| §11.4 management forwarding | not reached here, but broken for this case | Never reached — the send is abandoned before any management message is produced. Code review found it lacks the nearest-gateway fallback, so a profile fetch for a foreign user cannot leave a member either ([defect 2](#2-management-forwarding-has-no-nearest-gateway-fallback)) |
| Topology, harness, group service | working | Same-community invite, accept and delivery complete in 1.1 s using identical code paths |
| Dial order of the gateways | not a factor | Forward and reversed order both fully healthy once settled |

Two of those deserve emphasis, because they were each mistakenly reported as the
cause during investigation:

- **§9.2's nearest-gateway anycast is present and correct.** It was briefly believed
  missing from `state/lookup.rs`. It is not: it lives in `resolve_forwarding`, which
  `next_hop_for_user` feeds as step 1. It is wired into messaging only — the
  management sub-protocol calls `next_hop_for_user` directly and has no fallback
  ([defect 2](#2-management-forwarding-has-no-nearest-gateway-fallback)).
- **The cross-sphere directory does assemble.** Earlier runs that showed it empty or
  partial were sampled before it had converged.

## Open questions

All three are answered by the code review.

**What should trigger a profile fetch for a user whose key the sender lacks?** For
*encryption*, nothing: the key is inside the recipient's `PeerId` (defect 1), so no
fetch is needed to send. A fetch is still wanted for the *display* profile (name,
avatar, the signed extended profile), and the natural trigger is the first message
sent to, or received from, a user absent from the directory. But a trigger alone
would not work today. `request_profile` hands the request to `forward_management`,
which calls `next_hop_for_user` with no nearest-gateway fallback and gets `None` for
a user the member has never heard of; the `request_profile_via_host` fallback needs
the user in a manifest the member holds, which the membrane forbids. The response
leg, from the foreign user's leaf host back to the member, fails the same way. See
[defect 2](#2-management-forwarding-has-no-nearest-gateway-fallback).

**Is the member meant to hold the key at all, or route around it?** The two readings
collapse: the member holds the key by holding the ID. Using it violates nothing.
§3.3 forbids basing security decisions on the 8-byte routing ID because 8 bytes are
forgeable; a full identity-multihash `PeerId` is not a hash of the key, it *is* the
key. And §2.3 is untouched — the member learns no routing state and no foreign entry,
only the identifier its own user typed in. The recipient's host needs the same
fallback to answer the KK handshake (`noise.rs:281`), since it has never heard of the
sender either; the reply then routes by nearest-gateway anycast, which messaging
already has.

**Does `users get` matter here?** It stays a local-store lookup
(`with_resolved_user`, `router/users.rs:1091`). Once defect 2 is fixed it is the
obvious operator-facing face of the display-profile fetch; until then it cannot be.

### The stranded gateway (intermittent, 1 of 3 runs)

In 1 of 3 runs, the hub held node entries for only 2 of 3 gateways. The stranded
gateway then held 0 foreign manifests, because no other gateway could route a reply
to it: 72 `management: no route to [..], dropping (§11.2 best-effort)` on a peer, 94
node-profile fetches timed out with 0 succeeding, and 46 manifests deferred with
`origin key unknown`. It was still broken 29 minutes in, so it was not a transient.
The hub also logged 176 `receive-loop drop: no mapping for incoming idx=N
(space=Node)` and, from that one peer only, `index_dump: dropped stale node mirror
0` — mirror index 0 being `RESERVED_INDEX` (`index.rs:10`).

It is recorded separately because it is **not** the cause of the failure documented
above — that failure reproduces with the hub holding all 3 gateways and every
directory complete.

**The logs are gone.** `communities/run.sh` tears the lab down and `qauld` recreates
`/tmp/qaul-*/qauld.err` on every bring-up; every surviving file starts at
08:54:58Z, the settled forward run, which was healthy. test.qaul.net holds no lab
logs, and nothing was archived. What follows is a reconstruction from code, checked
against what the healthy run's debug logs show about the same mechanisms — which
turns out to be most of it.

**Three deductions from the numbers that were recorded.**

1. *176 drops in ~29 minutes is one per origin cycle* (29 × 6 = 174). What the hub
   kept dropping was the gateway's own 10-second origin entry, its node index 0 —
   not some relayed index.
2. *The `stale node mirror 0` line proves the gateway sent the hub a second
   `INDEX_DUMP`.* A dump is only sent from `on_neighbour_connect`, which runs when
   `add_neighbour_transport` reports a transport it did not have. A gateway's first
   Internet dump goes out while it is still in user form — the form flips at the next
   origin tick — so node index 0 is not even bound and cannot be pruned. A second dump
   means the gateway's Internet connection to the hub closed completely
   (`num_established == 0`, `lib.rs:820`) and came back after it had switched to node
   form. In the healthy run each gateway dialled once and retired no neighbour.
3. *On that reconnect the gateway's dump and its introduction disagree.* A gateway
   offline for at least one relay tick is still in node form — it carries its
   members' cross-host delegations (§3.2) — but `sync_gateway_role` has cleared its
   `is_gateway`. The dump filters node index 0 out over Internet, because
   `should_introduce` reads that flag (`propagation.rs:516-544`); the origin that
   `on_neighbour_connect` sends straight after it carries the index-0 introduction
   unconditionally (`propagation.rs:405-429`).

**The resulting mechanism, step by step:**

| Step | Evidence |
|---|---|
| Reconnect: dump without node index 0, then an origin that introduces it | Code, above |
| Frames to one neighbour arrive out of order | **Observed.** Sender `0004` queued `[dump 23 B, origin 18 B, …, origin+intro 33 B]`; receiver `0003` got 33 → 18 → 23 |
| The late dump prunes the newer binding | **Observed.** 40 `dropped stale user mirror 0` on 25 of 31 nodes, each within ~100 ms of a connect |
| Nothing re-introduces the gateway's own node index 0 afterwards | Code: the own index is marked once, at user→node (`hosted_users.rs:199`); 3-J's fix re-marks *received* manifests only, and neither `sync_gateway_role` nor the manifest bump re-marks |
| The hub drops that gateway's origin every 10 s, for good | Matches 176 ≈ 174 |

The healthy run survived the same prune by luck. Its 40 pruned bindings were all
users' index 0, introduced by `originate_to_all_neighbours` when the account was
created — a path that does not consume the §3.8 mark — so the next relay tick sent the
introduction a second time, about a second later (traced on `0003`: pruned at
01.077, re-introduced at 01.976). Not one user-space drop followed in the next two
hours. A gateway's node index 0 has no second copy.

There is a second route to the same end state that needs no reconnect. Both tick
tasks start together in `init_router_v2`, so origin and relay ticks fire on the same
millisecond (`0001`: relay at 08:55:03.972, origin at 08:55:08.973), and they
interleave. That was **observed** at the user→node switch: the relay task took the
node-index-0 mark the origin task had just set and sent the mapping alone (25 B),
the origin task sent the entry without it (22 B), the hub received them in reverse
and dropped the entry — once, harmlessly, on 6 nodes. Had the gateway's Internet
neighbour registered in the second before that tick, `sync_gateway_role` would not
yet have run, the introduction would have been filtered off the Internet peer while
still delivered to the LAN members (so never requeued), and the gateway would be
stranded with no `stale` line — roughly a 10% chance per gateway per bring-up. That
is not what happened in the logged run, but it is the same defect.

**Still unknown:** what closed the gateway's Internet connection. Candidates are a
ping-failure retirement (`unreachable_neighbour`), the close of a duplicate
connection after a simultaneous dial, and the `ReConnecting` redial path. Finding it
needs the next occurrence's logs, which is why the fix plan starts by archiving them.

## Related defects found in the same review

A full read of the spec against `router_v2` on 2026-10-27, prompted by this
investigation. Evidence is marked **observed** (lab logs), **code** (certain from
reading, not yet reproduced) or **spec** (the implementation conforms; the rule is
wrong). Line references are to `rust/libqaul/src/` unless stated.

#### 1. The crypto key lookup ignores the key inside the `PeerId`

**Observed** — the cause of the failure above. `Users::get_pub_key`
(`router/users.rs:521`) reads only the local directory; both handshake legs use it
(`services/crypto/noise.rs:51`, `:281`). The key it fails to find is inlined in the
`PeerId` it was given.

#### 2. Management forwarding has no nearest-gateway fallback

**Code.** `forward_management` (`router_v2/management/mod.rs:159-172`) calls
`next_hop_for_user` directly. §11.4 says management messages are forwarded "exactly
as a user packet is forwarded", and user packets get §9.2's tier fallback through
`resolve_forwarding` (`router_v2/forwarding.rs:63`). Any management exchange with a
foreign user — profile fetch in either direction — dies on the first leaf. §11.5's
"a node that holds the requested profile cached MAY answer on the subject's behalf"
is also unimplemented: `on_management_received` only answers for local identities.

#### 3. Routing frames to a neighbour are not delivered in order

**Observed** (25 of 31 nodes). §8.4 makes the `INDEX_DUMP` replace rule safe *because*
"LAN and INTERNET … are reliable ordered streams". The implementation does not
provide that: `qaul_info` is a `OneShotHandler` (`libp2p_modules/qaul_info/src/lib.rs:66`)
that opens one substream per frame, up to 8 concurrently (`max_dial_negotiated`
default, libp2p-swarm 0.47.1), and `NotifyHandler::Any` (`:59`) may spread a peer's
frames across connections. A dump that lands after a newer introduction prunes it
(`router_v2/receive/translate.rs:274-288`).

#### 4. A node never re-introduces its own node index 0

**Code.** Marked once at the user→node switch (`router_v2/state/hosted_users.rs:199`).
§3.8 trigger 3 applies to the originator's own `manifest_version`, and the
`is_gateway` flip changes who may receive the mapping, but neither
`sync_gateway_role` (`router_v2/state/origin_manifest.rs:233`) nor
`try_bump_manifest_version` re-marks it. This is 3-J's defect, still open for the
node's own identity.

#### 5. The connect burst applies a different sphere filter than the dump

**Code.** `originate_to_neighbour` (`router_v2/propagation.rs:395-462`) introduces the
node's own index and sends its own entry without `should_introduce` or
`should_propagate`, while the dump it follows filters both. Besides feeding the
stranded-gateway sequence, in user form this sends a user mapping and user entry over
Internet — the §2.3 leak previously measured on the `tick_origin` path, here on a
second path.

#### 6. The origin and relay ticks race each other

**Observed.** Two tokio tasks on phase-locked intervals (`router_v2/init.rs`), both
taking from the same `ReintroductionTracker`, both deciding form and gateway role at
different points of their iteration.

#### 7. Frames from a neighbour we have not registered yet are discarded

**Code, not observed in this run** (0 `index_dump: no mirror` lines on 31 nodes). A
mirror is created only when *our* first ping to the peer succeeds
(`connections/events.rs:168`); a dump or introduction that beats it is dropped
(`router_v2/receive/translate.rs:76`, `:239`), and the sender believes it delivered.
Worth closing because it is cheap and fails silently.

#### 8. Delegation revocation is forgeable

**Spec** (§10.5, §11.7). The revocation signs the same bytes as the delegation entry
(`host_node_multikey || timeout`), and ed25519 is deterministic, so the valid
revocation signature *is* the `entry_signature` published in every manifest. Any
node holding a manifest can revoke any delegation in it. The implementation shows it
plainly: `lib.rs:1285` signs a revocation by calling `issue_self_delegation`, and
`router_v2/management/delegation.rs:161` verifies both with `verify_entry`. §10.5's
"replay protection" claim does not hold.

#### 9. The §6.3 reboot rule accepts any older sequence number

**Spec.** "Forward distance exceeds 100" covers the whole backward half of the
circle, so a stale copy one sequence behind replaces the fresher route
(`router_v2/seq.rs:54`, `router_v2/receive/routing_update.rs:138`), contradicting
§7.2. The round-2 audit rightly called the code conformant; the rule is what is
wrong. Low impact at lab scale — it needs about 20 hops of path-length skew.

#### 10. A user dropped from a manifest stays routable through it

**Code.** `refresh_delegation_trust` only re-adds the origin to users it *still*
trusts (`router_v2/receive/manifest_apply.rs:97`). A user removed, revoked or expired
keeps a live weak reference, because `Node` records are never removed, and §9.2 keeps
routing to a gateway that no longer carries them — the black hole §10.7 exists to
prevent.

#### 11. A completed manifest pull is never marked complete

**Code.** `outstanding_manifest_requests` is cleared only by the 10 s timeout
(`router_v2/state/pull.rs:152`), never by a response, so each origin can be pulled at
most once per 10 s per node.

#### 12. Oversize frames are corrupted rather than split

**Code.** Mapping counts are written `len() as u8`
(`router_v2/codec/messages.rs:75`, `:84`) and relay frames `payload_len: body.len()
as u16` (`router_v2/propagation.rs:205`, `:352`, `:448`). More than 255
introductions in one tick, or a batch over 64 KiB, produces a corrupt frame. Out of
reach at 31 nodes; within reach after a partition merge at the design scale.

#### 13. Smaller conformance items

- §8.6's `MANIFEST_DELTA` add diagram (`user_id, record_version, profile_version,
  timeout, sig`) contradicts its own prose ("a NODE_MANIFEST entry … plus a leading
  `record_version`"). The code follows the prose; the diagram needs fixing.
- `ManifestRequest::decode` rejects reserved `item_flags` bits that §8.7 says to
  ignore.
- `relay_interval`, `origin_interval_time` and `reboot_for_seq_num` are read nowhere.

## Fix plan

Ordered to unblock the headline failure first and to make the lab trustworthy before
anything else is measured on it. Each fix lands as its own commit with a unit test
that fails before it, in the style of the conformance audit. Sizes are rough: S under
half a day, M one to two days, L longer.

**Phase 0 — make the lab keep its evidence (S). Implemented 2026-10-28.**

- `lab/archive_lab.sh` copies every node's log, gzipped (about 5 MB for a 31-node run
  of a day), with the run's results, roles and PeerIds — never `config.yaml`, which
  holds private keys — into `tests/integration/runs/<tag>-<UTC start>/`.
  `runs/` is git-ignored. `communities/run.sh` calls it before its teardown, at
  the end, and from an `EXIT` trap if a step fails. Because the directory is named
  by the run's start time, the end-of-run copy and the next run's pre-teardown copy
  refresh one directory. Without this, the stranded gateway's trigger cannot be
  found.
- `lab/log_counters.py` prints the per-node counters, against the live lab or an
  archive: `dropped stale` and `no mapping for incoming` by space, `no mirror`,
  repeat dumps per peer (reconnects), dials, ping retirements, gateway-role and
  form changes, management no-route, profile timeouts, deferred manifests, missing
  keys. Phases 1–3 are judged on these numbers. The first reading of the healthy
  run also showed 455 `management: no route` drops across all 31 nodes, which
  Phase 1b should account for.

**Phase 1a — send across communities (S). Defect 1.**

- Make `Users::get_pub_key` fall back to the key inlined in the `PeerId` when the
  directory misses: identity multihash only, decoded the way
  `Multikey::try_from_peer_id` already does. That covers both handshake legs.
- Test: a KK handshake to, and from, a `PeerId` absent from the directory.
- Lab: `crossing.py` should reach the message leg. Watch for the next wall —
  the group service or the UI may also want a directory entry for display. This
  phase alone may close the documented failure.

**Phase 2 — index-binding robustness (M). Defects 3–7; fixes the stranded gateway.**

- **2a, ordering (defect 3):** serialise each neighbour's routing frames —
  `OneShotHandlerConfig { max_dial_negotiated: 1, .. }` for `qaul_info`, and pin a
  peer's frames to one connection instead of `NotifyHandler::Any`. Success: `dropped
  stale` falls to zero except after a genuine peer restart. A persistent per-peer
  stream is the long-term form. The same applies to `qaul_management`.
- **2b, own index 0 (defect 4):** re-mark node `RESERVED_INDEX` when `is_gateway`
  flips and on every own `manifest_version` bump. Also carry the self mapping in every
  origin update — 13 bytes per neighbour per 10 s — so a lost reserved binding heals
  within one origin cycle whatever lost it. §3.6 already makes repeated introductions
  idempotent at the receiver, so this needs no wire change.
- **2c, one filter (defect 5):** route `originate_to_neighbour` through
  `should_introduce` and `should_propagate`, and decide the node's own gateway status
  from `has_active_internet_transport()` rather than its `Node` record. Fix the
  `tick_origin` entry leak in the same commit.
- **2d, one task (defect 6):** fold the origin tick into the relay loop, every Nth
  relay tick, in a fixed order: `sync_gateway_role` → `sync_propagation_form` →
  origin → relay. This also gives `relay_interval` and `origin_interval_time` a
  reader.
- **2e, early frames (defect 7):** register a neighbour on its first received
  `qaul_info` frame, deriving the node ID from the `PeerId` exactly as `ping_event`
  does.
- Lab: 10 back-to-back community bring-ups with 0 stranded gateways. A forced
  gateway-to-hub disconnect and reconnect — the reconstructed sequence — after which
  the hub still holds the gateway. The `line-50` run that showed 9–24 index-0 drops
  should show none that persist past 5 s.

**Phase 1b — management across communities (M). Defect 2.**

- Give `forward_management` the same tier fallback as `resolve_forwarding`; share
  the code rather than copy it. A gateway that cannot resolve drops, per §11.2.
- Trigger a display-profile fetch on first contact with a user absent from the
  directory, so names and avatars appear.
- Optionally implement §11.5's answer-on-behalf at gateways from
  `cached_profiles`, which keeps a fetch inside one sphere when the gateway already
  has the profile.
- Tests: a `ProfileRequest` from a member to a foreign user reaches its gateway; the
  response from the foreign leaf host returns to the member.

**Phase 3 — delegation correctness and security (M). Defects 8, 10.**

- **Spec first:** §10.5 signs a domain-separated input, for example
  `"qaul-delegation-revoke-v1" || host_node_multikey || timeout`. Then add a
  `revocation_signing_input()` and use it for both issue and verify. Subscribe and
  manifest entry signatures are unchanged.
- Test: an `entry_signature` replayed as a `revoke_signature` is refused.
- When a manifest commits, remove that origin from every user it no longer trusts.
  Add a timed sweep for delegations whose `timeout` passes with no new manifest.
- Consider collecting `Node` records that have no routing entry and no live manifest.

**Phase 4 — sequence numbers (S, needs a spec change). Defect 9.**

- §6.3: treat `new` as a reboot only when it is *more* than 100 *behind* `old`;
  reject it when it is 1–100 behind.
- The cost is at most one 35 s expiry for a node that reboots onto a seq within 100
  below its old one, about a 0.15% chance.
- Wire `reboot_for_seq_num`. Add a test that a seq one behind is rejected; keep the
  existing large-backward-jump test.

**Phase 5 — hygiene (S). Defects 11–13.**

- Clear the outstanding pull when a manifest or delta commits, or is dropped.
- Cap introductions at 255 per frame and requeue the rest; split relay batches at 60
  KiB.
- Accept and ignore reserved `item_flags`.
- Fix the §8.6 diagram.

**Spec amendments**, best bundled into one change to `Qaul-Routing-Protocol.md`:

- §10.5 domain separation.
- §6.3 corrected rule.
- §8.4 states the ordering requirement on implementations explicitly.
- §3.8 permits the self mapping in every origin update.
- §8.6 diagram.
- §11.4 says management forwarding uses §9.2's tier fallback.

**Order:** 0 → 1a → 2 → 1b → 3 → 4 → 5.

- 1a is the cheapest change that may close the headline failure.
- 2 must come before further lab conclusions, because defects 3, 4 and 6 make
  bring-ups nondeterministic.
- 1b restores profiles across the boundary once routing underneath it is stable.
