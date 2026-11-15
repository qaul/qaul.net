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

---

## Cross-community testing

Three topologies, each a set of LAN communities whose gateways peer with one hub over
Internet. A is the first community's last member, Z the last community's last member.

| Topology | Communities × members | Nodes | Farthest member from its gateway | Lab host |
|---|---|---|---|---|
| `communities-3x10` | 3 × 10 | 31 | 4 hops | WSL2 box |
| `communities-10x10` | 10 × 10 | 101 | 4 hops | WSL2 box |
| `communities-3x50` | 3 × 50 | 151 | 12 hops | test.qaul.net |

**Runs:** 2 per arm on each topology, all from commit `46800bd2`. Both arms of a topology
ran on the same host and binary.

> **How "reachable" is read.** v1 holds every user at every node, so "Z reachable from
> A" is read at A. A v2 member holds no foreign user (§2.3) and forwards to its
> gateway, so it is read as A routing its gateway *and* the gateway routing Z. Every
> time comes from a once-a-second poll of `users list` on every node, so it carries
> up to a second of error, the same in both arms.

### Headline

Each cell is v1 → v2 over the two runs, in seconds unless stated. Bold marks the
better arm.

| | 3x10 | 10x10 | 3x50 |
|---|---|---|---|
| Own community converged | 187–388 → **14–21** | 117–130 → **22** | 112–162 → **30** |
| Every gateway routes every user | 435–566 → **128** | 192–293 → **146–147** | 176–449 → 247 |
| Z reachable from A | 120–328 → **66–127** | 80–89 → **65–77** | **104–118** → 134 |
| Members holding foreign users | 27 of 27 → **0** | 90 of 90 → **0** | 147 of 147 → **0** |
| Crossing A→Z, invite to delivery | 64–79 → **3.1–3.2** | failed, 23 → **3.3** | 3.3–66 → **3.3–3.4** |
| A's gateway loses its hub link: noticed | 98–108 → **15–33** | 105–106¹ → **12–37** | never, stale → **12–36** |
| …back after the heal, A→Z / Z→A | 16 or >120 / 49–63 → **7** / 68 | 40–66 / 23–33 → **12–22** / 66–78 | n/a → **14–21 / 66–79** |
| Z leaves its community: A's side notices | 115¹ → **93–94** | 106¹ → **98–99** | never, stale → **101–102** |
| …back after the heal, A→Z / Z→A | 37–115 / 13–33 → **10–12 / 6** | 26 / 32¹ → **14 / 9–10** | n/a / **4** → **19–20** / 12–13 |
| Wire bytes sent, steady, KB/s | **25.9–27.6** → 39.3–40.5 (+50%) | **105–109** → 130–134 (+24%) | **191–194** → 355–365 (+87%) |
| Wire bytes sent, bring-up, KB/s | **34.8–35.5** → 56.0–56.5 (+60%) | **140–142** → 188–191 (+34%) | **270–272** → 1027–1056 (3.8×) |

¹ One valid v1 run. In the other, the route was already down when the cut began, or,
for the 10x10 member leave, the host slept through the cut.

v2 converges each community about 4–28× faster and contains every community, and it
crosses between communities in 3.1–3.4 s on all three topologies. v1 ranged from 3.3
to 79 s and failed once. v2 notices a gateway losing its hub link in 12–37 s, against
about 100 s for v1. On 3x50, v1 never noticed either cut: it kept stale routes through
both.

Two things go the other way as communities grow:

- **The directory.** v2 is bounded by §10.8's 60 s windows: 2 of them on 3x10, 4 on
  3x50. On 3x50 v2 makes Z reachable from A later than v1 does, 134 s against
  104–118 s.
- **The wire.** v2 costs more on every topology, and most on 3x50 (see
  [Why v2 costs more on the wire](#why-v2-costs-more-on-the-wire)).

### Full table

#### communities-3x10

| Measure | v1 | v2 |
|---|---|---|
| **Bring-up (seconds)** | | |
| Own community converged, slowest community (from bring-up) | 187 / 388 | 21 / 14 |
| Hub routes every user (from dial) | 143 / 304 | 126 / 126 |
| Every gateway routes every user (from dial) | 435 / 566 | 128 / 128 |
| Z reachable from A (from dial) | 120 / 328 | 127 / 66 |
| A reachable from Z (from dial) | 130 / 272 | 127 / 128 |
| Every member routes every foreign user (from dial) | 502 / 684 | contained / contained |
| Members holding foreign users before contact | 27 / 27 | 0 / 0 |
| A knows Z's name (from dial) | 120 / 327 | on contact, +0.0 / on contact, +0.0 |
| **Crossing, A to Z (seconds)** | | |
| Invite | 62 / 1.1 | 1.1 / 1.1 |
| Accept | 1.1 / 77 | 1.1 / 1.0 |
| Deliver | 1.0 / 1.1 | 1.0 / 1.0 |
| Total | 64 / 79 | 3.2 / 3.1 |
| Local control total | 1.1 / 1.1 | 1.0 / 0.0 |
| **A's gateway loses its hub link (seconds)** | | |
| A→Z lost after the cut | 102 / 98 | 15 / 16 |
| A→Z back after the heal | >120 / 16 | 7.5 / 6.6 |
| Z→A lost after the cut | 103 / 108 | 32 / 33 |
| Z→A back after the heal | 63 / 49 | 68 / 68 |
| **Z leaves its community (seconds)** | | |
| A→Z lost after the cut | down already / 115 | 93 / 94 |
| A→Z back after the heal | 115 / 37 | 9.7 / 12 |
| Z→A lost after the cut | 54 / 89 | 22 / 31 |
| Z→A back after the heal | 33 / 13 | 5.7 / 5.8 |
| **Wire, every node's uplink (sent)** | | |
| Bring-up to settle end, KB/s | 34.8 / 35.5 | 56.0 / 56.5 |
| Steady window, KB/s | 25.9 / 27.6 | 39.3 / 40.5 |
| Steady window, packets/s | 206 / 218 | 375 / 385 |
| Steady window, bytes per packet | 126 / 126 | 105 / 105 |
| Steady, per hub B/s | 1321 / 1383 | 1364 / 1356 |
| Steady, per gateway B/s | 1415 / 1417 | 1692 / 1740 |
| Steady, per member B/s | 751 / 815 | 1217 / 1256 |
| **router_v2 frames, steady (logged payload, no framing)** | | |
| Local sphere B/s | — / — | 730 / 729 |
| Internet sphere (cross-membrane) B/s | — / — | 33 / 33 |

#### communities-10x10

| Measure | v1 | v2 |
|---|---|---|
| **Bring-up (seconds)** | | |
| Own community converged, slowest community (from bring-up) | 117 / 130 | 22 / 22 |
| Hub routes every user (from dial) | 112 / 102 | 126 / 125 |
| Every gateway routes every user (from dial) | 293 / 192 | 147 / 146 |
| Z reachable from A (from dial) | 89 / 80 | 77 / 65 |
| A reachable from Z (from dial) | 90 / 49 | 126 / 67 |
| Every member routes every foreign user (from dial) | 512 / 350 | contained / contained |
| Members holding foreign users before contact | 90 / 90 | 0 / 0 |
| A knows Z's name (from dial) | 89 / 80 | on contact, +0.1 / on contact, +0.1 |
| **Crossing, A to Z (seconds)** | | |
| Invite | 1.1 / 1.1 | 1.1 / 1.1 |
| Accept | failed / 21 | 1.1 / 1.1 |
| Deliver | failed / 1.1 | 1.1 / 1.1 |
| Total | failed / 23 | 3.3 / 3.3 |
| Local control total | 1.2 / 1.2 | 1.1 / 1.1 |
| **A's gateway loses its hub link (seconds)** | | |
| A→Z lost after the cut | down already / 105 | 17 / 12 |
| A→Z back after the heal | 40 / 66 | 12 / 22 |
| Z→A lost after the cut | down already / 106 | 37 / 28 |
| Z→A back after the heal | 33 / 23 | 66 / 78 |
| **Z leaves its community (seconds)** | | |
| A→Z lost after the cut | invalid / 106 | 99 / 98 |
| A→Z back after the heal | invalid / 26 | 14 / 14 |
| Z→A lost after the cut | invalid / 73 | 12 / 21 |
| Z→A back after the heal | invalid / 32 | 10 / 8.6 |
| **Wire, every node's uplink (sent)** | | |
| Bring-up to settle end, KB/s | 142.4 / 140.0 | 190.9 / 187.6 |
| Steady window, KB/s | 104.5 / 108.6 | 130.4 / 133.5 |
| Steady window, packets/s | 709 / 722 | 1238 / 1273 |
| Steady window, bytes per packet | 147 / 150 | 105 / 105 |
| Steady, per hub B/s | 5627 / 5938 | 4723 / 4991 |
| Steady, per gateway B/s | 1688 / 1734 | 1733 / 1878 |
| Steady, per member B/s | 911 / 948 | 1204 / 1219 |
| **router_v2 frames, steady (logged payload, no framing)** | | |
| Local sphere B/s | — / — | 2399 / 2417 |
| Internet sphere (cross-membrane) B/s | — / — | 184 / 182 |

#### communities-3x50

| Measure | v1 | v2 |
|---|---|---|
| **Bring-up (seconds)** | | |
| Own community converged, slowest community (from bring-up) | 112 / 162 | 30 / 30 |
| Hub routes every user (from dial) | 176 / 118 | 245 / 245 |
| Every gateway routes every user (from dial) | 176 / 449 | 247 / 247 |
| Z reachable from A (from dial) | 104 / 118 | 134 / 134 |
| A reachable from Z (from dial) | 136 / 114 | 131 / 134 |
| Every member routes every foreign user (from dial) | 271 / 502 | contained / contained |
| Members holding foreign users before contact | 147 / 147 | 0 / 0 |
| A knows Z's name (from dial) | 104 / 118 | on contact, +0.1 / on contact, +0.1 |
| **Crossing, A to Z (seconds)** | | |
| Invite | 1.1 / 1.1 | 1.1 / 1.1 |
| Accept | 64 / 1.1 | 1.2 / 1.1 |
| Deliver | 1.1 / 1.1 | 1.1 / 1.1 |
| Total | 66 / 3.3 | 3.4 / 3.3 |
| Local control total | 3.2 / 0.8 | 3.3 / 2.7 |
| **A's gateway loses its hub link (seconds)** | | |
| A→Z lost after the cut | never (stale) / never (stale) | 12 / 12 |
| A→Z back after the heal | n/a / n/a | 14 / 21 |
| Z→A lost after the cut | never (stale) / never (stale) | 36 / 35 |
| Z→A back after the heal | n/a / n/a | 66 / 79 |
| **Z leaves its community (seconds)** | | |
| A→Z lost after the cut | never (stale) / never (stale) | 102 / 101 |
| A→Z back after the heal | n/a / n/a | 20 / 19 |
| Z→A lost after the cut | 166 / 101 | 22 / 26 |
| Z→A back after the heal | 4.4 / 4.0 | 12 / 13 |
| **Wire, every node's uplink (sent)** | | |
| Bring-up to settle end, KB/s | 272.2 / 269.6 | 1055.7 / 1026.6 |
| Steady window, KB/s | 193.6 / 191.2 | 354.5 / 364.6 |
| Steady window, packets/s | 1252 / 1231 | 3540 / 3667 |
| Steady window, bytes per packet | 155 / 155 | 100 / 99 |
| Steady, per hub B/s | 1651 / 1742 | 1498 / 1531 |
| Steady, per gateway B/s | 1579 / 1721 | 2405 / 2307 |
| Steady, per member B/s | 1273 / 1253 | 2353 / 2423 |
| **router_v2 frames, steady (logged payload, no framing)** | | |
| Local sphere B/s | — / — | 16018 / 16403 |
| Internet sphere (cross-membrane) B/s | — / — | 87 / 86 |

What the words in the cells mean:

- `>120`: the direction had not come back when the next cut began.
- "down already": not reachable when the cut began, so a loss time would mean nothing.
- "never (stale)": the route outlived a cut that removed the only path, so it was
  stale, and its recovery is "n/a".
- "invalid": the host slept through that phase.
- "failed": the crossing timed out at that leg.

### Why v2 costs more on the wire

The uplink counters count everything a node sends, framing included: libp2p, mDNS,
pings, the feed, management and routing. On 3x10 the hub sends the same in both arms,
about 1.35 KB/s. Gateways send about 20% more under v2, and members about 60% more.

v2 sends many more, much smaller messages:

- **v1** sends one full table per neighbour every 10 s (`sending_table_period: 10`),
  about 8.4 messages/s network-wide.
- **v2** sends about 28 frames/s (§7.1: one relay batch per second to every
  neighbour that has something to relay, plus each origin every 10 s). The average
  payload is 27 bytes.

Each v2 frame costs about 10 packets and 900 bytes, counting both ends, for a 27-byte
payload. 3x50, on the other host, gives the same: 9.9 packets and 911 bytes per frame,
for a 51-byte payload.

A packet capture confirmed the cause: every frame opens and closes a libp2p substream
of its own. Over 60 seconds on member 0005 of a 3x10 lab, its 57 logged sends matched
57 substreams on the wire, and its 67 logged receives matched 67. A frame over TCP took
9.8 packets and 933 bytes on average, about 536 from the sender and 397 from the
receiver, for a 28-byte average payload. One 33-byte frame broke down like this:

| Part | Packets | Bytes |
|---|---|---|
| Opening the substream and negotiating the protocol | 3 | 364 |
| The data | 1 | 129 |
| Closing it, one FIN each way | 2 | 192 |
| Bare ACKs | 4 | 264 |

These bursts were 61% of the bytes 0005 sent. In an earlier 10-second capture in
another lab, a link that ran over QUIC took 6 packets and about 575 bytes per frame.
Both captures are of one node.

**The gap depends on community size.** v2's frames grow with the community, since
every member relays every origin in it. v1's grow with the whole network, since every
neighbour gets the full table. So more communities favour v2, and bigger communities
favour v1:

- 10x10: +24%. v2's hub sends less than v1's, 4.7–5.0 against 5.6–5.9 KB/s.
- 3x50: +87%. v2's Local routing payload is 16 KB/s, against 0.73 KB/s on 3x10.

**Bring-up is mostly profile fetching.** Management traffic is 55% of v2's logged
bring-up bytes on 3x10, 62% on 10x10 and 85% on 3x50. It is mostly profile requests
and responses, relayed hop by hop as each member fetches every community-mate's
profile. On 3x50 that is 29 MB in about 25 seconds as the nodes start, peaking at
2.5 MB/s network-wide, with 4,378 fetch timeouts. Then it stops: there is none from
about four minutes on, and the cuts and heals add at most 1.5 KB/s for a few seconds.

v2's logged traffic over a whole 3x50 run (`…174401Z`), all 151 nodes together:

| After the nodes start | Management (profile fetches) | Routing, logged payload |
|---|---|---|
| 4–28 s | **29.4 MB in total**, up to 2.5 MB/s | about 30 KB/s |
| 30 s to 4 min | 0.6–5.7 KB/s | 15–35 KB/s |
| after about 4 min | **0** | 15.2 KB/s, flat |
| during cuts and heals | at most 1.5 KB/s, for a few seconds | 15–22 KB/s |

Each member fetches each community-mate's profile once and keeps it, so cuts and
heals do not set the burst off again. It should recur whenever many nodes meet for
the first time, such as two large partitions merging; that was not measured. At the
peak each node sends about 17 KB/s.

### Where v2 is stronger

- **Crossing between communities.** 3.1–3.4 s on every topology, about a second per
  leg, in all 6 runs. v1 had a slow leg or failed in 5 of its 6 runs: 64 and 79 s on
  3x10, a failure and 23 s on 10x10, 66 and 3.3 s on 3x50.
- **Containment.** No member holds a foreign user on any topology, where every v1
  member holds all 31, 101 or 151. A v2 member's routing state grows with its
  community, not the network. Only 33–184 B/s of routing crosses the membrane.
- **Convergence inside a community.** 14–30 s against 112–388 s, 4–28× faster.
- **The directory, on shallow communities.** Every gateway routes every user after
  128 s against 435–566 s on 3x10, and 146–147 s against 192–293 s on 10x10. Z is
  reachable from A after 65–127 s, against 80–328 s.
- **Noticing failures.** A lost hub link is noticed in 12–37 s, against 98–108 s for
  v1. A departed member is noticed in 93–102 s, against 106–115 s. On 3x50 v1 noticed
  neither cut: its stale routes would have lost messages silently. Its routes also
  flap: in one 3x10 run, A's route to Z dropped four times in the 13 minutes before
  the crossing.
- **Coming back.** After a gateway heal, A reaches Z again in 7–22 s, against 16–66 s
  or more than 120 s. After a member rejoins, in 10–20 s, against 26–115 s.
- **Repeatability.** Convergence and directory times agree within a second or two
  between runs, such as 128 / 128 s and 247 / 247 s. v1's vary by minutes, such as
  435 / 566 s and 176 / 449 s.
- **The hub, with many communities.** On 10x10 v2's hub sends less than v1's, 4.7–5.0
  against 5.6–5.9 KB/s, because it no longer carries every user's table.
- **Names on demand.** A has Z's name within 0.1 s of first contact, without holding
  every user's profile in advance.

### Where v1 is stronger

- **Wire cost, on every topology.** v2 sends 24–87% more in steady state and 34% to
  3.8× more at bring-up, in 1.8–2.9× as many packets.
- **Deep communities.** On 3x50, v1 made Z reachable from A sooner, 104–118 s against
  134 s. In one run its directory was also complete sooner, 176 s against 247 s.
- **Some recoveries.** After A's gateway gets its hub link back, the far side sees A
  again in 23–63 s under v1, against 66–79 s under v2. On 3x50, Z regains A 4 s after
  rejoining under v1, against 12–13 s under v2.
- **Simplicity** (not measured). v1 has one mechanism. v2 adds gateways, manifests,
  delegation and a management protocol, and each has needed fixes during this
  testing.

### v2's downsides

1. **About 900 wire bytes per routing frame**, whatever its payload. This is most of
   the steady gap. The cause, one substream per message, was confirmed by a packet
   capture.
2. **The bring-up burst:** 29 MB in 25 s on 3x50, about 17 KB/s per node at the peak.
3. **A directory that moves in 60 s steps** (§10.8): 128, 147, then 247 s as
   communities get deeper, because far members wait for their gateway's key at every
   hop.
4. **iRecoveries tied to the same steps.** After a gateway heal, the other side waits
   66–79 s. A member who leaves is noticed only after 93–102 s. 60 s of that is a grace
   the spec does not define, and it can conflict with §10.4.
5. **No browsing of other communities.** A member cannot list foreign users, so
   reaching someone elsewhere needs their ID. This is by design (§2.3), but users will
   notice.

### Was v2 worth the upgrade?

Yes, for what routing is for, with one condition. v2 is better at delivery,
containment, convergence and noticing failures on every topology. Those are things v1
cannot get without a redesign, because it hands every user to every node by
construction. v1's advantages are wire cost and, in deep communities, a little latency.

Most of v2's costs come from the implementation, not the protocol. Its routing payload
is small: 0.76, 2.6 and 16 KB/s across whole networks of 31, 101 and 151 nodes. The
per-frame cost and the profile burst are both things the code does, and both look
fixable. So does the far members' wait for their gateway's key, which should bring the
directory down to one window, about 65 s. Fixing the per-frame cost alone would likely
bring v2's steady traffic below v1's as v1 runs today. v1 sends through the same code,
though, so the fix would make v1 cheaper too, by less, since v1 sends fewer and larger
messages. How the two compare once both are fixed has not been estimated. All of this
is estimated, not measured.

**The condition:** every run here was in the lab, on simulated links with no bandwidth
limit, two runs per arm. Wire cost and the bring-up burst matter most on slow links, so
both need fixing before v2 is relied on over BLE, which has not been tested.

### Possible fixes

None is built. Each was checked against the routing spec, and each needs a lab rerun
against the numbers above. In order of payoff:

**1. About 900 wire bytes per message.** Routing, management and chat all send through
libp2p's `OneShotHandler` (`rust/libp2p_modules/qaul_info`, `qaul_management`,
`qaul_messaging`), which opens a new substream for every message. The capture in
[Why v2 costs more on the wire](#why-v2-costs-more-on-the-wire) confirmed this for
routing frames. The bring-up burst's 156,000 management sends were not captured, but
they go through the same handler, so they very likely pay the same cost.

- **Keep one long-lived stream per neighbour** for each behaviour, and write frames
  into it back to back. **Spec: silent.** §7.1 sets a batch's content and timing, not
  its transport framing. §11.2 only requires management to stay a behaviour of its
  own. It needs a new protocol version, so old and new nodes must both be supported
  during a migration.
- **Smaller: fold the 10-second origin into that second's relay batch.** This saves
  about 10% of frames. §7.1 describes the two as separate sends, so it needs a
  clarification first.

**2. Far members wait about 19 s per hop for their gateway's key**, so the directory
moves in 60 s steps.

- **Ask again for the key when the gateway's route arrives**, instead of losing the
  request. **Permitted:** §11.2 lets each use case re-issue "the request at its own
  layer", and §8.8 only says "schedule".
- **Hold the deferred manifest and apply it as soon as the key arrives.**
  **Permitted:** §8.8 says to fetch the key if it is not cached, then verify. It does
  not say to discard the manifest.
- **Clear an outstanding pull when its answer arrives**, not only after its 10 s
  timeout. **Consistent with §10.8**'s "at most one request … outstanding".
- Together these should bring the directory to one window, about 65 s, on all three
  topologies (predicted). Going below one window needs a §10.8 amendment.

**3. 66–79 s for the far side to see a community again after its gateway
reconnects.** When a gateway briefly loses its role, its members revoke their
delegations (`router_v2/delegation.rs:181`), and re-delegating then waits for a 60 s
window.

- **Keep delegations through a role loss.** Revoke only when the gateway leaves local
  reach, or when switching to another gateway. The entries are then still in its
  manifest, and the role-flip bump, which bypasses the rate limit, carries them.
  Recovery should drop to about 5 s.
- **Spec: silent.** §10.3 requires treating a delegation as broken only when
  `local_only` goes from 1 to 0. Even then it lets the old entry lapse "naturally", with
  no revoke, and it uses `is_gateway` only to select a target. A short §10.3
  clarification would settle it.

**4. The 29 MB profile burst at bring-up.**

- **Answer from a cache.** **Permitted:** §11.5 says "A node that holds the requested
  profile cached MAY answer on the subject's behalf". Answers would then come from about
  one hop away instead of about six.
- **Pace the fetches** over several seconds, with jitter. **Permitted:** §8.8 says
  "schedule", which leaves the timing open. Pacing also makes the cache work: when
  everyone asks at once, nobody has a cached copy yet.
- **Cap the fetches each node has outstanding**, to cut the 4,378 timeouts and their
  retries. **Silent**, and §14's 5 s timeout stays.
- **Not allowed: fetch own-community profiles only on first contact.** It **conflicts
  with §8.8**, where scheduling the fetch is one of the receiver's SHALL steps.
- **Bigger: neighbours swap profiles in bulk when they connect**, like `INDEX_DUMP`.
  This is a new message type, so it needs a spec amendment.

**5. The 60 s grace before a departed member is dropped**
(`router_v2/state/origin_manifest.rs:19`). A decision rather than a fix:

- **Remove it.** **Complies:** §10.4 says to drop the entry "from its next manifest
  version", and §10.8 lets that removal skip the rate limit. Departures would be
  noticed after about 40 s instead of 93–102 s. But a member whose link flaps for a
  moment is dropped, and re-adding it is rate-limited, so a short flap could cost up to
  a minute of unreachability.
- **Keep it, but drop the entry in any update that happens during it.** This complies
  with §10.4 literally and keeps the flap protection when nothing else changes.
  Departures would still take about 90 s.
- **Amend §10.4** to define a grace.

Suggested order: fix 1, then 2 and 3, which are small and permitted, then 4. 5 waits
on a decision.
