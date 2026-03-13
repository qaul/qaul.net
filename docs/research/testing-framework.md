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

## 4. Bugs Found During Phase 1 and 2

These were discovered during the process of getting the first three correctness tests
passing. They are documented here because understanding them is critical for writing
correct tests and for planning future improvements.

### Bug 1: RPC response `request_id` always empty in Users module

**Location:** `rust/libqaul/src/router/users.rs`, `send_users_rpc_message()`

**Symptom:** `qauld-ctl users list` and `qauld-ctl users online` would hang indefinitely.
qauld would log `client ID not found in register`.

**Root cause:** The socket server (in `rust/clients/qauld/src/socket.rs`) matches RPC
responses back to waiting clients using a `request_id` UUID. The Users module's internal
helper `send_users_rpc_message()` was calling `Rpc::send_message(..., String::new(), ...)`
— always sending an empty string as the response `request_id`. The socket server's
response poller looked up `""` in its HashMap, found nothing, logged the warning, and
dropped the response. The qauld-ctl client waited forever.

**Fix:** `send_users_rpc_message` was made to accept a `request_id: String` parameter,
and `build_user_list` and all other callers were updated to pass the request_id through
from the incoming RPC message.

**Implication for testing:** Any hanging `qauld-ctl` command that qauld acknowledges
(logs say it received the request) but never returns is likely the same pattern —
the module's response helper is not propagating the `request_id`. Check other modules
for the same pattern if new commands hang.

### Bug 2: Feed messages silently dropped when sender is unknown

**Location:** `rust/libqaul/src/services/feed/mod.rs`, `Feed::received()`

**Symptom:** A feed message sent from node 0000 never appeared on any other node, even
directly connected node 0001. The feed list was empty on all intermediate nodes after
120 seconds.

**Root cause:** When a node receives a feed message via floodsub, it validates the
signature against the sender's public key — but only if the sender is already in the
user store (`router::users::Users::get_pub_key`). If the sender is not yet known,
the message is dropped with `log::error!("Sender of feed message not known")` and
never forwarded. On cold start, user discovery takes 60-120s on a 5-node line, so
a message sent immediately after startup is dropped at every hop.

**Fix (in tests):** Send feed messages only after waiting for user discovery to
complete (at least 120s on a 5-node line). The test `test_message_routing.py` now
waits for user discovery before sending.

**Implication for the system:** This is a security design decision (signature
validation before accept), but it creates a usability problem: any message sent
before the network converges is permanently lost. There is no retry or re-request
mechanism for messages dropped at intermediate hops. This is a significant scalability
concern — in large or sparse networks where convergence is slow, early messages
will routinely be lost.

**Potential improvements:**
- Buffer messages for unknown senders for a short window (e.g. 30s), retry on
  user discovery
- Use the feed requester mechanism to re-request messages from neighbours after
  convergence (this mechanism exists in `router/feed_requester.rs` but only covers
  messages the node already knows the ID of)

### Bug 3: Convergence required before any message delivery

**Observation:** The routing tables converge correctly on a 5-node line. Node 0003
(the bridge) shows all 5 nodes in its routing table within ~60s. However, even after
routing convergence, feed messages do not propagate unless user accounts have also
been discovered (separate from routing). User account propagation follows the same
periodic schedule (RouterInfo carries user info) but the two processes are not
synchronised — a node may be in the routing table before its public key is in the
user store.

**Implication for testing:** Discovery wait time in tests must account for both
routing convergence and user store population. In practice 120s is sufficient for
a 5-node line, but this should be measured empirically for each topology.

---

## 5. Test Categories and What Each Requires

### 5.1 Correctness Tests (COMPLETE)

These verify that the network works at all.

| Test | Status | What it checks | Topology |
|------|--------|---------------|----------|
| `test_node_startup` | ✅ passing | All nodes respond, node info fields present, IDs distinct | line-5 |
| `test_user_discovery` | ✅ passing | Nodes discover neighbours, user fields present | line-5 |
| `test_message_routing` | ✅ passing | Feed message crosses 4 hops, message fields present | line-5 |

Key timing constants used:
- User discovery wait: 120s (line-5)
- Feed propagation wait after discovery: 30s (line-5)

### 5.2 Routing Protocol Tests (next to implement)

These test the routing protocol itself — not just whether messages get through, but
whether the routing tables are correct and respond predictably to topology changes.

**Route discovery speed (`test_route_discovery.py`)**
- Start a 5-node line topology
- Poll `router table` on node 0000 every 5 seconds
- Record the timestamp when each of the 4 remote users first appears
- Assert all 4 are discovered within 120s
- Record actual convergence times to build an empirical baseline

**Route stability (`test_route_stability.py`)**
- Start a 3-node line
- Wait for full convergence (60s)
- Poll `router table` on node 0000 every 10 seconds for 5 minutes
- Assert: no user disappears and reappears (route flapping)
- Assert: RTT and hop count for stable routes do not change by more than ±20%

**Defunct route removal (`test_route_expiration.py`)**
- Start a 3-node line: 0000 — 0001 — 0002
- Wait for full convergence
- Kill node 0001 (remove it from the topology via meshnet-lab)
- Poll `router table` on node 0000 every 2 seconds
- Assert: node 0001's route disappears within 20s (1-hop expiration = (1+1)×10)
- Assert: node 0002's route disappears within 30s (2-hop expiration = (2+1)×10)

**Neighbour detection (`test_neighbour_detection.py`)**
- On a 5-node line, query `router neighbours` on each node after 60s
- Assert: node 0000 has exactly 1 LAN neighbour (node 0001 only)
- Assert: node 0002 has exactly 2 LAN neighbours (node 0001 and node 0003)
- Assert: no node appears as a direct neighbour if it is 2+ hops away

**Route re-convergence after link restore**
- Start a 3-node line
- Wait for convergence
- Disconnect node 0001 from the topology (remove its links via meshnet-lab)
- Wait for routes to expire (~30s)
- Restore node 0001's links
- Measure time until routes re-converge
- Assert: full re-convergence within 60s of link restore

### 5.3 Network Options Tests

**Feed message distribution:**
- Verify messages are flooded to all nodes (not just neighbours)
- Verify duplicate messages are deduplicated (same message_id should not appear twice)
- Verify message ordering: messages sent sequentially appear in the same order at each node

**Direct messages (point-to-point):**
- Node A sends a chat message to node B via `chat send --group-id <direct-group>`
- Verify it appears in node B's `chat conversation` but NOT in node C's
- This tests that direct messages use unicast routing, not flooding

**Message delivery under degraded links:**
- Apply `tc netem` with 10% packet loss on all links
- Send 10 feed messages from node 0000
- Count how many arrive at node 0004
- Assert delivery rate ≥ 80% (configurable threshold)

### 5.4 Scalability Tests (performance layer)

See section 7 for detailed topology definitions and metrics.

---

## 6. Traffic Overhead Measurement

### How to measure without new RPC

meshnet-lab runs each node in its own Linux network namespace. After running nodes,
query `tc -s qdisc show` on each namespace interface to get bytes and packets sent.
This requires no new RPC and works today:

```bash
# inside the namespace for node 0000:
ip netns exec node-0000 tc -s qdisc show dev uplink
```

The output includes `Sent X bytes Y pkts` which can be read before and after a test
window to get bytes/second per link.

To separate routing overhead from user data: the RouterInfo messages are sent every
10 seconds, with a size proportional to the number of known nodes. On a 5-node line,
a RouterInfo message is roughly 200-400 bytes. With 2 neighbours each sending every
10s, baseline overhead is ~40-80 bytes/s per node — negligible. On a 100-node network
where each node knows 99 others, a RouterInfo message could be 5-10 KB, giving
~500 bytes/s - 1 KB/s overhead per node just from routing. This scales linearly with
network size, which is a known limitation of distance-vector protocols.

### Adding traffic counters to RPC (future)

For more precise measurement, add a `Modules::Stats` RPC that exposes:
- Bytes sent/received per module per time period
- Number of RouterInfo messages sent/received
- Number of feed messages forwarded vs. accepted

This is a post-correctness-test improvement.

---

## 7. Topologies

### 7.1 Topology naming convention

All topology files live in `tests/integration/topologies/`. The naming format is:
`<type>-<nodes>[-<variant>].json`

### 7.2 Already defined

| File | Nodes | Links | Diameter | Use |
|------|-------|-------|----------|-----|
| `line-5.json` | 5 | 4 | 4 hops | baseline correctness tests |

### 7.3 Topologies to define next

The topologies below are ordered by priority. The first group covers the routing
protocol tests. The second group covers real-world scenarios for scalability.

#### Basic scaling topologies

| File | Nodes | Description | Why |
|------|-------|-------------|-----|
| `line-10.json` | 10 | 10-node line | tests convergence on longer chain, 9-hop diameter |
| `grid4-3x3.json` | 9 | 3×3 grid, 4-connected | first topology with path redundancy |
| `grid4-5x5.json` | 25 | 5×5 grid, 4-connected | medium scale, stress-tests routing table size |
| `grid8-4x4.json` | 16 | 4×4 grid, 8-connected (also diagonals) | denser mesh, shorter diameters |

Generate with meshnet-lab's topology.py:
```bash
cd /home/qaul/meshnet-lab
./topology.py line 10 > /path/to/tests/integration/topologies/line-10.json
./topology.py grid4 3 3 > /path/to/tests/integration/topologies/grid4-3x3.json
./topology.py grid4 5 5 > /path/to/tests/integration/topologies/grid4-5x5.json
./topology.py grid8 4 4 > /path/to/tests/integration/topologies/grid8-4x4.json
```

#### Real-world scenario topologies

These model actual deployment scenarios for qaul. Each is written directly as JSON
because meshnet-lab's built-in generators don't cover them.

---

**`two-clusters-bridge.json` — Two communities connected by a relay**

```
[0000]--[0001]--[0002]
  |                |
[0003]           [0007]--[bridge:0004]--[0005]--[0006]
  |                                              |
[...]                                           [...]
```

9 nodes total: two dense 4-node clusters (left: 0000-0003, right: 0005-0008),
connected by a single bridge node (0004). Each cluster is fully connected internally.

```json
{
  "nodes": {
    "0000": {}, "0001": {}, "0002": {}, "0003": {},
    "0004": {},
    "0005": {}, "0006": {}, "0007": {}, "0008": {}
  },
  "links": {
    "0": {"node1": "0000", "node2": "0001"},
    "1": {"node1": "0000", "node2": "0002"},
    "2": {"node1": "0000", "node2": "0003"},
    "3": {"node1": "0001", "node2": "0002"},
    "4": {"node1": "0001", "node2": "0003"},
    "5": {"node1": "0002", "node2": "0003"},
    "6": {"node1": "0003", "node2": "0004"},
    "7": {"node1": "0004", "node2": "0005"},
    "8": {"node1": "0005", "node2": "0006"},
    "9": {"node1": "0005", "node2": "0007"},
    "10": {"node1": "0005", "node2": "0008"},
    "11": {"node1": "0006", "node2": "0007"},
    "12": {"node1": "0006", "node2": "0008"},
    "13": {"node1": "0007", "node2": "0008"}
  }
}
```

**Why:** Models a realistic deployment where two villages or neighbourhoods are each
well-connected internally but only reach each other through one relay point. Tests:
- Feed message delivery when the bridge is the only path
- Route redundancy within each cluster
- What happens when the bridge node goes offline (complete network partition)

---

**`village-20.json` — A small village**

20 nodes arranged in a 4×5 irregular grid where some links are missing (simulating
obstructions like buildings, trees, terrain). Average degree: 3. Diameter: ~7 hops.

This models a rural village where every house has a qaul node, but not every house can
directly see every other (walls, distance). Generated as a grid4-4x5 with 20% of links
randomly removed.

**Why:** The most common real-world deployment scenario. Tests:
- Full convergence at 20 nodes (expected ~200s based on 10s×7hops + margin)
- Feed message delivery across irregular topology
- Route stability when some paths are better than others

---

**`street-corridor-15.json` — Street with facing buildings**

15 nodes: 8 on one side of the street, 7 on the other. Each node connects to its
immediate left/right neighbour on the same side, plus 1-2 nodes directly across the
street. Diameter: ~7 hops.

```
[0]-[1]-[2]-[3]-[4]-[5]-[6]-[7]   (side A, nodes 0000-0007)
 |   |       |       |       |
[8]-[9]-[10]-[11]-[12]-[13]-[14]  (side B, nodes 0008-0014)
```

**Why:** Models a linear residential street or a building corridor. The cross-links
create redundant paths (unlike a pure line), but the topology is still fundamentally
linear in character. Tests whether route redundancy improves delivery time vs line-15.

---

**`sparse-rural-12.json` — Rural relay chain with sparse coverage**

12 nodes. A central backbone of 4 relay nodes (well-connected to each other), with
2-3 leaf nodes hanging off each relay. Leaf nodes can only reach their relay; they
cannot see each other directly.

```
[leaf0]-[relay0]-[relay1]-[relay2]-[relay3]-[leaf9]
  [leaf1]  [leaf3][leaf4]  [leaf6]  [leaf7][leaf10]
  [leaf2]          [leaf5]  [leaf8]         [leaf11]
```

**Why:** Models a rural area where a few high-placed relay nodes serve many local users.
Tests: leaf-to-leaf messaging must traverse backbone. If one relay goes down, its leaves
become isolated. Critical test for resilience.

---

**`event-dense-25.json` — Festival or emergency response**

25 nodes arranged in a 5×5 grid8 (8-connected, including diagonals). Every node has
8 neighbours. Diameter: 4 hops. Very high redundancy.

**Why:** Models a crowded scenario (festival, emergency shelter, protest) where many
devices are in close physical proximity. In grid8 every node has degree 8, so routing
tables converge extremely fast. Tests whether qaul handles high-degree neighbours
gracefully (mDNS, routing table size). Contrast with sparse topologies to show
performance improvement from density.

---

**`disaster-degrading-20.json` — Progressive failure scenario**

Start with `grid4-4x5.json` (20 nodes, full connectivity). Then progressively remove
nodes or links during the test to simulate infrastructure collapse.

The test proceeds in phases:
1. Start with full 20-node grid, wait for convergence
2. Remove 5 nodes (simulate power outage in one area)
3. Measure re-convergence time
4. Remove 5 more nodes (further degradation)
5. Assert: remaining connected component still delivers messages internally

**Why:** Emergency communication is qaul's primary use case. This test directly
validates whether the network degrades gracefully or catastrophically when nodes
disappear. Route expiration timing (N+1 × 10s) becomes critical here.

---

### 7.4 Topology summary and expected convergence times

Based on the routing protocol: convergence requires one RouterInfo cycle (10s) per
hop to propagate discovery. Full convergence = `diameter × 10s + safety_margin`.

| Topology | Nodes | Diameter | Expected convergence | Feed delivery total wait |
|----------|-------|----------|---------------------|--------------------------|
| line-5 | 5 | 4 hops | ~60s | 120s (discovery) + 30s |
| line-10 | 10 | 9 hops | ~120s | 200s + 30s |
| grid4-3x3 | 9 | 4 hops | ~60s | 90s + 30s |
| grid4-5x5 | 25 | 8 hops | ~110s | 180s + 30s |
| grid8-4x4 | 16 | 3 hops | ~50s | 80s + 30s |
| two-clusters-bridge | 9 | 5 hops | ~70s | 100s + 30s |
| village-20 | 20 | ~7 hops | ~90s | 150s + 30s |
| street-corridor-15 | 15 | ~7 hops | ~90s | 150s + 30s |
| sparse-rural-12 | 12 | ~4 hops | ~60s | 100s + 30s |
| event-dense-25 | 25 | 4 hops | ~60s | 90s + 30s |

These are estimates. All values should be measured empirically and the actual timings
recorded in a results file (see section 9).

---

## 8. Scalability Plan

The goal is to both achieve and prove scalability. The plan has three stages.

### Stage 1: Correctness across topology diversity (current work)

Run the existing correctness tests (node startup, user discovery, feed delivery)
across all defined topologies. This establishes a baseline:
- Does qaul work on a grid? On a sparse topology? On a cluster?
- Are there topologies where delivery fails entirely?

Expected outcome: feed delivery will fail on topologies where convergence time exceeds
the current wait times. Tests will be updated with topology-appropriate wait times.

### Stage 2: Routing protocol metrics across topologies

Run routing protocol tests (section 5.2) across topologies and measure:

**Metric 1: Convergence time**
- For each topology, record the time from startup until all nodes appear in all other
  nodes' routing tables
- Plot convergence time vs. network diameter
- Expected: linear relationship (diameter × ~10s)
- If non-linear: investigate bottleneck (likely mDNS peer discovery on first hop)

**Metric 2: Feed delivery success rate**
- Send 10 messages from the node with the highest eccentricity (farthest from everyone)
- Count how many arrive at all other nodes within the expected window
- A message counts as delivered only if it appears on 100% of reachable nodes

**Metric 3: Route stability**
- Run each topology for 10 minutes after convergence
- Count route flaps (a route that disappears and reappears counts as one flap)
- Record RTT variance for 1-hop and multi-hop routes

**Metric 4: Routing overhead**
- Use `tc -s qdisc show` before and after a 60-second idle window post-convergence
- Calculate bytes/second/node attributable to RouterInfo broadcasts
- Compare across topologies to see how it scales with node count

### Stage 3: Large-scale testing (100+ nodes)

Once stage 2 is complete and any bugs are fixed, scale up:

**50-node test:** `grid4-7x8.json` or similar. Expected convergence ~130s.
The routing table on each node will contain 49 entries. This is the first scale
where routing table size may begin to matter for memory.

**100-node test:** `grid4-10x10.json`. 100 nodes, ~18 hop diameter. Expected
convergence ~200s. RouterInfo messages will be ~5KB each. At 2 neighbours × 1
message/10s, this is ~1KB/s routing overhead per node.

**Failure mode investigation:** At what node count does qaul start to fail?
Expected failure modes:
- mDNS peer discovery becomes slow when many nodes are on the same broadcast domain
  (meshnet-lab puts all nodes in the same LAN subnet)
- RouterInfo messages become large enough to exceed the libp2p message size limit
- Route expiration races with convergence — on a 10-hop line, hop-10 routes expire
  in 110s, which is close to the convergence time of ~90s. If RouterInfo propagation
  is delayed, routes expire before re-advertisement

The meshnet-lab server has sufficient RAM and CPU for 100+ nodes (each qauld instance
is lightweight). The limiting factor is likely the routing protocol design, not hardware.

---

## 9. Findings Documentation

### 9.1 Results file format

All test runs should be documented in `tests/integration/results/`. Each file is named
`YYYY-MM-DD-<topology>-<test>.json` and contains:

```json
{
  "date": "2026-03-13",
  "topology": "line-5",
  "test": "route_discovery",
  "qaul_commit": "<git sha>",
  "measurements": {
    "convergence_time_s": 87,
    "first_hop_discovery_s": 12,
    "last_hop_discovery_s": 87
  },
  "pass": true,
  "notes": "cold start, no prior data"
}
```

This format allows:
- Tracking improvement over time (compare same topology at different commits)
- Identifying regressions (if convergence_time_s increases after a commit)
- Building plots of scalability (convergence_time_s vs. node_count across topologies)

### 9.2 Findings so far (as of 2026-03-13)

| Finding | Impact | Improvement candidate |
|---------|--------|-----------------------|
| Feed messages dropped if sender not known | High — messages sent before convergence are permanently lost | Buffer for unknown senders; retry on user discovery |
| Convergence is purely periodic (no triggers) | Medium — 10s interval means minimum 10s delay per hop | Triggered RouterInfo on new peer detection |
| RouterInfo size scales linearly with node count | Medium — overhead grows as network grows | Partial table updates; bloom filter for large networks |
| Users module `request_id` not propagated to response | Bug (fixed) — all users list commands hung | Fixed: pass request_id through send_users_rpc_message |
| Route expiration uses hop-count-weighted timeout | Design tradeoff — distant routes expire faster, may cause flapping in large networks | Configurable per-module expiration thresholds |

### 9.3 System improvements identified

Based on the above findings, the following improvements are worth investigating.
These are documented in `docs/research/.improvements.md` and referenced here:

1. **Polling vs. triggered route advertisement** — immediate RouterInfo on new peer
   discovery would dramatically reduce cold-start convergence time, especially in
   dense topologies where all nodes start simultaneously.

2. **Message buffering for unknown senders** — feed messages dropped due to unknown
   sender are permanently lost. A short buffer (e.g. 30s, cleared on user discovery
   or timeout) would dramatically improve delivery on cold-start networks.

3. **Floodsub adequacy at scale** — floodsub sends to all direct subscribers. On a
   high-degree node (degree 8 in grid8), one feed message generates 8 outgoing copies.
   This is O(degree) per hop, which is fine for sparse topologies but should be
   measured at high node counts.

4. **Configurable RouterInfo interval** — 10s is good for real-time operation but
   means tests must wait 10s per hop. A configurable interval (passed as a CLI flag
   to qauld) would allow faster test runs and more granular timing experiments.

5. **Pre-warmed topology option** — for tests that don't care about cold-start
   behaviour, a `--preload-peers <file>` flag on qauld could load a list of known
   peers at startup, bypassing mDNS discovery and getting to a converged state faster.

---

## 10. Implementation Order (Updated)

1. ✅ Build qauld-ctl, fix PID guard, verify meshnet-lab + qaul
2. ✅ Add `--json` flag to all qauld-ctl commands
3. ✅ Write `lib/node.py`, `lib/network.py`, `run.py`
4. ✅ Get correctness tests passing on `line-5`: node startup, user discovery, feed routing
5. ✅ Implement `router` subcommand in qauld-ctl
6. ✅ **Generate topology files** for all topologies in section 7.3
7. **Run correctness tests across all topologies** — update wait times per topology
8. **Write routing protocol tests** (`test_route_discovery.py`, `test_route_stability.py`,
   `test_route_expiration.py`, `test_neighbour_detection.py`)
9. **Write network options tests** (`test_direct_messages.py`, `test_feed_distribution.py`)
10. **Write scenario tests** (`test_two_clusters_partition.py`, `test_disaster_degrading.py`)
11. **Measure baseline metrics** on all topologies using `tc` statistics
12. **Scale to 50 nodes**, measure convergence time, compare to model
13. **Scale to 100 nodes**, identify failure modes
14. **Write results files** for each topology+test combination
15. **Implement improvements** from section 9.3 based on findings, re-run to measure impact

---

## 11. Key Timing Constants (Summary)

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
