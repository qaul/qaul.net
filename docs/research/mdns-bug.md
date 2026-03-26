# mDNS Configuration Bug: Deep Analysis

**Date:** 2026-03-25
**Status:** Unfixed (TODO in source)
**Impact:** Blocks integration test scaling beyond ~25 nodes
**Location:** `rust/libqaul/src/connections/lan.rs:177-178`

---

## Summary

The LAN module uses `mdns::Config::default()` instead of the intended custom configuration. The libp2p-mdns 0.48.0 defaults set `query_interval` to **5 minutes** (300s) and `ttl` to **6 minutes** (360s). The TODO in the code specifies `query_interval: 30s` and `ttl: 300s`, which were never applied. This 10x-slower rediscovery interval causes cascading convergence failures on any topology with more than ~4 hops.

---

## The Exact Code

```rust
// lan.rs:177-178
// TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), Node::get_id()).unwrap();
```

### libp2p-mdns 0.48.0 defaults (`~/.cargo/registry/.../libp2p-mdns-0.48.0/src/lib.rs:74-80`)

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(6 * 60),          // 360 seconds
            query_interval: Duration::from_secs(5 * 60), // 300 seconds (5 MINUTES)
            enable_ipv6: false,
        }
    }
}
```

### Intended values (from the TODO)

| Field | Default | Intended | Ratio |
|-------|---------|----------|-------|
| `query_interval` | 300s (5 min) | 30s | **10x too slow** |
| `ttl` | 360s (6 min) | 300s (5 min) | Minor difference |
| `enable_ipv6` | false | false | Same |

---

## Why It's a Blocker

### How mDNS drives the system

mDNS is the **sole discovery mechanism** on the LAN module. When a node starts, it broadcasts an mDNS query immediately. If a peer responds, `mdns::Event::Discovered` fires and the peer is added to the **floodsub partial view** (`lan.rs:88`):

```rust
mdns::Event::Discovered(discovered_list) => {
    self.floodsub.add_node_to_partial_view(peer_id);
}
```

If the initial mDNS query packet is lost (common during namespace startup), the next retry is **5 minutes later**. During that time:

- The link between two nodes is dead
- Routing information cannot propagate past that link
- All downstream nodes remain undiscovered
- Floodsub cannot deliver feed messages across that gap

### Evidence from integration test results

**line-10 discovery timeline** (`results/2026-03-17-routing-line-10.json`):

```
Hop 0:   5.0s    <- immediate
Hop 1:  15.1s    <- 10s RouterInfo cycle
Hop 2:  15.1s    <- same cycle
Hop 3:  25.1s    <- next cycle
Hop 4:  45.3s    <- slight delay
                  <- 85 SECOND GAP (mDNS retry at hop 5-6)
Hop 5-8: 130.8s  <- all appear at once after mDNS retry
Hop 9:  145.9s   <- final hop
```

The 85-second gap between 45.3s and 130.8s corresponds to an mDNS rediscovery cycle. With a 30s query_interval, this gap would be ~30s instead.

**Convergence overhead across topologies:**

| Topology | Theory (10s/hop) | Actual | Overhead | Cause |
|----------|-----------------|--------|----------|-------|
| line-5 (4 hops) | 40s | 100.6s | +60s | ~1 mDNS miss in chain |
| line-10 (9 hops) | 90s | >400s (FAIL) | +310s | Multiple mDNS misses cascade |
| grid4-3x3 (4 hops) | 40s | 55.4s | +15s | Redundant paths mask misses |
| grid8-4x4 (3 hops) | 30s | 50.4s | +20s | High connectivity compensates |

**line-10 user discovery**: consistently fails. Only 7 of 10 nodes discovered after 400 seconds in both the 2026-03-13 and 2026-03-23 test runs.

**grid8-4x4 feed distribution**: node `000c` is **completely isolated** in floodsub despite correct routing table entries and neighbour detection. This is because mDNS records expired on its neighbours, removing `000c` from their floodsub views. The 5-minute re-query never fired within the test window.

---

## The Floodsub/TTL Interaction

A second TODO at `lan.rs:93-96` reveals a related subtlety:

```rust
mdns::Event::Expired(expired_list) => {
    for (peer, _addr) in expired_list {
        // TODO: why to remove it at all? does it not get removed automatically, when disconnected?
        if !self.mdns.discovered_nodes().any(|p| p == &peer) {
            self.floodsub.remove_node_from_partial_view(&peer);
        }
    }
}
```

When an mDNS record expires (after `ttl` seconds), the peer is checked against the mDNS cache. If it's no longer there, it's removed from floodsub. With the current defaults (TTL=360s, query=300s), there's only a 60s margin. A single missed query causes the peer to be removed from floodsub even though it's still alive and connected at the transport level.

With the intended values (TTL=300s, query=30s), the margin is 270s — a peer would need to miss ~9 consecutive queries before floodsub removal. This makes the expiry path function as a dead-node detector rather than a normal-operation event.

---

## What This Does NOT Fix

Even with the mDNS configuration corrected, three secondary issues remain:

1. **Feed messages dropped for unknown senders** — If a feed message arrives before the sender's public key is in the user store, it is permanently discarded. No retry, no buffering. This means feed distribution requires full user-store convergence, not just routing convergence. (By design for security, but impacts testing.)

2. **Floodsub is unstructured flooding** — libp2p's `floodsub` forwards every message to every peer in the partial view. At 100+ nodes, this creates O(N) traffic per message. Not a correctness issue, but a performance concern at scale. The alternative (`gossipsub`) uses bounded-fanout mesh overlay.

3. **RouterInfo broadcasts grow linearly** — Each RouterInfo exchange sends the full routing table. At 100 nodes, that's ~5KB per exchange, every 100ms, to every neighbour. On a grid8 node with 8 neighbours, that's ~40KB/s of routing overhead.

None of these prevent convergence. The mDNS issue does.

---

## Fix Plan

### Step 1: Apply the mDNS Config (trivial, high impact)

Replace `lan.rs:177-178` with:

```rust
let mdns_config = mdns::Config {
    ttl: Duration::from_secs(300),
    query_interval: Duration::from_secs(30),
    enable_ipv6: false,
};
let mdns = mdns::tokio::Behaviour::new(mdns_config, Node::get_id()).unwrap();
```

**Why these values:**

- `query_interval: 30s` — 10x faster rediscovery. Matches RouterInfo interval (10s) in order of magnitude. Fast enough to recover from packet loss within one routing cycle. Not so fast that it floods the network with mDNS traffic.
- `ttl: 300s` — Records live 5 minutes. Since queries happen every 30s, records are refreshed ~10 times before expiry. A peer would need to miss 10 consecutive queries before being removed from floodsub.
- `enable_ipv6: false` — Unchanged. meshnet-lab namespaces use IPv4.

### Step 2: Make mDNS Config Configurable (optional, medium effort)

Add fields to `LanConfig` in `storage/configuration.rs`:

```rust
pub struct LanConfig {
    pub active: bool,
    pub listen: Vec<String>,
    pub mdns_query_interval: u64,  // seconds, default 30
    pub mdns_ttl: u64,             // seconds, default 300
}
```

Then in `lan.rs`, read from config:

```rust
let config = Configuration::get();
let mdns_config = mdns::Config {
    ttl: Duration::from_secs(config.lan.mdns_ttl),
    query_interval: Duration::from_secs(config.lan.mdns_query_interval),
    enable_ipv6: false,
};
```

This allows tests to crank `query_interval` down to 5s for fast convergence, or production deployments on battery-powered devices to use 60s.

### Step 3: Upgrade Expiry Logging

Change `log::trace!` to `log::info!` for floodsub peer removal events at `lan.rs:98`, so mDNS-driven floodsub removals are visible in test logs without enabling trace-level logging.

### Step 4: Validate with Existing Tests

Re-run integration tests after the fix and compare:

| Test | Before Fix | Expected After Fix |
|------|-----------|-------------------|
| line-5 routing convergence | 100.6s | ~50-60s |
| line-10 routing convergence | 145.9s (incomplete users) | ~100-120s (complete) |
| line-10 user discovery | FAIL (7/10 after 400s) | PASS (10/10 within 200s) |
| grid8-4x4 feed distribution | FAIL (node 000c isolated) | PASS (no isolation) |
| grid4-5x5 feed distribution | FAIL (7 msgs missing) | PASS or near-pass |

### Step 5: Scale Testing

With the fix, estimated convergence times:

| Topology | Nodes | Diameter | Est. Convergence |
|----------|-------|----------|------------------|
| grid8-10x10 | 100 | 9 hops | ~120s |
| grid8-15x15 | 225 | 14 hops | ~180s |
| grid4-20x20 | 400 | 38 hops | ~450s |
| grid8-32x32 | 1024 | 31 hops | ~370s |

Formula: `convergence ~ diameter x 10s + 30s (one mDNS miss margin)` instead of `diameter x 10s + 300s`.

---

## Related Files

- **Bug location:** `rust/libqaul/src/connections/lan.rs:177-178`
- **libp2p-mdns source:** `~/.cargo/registry/.../libp2p-mdns-0.48.0/src/lib.rs:74-80`
- **Config system:** `rust/libqaul/src/storage/configuration.rs`
- **Test report:** `tests/integration/REPORT.md`
- **Test results:** `tests/integration/results/2026-03-17-routing-line-10.json` (discovery timeline)
- **Testing research:** `docs/research/testing-framework.md`
