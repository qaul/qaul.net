# Router v2 — Session Handoff

**Branch:** `#feat/v2-router/#880`
**Audit file:** `docs/proposals/implementation.md` (gitignored, local only)
**Cutover plan:** `docs/proposals/v1-to-v2-cutover.md`
**Spec:** `docs/protocols/Qaul-Routing-Protocol.md`
**State at handoff:** 749 libqaul lib tests passing. One uncommitted file
(`qauld-ctl/src/commands/router.rs`) plus two untracked docs.

---

## 1. Standing instructions (carry these forward)

- **Guide, don't implement, by default.** Point at the code to read, ask
  questions, give directions in English. Write code only when explicitly asked.
- **Follow the spec to the dot.** Quote the clause; don't paraphrase from memory.
- The user knows Rust well — don't over-explain syntax.
- **Verify before asserting a defect, and settle a question rather than
  re-stating it.** Several claims across both sessions had to be retracted (§6).

---

## 2. Findings closed

Each landed with tests, and each was A/B-verified by stubbing the fix out and
confirming the new tests fail — except 3-K, noted below.

### Round 1 (previous session)

| ID | What was wrong | Fix |
|----|----|----|
| 2-A | `is_gateway` never set at runtime; four mechanisms dead | `sync_gateway_role()` on the relay tick + `dirty_manifest_flags` |
| 2-B | Sphere filter applied to entries but not index mappings | `should_introduce()` |
| 2-C | Hop counter not sphere-local | `compute_outgoing_hop_count()`; spec §2.2/§5.3 amended |
| 2-D | `Sphere::of` used a catch-all arm | Exhaustive match |
| 3-A | A node accepted routing entries for its own identities | Reject in `evaluate_entry` before `translate_incoming` |
| 3-B + 3-C | `profile_version` never reached routing state | `register_hosted_profile` writes through; `mark_profile_version_bump()` |
| 3-D + 3-E | `INDEX_DUMP` had no chunk framing; oversized dumps dropped | `chunk_index`/`chunk_count`; `split_index_dump()` |
| 3-F | `idx_cooldown` config ignored | Allocator on epoch-ms, reads config |
| 3-H | §3.2's third propagation-form trigger unimplemented | `holds_foreign_delegation()` |
| 4-A | BLE not wired into router_v2 | Three ingress sites branch to v2 |
| 4-C | §4.1 `Local` transport unimplemented | Step 0 in `resolve_forwarding` |

### Round 2 (this session)

| ID | What was wrong | Fix |
|----|----|----|
| 11-B | Management replies unroutable beyond one hop; profile fetch dead past a neighbour | §11.4 "Source addressing" + `propagated_identity()` |
| — | Dedup collision introduced by 11-B's fix: a node's own request and a reply it sourced as the subject shared `(source, request_id)` | Key widened to `(source, destination, request_id, is_response)`; `next_request_id` seeded randomly |
| 2-G | `tick_origin` did not sphere-filter its inline mappings | Filter moved inside the per-peer loop + requeue |
| 3-J | §3.8 trigger 3 had no node-space caller, so a node becoming a gateway was never re-introduced across the membrane | `mark_manifest_version_bump()` from both manifest commit paths |
| 3-K | `on_neighbour_connect` took a recursive read lock on `nodes` | Snapshot, filter, then read versions — one lock at a time |
| 11-A | A user known only through a manifest could never resolve its profile, so its entry was never trusted and never reachable | `ProfileRequest.subject` + fallback addressing the carrying host |
| 10-D | Our own `Node` record's `manifest_version` was never updated, so our index mappings under-advertised | `try_bump_manifest_version` writes it through |
| 11-C | Management requests borrowed the §10.8 manifest-pull timeout | New `management_request_timeout` (5 s), added to §14 |
| 4-G | LAN/Internet neighbour loss was only detected when TCP gave up — 943–953 s measured, against §7.5's 35 s route-expiry budget | Explicit ping timeout + consecutive-failure count; retire by closing the connection |
| 4-H | **Node identity was regenerated on every start.** `QaulState::replace_node` (`lib.rs:128`) had zero callers, so the throwaway bootstrap keypair from `lib.rs:286` was never replaced by the one `NodeModule::new` loads from config. Every `Node::get_id`/`get_keys` returned it | Install the real identity right after `NodeModule::new` |
| 4-I | **A new user account was created on every start**, so the hosted user identity changed each time. `QaulState.user_accounts` is built empty at `lib.rs:292`; `UserAccounts::create_from_config` is called only inside `NodeModule::new` and never installed, so `UserAccounts::len(state)` always read 0 and `qauld/main.rs:164` minted a fresh "Community Node" | Install the config-loaded accounts into `QaulState` after `NodeModule::new` (same shape as 4-H) |
| 11-D | **A profile fetch that failed once was never re-issued**, so a delegated user stayed stored-but-never-trusted permanently. `refresh_delegation_trust` re-requests missing keys, but only runs when a manifest or profile *arrives*; the manifest never changes, so §10.8's pull never retriggers | `sweep_delegation_trust` on the relay tick re-drives it for origins holding an unverifiable, unexpired delegation |

**3-K is the one exception to A/B verification.** Its failure is a race that
presents as a hang, so a regression test would be flaky or would hang CI rather
than fail it. Behaviour is verified unchanged (the `mapping_sphere` suite passes
untouched); the lock shape is verified by inspection across all five
`should_introduce` call sites.

### Also delivered

- **v1 → v2 client cutover** (`v1-to-v2-cutover.md` §5.1, §5.2, §5.3a). Six
  commands that read v1 state now work under v2, and so does the Flutter UI,
  which reads the same RPC. One v2-owned file, one-way dependency, so removing
  v1 later stays a deletion rather than an untangling.
- **v2-native inspector**: `router v2 status | table | neighbours | manifests |
  delegations`. Built against a topology small enough to check by hand.
- **`qaul-proto/build.rs`** now emits `cargo:rerun-if-changed` for the proto
  directory. Without it, editing a `.proto` left the generated Rust stale until
  the crate rebuilt for an unrelated reason, surfacing as "type not found".

---

## 3. What the lab runs established

All runs LAN-only, v2 forced on, meshnet-lab.

### line-4

First multi-hop run. Routing converged but `users list` showed only direct
neighbours — that was 11-B. After its fix: 4/4 users on every node, management
drops from 2123/1064/1064/2124 to zero.

### line-8 (diameter 7)

| Check | Result |
|---|---|
| Users known, all nodes | 8/8 |
| Chat 0000 → 0007 | works, ~139 ms over 7 hops |
| Chat 0007 → 0000 | works, ~103 ms |
| Delivery receipt | `✓✓` returns across the full diameter |
| Relay from a middle node (0004 → 0000) | works |
| Message content | intact, no truncation |

### circle-8 (diameter 4)

Metrics from any node: **10, 10, 20, 20, 30, 30, 40** — the exact distance
multiset for a circle of 8. Shortest-path selection chooses correctly between
two directions, which a line cannot test. Chat across the antipode delivered
exactly once. `entry for our own identity` (3-A's rejection) fired twice during
convergence — its first live exercise; previously only reasoned about.

`already forwarded` was 0. Management messages are unicast, so they do not
duplicate on a cyclic topology; loop suppression is a convergence-transient
safety net and stays untested until a link-churn run.

### grid4 3×3 (9 nodes)

Metrics from a corner: **10, 10, 20, 20, 20, 30, 30, 40** — exact Manhattan
distance × 10. Corners 2 neighbours, edges 3, centre 4.

**Multi-user host.** A second account on the centre node exercised the manifest
machinery for the first time without needing a gateway:

- §3.2 form transition: the node moved to node entry with a 2-entry manifest.
- **First node-space routing entry in any run**; that column had always read 0.
- Manifest sync end to end: advertised → pulled → verified → committed,
  `signed yes`, `sphere local`.
- The host's original user lost its user entry, as §3.2 requires, and became
  reachable only through the manifest — §9.2 step 3, also a first.
- **11-A reproduced**: `trusted 1 / delegated 2`, new user absent from
  `users online`. After the fix: `trusted 2 / 2`, present.

### Convergence timings (grid, 9 nodes)

Measured with `tests/integration/convergence.sh`.

| | before 11-C | after 11-C |
|---|---|---|
| neighbour | 3 s | 1–2 s |
| routing | 11–13 s | 10–13 s or 20–22 s |
| users | 13 s ×3, **31–33 s ×6** | 13–23 s |
| **full mesh** | **33 s** | **22–23 s** |

11-C removed the 31–33 s outlier class. Three runs after the fix.

### Churn (grid 3×3, cutting link 0000–0001)

Baseline confirmed before the cut: `0000` 2 neighbours / `10 10 20 20 20 30 30 40`,
`0001` 3 / `10 10 10 20 20 20 30 30`, `0008` 2 / `10 10 20 20 20 30 30 40`.

**Routing handled the cut correctly.** `0000` re-routed all eight entries onto its
surviving neighbour and settled on `10 20 20 30 30 30 40 40` — the exact shortest-path
multiset for the grid minus that edge — and held it, entries refreshing at 2–5 s age.
Recovery is driven by §7.5 sequence-number freshness, not by neighbour removal: the
neighbour was still listed the whole time.

**Neighbour loss itself was not detected for ~950 s**, measured on both endpoints:

| event | t |
|---|---|
| link cut | 0 s |
| `0001` drops `0000` | **943 s** |
| `0000` drops `0001` | **953 s** |

Cause and fix shape under §4, "Outside the routing protocol".

Two things a shorter cut hides, learned the hard way: a 33 s cut never reaches the
detection threshold at all, and without a baseline read the post-cut multiset looks
like it appeared instantly. Take the baseline first, and hold the cut past 950 s.

### Churn, node kill (grid 3×3, killing the centre 0004)

Killing the centre of a 3×3 grid4 leaves exactly a circle-8, so the end state is known
independently and can be checked against the standalone circle-8 run above.

| prediction | result |
|---|---|
| `0001` neighbours 3 → 2 | **≤1 s** |
| `0000` users 9 → 8 | **34 s** |
| `0000` metrics → `10 10 20 20 30 30 40` | exact |
| entries 8 → 7, not a re-route | exact |

All eight survivors converged to `10 10 20 20 30 30 40`, the uniform multiset of a ring
of 8 — two independent routes to the same topology agreeing. `already forwarded` 0 on
every node, as in the standalone circle-8.

**The contrast is the finding.** Same mesh, same node pair, two ways to lose a neighbour:

| loss mode | detection |
|---|---|
| process killed, sockets closed | **≤1 s** |
| link cut, sockets silently blackholed | **943–953 s** |

So the ~950 s is specific to *silent* link loss. The socket-close path works correctly;
the timeout path does not exist. A crash is handled; a node walking out of range is not.

The 34 s pins `route_expiry_ms` (35 s) to within one refresh interval.

**Still open — the post-detection black hole.** From `0000` this run could not see it:
removing the centre deletes `0004`'s entry and changes no other distance from a corner,
so there was no re-route to observe. `0001` is where it shows — `0001 → 0007` is 2 hops
through `0004` in the grid and 4 around the ring, so `0001` held a next hop pointing at a
neighbour it knew was gone at t=1 s and could only correct it at expiry: up to 34 s of
black hole *after* the loss was known. `remove_neighbour_transport` does not retire
entries whose next hop was that peer. Spec-conformant under the passive reading of §7.5
(line 580 is ambiguous on active vs. passive), but worth a dedicated run: watch `0001`'s
multiset and the `via` column for `0007` across a centre kill.

### Churn after the 4-G fix (grid 3×3, same cut, same mesh)

| | before | after |
|---|---|---|
| detection, silent link loss | 943–953 s | **5 s** |
| recovery after the link returns | never cleanly measured | **~12 s** |
| total outage | ~16 min | **~17 s** |

The whole cycle now fits inside §7.5's 35 s route expiry, so a link that drops and returns
within that window loses the adjacency and regains it **without the routing table retiring
anything**. Before the fix the neighbour stayed stale for 16 minutes while the routes
expired underneath it — the inverse of what is wanted.

Detection came in faster than the arithmetic predicted (15–35 s): libp2p swallows the
first failure of each run, so a threshold of 2 is three failed cycles, but the
substream-reopen leg fails fast rather than burning a full timeout. Whether recovery at
~12 s was a floodsub retry or a fresh mDNS discovery is not established — the redial on
`ConnectionClosed` fired at 5 s while the link was still cut, so something later drove it.

**Regressions checked, all unchanged:**

- 9-node convergence 22 s (band was 16–23 s); corner multiset exact; `nb` stable, so the
  5 s ping timeout produces no false positives against sub-millisecond lab RTTs.
- Crash path: `0001` still drops in ≤1 s, users 9 → 8 at 31 s, `10 10 20 20 30 30 40`.
  The two disconnect routes do not interfere.

**Tuning left deliberate, not settled.** `neighbour_ping_failures: 2` at
`ping_timeout: 5` means a peer that misses two cycles is dropped. That is ample against
lab RTTs and may be aggressive on a congested LAN or a lossy BLE link. A false positive
costs a reconnect, not a lost neighbour, and both values are config fields.

**The black-hole gap grew in importance.** The neighbour is now known gone at 5 s while
routes through it still wait out the 35 s expiry, so the window between "known gone" and
"routes retired" went from effectively zero to ~30 s. Retiring entries whose next hop was
the dead peer is worth more now than before the fix. Written up as
`v1-to-v2-cutover.md` §5, with the reproduction and the spec question to settle first.

### Gateway stage 1 — two nodes joined only by the Internet transport

Off-lab: two local daemons, `lan.active: false` on both, one listening on
`/ip4/127.0.0.1/tcp/9229`, the other dialling it. No netns. Possible only after
4-H, since a `/p2p/<id>` dial target needs a stable node id.

**First live exercise of the whole sphere/membrane subsystem.** Everything below
had only ever been unit-tested:

| | result |
|---|---|
| 2-A — gateway role at runtime | `gateway yes (§2.3)` on both |
| §3.2 trigger 2 — INTERNET transport forces node form | `propagation form node entry`, with only **one** hosted user |
| Internet sphere instantiated | `sphere internet` in the manifests view |
| §7.4 `local_only` | `local no` — every LAN entry ever recorded was `yes` |
| §5 path cost off LAN | **metric 15** for one Internet hop, against 10 per LAN hop |
| Manifest sync across the membrane | advertised 2 → pulled → verified → committed 2, `signed yes`, `trusted 1/1` |
| Routing shape | 0 user entries / 1 node entry — node space is the only path |
| §9.1/§9.2 reachability | both nodes list both users with **zero** user-space entries, so purely through manifests |
| Chat + receipt | delivered ~30 ms, `✓✓` returned |

**Second run, after 4-H and 4-I: the asymmetry reproduced with the sides
reversed** — `gw-a` blind, `gw-b` trusting — which made it a race rather than a
property of dialer vs listener. Diagnosed as **11-D**. The §11.5 fallback itself
works: both nodes logged `profile … requested via carrying host (§11.5)`. What
failed is that `gw-a` dials 3 s after `gw-b` starts, so a `ProfileRequest` can
land before the carrying host has registered its hosted profile;
`profile.rs:172` then logs *"profile not found; leaving the fetch to expire"* and
drops it, which §11.2 permits. Nothing re-asked, so a lost startup race became
permanent. Re-test after 11-D.

**Setup gotcha, cost a round — since fixed as 4-I.** `qauld` created a new user
account on every start, with or without `--name`, so each restart changed the
hosted user identity and left another dead account in `config.yaml`. That
inflates the host's user count and can flip propagation form through §3.2's
*first* trigger, easily confused with the INTERNET trigger under test. It also
made runs incomparable, since user ids changed between them. Verified fixed:
three starts, one account, stable node and user id. On any run predating 4-I,
check `grep -c '^- name:' config.yaml` before reading a form transition.

### Scale (grid4, LAN, one user per node)

| nodes | grid | diameter | neighbour | routing | users | full mesh |
|---|---|---|---|---|---|---|
| 9 | 3×3 | 4 | 0–2 s | 2–13 s | 13–23 s | 16–23 s |
| 16 | 4×4 | 6 | 2 s | 19–26 s | 23–33 s | **33 s** |
| 20 | 4×5 | 7 | 3 s | 19–27 s | 23–35 s | **35 s** |

**Growth tracks diameter, not node count** — +25% nodes and +1 diameter cost 2 s. At 16
nodes, 13 of 16 hit exactly 23 s for both routing and users: a tight cluster, no long
tail, none of the bimodality the 9-node runs showed.

Metrics exact at every size:

- 4×4 corner — `10 10 20 20 20 30 30 30 30 40 40 40 50 50 60` (15 entries)
- 4×4 interior (1,1) — `10 10 10 10 20 20 20 20 20 20 30 30 30 30 40` (15)
- 4×5 corner — `10 10 20 20 20 30 30 30 30 40 40 40 40 50 50 50 60 60 70` (19)

The `70` is 7 hops, equal to line-8's diameter and the first depth-7 run on a *mesh*,
where alternate paths exist to choose wrongly between.

`router v2 status` clean at 16 on corner, interior and opposite corner: pending intros
0 user / 0 node, dictionaries 16 user, nothing in flight.

**`no route to … dropping` is non-zero and that is correct.** 18–34 per node at 16;
606 total at 20, and **static** — 606 twice across 60 s. §11.2: *"An undeliverable
management message is dropped. Each use case detects a missing outcome … and re-issues
the request at its own layer."* Profile fetches issued before routes exist are dropped
and re-issued; every node still reached full user knowledge. **Read this counter as a
delta, never as an absolute** — the zero recorded on line-4 was a post-convergence
delta, not a lifetime total.

---

## 4. Open findings

Ordered by what I would take next.

1. **Discovery floor, ~10 s.** `spawn_origin_tick` consumes the immediate first
   tick before its loop, so no node originates its own entry until t=10 s.
   Measured as the dominant term in every convergence run. Fix shape: originate
   to a neighbour when it first connects, at the current sequence number, no
   increment — `on_neighbour_connect` already fires at that moment. Needs care
   on BLE, where §3.6 sends no `INDEX_DUMP`, so the entry must carry its own
   inline mapping or it is dropped as an unknown mapping.
   **Amended.** One grid run measured routing entries at 5–7 s, below the 10 s tick,
   so the "nothing originates until t=10 s" premise is wrong as written —
   `propagation::on_neighbour_connect` already fires from `ping_event`
   (`events.rs:166`). Earlier runs measured 11–13 s. Re-derive before acting.
2. **10-A (High)** — the §10.4 liveness limit checks only the user's own routing
   entry, not the host node's. Bites a multi-user non-gateway host's users at
   their cross-host gateway. Needs a gateway to reproduce.
3. **8-A (Medium)** — §8.8's `NODE_MANIFEST` freshness check is not implemented;
   the full-manifest path commits whatever assembles.
4. **10-C (Medium)** — a delegated user's `profile_version` change does not bump
   `manifest_version`, so §10.8's trigger never fires. Testable on the
   multi-user grid.
5. **11-A, cross-host half.** The fallback covers a host that hosts the subject.
   A gateway carrying a cross-host delegation holds the user's key only in
   `UsersMap`, while `handle_profile_request` answers from `hosted_profiles`.
   Needs §11.5's "answer on the subject's behalf" against a cache of foreign
   signed profiles — `table::User` keeps neither `name` nor `self_signature`.
6. **4-B (High)** — RSSI hardcoded `None`. Investigate first: the Linux BLE
   module's unknown-RSSI sentinel `999` casts to `i8` as **−25 dBm**, the best
   bucket.
7. **2-E (Minor)** — the audit's proposed gate (`host_is_gateway()`) is wrong; it
   would break multi-user non-gateway hosts. Correct gate is node form. Not
   implemented.
8. **10-B, 3-L, 14-A, 3-G, 2-F, 4-D/E/F, 5-A, 3-I** — low/note severity.

### Unexplained, recorded as an observation only

**Routing convergence is bimodal**: ~12 s or ~21 s, a gap of about one origin
cycle, across four grid runs (one before 11-C, three after). Not caused by 11-C —
a post-fix run hit the fast mode. Cause not established. The direct evidence to
gather is `grep -c "no mapping for incoming" /tmp/qaul-*/qauld.err` on a slow run
versus a fast one. It did **not** reappear at 16 or 20 nodes, where the spread was
tight — so it may be specific to the 9-node grid rather than general.

### Outside the routing protocol

**LAN neighbour loss** was detected ~950 s late. **Fixed** — see §2, Round 2.

**Feed propagates exactly one hop.** The re-flood guard in the event loop is at
*module* granularity (`if !matches!(msg.incoming_via, ConnectionModule::Lan)`),
so on a single-transport mesh a received feed message is never re-published.
Measured: the origin and its one neighbour had it, the other six nodes did not.
Pre-existing, fails identically under v1, unrelated to routing. Wants its own
issue.

---

## 5. Test harnesses

**Unit:** `cargo test -p libqaul --lib` → 743 passing.

**Convergence:** `tests/integration/convergence.sh [node_count]`. Run it
immediately after `software.py start` — t=0 is when the script begins. Reports
neighbour / routing / users milestones per node.

**meshnet-lab** at `/home/lilit/meshnet-lab`.

```sh
cd ~/meshnet-lab
./topology.py grid4 3 3 > /tmp/grid9.json      # or: line 8 | circle 8
sudo ./network.py apply /tmp/grid9.json
export QAUL_ROUTING_V2=1
export RUST_LOG='libqaul::router_v2::management=debug,libqaul=info'
sudo -v && sudo -E ./software.py start qaul && ~/qaul.net/tests/integration/convergence.sh 9
# teardown: sudo ./software.py stop qaul && sudo ./network.py clear
```

### Gotchas

- Node IDs are `0000`-based. `topology.py` offers grid4, grid8, circle, line,
  tree, rtree, full, clusters, nodes.
- **`/usr/local/bin/qauld` pointed at `target/debug/`.** Cost a full test cycle
  chasing a fix that appeared not to work. Repointed at `target/release/`; check
  `readlink -f` after any build.
- **`qauld.sock` is 0600** (commit `d3bc8475`), and meshnet-lab runs qauld as
  root, so every `qauld-ctl` call against a lab node needs `sudo`.
- **The lab wipes node state on start** (`qaul_start.sh` does `rm -rf "$DIR"`),
  so nothing carries across runs. Note this was *not* why ids changed between
  runs — that was 4-H, and ids changed on every restart with or without the
  wipe. Since 4-H, a node id is stable for a given storage directory.
- `sudo -E` is required or the env vars are stripped.
- `software.py run` never prints output; read `/tmp/qaul-*/` from the host.
- Unix socket paths must stay under `SUN_LEN` (108 chars).
- A config with an empty `peers:` list silently resets the whole config to
  defaults. Write `peers: []`.
- `qauld-ctl router table / neighbours / connections` now read **v2** state via
  the bridge, as do `users online` and the Connectivity column.
- Counting rows: use `grep -c "^[0-9][0-9]* |"`. A single-digit pattern silently
  undercounts once a mesh exceeds nine users.

---

## 6. Claims retracted

Recorded so they are not re-derived.

**From round 1:**
- Mixed v1/v2 meshes were called "silent corruption". They are not — decode
  fails deterministically (`InvalidTag`). Wasted bandwidth, no corruption.
- `dtn/mod.rs`'s use of `next_hop_for_user` was called a divergence. It is
  correct: DTN asks §9.1's reachability question, not §9.2's forwarding one.
- 2-E's proposed fix would break multi-user non-gateway hosts.
- A `metric=10`-only reading was an artifact of `sort -un` on a `metric=`-prefixed
  string. Use `sort -u`.

**From round 2:**
- **11-A's caveat was wrong.** It claimed a mixed village/gateway topology was
  needed before calling it a defect. §3.2 reproduces the identical cycle on LAN
  with a multi-user host.
- **11-B's finding claimed §11.5 mandates node-sourced requests.** It does not;
  §11.5 constrains only the destination. That objection was why its direction 2
  was half-closed.
- **`already forwarded` was predicted non-zero on a circle.** It is zero, and
  correctly so — unicast forwarding does not duplicate on a cyclic topology.
- **Routing was called a regression after 11-C** on the strength of one slow
  run. It is pre-existing bimodality; a later run hit the fast mode on the same
  binary.
- **Re-dial after a LAN disconnect was called mDNS-TTL-gated.** It is not. Floodsub
  re-dials every `target_peers` member on disconnect (`layer.rs:333`, *"we always try
  to reconnect"*). The `target_peers.insert` gate is real but governs only first
  discovery, never recovery.
- **A 33 s link cut was read as "the router ignored the cut".** It did not — detection
  simply takes ~950 s, and routing had already re-routed correctly via §7.5 freshness.

---

## 7. Minor issues noticed, not filed

- `qauld-ctl` panics on `SIGPIPE` (`... | head -1`).
- Pre-existing clippy: 3 `never_loop` errors in
  `services/crypto/mod.rs:782,839,941`; `type_complexity` on `declined_targets`;
  needless borrows in `get_resource_mk`.
- Pre-existing test failure unrelated to this work: `instance_init.rs:152`.
