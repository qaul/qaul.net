# v1 → v2 Cutover: Making the Client Surface Work Under router_v2

**Status:** Delivered, except §4 and §5
**Scope:** The RPC-facing state that `qauld-ctl` and the Flutter UI read
**Branch:** `#feat/v2-router/#880`
**Related:** `docs/proposals/v2-router-audit-handoff.md` (current state),
`docs/proposals/implementation.md` (routing conformance audit),
`docs/protocols/Qaul-Routing-Protocol.md`

---

## 1. What this is

With `router_v2` enabled, six client-facing commands read v1 state that v2
never writes, so they showed empty or default values while routing and
messaging worked fine. This plan covered bridging that state so the existing
RPC surface — and therefore the app — reflects reality under v2.

What remains of it is §4, plus the reference material the bridge needs to stay
correct (§3), the behaviour differences worth documenting (§6), and the rules
that keep v1's eventual removal a deletion rather than an untangling (§7).
§5 is an open `router_v2` defect rather than cutover work, parked here because
this is where open work now lives.

## 2. Delivered

| Piece | Where it landed |
|---|---|
| Routing-table adapter | `router_v2/state/v1_bridge.rs::create_v1_routing_table`, branched at the `EventType::RoutingTable` tick |
| Neighbours and connections adapters | `rpc_send_neighbours_list`, `rpc_send_connections_list`, same file |
| Neighbour round-trip time carried through | `NeighbourInfo.rtt_micros`, set from the v2 ping branch |
| `EventType::RoutingInfo` guarded under v2 | stops v1 routing frames reaching a v2 mesh |
| v2-native inspector | `router v2 status \| table \| neighbours \| manifests \| delegations` |
| Unit tests | `router_v2/tests/v1_bridge.rs` |

`users online`, `users list` (Connectivity column), `users get`, `router table`,
`router neighbours` and `router connections` all read v2 state through the
bridge, as does the Flutter UI, which uses the same RPC.

Both inspector cuts were built, including the manifests and delegations views
originally deferred — the multi-user grid runs needed them sooner than expected.

The adapter iterates `UsersMap` rather than the 65,536-slot index array, so its
cost scales with known users rather than with index space.

---

## 3. The identifier problem

Kept because `v1_bridge.rs` depends on it and it is not obvious from the code.

**This is the part to get right first.** The two routers key users by
different 8-byte values:

| | Derivation | Source |
|---|---|---|
| v1 `q8id` | `PeerId.to_bytes()[6..14]` — a byte *slice* | `utilities/qaul_id.rs:19-20, 62` |
| v2 routing id | `sha256(multikey)[..8]` — a *hash* | `router_v2/identity.rs:37-42` |

They are unrelated values for the same user. Neither can be used as a key into
the other's map.

**The bridge:** `Multikey::to_peer_id()` → `QaulId::to_q8id()`. This works for
any v2 `User` whose `public_key` is `Some`, i.e. whose profile has been
fetched.

Note the dependency: before the 11-B fix, profiles never resolved beyond one
hop, so most of the table would have been unmappable. That fix is a
precondition for this adapter being useful at all.

Users with `public_key: None` are skipped. They are unrenderable in v1 terms
anyway, and a v1 node would not have listed them either.

---

## 4. Remaining work — path cost has no `rtt` and needs `metric`

**Files:** `protobuf/` routing definitions, `router/users.rs` render path,
`qauld-ctl` display.

`RoutingTableConnection` has no field for a path cost. Rather than overload
`rtt` — which the UI labels as milliseconds — add `metric` and render it when
v2 is active. Until then the bridge emits 0 and the client shows an honest
blank rather than a fabricated latency.

**Acceptance:** `users list` shows the same metric the v2 table holds.

---

## 5. Remaining work — routing entries outlive a removed neighbour

Not cutover scope — a `router_v2` defect — but tracked here because it is open
work and this is where open work now lives.

When a neighbour is removed, nothing touches routing entries whose next hop was
that neighbour. They stay in the table, still naming the dead peer, until the
route-expiry timer retires them independently. Traffic for those targets is
forwarded into a black hole for the remainder of the window.

This was always true but invisible while neighbour loss itself took ~950 s to
detect. Now that silent loss is detected in ~5 s, the gap between "we know this
neighbour is gone" and "routes through it are retired" is ~30 s of the 35 s
expiry window.

**Where.** `remove_neighbour_transport` (`router_v2/state/neighbours.rs:128`)
drops the mirror and the two rate-limit windows; it neither reads nor writes
`routing_table`. Retirement happens only in `sweep_expired`
(`router_v2/state/expiry.rs:13`), on the 1-second relay tick, purely on
`last_update + route_expiry_ms` (35 s default).

**Reproduction.** 3×3 grid4, LAN only, one user per node. Converge, then kill
the centre `0004` by pid. `0001 → 0007` is 2 hops through `0004` in the grid and
4 around the ring that remains, so `0001` must re-route. `0001` drops `0004`
from its neighbour list in ≤1 s but keeps its entry for `0007` with `via` still
naming `0004` until expiry fires. Measured on `0000` for the simpler case of
`0004`'s own user: gone from `users online` at 31 s and 34 s across two runs.

**Settle the spec question first.** §7.5 defines expiry as purely time-based —
no update with a current or fresher sequence number for 35 seconds — and says
nothing about neighbour withdrawal. §4 (line 580) is ambiguous:

> The routing layer consumes the neighbour list and expires routing entries via
> the standard route expiry timeout when a transport withdraws a neighbour.

That reads as expiry being *triggered* by withdrawal; the implementation is
passive. If the active reading is intended, §7.5 needs a companion clause. If
the passive reading is intended, §4's wording wants tightening so the next
reader does not re-derive this.

**What actually needs fixing is narrower than it first looks.** A fresher
sequence number already wins regardless of metric, so for any target still
reachable another way the correction arrives at that target's next origin tick,
within ~10 s. The real exposure is targets reachable *only* through the dead
neighbour, where nothing fresher will ever arrive and the entry must be retired
rather than replaced. That distinction should decide the approach:

1. **Poison, don't delete.** Mark affected entries unusable — infinite metric or
   an explicit invalid flag — so an alternative is immediately acceptable while
   the index binding and sequence number survive. Avoids index churn.
2. **Retire outright.** Simple, but frees index slots into the §3.7 cooldown
   (60 s), so a target still reachable by another path cannot be re-bound
   promptly, and downstream nodes see a withdrawal followed by a re-learn.
3. **Solicit.** Ask remaining neighbours for a fresh update covering the
   affected targets. Lowest risk of wrong state, highest message cost, and needs
   a message type that does not exist.

**Verification.** Watch `0001`'s metric multiset and the `via` column for `0007`
across a centre kill. Regressions that must not move: 9-node convergence
(16–23 s), corner multiset `10 10 20 20 20 30 30 40`, users 9 → 8 at ~34 s.

---

## 6. Semantic differences to document

These will look like bugs if they are not written down:

1. **One connection per user, not one per module.** §9.2 gives v2's table at
   most one entry per target, chosen at receive time by relay inclusion (§7.2).
   v1 kept a best entry per module. The Connections list will therefore show a
   single row under v2 where v1 might have shown three. Conformant, not data
   loss.
2. **A routing entry's `rtt` is 0, and so is `lq`.** Not "unmeasured" — not
   applicable. v2's cost model is the §5 metric, and a path has no round-trip
   time. Distinct from a *neighbour's* rtt in `router neighbours`, which is a
   real ping measurement and is now reported.
3. **Users without a fetched profile are absent.** v1 could list a user it had
   only heard of; v2's bridge cannot key it without the public key.

---

## 7. How this gets deleted with v1

v1 is expected to be removed once v2 is validated in the field. This plan is
designed so that removal is a deletion, not an untangling. Two rules:

1. **The dependency points one way.** `v1_bridge.rs` reads v2 state and
   *returns* v1-shaped values. It calls no v1 logic and reads no v1 state, so
   nothing in v1 needs to keep working for it to compile.
2. **No v2 knowledge inside v1 rendering or logic.** All translation lives in
   the one v2-owned file. `router/users.rs`, `router/neighbours.rs` and
   `router/connections.rs` keep no `get_router_v2()` calls, which is what makes
   them straightforwardly deletable.

   The one permitted exception is the **RPC dispatch** at `router/mod.rs:167-175`,
   which may branch on router version. It is a boundary rather than logic:
   something has to answer these requests after v1 is gone, so that match arm
   survives v1's deletion by definition. Three adjacent lines in one dispatch
   is not the scattering this rule exists to prevent — branching inside the
   four `router/users.rs` call sites would be.

This is why the neighbours and connections adapters were built as a bridge
rather than by populating v1's live tables through `Neighbours::update_node`.
The latter was the original design and was reversed on review: it would put v2
in the position of feeding v1 machinery, and would have to be unpicked later.
It would also have woken `EventType::RoutingInfo`, which is unguarded, and
started emitting v1 routing frames onto a v2 mesh.

At deletion time there are two cases, and both are cheap:

* **The RPC keeps its current shape** (likely, because the Flutter UI depends
  on it). Then `RoutingTable`, `RoutingUserEntry` and `RoutingConnectionEntry`
  are not v1 router types at all — they are RPC payload types that move out of
  `router/` into the RPC layer. `v1_bridge.rs` survives with changed imports
  and should be renamed.
* **The RPC is redesigned around v2 concepts.** Then `v1_bridge.rs` is deleted
  outright, along with the branch at `lib.rs:996`.

The cost of the bridge, in other words, is bounded at one file. The cost of
*not* having it is that the app stays blind until a full v2-native RPC surface
and a matching UI change both land.

---

## 8. Open questions

1. **Does the retransmit path behave correctly now the table is populated?**
   `retransmit.rs:48` saw an empty online-user set under v2 and was effectively
   inert. The bridge switched it on for the first time, and its behaviour under
   v2 has not been exercised deliberately — no lab round so far has forced a
   retransmission. Worth a run that drops a message and watches what it does.
