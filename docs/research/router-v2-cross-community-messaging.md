# Cross-Community Messaging Failure

**Status:** the headline failure is **fixed by Phase 1a** (2026-10-28, verified on
the lab; not yet committed). The related defects stay open. · **Found:** 2026-10-27 ·
**Updated:** 2026-10-28 (code review, log forensics, Phase 0 and 1a; see
[Related defects](#related-defects-found-in-the-same-review) and
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

**Why does a feed post stop at one LAN hop?** Still open, and outside router_v2.
- It happened on `line-50` in 5 of 6 runs between 2026-11-01 and 2026-11-03, on
  both the 2d and 1b binaries, each time at a different hop. The link at that
  hop was up.
- Neither mDNS nor the feed service is the cause: discovery was complete, and
  no node rejected the post.
- **Candidate:** floodsub sends a peer its subscriptions once, on the first
  connection (`on_connection_established`, libp2p-floodsub 0.47.0), through a
  `OneShotHandler`. qaul does not re-publish LAN-received posts on LAN
  (`lib.rs`, `EventType::Flooding`). So one lost subscription cuts the line at
  that hop.
- **Unconfirmed:** that needs `libp2p_floodsub` and `libp2p_swarm`
  connection-level logs. See the Phase 1b lab notes.
- **Under v2 nothing repairs the gap.** v1 advertised recent feed IDs in its
  routing info, and a neighbour requested the posts it was missing
  (`router/info.rs`, `FeedIdsTable` → `FeedRequester`). With router_v2 on, v1
  routing info is not sent (`lib.rs`, `EventType::RoutingInfo`), so that sync
  is off. A post missed at one hop stays missed until that connection is
  re-established.
- **Scope:** only public feed posts. Direct chat, group chat and files go
  through the messaging path, which router_v2 routes with retransmission. The
  crossing test uses that path.

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
`PeerId` it was given. The recipient's signature check
(`services/messaging/process.rs:306`) has the same lookup, so even a message that
was sent would have been dropped on arrival. **Fixed by Phase 1a.**

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
(`router_v2/receive/translate.rs:274-288`). **Fixed on the lab by Phase 2a**
(2026-10-29, 2 runs); not yet for QUIC under packet loss.

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

#### 14. Inline mappings carry foreign identities across the membrane

**Observed** (2026-10-28, run `communities-3x10-v2-20261028T072027Z`). Over a
Local-sphere transport `should_introduce` sends every mapping
(`router_v2/propagation.rs:62-78`, `(Sphere::Local, _) => true`). Unlike
`should_propagate`, it never asks where the target was learned. A gateway that
learns a node or user over Internet allocates its own index for it, and its next
relay batch introduces that index to its LAN neighbours. The entries stay behind,
but the identities cross. On every gateway's two LAN members:

- node mappings for both foreign gateways and the hub, from which they queue
  manifest pulls that their gateway refuses — 42 `Sealed` refusals in the run;
- the hub's user mapping, from which they create a user record and issue a profile
  fetch that can never route — the only management drops in the run that never
  recover.

It stops one hop in: a member that holds a mapping but no entry never allocates an
index of its own, so it has nothing to introduce onward. No foreign manifest or user
entry crosses — the serve-side seal holds — so this is identifier leakage and wasted
traffic, not broken containment. The hub's user mapping exists in the first place
only because of the origin-path leak (defect 5).

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

**Phase 1a — send across communities (S). Defect 1. Implemented and verified
2026-10-28.**

- `Users::resolve_pub_key` (`router/users.rs`) returns the directory's key, else
  the one inlined in the `PeerId`. The inline key is decoded with
  `Multikey::try_from_peer_id`, so only identity-multihash IDs qualify. The key is
  not stored: holding a user's ID does not make it a known user, so `users list`
  and the containment measurement are unchanged.
- It is used where a cross-community exchange needs it: both handshake legs
  (`noise.rs`) and the recipient's signature check (`messaging/process.rs`), which
  the original plan missed.
- `get_pub_key` itself is unchanged, so the feed and BLE link crypto keep their
  directory-only lookups. Today the feed drops, and does not re-flood, a post from a
  sender it does not know (`services/feed/mod.rs`). A global fallback would have
  changed that silently. That drop may contribute to the feed flakiness, but it
  needs its own measurement.
- Tests: a KK handshake between two fresh nodes whose directories hold neither
  user — it failed before the fix, at the sender, and passes after. Plus the inline
  lookup on an ed25519 and a hashed `PeerId`. The `libqaul` suite passes, 796 of
  796.
- Lab, `communities-3x10`, archived as `runs/communities-3x10-v2-20261028T072027Z`:

  | | Before | After |
  |---|---|---|
  | Cross-community `000a` → `001e` | failed at invite | invite 1.0 s, accept 1.1 s, delivery 1.0 s; 3.1 s total |
  | Containment, members | 0 of 20 foreign users | 0 of 20 |
  | `No key found` | 1 | 0 on every node |
  | Errors on the path | — | none |

  The communities share no links and the hub's LAN is off, so a delivery can only
  have gone member → gateway → hub → gateway → member. Neither endpoint fetched
  the other's profile, so foreign users still have no display name; that is
  Phase 1b.

**Phase 2 — index-binding robustness (M). Defects 3–7; fixes the stranded gateway.**

- **2a, ordering (defect 3). Implemented and measured 2026-10-29.**
  - **The change**, all in `libp2p_modules/qaul_info/src/lib.rs`:
    - Each connection opens one outbound substream at a time
      (`max_dial_negotiated: 1`; since replaced by one frame in flight per peer,
      see the follow-up below).
    - A peer's frames always go to its oldest connection. The choice is made when a
      frame is handed to the swarm, so one that closes in between cannot lose it.
    - v1 shares this protocol and is serialised too, which is harmless.
  - **Measurement.** Frame order was measured directly. For every directed link, the
    first 40 routing frames the neighbour received were compared with the order the
    sender sent them, over the first 210 s of each run. Management messages were
    excluded:

    | | Baseline, 3 runs | 2a, 2 runs |
    |---|---|---|
    | Links with any reordering | 83–84 of 84 | 0 of 84, both runs |
    | Routing frames out of order | 285–384 | 0, both runs |
    | Bindings pruned by a late dump | 40, 52, 48 | 0, 0 |
    | Node-space drops at the form switch | 6, 3, 3 | 0, 0 |
    | User-space drops | 0, 3, 0 | 31, 46 |
    | Crossing | 3.1 s on the 1a baseline | 3.2 s, 3.2 s |
    | Gateways in hub node space | 3 / 3 / 3 | 3 / 3 |

  - **The user-space drops are not a regression.** Each is index 0, exactly one per
    affected link, about 2 s after start, and none follow.
    - *Cause:* `on_neighbour_connect` sends the origin entry before the node's user
      exists, so it carries no mapping. The introduction follows a frame later.
    - *Before 2a:* reordering let the introduction arrive first, and the late dump
      then pruned the binding.
    - *Now:* the entry arrives first and is dropped once, and the binding survives.
    - *Fix:* belongs in 2c — don't send an origin entry that has no binding.
  - **Limits.**
    - Two runs, on loss-free veth links.
    - QUIC with packet loss can still reorder, because a stream's close does not wait
      for the peer; only one persistent stream per peer rules that out.
    - `qaul_management` is unchanged.
  - **Follow-up, 2026-10-30: one failed send blocked a connection for good.**
    - libp2p-swarm 0.47.1's `OneShotHandler` frees a substream slot when a send
      succeeds but not when it fails (`one_shot.rs`, `DialUpgradeError`). With one
      slot, a single timeout, reset or frame over 65,535 bytes stalled every later
      frame on that connection until it closed.
    - *Fix:* the behaviour now keeps one frame in flight per peer. The next frame
      goes once the last is sent or fails, or its connection closes. The handler's
      own limit is raised so it never binds. 6 new unit tests in
      `qaul_info/src/lib.rs`.
    - Ordering and pace are the same as before: one frame at a time per peer.
    - No failed send appeared in a v1 run with `qaul_info=trace`, so this is a
      safeguard, not a fix for an observed failure. v1 crossing passed.
  - **Measurement trap.** `lib.rs` logs management messages as
    `router_v2 SEND … type=0x01` too: the second byte of a management envelope is its
    version. Any count of routing frames from `SEND` lines must subtract the
    matching `management: queued for` sends first.
- **2b, own index 0 (defect 4). Implemented and measured 2026-10-29.**
  - **The change.**
    - Every origin update carries the node's own mapping, marked or not, and only
      once an identity is bound at index 0 (`propagation.rs`, `state/lookup.rs`).
    - Index 0 is re-marked when the gateway role flips and on every own manifest
      bump (`state/origin_manifest.rs`).
    - Repeated introductions are idempotent at the receiver (§3.6), so the wire
      format is unchanged.
  - **Unit tests** (`tests/mapping_sphere.rs`) include the stranded-gateway race
    itself: the form switch runs before `sync_gateway_role`, and index 0 is withheld
    from the Internet peer. Before the fix it was never sent afterwards; now the next
    relay batch and every origin update carry it.
  - **Lab, 2 runs.**
    - All 1260 origin frames in the 60–210 s window — 15 cycles × 84 directed links
      — carried the self mapping. It was 0 on the 2a runs.
    - The cost is 15 B per neighbour per 10 s.
    - No regression against 2a: 0 prunes, 0 reordering, 0 node-space drops, the
      crossing at 3.1–3.2 s, and 3 of 3 gateways.
  - **Forced reconnect, 1 run** (`communities-3x10-v2-20261029T075555Z`): the
    `0000`–`0001` link was cut for about 2 min 37 s.
    - About 19 s in, `0001` lost the connection and its gateway role.
    - Its 9 members then revoked their delegations, as §10.3 requires once the
      target stops qualifying, so `0001` fell back to user form.
    - The link returned at 08:12:33. The role came back at 34.6 and node form at
      35.6. The hub accepted `0001`'s node entry 7 ms later and logged no node-space
      drops from it afterwards.
    - The members re-delegated within 9 s. The directory caught up only at the next
      rate-limited manifest bump (v9), a minute after the link returned.
  - **What the reconnect did not test.** The role came back a second before the
    form switch, so the race 2b closes never happened; the old code would have
    healed too. Hitting it needs a reconnect that lands within the second before an
    origin tick, roughly 1 in 10. Confirming 2b on the lab therefore takes repeated
    reconnects, not one.
- **2c, one filter (defects 5 and 14). Implemented and measured 2026-10-30.**
  - **The change**, in `propagation.rs` unless noted:
    - `originate_to_neighbour` takes its self mapping from `self_introduction`,
      filtered by `should_introduce`, so the connect burst and the dump apply the
      same sphere rules.
    - Over Local, `should_introduce` withholds any target known only through the
      Internet: an entry learned over Internet, or a neighbour whose transports are
      all Internet. This closes the defect 14 leak.
    - Over Internet, the node's own gateway status comes from
      `has_active_internet_transport()` instead of its `Node` record, which lagged by
      up to a second.
    - `origin_allowed` sends an origin entry only once an identity is bound at index
      0, and over Internet only in node form (§2.3). It gates both `tick_origin` and
      `originate_to_neighbour`.
    - `receive/routing_update.rs`: an entry whose sphere changes is re-marked for
      introduction, so a target that becomes Local reaches the Local peers.
  - **Unit tests.**
    - 5 new tests in `tests/mapping_sphere.rs`.
    - 8 existing tests changed, because each asserted behaviour §2.3 forbids or
      depended on an unbound origin.
  - **Lab, 2 runs** (`communities-3x10-v2-20261030T160322Z`, `…T161540Z`):

    | | 2a and 2b, 3 runs | 2c, 2 runs |
    |---|---|---|
    | User-space drops at connect | 31–46 | 0, 0 |
    | `Sealed` refusals | 42 each | 0, 0 |
    | Members' manifest pulls for foreign gateways | 36 each | 0, 0 |
    | Members' manifest pulls for the hub | 6 each | 0, 0 |
    | Members' manifest pulls for their own gateway | 107–114 | 109, 108 |
    | Management drops that never recover | 6, 8, 6 | 0, 0 |
    | All management no-route drops | 486–491 | 479, 492 |
    | Profile timeouts | 64–78 | 55, 59 |
    | Crossing | 3.2 s | 2.3 s, 3.3 s |
    | Directory settled | 10.1–10.2 s | 10.3 s, 10.4 s |
    | Gateways in hub node space | 3 / 3 | 3 / 3 |

  - **Spec notes.**
    - Conforms to §2.3: no user entries over Internet, and the gateway role takes
      effect as soon as the node has Internet.
    - §3.2 does not say what a node hosting no users originates. 2c sends nothing,
      and receivers dropped those entries anyway.
    - §3.6 and §8.4 say a dump lists every index in use. Filtering Local dumps
      follows the existing Internet-dump filter, but contradicts that wording, so the
      spec text should be amended.
- **2d, one task (defect 6). Implemented and measured 2026-11-01.**
  - **The change.**
    - `init.rs` runs one task instead of two.
    - Each pass calls `propagation::tick`, which runs `sync_gateway_role`, then
      `sync_propagation_form`, then the origin update when it is due, then the relay
      batch. Housekeeping and the manifest bump follow, as before.
    - `origin_due` reads `relay_interval` and `origin_interval_time`, which nothing
      read before. The defaults keep a 1 s relay, a 10 s origin and the first origin
      at 10 s.
    - The form is now checked every pass instead of every origin tick.
  - **Unit tests.** 5 new in `tests/propagation.rs`: the origin timing, the order
    within a pass, and a pass with no origin due.
  - **Lab, 2 runs** (`communities-3x10-v2-20261101T070757Z`, `…T072243Z`):
    - Every 2c result held:
      - 0 user drops at connect, 0 `Sealed` refusals, 0 members' pulls for foreign
        gateways, 0 management drops that never recover.
      - Crossing 3.3 s and 2.2 s, contained.
      - Directory settled at 10.1 s, 3 of 3 gateways in the hub's node space.
    - Gateways switch to node form 0.6–0.85 s after dialling the hub, against
      4.5–5.6 s on the 2c runs.
    - Members deferred more manifests for an unknown origin key: 31 and 33, against
      21 on 2c. Every deferral resolves to a trusted manifest. The most likely cause
      is the earlier form switch: the gateway bumps its manifest before members have
      its key.
- **2e, early frames (defect 7):** register a neighbour on its first received
  `qaul_info` frame, deriving the node ID from the `PeerId` exactly as `ping_event`
  does. **Deferred:** the path has had 0 occurrences in every run so far.
- **Exit check, 2026-11-01** (`communities-3x10-v2-20261101T073854Z` to
  `…T075219Z`):
  - **10 back-to-back bring-ups, 60 s settle each.**
    - All 3 gateways were in the hub's node space every time. None stranded.
    - On every node, every run: 0 `stale_user`, `stale_node`, `unknown_user`,
      `unknown_node`, `no_mirror`, `vanished`, `retired` and `no_key`.
    - The hub held 18–22 of 30 delegated users at 60 s; the 180 s runs reach
      30. Origin-key deferrals take up to 35 s to resolve and the manifest bump is
      rate-limited, so 60 s is too short for the full directory, not a fault.
    - 1–2 deferrals per run were still open when the logs ended. Each started
      16–17 s before teardown, within the normal latency. All 27 resolved in the
      run that ran on through the reconnects.
  - **Forced reconnect of each gateway**, on the last lab: its hub link was cut
    for 60 s, then 90 s were left to heal.

    | Gateway | Dump at hub after link-up | Role and node form back | Hub accepts its node entry | Hub after 90 s |
    |---|---|---|---|---|
    | `0001` | 07:54:41.16 | +0.9 s, same pass | +8.9 s | 3 / 3 gateways, 30 / 30 users |
    | `000b` | 07:57:23.11 | +0.9 s, same pass | +6.9 s | 3 / 3, 30 / 30 |
    | `0015` | 07:59:53.03 | +0.9 s, same pass | +6.9 s | 3 / 3, 30 / 30 |

    - The role and the form flipped in the same pass every time. Since 2d, the
      race 2b closes cannot happen at all: each pass syncs the role before the form.
    - The directory was whole again within the 90 s. That fits the 2b reconnect,
      which caught up at the next rate-limited bump, a minute after the link
      returned.
    - The 31 node logs of that lab have 0 `WARN` and 0 `ERROR` lines.
  - **`line-50`, 1 run** (`line-50-v2-20261101T084557Z`). This topology used to
    show 9–24 index-0 drops. Now:
    - 0 `unknown_node`, `unknown_user`, `stale_node` and `stale_user` on all 50
      nodes, so no drops at all, not merely none that last past 5 s.
    - All three tests pass. Delivery end to end took 1.0 s.
    - Discovery took 45.4 s, against 32.0 s and 36.4 s in the 2026-10-17 sweeps.
      The routing entry reached `0000` 37 s after `0031` bound its user, in line
      with those sweeps. The rest is the profile fetch:
      - `0000`'s request reached `0031` in 2.5 s.
      - `0031` dropped its reply, because `0000`'s own routing entry reached it
        1.2 s later.
      - `0000` waited out its 6 s timeout, retried, and that one landed.
      - This is the in-flight drop described under Phase 1b, not a round trip
        longer than the timeout. From one run only.
  - **Phase 2 is closed.**

**Phase 1b — management across communities (M). Defect 2.**

**What the `management: no route` drops are.** First classified on one run
(2026-10-28, `communities-3x10-v2-20261028T072027Z`). Checked on 2026-11-01 over the
12 community runs `communities-3x10-v2-20261101T070757Z` to `…T075219Z` (428–495
drops each) and `line-50-v2-20261101T084557Z` (3,303). Each drop is classified by
destination, by the line logged just before it, and by whether a later attempt went
out. Scripts: scratchpad `noroute.py`, `noroute_multi.py` (not in the repo).

| Share, 12 runs | What | Recovers? |
|---|---|---|
| 61–70% | A fetch for a user *in the dropper's own community*, fired before that user's routing entry exists. Mostly `apply_mapping`, which requests the profile as the mapping arrives, ahead of the entry in the same frame; also version-bump re-fetches. 70–82% of all drops fall in the first 5 s | yes: re-asked as the entry lands, median 0 s, p90 0.8 s |
| 11–17%, 50–80 a run | A request or reply **already in flight**, dropped by a relay or by the responder, because the route toward its destination has not arrived yet | only through the requester's 6 s profile timeout and retry. The two counts track each other: 50–80 drops against 48–72 timeouts a run, and exactly 814 against 814 on `line-50` |
| 14–23% | A gateway or the hub verifying a foreign user. The direct attempt cannot route, because there is no route until the user is trusted and no trust until its key arrives. The carrying-host request goes out in the same millisecond and works | yes, via the host |
| 3 a run | A gateway verifying the hub's user, the same way | yes |
| 0 | A member fetching the hub's user, through the defect 14 leak | gone since 2c |
| 0 | A drop that never recovers, or a member's request for a foreign user | — |

On `line-50`, every drop is in the dropper's own sphere, as expected. 814 of the
3,303 are in flight.

**What this means for the design.**

- **The drop counter cannot see defect 2.** In both crossing runs the recipient
  `001e` logged no line at all about the sender's user, because it never asks for
  the profile. The gateway fallback alone would leave names blank; the
  first-contact fetch (change 3) is what starts the exchange.
- **Every 1b change is checked against the spec before it is built.** Two earlier
  ideas failed that check and were dropped:
  - Holding a message with no route for about 2 s conflicts with §11.2: "no
    retransmission within the sub-protocol, and no delay-tolerant storage. An
    undeliverable management message is dropped."
  - Limiting the fallback to destinations absent from `UsersMap` is narrower
    than §9.2, which applies it to every user a leaf cannot resolve. During
    convergence "absent" does not mean "foreign" anyway: in the `line-50`
    trace, the responder learned the requester 1.2 s after the request arrived.

**Changes, each compared against the spec.**

1. **Nearest-gateway fallback for management messages. Implemented 2026-11-01.**
   - **Spec:** §11.4 step 2 forwards a message "exactly as a user packet is
     forwarded" (§9.2). Under §9.2, a leaf sends any user it cannot resolve
     toward its nearest gateway, and a gateway that cannot resolve it gives up.
     §11.4 step 3 and §11.2 then drop the message. **Required;** the old code
     dropped at the leaf.
   - **Code:** `next_hop_toward_user` in `forwarding.rs` is §9.2 for a remote
     user: its own route, then a delegation gateway, then, on a leaf, the
     nearest gateway. `resolve_forwarding` and `forward_management`
     (`management/mod.rs`) now share it.
   - **Node destinations** keep the plain lookup. §9.2 defines resolution, and
     its fallback, for users only; for a node destination **the spec is
     silent** (see the spec amendments).
   - **Unaffected:**
     - Gateways and the hub. All 852 via-host requests in the 12 runs came
       from them.
     - Node-addressed messages.
     - A Local sphere with no gateway, such as `line-50`.
   - **Changed, at members:**
     - A reply to a requester in another community goes to the gateway.
     - So does the member's own request for a foreign user.
     - So does a relayed message the member cannot route.
     - `request_profile`'s via-host fallback no longer runs once the gateway
       fallback has sent. Members made 0 via-host requests in the 12 runs.
   - **Correction:** I expected the in-flight guard to hold up the re-ask at
     route commit for 6 s. It does not, because `commit_routing_entry` clears
     the guard before re-asking (`a_route_commit_re_asks_for_a_key_still_missing`).
2. **No profile fetch from a mapping before the route exists. Implemented
   2026-11-01.**
   - **Spec:** §8.8, ROUTING_UPDATE step 2: "schedule a profile fetch if fresher
     than cached". INDEX_DUMP step 1: "schedule a fetch if newer". **Permitted:**
     the fetch is still scheduled. It goes out when the entry commits, through
     `commit_routing_entry`, with `sweep_user_profiles` as the backstop.
   - **Code:** `apply_mapping` skips the request only for a user with no key and
     no live route. A fresher version for a user whose key we hold is still
     fetched at once, because the commit path re-asks only for a missing key.
   - **Why now:** with change 1, these early requests would otherwise reach the
     gateway instead of being dropped.
   - **On hold:** the planned `refresh_delegation_trust` half. It would rely
     only on the via-host request, which deviates from §11.5: the request is
     addressed to the host node and carries a `subject` field that §11.5's
     `ProfileRequest` does not have. It waits for the §11.5 amendment.
3. **First-contact profile fetch. Implemented 2026-11-03.**
   - **Spec:**
     - §11.5: a node "fetches a profile when it … needs … a fresher profile
       than it has cached". With nothing cached it asks with version 0.
       **Permitted.**
     - §2.3 divides nodes "by the routing state they hold". A fetched foreign
       profile goes into `cached_profiles` and the app's user list
       (`management/profile.rs`, `connections/events.rs`), never into
       `UsersMap`, so it is no routing state. **Permitted.**
   - **Code:** `Users::resolve_pub_key_on_contact` (`router/users.rs`) replaces
     `resolve_pub_key` at the three first-contact points:
     - `messaging/process.rs`, checking a received message;
     - `noise.rs`, both legs of the handshake.

     On a directory miss it still returns the key inside the PeerId (1a), and
     with router_v2 on it also calls `request_profile`. With change 1 a
     member's fetch for a foreign user goes to its gateway.
   - **Found while checking:** 1a's signature check conflicts with §3.4, which
     says a node "MUST first obtain U's profile and use the public key from it".
     Resolved by the §3.4 amendment below, decided 2026-11-03 over changing the
     code. The code change would make first contact wait on profile fetches
     both ways, and need a hold queue, for no security gain: a full ed25519
     PeerId contains its key verbatim.
   - **Harness:** after a successful crossing, `crossing.py` waits for each end
     to list the other by name (`names` leg, and a line in the verdict).
     Containment is read before the crossing, so it is unaffected. My earlier
     note that it would break was wrong.
   - **Tests:** 2 new in `gateway_fallback`:
     - first contact with a user missing from the directory fetches its
       profile;
     - a user already in the directory is not fetched.

     820 libqaul tests pass.
   - **Lab, 1 run** (`communities-3x10-v2-20261103T073222Z`, re-archived after
     the `archive_lab.sh` fix; the earlier `untagged-20261103T065828Z` mixes in
     19 logs from an old `line-50` lab).
     - Crossing `000a`→`001e`: invite 1.1 s, accept 1.1 s, deliver 1.1 s.
       Both ends listed each other by name 0.1 s after delivery.
     - Containment held: 0 foreign users before the crossing. 3 of 3 gateways,
       30 of 30 users delegated, zero counters 0.
     - **Defect 2 is fixed end to end.** At first contact (07:35:54.0),
       `000a` asked for `001e`'s profile and `001e` asked for `000a`'s. Each
       answered the other's request although it had never heard of the
       requester; the reply went to its gateway (change 1). Both profiles
       arrived 0.7–0.8 s after the request.
     - The 31 community nodes logged 183 `no route` drops and 71 profile
       timeouts, against 179 and 66 on the previous 1b run.
   - **Lab, run 2** (`communities-3x10-v2-reversed-20261103T074423Z`). `-r`
     reverses the order the gateways dial the hub, not the message direction.
     - Results were identical: 1.1 s per leg, names after 0.1 s, containment
       held, 3 of 3 gateways, 30 of 30 users.
     - 183 drops, 68 timeouts, zero counters 0.
     - The return direction is covered in every run anyway: the accept leg
       is a message from `001e` back to `000a`, and each end fetches the
       other's profile.
4. **Gateways answer from their cache.** Optional, not built. **Spec:** §11.5,
   "MAY answer on the subject's behalf". Permitted.
5. **In-flight drops. Dropped, 2026-11-03: it would conflict with §14.**
   - I had claimed the spec sets no profile-fetch timeout. That was wrong.
     §14 sets the management request timeout to 5 seconds: "the window a
     requester waits for a response before the subject may be retried". A
     sooner first retry would break that.
   - The spec's own lever is the value itself, which "should be sized to the
     deployment's diameter". It is already configurable, as
     `management_request_timeout`. 5 s plus the 1 s sweep is the 6 s seen in
     the lab.
   - A fixed lower value would not help everywhere: on `line-50` one round
     trip takes about 5 s. An adaptive timeout would need a §14 amendment;
     not proposed.

**Tests:** 8 new in `tests/management.rs`, module `gateway_fallback`:
- A leaf sends a fetch for a foreign user to its gateway, still addressed to
  the subject.
- A reply to an unknown requester goes to the gateway.
- A relayed message for an unknown user goes to the gateway.
- A gateway drops a user it cannot resolve.
- A node destination gets no fallback.
- A mapping leaves a user with no key and no route to its route commit.
- A mapping still fetches a fresher profile for a user whose key we hold.
- A mapping fetches a user with no key that already has a route.

All 818 libqaul tests pass; fmt and clippy are clean on the touched files.

**Lab, changes 1 and 2, 1 run each** (preliminary):

- **Communities, crossing.** The archive is misnamed `untagged-20261101T084557Z`;
  only nodes `0000`–`001e` are from this run.
  - Reachable in 3.0 s and contained. 3 of 3 gateways in the hub's node space,
    30 of 30 users delegated.
  - The zero counters stayed 0.
  - `management: no route` drops fell to 179, against 474 on the 2d run
    `…T072243Z`:
    - Own-community early fetches: about 375 → 79.
    - Gateways and hub checking foreign users: about 95, unchanged as
      expected.
    - Never recovered: 0.
  - No burst at the gateways: they dropped 23–26 each, against 34–53.
  - Profile timeouts: 66, against 62. The in-flight row is untouched, as
    expected, since change 5 is not built.
- **`line-50`** (`line-50-v2-20261101T102710Z`): discovery took 61.7 s, and the
  feed test failed. Neither traces to 1b. Change 1 has no gateway to act on
  there, and change 2 does not touch links or the feed.
  - The `0026`–`0027` LAN link carried its first frame 32 s after bring-up. It
    was the only late link of the 49. `0031`'s route waited 28 s there; every
    other hop took at most 2.5 s, as on the earlier run.
  - The feed post reached `0000`–`0012` and never `0013`, although that link
    had carried routing frames since bring-up. `0013` logged nothing about the
    post, not even an unknown-sender error, so it was lost below the feed
    service. This is the known feed flakiness.
- **`line-50` rerun** (logs left in `/tmp`, not archived): discovery took 39.2 s,
  which fits the late-link explanation above. The feed test failed again.
  - The post reached `0000`–`0009` and never `000a`.
  - That link was up 2 s after bring-up, on one connection (one INDEX_DUMP
    each way), and no node logged an unknown-sender error.
  - Floodsub forwards only to peers whose subscription it holds, and it
    exchanges subscriptions only with peers that mDNS has discovered
    (`lan.rs:111`, `add_node_to_partial_view`). The discovery events are logged
    at trace level only, so these logs cannot show it.
  - The feed passed on the 2d binary (`…T084557Z`) and failed twice on the 1b
    binary. No 1b code is on that path, but with three runs that is not yet
    proven.
- **`line-50` with mDNS tracing** (`line-50-v2-mdns-20261103T063808Z`):
  discovery took 57.7 s, and the feed failed a third time, stopping at
  `0001`→`0002`.
  - **mDNS is ruled out.** Every node discovered both its neighbours within
    2 s of bring-up, long before the post.
  - **What is left is floodsub itself** (libp2p-floodsub 0.47.0, `layer.rs`).
    - A post that arrives over LAN is not re-published on LAN
      (`lib.rs`, `EventType::Flooding`), so the LAN hops rely entirely on
      floodsub's own forwarding.
    - Floodsub forwards only to a peer whose subscription it holds.
    - It sends its subscriptions once, when the first connection to a peer
      opens (`on_connection_established`), and never again. One lost
      subscription message cuts the line at that hop for the whole run.
    - The handler is libp2p's `OneShotHandler`, the same one whose failed sends
      wedged qaul_info (2a).
  - **Tally:** the feed passed 3 of 3 times before 1b (the 2026-10-17 sweeps
    and `…T084557Z`) and failed 3 of 3 times on the 1b binary, each time at a
    different hop.
- **A/B on the 2d binary** (commit `5c188758` built separately as
  `target/ab2d/qauld`; runs `line-50-2d-20261103T065409Z` and `…T065828Z`):
  the feed **failed both times**, stopping before `001c` and before `0006`.
  **The feed failure is not caused by 1b.** It is a floodsub problem with its
  own item (see Open questions).

  | Binary | Run | Discovery | Feed | `no route` drops | Profile timeouts |
  |---|---|---|---|---|---|
  | 2d | `…20261101T084557Z` | 45.4 s | pass | 3,303 | 814 |
  | 2d | `…20261103T065409Z` | 37.1 s | fail | 3,354 | 860 |
  | 2d | `…20261103T065828Z` | 51.3 s | fail | 3,691 | 944 |
  | 1b | `…20261101T102710Z` | 61.7 s (late link) | fail | 1,327 | 1,027 |
  | 1b | `…20261101T104222Z` | 39.2 s | fail | 729 | 690 |
  | 1b | `…20261103T063808Z` | 57.7 s | fail | 1,335 | 1,006 |

  - On `line-50`, 1b cuts the `no route` drops by about 60%.
  - Discovery times and profile timeouts overlap between the two binaries.

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
- §9.2 or §11.4 says whether a management message addressed to a node gets the
  nearest-gateway fallback. §11.4's claim that a response is "routable exactly
  when the request was" does not cover a requester in node form in another
  sphere.
- §11.5 either adds `subject` to `ProfileRequest`, for a request addressed to a
  carrying host, or that path is removed from the code.
- **§3.4, key from an identity PeerId** (decided 2026-11-03; 1a already relies
  on it). Proposed text, replacing the sentence "A node that needs to verify a
  signature by user U MUST first obtain U's profile and use the public key from
  it.":

  > A node that needs to verify a signature by user U MUST use U's full public
  > key. When U's PeerId embeds that key (an identity multihash), the node MAY
  > take the key from the PeerId; otherwise it MUST first obtain U's profile
  > (Section 11.5) and use the public key from it. The 8-byte ID alone never
  > suffices (Section 3.3).

  Why it is safe: an identity-multihash PeerId is the key itself, and §11.5
  accepts a profile only if `hash(multikey)` equals the ID and its signature
  verifies under that same multikey. So the profile cannot supply a different
  key.

**Order:** 0 → 1a → 2 → 1b → 3 → 4 → 5.

- 1a was the cheapest change that could close the headline failure, and did.
- 2 next: it is the most important reliability work, and every later lab result
  is cleaner without defects 3, 4 and 6.
- 1b after it, once the management-drop analysis has held up over more runs.
  1b shares no code with 2, so it could move earlier; a lab run would then count
  only if `manifests.py` shows every gateway in the hub's node space.
