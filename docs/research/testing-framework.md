# qaul Testing Framework Research

This document covers what can be tested in qaul's routing protocol and network behaviour,
what is currently observable via the RPC interface, and what new RPC commands would be
needed to make the full test suite possible.

---

## 1. The Routing Protocol

qaul uses a **distance-vector routing protocol** with RTT-based link quality metrics.
Understanding it is essential before writing tests, because the timing constants embedded
in the protocol determine how long tests need to wait.

### How routes are discovered

Route discovery is entirely periodic. There is no triggered/reactive advertisement when
a new node appears — everything relies on scheduled broadcasts:

- **Every 5 seconds:** Each node sends libp2p ping packets to all direct neighbours to
  measure RTT.
- **Every 10 seconds:** Each node sends a `RouterInfoMessage` to all its neighbours.
  This message contains the node's full routing table — every user it knows about, with
  hop counts and RTTs. Neighbours process this, increment hop counts, and re-broadcast
  to their own neighbours.

This means: on a 4-hop line topology (nodes 0→1→2→3→4), a new node at position 4 takes
at least 40 seconds to be known at position 0, because the routing information must
propagate one hop per broadcast cycle (10s × 4 hops). In practice it takes longer
because cycles don't align perfectly. **Tests that assert full discovery on a 5-node
line should wait at least 90 seconds from a cold start.**

The relevant configuration constants are in
`rust/libqaul/src/storage/configuration.rs` line 187-196:
- `sending_table_period` = 10 seconds (RouterInfo interval)
- `maintain_period_limit` = 300 seconds (5 min, maximum staleness before user removed)

### How routes are maintained

Routes are updated on every RouterInfo cycle. When a better route arrives (lower link
quality score), it replaces the existing one. The link quality formula is:

```
LQ = RTT + (hop_count × hop_count_penalty × 1_000_000)
```

Lower LQ is better. The hop_count_penalty strongly penalises extra hops, so a 1-hop
route with higher RTT will usually beat a 2-hop route with lower RTT.

### How defunct routes are removed

Route expiration uses **hop-count weighted timeouts**
(`rust/libqaul/src/router/connections.rs` lines 455-481):

```
Expiration threshold = sending_table_period × 1000 × (hop_count + 1) milliseconds
```

Concretely:
- 1-hop route expires after ~20 seconds without update
- 2-hop route expires after ~30 seconds
- 3-hop route expires after ~40 seconds
- N-hop route expires after ~(N+1) × 10 seconds

If all routes for a user expire, the user is removed from the routing table.
The user entry itself is removed if not updated for 5 minutes (`maintain_period_limit`).

This means: **a test that verifies route removal after a node goes offline must wait
at least (hop_count + 1) × 10 seconds** before asserting the route is gone. For a
5-node line, the worst case is 50 seconds.

### Propagation ID (pgid)

Each node maintains a `pgid` (propagation ID) that increments approximately every 10
seconds. When RouterInfo is forwarded by intermediate nodes, they attach the original
sender's pgid. This prevents routing loops and lets receivers identify stale information:
if a received message has an older pgid than what's already stored, it's discarded.

The propagation ID is the key mechanism that prevents infinite re-flooding of routing
information. It is implemented in `rust/libqaul/src/router/connections.rs` lines 252-287.

### Connection modules and priority

qaul supports four connection modules. Routes are selected in this priority order:

| Priority | Module | Description |
|----------|--------|-------------|
| 1 (best) | Local | User registered on this node |
| 2 | LAN | mDNS-discovered neighbour on local network |
| 3 | Internet | Statically configured internet peer |
| 4 (worst) | BLE | Bluetooth Low Energy (mobile) |

In meshnet-lab, all connections use the LAN module (simulated local network interfaces
in separate network namespaces).

---

## 2. What is Currently Observable via RPC

The existing RPC interface (`Modules::Router`) exposes three query types. None of these
are currently implemented as qauld-ctl subcommands — they exist in the proto definitions
and the libqaul handler but there is no CLI surface for them yet.

### 2.1 Routing Table (`RoutingTableRequest` / `RoutingTableList`)

Returns the current best route per user per module. Fields per entry:
- `user_id` — who the route leads to
- `module` — LAN / Internet / BLE
- `rtt` — round-trip time in milliseconds
- `hop_count` — number of hops
- `via` — node ID of the next hop

**Test uses:** verify a user is reachable, check hop count matches topology, verify
route uses expected module.

Proto: `protobuf/proto_definitions/router/router.proto` lines 28-59

### 2.2 Neighbours (`NeighboursRequest` / `NeighboursList`)

Returns direct neighbours — nodes with a single-hop connection. Fields per entry:
- `node_id` — the neighbour's libp2p peer ID
- `rtt` — round-trip time in milliseconds
- Separated by module: `lan`, `internet`, `ble`

**Test uses:** verify that adjacent nodes in the topology are correctly detected as
direct neighbours, verify RTT is non-zero on active links.

Proto: `protobuf/proto_definitions/router/router.proto` lines 100-118

### 2.3 Connections (`ConnectionsRequest` / `ConnectionsList`)

Returns all known paths to each user, not just the best one. This is the pre-selection
view — every candidate route the routing table knows about, before the best-route
selection step. Fields per entry:
- `user_id`
- `rtt`, `hop_count`, `via` per path

**Test uses:** verify route redundancy (multiple paths exist), observe all candidate
routes before best-route selection.

Proto: `protobuf/proto_definitions/router/router.proto` lines 62-97

---

## 3. `router` Subcommand in qauld-ctl

The three RPC query types are now exposed via `qauld-ctl router`:

```
qauld-ctl router table          → RoutingTableRequest / RoutingTableList
qauld-ctl router neighbours     → NeighboursRequest / NeighboursList
qauld-ctl router connections    → ConnectionsRequest / ConnectionsList
```

JSON output shapes:

**`router table`**
```json
[
  {
    "user_id": "<base58>",
    "connections": [
      { "module": "LAN", "rtt": 273, "hop_count": 1, "via": "<base58>" }
    ]
  }
]
```

**`router neighbours`**
```json
{
  "lan": [ { "node_id": "<base58>", "rtt": 120 } ],
  "internet": [],
  "ble": []
}
```

**`router connections`**
```json
{
  "lan": [
    {
      "user_id": "<base58>",
      "connections": [
        { "rtt": 273, "hop_count": 1, "via": "<base58>" }
      ]
    }
  ],
  "internet": [],
  "ble": [],
  "local": []
}
```

---

## 4. Test Categories and What Each Requires

### 4.1 Correctness Tests (implement first)

These verify that the network works at all. They use only `users list` and `feed list`
which are already implemented.

| Test | What it checks | Wait time | Commands needed |
|------|---------------|-----------|-----------------|
| Node startup | All nodes respond to `node info` | 0s | `node info` |
| User discovery | Each node eventually sees all others | 90s (5-node line) | `users list` |
| Feed delivery | Message sent on node 0 appears on node 4 | 90s + propagation | `feed send`, `feed list` |
| Direct message | Chat message delivered point-to-point | 90s + propagation | `chat send`, `chat conversation` |

### 4.2 Routing Protocol Tests (require `router` subcommand)

These test the routing protocol itself — not just whether messages get through, but
whether the routing tables are correct.

**Route discovery speed:**
- Start a 5-node line topology
- Poll `router table` on node 0000 every 5 seconds
- Record the timestamp when each of the 4 remote users first appears
- Assert all 4 are discovered within a defined threshold (e.g. 120s)

**Route stability:**
- Start a 3-node line
- Wait for full convergence
- Poll `router table` on node 0000 every 10 seconds for 5 minutes
- Assert: no user disappears and reappears (route flapping)
- Assert: RTT and hop count for stable routes do not change by more than ±20%

**Defunct route removal:**
- Start a 3-node line: 0000 — 0001 — 0002
- Wait for full convergence
- Kill node 0001 (remove it from the topology)
- Poll `router table` on node 0000
- Assert: node 0001's route disappears within (1+1)×10 = 20 seconds
- Assert: node 0002's route disappears within (2+1)×10 = 30 seconds

**Neighbour detection:**
- On a 5-node line, query `router neighbours` on each node
- Assert: each node has exactly 1 or 2 direct LAN neighbours (topology-dependent)
- Assert: no node appears as a direct neighbour if it is 2+ hops away

**Route re-convergence after link restore:**
- Start a 3-node line: 0000 — 0001 — 0002
- Wait for full convergence
- Disconnect node 0001 from the topology (remove its links)
- Wait for routes to expire (~30s)
- Restore node 0001's links
- Measure time until routes re-converge
- Assert: full re-convergence within 60s of link restore

### 4.3 Traffic Overhead Tests (require packet capture or counters)

These measure how much network traffic the routing protocol generates. The question is:
what percentage of total bytes on the wire are routing overhead vs. user data?

The current RPC interface does not expose traffic counters. Options:

1. **Use meshnet-lab's `tc` statistics** — after running nodes, query `tc -s qdisc show`
   on each namespace interface to get bytes/packets sent. This is external to qaul and
   doesn't require any new RPC.

2. **Add traffic counters to RPC** — expose bytes sent/received per module per time
   period. This is a larger change and should come after correctness tests are working.

For now, option 1 is sufficient and requires no code changes.

### 4.4 Network Options Tests

**Feed message distribution:**
- Verify messages are flooded to all nodes (not just neighbours)
- Verify duplicate messages are deduplicated (same message_id should not appear twice)
- Verify messages arrive in the correct order at each node

**Direct messages (point-to-point):**
- Node A sends a chat message to node B via `chat send --group-id <direct-group>`
- Verify it appears in node B's `chat conversation` but NOT in node C's
- This tests that direct messages use unicast routing, not flooding

**Multi-hop message routing:**
- 5-node line: 0000 — 0001 — 0002 — 0003 — 0004
- Node 0000 sends a feed message
- Verify it appears on all 5 nodes
- Verify it does NOT appear before nodes have discovered each other (ordering test)

**Message delivery under degraded links:**
- Apply `tc netem` with 10% packet loss on all links
- Send 100 feed messages from node 0000
- Count how many arrive at node 0004
- Assert delivery rate ≥ 90% (configurable threshold)

---

## 5. What New RPC Commands Are Needed

In priority order:

### Priority 1: `router` subcommand in qauld-ctl (blocking for routing tests)

Proto already exists. Only qauld-ctl CLI work needed:

```
qauld-ctl router neighbours     → NeighboursRequest
qauld-ctl router table          → RoutingTableRequest
qauld-ctl router connections    → ConnectionsRequest
```

JSON output would expose:
- Which nodes are direct neighbours (with RTT)
- Full routing table (who is reachable, via which path, with what metrics)
- All candidate routes (pre-selection)

### Priority 2: Route timing visibility (blocking for expiration tests)

The current routing table response does not include `last_update` timestamp or
expiration threshold. To test "how fast is a defunct route deleted", tests need to
either:

a) **Poll externally:** query `router table` repeatedly and timestamp when a user
   disappears. This works without any new RPC — just requires polling in the test.

b) **Expose `last_update_age_ms`:** add a field to `RoutingTableConnection` in the
   proto so the test can see how stale a route is without waiting for it to expire.

Option (a) is sufficient for correctness tests. Option (b) is better for debugging
but is a proto change. Defer to after correctness tests pass.

### Priority 3: Internet peers management (needed for internet module tests)

Already exists in the proto (`InternetNodesAdd`, `InternetNodesRemove`) but not yet
exposed in qauld-ctl. Needed to programmatically configure static peers in tests.
Not required for meshnet-lab tests (which use LAN module), but needed for internet
topology tests.

---

## 6. Implementation Order

1. **Get correctness tests passing** (current work)
   - `test_node_startup.py` — already written
   - `test_user_discovery.py` — fix wait time, fix LOCAL user detection
   - `test_message_routing.py` — write feed delivery test

2. ~~**Implement `router` subcommand in qauld-ctl**~~ — **DONE**
   - `router table`, `router neighbours`, `router connections` all implemented
   - Full `--json` support
   - `rust/clients/qauld-ctl/src/commands/router.rs`

3. **Write routing protocol tests**
   - `test_route_discovery.py` — convergence timing
   - `test_route_stability.py` — no flapping over 5 minutes
   - `test_route_expiration.py` — defunct route removal timing

4. **Write network options tests**
   - `test_direct_messages.py` — point-to-point, not flooded
   - `test_feed_distribution.py` — all nodes receive, no duplicates

5. **Performance layer** (separate concern, after all correctness tests pass)
   - Scaling to 100+ nodes
   - Traffic overhead measurement
   - Convergence time vs. topology size

---

## 7. Key Timing Constants (Summary)

| Constant | Value | Location | Affects |
|----------|-------|----------|---------|
| RouterInfo interval | 10s | `configuration.rs:196` | Discovery speed |
| Routing table rebuild | 1s | `lib.rs:996` | Route selection latency |
| RTT ping interval | 5s | `lib.rs:990` | Metric freshness |
| 1-hop route expiration | ~20s | `connections.rs:455` | Defunct route removal |
| N-hop route expiration | ~(N+1)×10s | `connections.rs:455` | Defunct route removal |
| User entry max staleness | 300s | `configuration.rs:187` | User removal |

These constants are the ground truth for test timeout values. Any test that sets a
fixed sleep duration should derive it from these constants with a safety margin of
at least 2×.
