# Topologies

Each topology file is a meshnet-lab JSON file that defines a simulated mesh network.
The files are used by `run_topologies.py` to run correctness tests across different
network shapes.

---

## line-5

```
0000 — 0001 — 0002 — 0003 — 0004
```

5 nodes in a straight chain. Each node only talks to its immediate left and right
neighbour. The endpoints (0000 and 0004) have only 1 neighbour each; the middle
nodes have 2. Diameter is 4 hops — to get a message from 0000 to 0004 you must
pass through every single intermediate node.

There is exactly one path between any two nodes. If any middle node fails the
network splits permanently. This is the baseline topology used for all initial
correctness tests.

- **Nodes:** 5
- **Diameter:** 4 hops
- **Redundancy:** none
- **discovery_wait:** 120s

---

## line-10

```
0000 — 0001 — 0002 — 0003 — 0004 — 0005 — 0006 — 0007 — 0008 — 0009
```

Same structure as line-5 but stretched to 10 nodes. Diameter is 9 hops — the
longest diameter of any topology in this set, which is why it has the longest
discovery wait (200s). The routing information has to relay through 8 intermediate
nodes one hop at a time, each hop taking one full 10s RouterInfo cycle.

No redundancy anywhere — a single node failure splits the chain. The extra nodes
also mean each node's routing table is larger (knows 9 others vs 4), so RouterInfo
messages are slightly bigger.

- **Nodes:** 10
- **Diameter:** 9 hops
- **Redundancy:** none
- **discovery_wait:** 200s

---

## grid4-3x3

```
0000 — 0001 — 0002
  |      |      |
0003 — 0004 — 0005
  |      |      |
0006 — 0007 — 0008
```

9 nodes in a square grid, connected only horizontally and vertically — no diagonals.
That is what "grid4" means: 4 directions. Corner nodes have 2 neighbours, edge nodes
have 3, the centre node has 4.

Diameter is 4 hops — from corner 0000 to opposite corner 0008 you need to go 2
steps right and 2 steps down. Same diameter as line-5 despite having nearly twice
the nodes, because the grid shape provides shortcuts. This is the first topology with
real redundancy: if one path breaks, routing can go around it via the other side of
the grid.

- **Nodes:** 9
- **Diameter:** 4 hops
- **Redundancy:** multiple paths between most pairs
- **discovery_wait:** 120s

---

## grid4-5x5

```
0000 — 0001 — 0002 — 0003 — 0004
  |      |      |      |      |
0005 — 0006 — 0007 — 0008 — 0009
  |      |      |      |      |
000a — 000b — 000c — 000d — 000e
  |      |      |      |      |
000f — 0010 — 0011 — 0012 — 0013
  |      |      |      |      |
0014 — 0015 — 0016 — 0017 — 0018
```

25 nodes in a 5×5 grid, still only 4-connected. Diameter is 8 hops — corner 0000
to corner 0018 requires 4 steps right and 4 steps down.

The largest and slowest topology in the set because of this large diameter combined
with the high node count. Each node's routing table holds 24 entries, so RouterInfo
messages are the largest here. High redundancy — interior nodes have 4 neighbours so
there are many alternative routes. This is the most realistic model of a
medium-density village or neighbourhood.

- **Nodes:** 25
- **Diameter:** 8 hops
- **Redundancy:** high — many alternative routes
- **discovery_wait:** 180s

---

## grid8-4x4

```
0000 — 0001 — 0002 — 0003
  |  ╲  |  ╲  |  ╲  |
0004 — 0005 — 0006 — 0007
  |  ╲  |  ╲  |  ╲  |
0008 — 0009 — 000a — 000b
  |  ╲  |  ╲  |  ╲  |
000c — 000d — 000e — 000f
```

16 nodes in a 4×4 grid where every node also connects to its diagonal neighbours —
that is what "grid8" means: 8 directions. The diagonal connections are what make
this fundamentally different from grid4.

Diameter is only 3 hops — from corner 0000 to opposite corner 000f you can travel
diagonally: 0000 → 0005 → 000a → 000f. Three steps instead of six. This is why it
converges fastest (90s) despite having more nodes than grid4-3x3. Interior nodes
have up to 8 neighbours, making this the most densely connected topology. It models
a crowded scenario like a festival or dense urban block where every device can see
many others directly.

- **Nodes:** 16
- **Diameter:** 3 hops
- **Redundancy:** very high — up to 8 neighbours per node
- **discovery_wait:** 90s

---

## Summary

The key insight: **diameter beats node count**. grid8-4x4 has 16 nodes but converges
in 90s. line-10 has only 10 nodes but takes 200s — because routing information must
relay through 9 sequential hops with no shortcuts. Adding diagonal connections
(grid4 → grid8) cuts the diameter in half and nearly halves the convergence time
even with the same number of nodes.

| Topology   | Nodes | Diameter | Why that diameter                  | discovery_wait |
|------------|-------|----------|------------------------------------|----------------|
| grid8-4x4  | 16    | 3 hops   | diagonal shortcuts across 4×4      | 90s            |
| line-5     | 5     | 4 hops   | straight chain, no shortcuts       | 120s           |
| grid4-3x3  | 9     | 4 hops   | 2 steps × 2 steps in grid          | 120s           |
| grid4-5x5  | 25    | 8 hops   | 4 steps × 4 steps in grid          | 180s           |
| line-10    | 10    | 9 hops   | straight chain, maximum stretch    | 200s           |

Node IDs are zero-padded hexadecimal — `000f` is node 15, `0018` is node 24.
This is meshnet-lab's naming convention and determines the socket path each node
listens on: `/tmp/qaul-<id>/qauld.sock`.
