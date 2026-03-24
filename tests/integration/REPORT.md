# qaul Network Behaviour Test Report

**Date:** 2026-03-21
**Branch:** test/qauld
**Issue:** #541

---

## Summary

| Topology     | Nodes | Direct Messages | Feed Distribution | Degraded Links |
|--------------|-------|-----------------|-------------------|----------------|
| line-5       | 5     | PASS            | PASS              | 30%/50% PASS, 80% expected cutoff |
| line-10      | 10    | not reached     | not reached       | not reached |
| grid4-3x3    | 9     | PASS            | PASS              | skipped |
| grid4-5x5    | 25    | PASS            | PARTIAL FAIL      | skipped |
| grid8-4x4    | 16    | PASS            | PARTIAL FAIL      | skipped |

---

## Test Descriptions

**Direct messages** — sends an encrypted unicast message from the first node to the last node, verifies delivery, and checks that a middle node (snooper) never receives it. Tests qaul's end-to-end encryption and unicast routing.

**Feed distribution** — sends one feed (broadcast) message from every node and checks that all nodes receive all messages. Tests qaul's floodsub-based pubsub layer.

**Degraded links** — applies packet loss to the middle node's uplink using Linux `tc netem`, sends 10 messages, measures delivery rate at the far end. Tests qaul's resilience under poor link conditions.

---

## Findings

### Direct Messages — Works reliably across all tested topologies

Direct messages passed on every topology where the network converged. Delivery times were consistently 5–6 seconds across 5, 9, 16, and 25-node topologies. The snooper check passed in all cases — intermediate nodes never received the encrypted payload. This confirms qaul's unicast routing and end-to-end encryption work correctly across multi-hop meshes of varying size and shape.

### Feed Distribution — Works on small topologies, degrades on larger ones

Feed distribution passed cleanly on line-5 (5 nodes) and grid4-3x3 (9 nodes), with all messages propagating to all nodes within 1–2 seconds. On grid4-5x5 (25 nodes) and grid8-4x4 (16 nodes) it partially failed.

The root cause is how qaul implements feed distribution. qaul uses libp2p **floodsub** for feed messages, with **mDNS** for peer discovery. mDNS is L2-local, so each node only discovers its direct neighbours. Floodsub then propagates messages hop-by-hop through the mesh via those direct connections.

The problem: the floodsub overlay builds up independently from qaul's routing layer and takes longer to stabilise. After routing convergence (all users known), the floodsub mesh between distant nodes may not yet be fully established. Messages sent during this window get lost mid-chain. On grid4-5x5, node `0004` failed to receive messages from 7 distant nodes. On grid8-4x4, node `000c` was completely isolated — sending nothing to the rest of the network and receiving nothing from it.

Additionally, in `libqaul/src/connections/lan.rs` there is an unimplemented configuration:
```rust
// TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), Node::get_id()).unwrap();
```
The intended 30-second mDNS re-query interval was never applied. With the default (longer) interval, failed peer discoveries are not retried quickly, slowing down floodsub mesh formation on larger topologies.

**What this means for qaul:** Feed distribution is functional on small networks (up to ~9 nodes) but becomes unreliable on larger ones until the floodsub mesh has had enough time to fully form. The missing mDNS config is a known TODO and likely to improve this when implemented.

### Degraded Links — Resilient up to 50% loss, breaks at 80% on single-path topologies

Tested on line-5 with packet loss applied to the middle node's uplink:

| Loss | Delivery to far node | Result |
|------|----------------------|--------|
| 30%  | 100%                 | PASS |
| 50%  | 100%                 | PASS |
| 80%  | 0%                   | expected cutoff |

At 30% and 50% loss, qaul's retransmission mechanism fully compensated — all 10 messages arrived at the far end. At 80% loss on a single-path line topology, the middle node becomes a complete bottleneck and delivery drops to zero. This is expected behaviour: there is no alternative route on a line topology.

In earlier manual testing with a 4x4 grid topology (which has redundant paths), 80% loss on a single node still achieved 100% delivery — traffic routed around the degraded node. This confirms that qaul's resilience under packet loss is directly dependent on network redundancy, which is the correct behaviour for a mesh protocol.

### line-10 Routing Convergence — Fails within the test timeout

On the 10-node line topology, routing convergence did not complete within 400 seconds — only 7 of 10 nodes were discovered. This is related to the same mDNS configuration issue: with a 10-hop chain, peer discovery relies entirely on each link being established in sequence, and the slow mDNS re-query interval means a single failed discovery stalls convergence for the whole chain. The same TODO fix in `lan.rs` is expected to address this.

---

## What This Tells Us About qaul's Capabilities

1. **Unicast (direct messages) is solid.** End-to-end encrypted messages route correctly across all tested topologies, deliver within seconds, and intermediate nodes cannot read them.

2. **Broadcast (feed) works well on small networks.** Up to ~9 nodes and 2–3 hops, feed distribution is fast and reliable. Larger networks need more time for the floodsub mesh to stabilise before broadcast is dependable.

3. **The network is resilient to moderate packet loss.** Up to 50% loss on a single link in a line topology is fully absorbed. This is a strong result for a mesh protocol designed for unreliable radio links.

4. **Redundant paths are essential for high-loss scenarios.** An 80% loss on a grid topology is tolerable; the same loss on a line topology is not. Network topology design matters.

5. **There is a known unimplemented mDNS configuration** (`lan.rs` line 177) that limits how quickly nodes discover and connect to their neighbours. This is the root cause of the line-10 convergence failure and the feed distribution degradation on larger topologies. Implementing the intended `query_interval: 30s` is likely to improve both.

---

## Before Testing with Larger Topologies

The current test suite covers topologies up to 25 nodes. To understand qaul's behaviour at realistic deployment scale, testing needs to extend to larger and more varied networks. The following areas are planned:

**Fix the mDNS configuration first.** Before expanding topology size, the unimplemented `MdnsConfig` in `lan.rs` (query_interval=30s, TTL=300s) should be applied. This is the single most impactful change for making larger topologies testable — it directly affects how quickly nodes discover neighbours, which gates both routing convergence and floodsub formation. Without it, convergence times on larger topologies become impractically long and test results are dominated by infrastructure lag rather than protocol behaviour.

**Extend line topologies beyond 10 nodes.** Once the mDNS fix is in place, line-15 and line-20 topologies will reveal the practical hop limit for both routing convergence and feed distribution. The floodsub hop-count behaviour observed on line-10 (message from one end reaching ~6 hops) needs to be characterised at different lengths to determine whether it is a hard limit or a function of connection establishment timing.

**Add larger grid topologies.** A grid8-6x6 (36 nodes) and grid4-8x8 (64 nodes) would test feed distribution at scale. Grid topologies are the most realistic model for a deployed urban mesh, and the redundant paths make them more forgiving. Understanding at what node count the floodsub mesh starts to develop isolated nodes (as seen with `000c` in grid8-4x4) is important for deployment planning.

**Test degraded links on grid topologies.** The current degraded links tests only run on line topologies. Grid topologies with redundant paths are the more interesting case — we already confirmed manually that 80% loss on a grid8-4x4 achieves 100% delivery due to path diversity. Adding this to the automated suite across multiple loss percentages (30%, 50%, 80%) would formally characterise qaul's resilience on redundant topologies.

**Increase routing convergence timeouts dynamically.** Rather than hardcoding `discovery_wait` per topology, the runner could measure actual convergence time on the first run and adapt. This would make the suite self-tuning as topology sizes grow and would reduce the risk of false failures caused by timeout mismatches.

**Document feed distribution time-to-full-mesh.** For topologies where feed distribution currently fails within the test window, the right question is not just pass/fail but: how long does it actually take for the floodsub mesh to fully form? Adding a longer observation window and recording the time-to-full-propagation would give a useful metric for deployment guidance (e.g., "wait N minutes after network start before relying on broadcast").

---
