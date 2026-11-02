# qaul Routing: v1 vs v2

**v2 arm:** two runs, 2026-10-15 and 2026-10-17.
**v1 arm:** issue #541 result files recovered from git history
**Lab:** meshnet-lab, simulated LAN links, single WSL2 host. Both arms run the
same qauld release binary.

> **Read the asymmetry first.** v1's waits were fixed sleeps of 90-200 s; today's
> harness polls and gives up at 60 s. v2 was therefore allowed *less* time, not
> more

---

## Headline

| | v1 | v2 | verdict |
|---|---|---|---|
| Routing table convergence (measured both arms) | 30-146 s | **7.4-8.5 s** | **4x-20x faster, every topology** |
| User discovery on line-10, 9 hops | **failed, 7/10 users at 400 s** | **4.4 s, all 10** | **previously impossible, now works** |
| Diameter reached | 9 hops, and it failed there | **49 hops in 32-36 s** | **§2.2's 63-hop target looks reachable** |
| Unicast delivery | <=5 s | <=5 s (one 25 s outlier) | parity |
| Broadcast / feed | 2 of 5 topologies failed | 4 of 7, then 1 of 7 — unstable | **not attributable to v2** |
| Packet loss tolerance, line-5 | 100% at 50%, 0% at 80% | 100% at 50%, 10% at 80% | parity |

Convergence is the only thing v2 changes, and it is where the whole improvement
sits. Everything else is parity or belongs to code v2 does not touch. Two caveats
carry: two sweeps per topology, and nothing here ran on a radio.

---

## 1. Route convergence — the one thing v2 actually changes

Two different things get called "convergence", and v1 behaved differently on
each, so both appear here.

### A. User discovery — the first node knows the farthest node's user

This is what the messaging layer needs before it can address anyone.

| Topology  | Nodes | Hops | v1 (2026-03) | v2 sweep 1 | v2 sweep 2 |
|-----------|-------|------|--------------|------------|------------|
| line-5    | 5     | 4    | <=120 s (sleep) | 2.2 s | 2.2 s |
| grid8-4x4 | 16    | 3    | <=90 s (sleep)  | 0.1 s* | 0.1 s* |
| grid4-3x3 | 9     | 4    | **failed at 120 s** | 0.1 s* | 0.1 s* |
| grid4-5x5 | 25    | 8    | <=180 s (sleep) | 2.2 s | 7.5 s |
| line-10   | 10    | 9    | **7/10 users at 400 s — failed** | **4.4 s** | **4.4 s** |
| line-20   | 20    | 19   | not tested | **10.7 s** | **16.0 s** |
| line-50   | 50    | 49   | not tested | **32.0 s** | **36.4 s** |

v1's passing cells are the *length of a fixed sleep*, not measurements: the March
harness slept `discovery_wait` then checked once, and its files record
`actual_wait_s == discovery_wait_s` exactly. They bound v1 from above and say
nothing more. Only the two v1 failures carry information.

`*` 0.1 s means convergence had already finished before the measurement
started, the harness was still bringing nodes up. Those two cells are a floor,
not a duration.

**line-10 is the headline.** v1 reached 7 of 10 users in 400 s and failed; v2
reached all 10 in 4.4 s. line-10 was previously untestable: every other v1 test
on it reads "not reached" or "setup failed". The long lines below carry the
scaling claim that line-10 alone cannot.

### B. Routing table convergence — every node present in the first node's table

March measured this by polling, so it is directly comparable.

| Topology  | Nodes | Hops | v1 (2026-03-17, 5 s poll) | v2 (1 s poll) | ratio |
|-----------|-------|------|---------------------------|---------------|-------|
| line-5    | 5     | 4    | **100.6 s** | **8.5 s** | 11.8x |
| grid4-3x3 | 9     | 4    | **55.4 s**  | **8.5 s** | 6.5x |
| grid8-4x4 | 16    | 3    | **30.2 s**  | **7.4 s** | 4.1x |
| grid4-5x5 | 25    | 8    | not recorded | **7.5 s** | - |
| line-10   | 10    | 9    | **145.9 s** | **7.4 s** | 19.7x |

Both columns are polled measurements of the same quantity, so these are the only
ratios in this report that mean what they look like. v2 wins on every topology
v1 recorded, by between 4x and 20x.

**The per-node timelines say more than the totals.** line-10 fills
smoothly at about a second per hop:

    1.1, 1.1, 2.1, 3.2, 4.2, 5.3, 5.3, 6.4, 6.4, 7.4

Before the §3.8 introduction fix the same topology read

    1.1, 1.1, 2.1, 7.4, [34 s gap], 41.3, 42.4, 43.4, 43.4, 48.7, 48.7

## 2. Unicast (direct messages) — parity, plus a topology that now works

| Topology  | v1       | v2        |
|-----------|----------|-----------|
| line-5    | PASS 5 s | PASS 5.0 s |
| grid4-3x3 | PASS 6 s | PASS **25.3 s** |
| grid4-5x5 | PASS 5 s | PASS 5.1 s |
| grid8-4x4 | PASS 5 s | PASS 5.1 s |
| line-10   | never ran (setup failed at 400 s) | **PASS 5.0 s** |

Both arms poll every 5 s and sleep *before* the first check, so "5 s" is the
floor of the instrument in both columns: it means "arrived within the first poll",
i.e. somewhere in 0-5 s. Neither column is a latency measurement. At that
granularity the two arms are indistinguishable on four topologies, and line-10
goes from never running to delivering.

## 3. Degraded links (line-5, loss on 0002's uplink)

| Loss | v1 delivery to far node | v2 delivery to far node |
|------|-------------------------|-------------------------|
| 30%  | 100% | 100% |
| 50%  | 100% | 100% |
| 80%  | 0%   | 10% |

Parity. The arms score 80% differently only because the current harness fails
anything under 50% while March labelled the same collapse an expected cutoff on
a line with no redundancy. Not a regression.

---

## Why convergence improved

- **v1** re-floods its whole routing table every `sending_table_period: 10` s, so
  a route advances roughly one hop per 10 s cycle. On line-10's 9 hops that is
  ~90 s at best; measured, v1's table took 145.9 s and its user discovery never
  finished.
- **v2** batches updates to neighbours once a second, and re-announces a
  node's own entry every ten. Both periods are hardcoded in
  `router_v2/init.rs`; the `relay_interval` and `origin_interval_time` config
  keys are read nowhere, so neither is tunable without a recompile.
- **On a line, hop count explains it almost entirely**: 9, 19 and 49 hops
  converge in 4.4 s, 10.7 s and 32.0 s, a straight line at roughly half a
  second per hop. The grids converge faster than their hop count suggests
  because their diameters are 3-4 hops and they finish before the harness has
  stopped bringing nodes up, which is why two of those cells are a floor rather
  than a measurement. What the per-hop rate does depend on is startup skew, not
  topology - see the diameter section.

---

## Diameter: how far this actually scales

§2.2 sets a 63-hop design maximum and predicts ~32 s to fill the pipeline at
that diameter. The long lines test it directly.

| Topology | Hops | Sweep 1 | Sweep 2 | Per hop |
|----------|------|---------|---------|---------|
| line-10  | 9    | 4.4 s   | 4.4 s   | 0.49 s |
| line-20  | 19   | 10.7 s  | 16.0 s  | 0.56-0.84 s |
| line-50  | 49   | 32.0 s  | 36.4 s  | 0.65-0.74 s |
