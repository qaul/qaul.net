# World suite

Lab tests for router_v2 on a realistic network: communities of different
sizes, several gateways in some, none in others, two servers, a direct link
between two communities, twin villages in radio range of each other, and
remote users. Why it is shaped this way, and the plan to put its far links on
the real Internet, is in [`../PLAN-real-internet.md`](../PLAN-real-internet.md).

The star suite in `../communities/` is left as it is: the results in
`../REPORT-v1-vs-v2.md` rest on it.

## What is here

| File | Role |
|---|---|
| `worlds/*.json` | World descriptions: communities, gateways, servers, remote users, who peers with whom |
| `gen_world.py` | Turns a description into a meshnet-lab topology and a roles file |
| `topologies/` | What `gen_world.py` writes |
| `link_filter.sh` | meshnet-lab per-link hook: drops mDNS on Internet links only |
| `wire.py` | `bringup`, `phase1` (servers' LAN off) and `phase2` (dial every Internet link) |
| `check.py` | Spheres, links and ping times, gateway role, containment, delegation split, directory |
| `sphere_scan.py` | Reads every log in an archive for LAN traffic between spheres, at any moment of the lab's life |
| `traffic.py` | Scenario 2: sends messages between pairs chosen by path (local, bridge, far-0, far-2, transit, no-gateway, remote, island) and times each from qaul's own `sent_at`/`received_at`. `run.sh -m` runs it after the check. |
| `run.sh` | All of the above in one root invocation, archived like the star suite |
| `egress.sh` | Hybrid runs: a NAT'd way out to test.qaul.net for the lab ends of far links, and nothing else |
| `egress_check.sh` | Plan step 3: hybrid lab with no daemons, egress up, ping the server from each far gateway |
| `common.sh` | Archive-and-scan and teardown, shared by `run.sh` and `egress_check.sh` |
| `faults.py` | Faults from the laptop (root): cut and restore a lab link (tc matchall at pref 1, both ends), isolate and rejoin a node (its `dl-` port), cut and restore a far gateway's egress (nft, top of the forward chain). `clear` undoes them all. |
| `scenario.py` | Failure scenarios (`s1-down`, `t1-outage`, `bridge-cut`, `walkaway`), run by `run.sh -f NAME`. A probe sends messages throughout; every 5 s it samples gateway role, links, subscriptions, manifests and directories. It prints a timeline against the spec's predictions and writes `results/scenario-<name>-<tag>.json`. A member left without a gateway for 30 s (`--stuck-after`) gets one snapshot of its own routing table, manifests and delegations, plus one contrast member that did move; the report shows which gateways each could pick under §10.3. |
| `stagger.py` | Scenario 1, run by `run.sh -j`: dials the backbone, then one community at a time (`-g`, default 180 s apart), instead of the whole Internet side at once. Every 5 s it samples each gateway's role, links, manifest version and directory, and each member's gateway. Per community it reports, from that community's dial: links up, gateway role, manifest bumps, members registered, when every gateway already online lists all of its users, and when its own gateways list everyone already online. Writes `results/stagger-<tag>.json`; the check that follows is timed from the last dial. |
| `draw_world.py` | Draws a world as an SVG (`diagrams/<name>.svg`): every node and link at the generator's own positions, real-Internet links marked. A world whose Internet links form one loop (ring-1) is drawn as a ring, laptop half left and server half right. |
| `worldlib.py` | Shared helpers |

## Running it

Generate the world, then run it as root in one invocation:

```sh
./gen_world.py worlds/world-1-half.json
sudo -v
sudo QAUL_ROUTING_V2=1 RUST_LOG='libqaul::router_v2=debug,libqaul=info' \
  MESHNET_LAB=$HOME/meshnet-lab ./run.sh -w world-1-half
```

| Flag | Effect |
|---|---|
| `-w NAME` | World name; default `world-1-half` |
| `-1` | v1 arm. Erase `QAUL_ROUTING_V2` too. |
| `-s SECS` | How long `check.py` keeps re-reading the directory after the dial; default 300 |
| `-m` | After the check, scenario 2: timed messages between chosen pairs (`traffic.py`) |
| `-f NAME` | After the check, a failure scenario (`scenario.py`): `s1-down`, `t1-outage`, `bridge-cut`, `walkaway` |
| `-j` | Scenario 1: step 5 dials the backbone, then one community at a time (`stagger.py`). v2 only. |
| `-g SECS` | With `-j`: seconds between two communities' dials; default 180 |
| `-H` | Hybrid run: S1 and r1 on test.qaul.net, started beforehand (see "The hybrid run") |

The sequence:

1. Archive the outgoing lab and scan it with `sphere_scan.py`. That catches a
   merge that came after the previous run's own check. It's reported, but it
   doesn't fail this run.
2. Apply the whole topology at once, before any daemon starts. Every Internet
   link gets mDNS dropped on it.
3. Start every daemon and wait for each node's account.
4. Switch LAN off on the servers, and assert they have no LAN neighbour and no
   user entry. That proves no server joined a community while it booted
   behind the filter.
5. Dial each Internet link in the roles file's plan, and verify the links and
   the gateway role. With `-j`, dial the backbone first (servers and remote
   users), then one community after another in the order T, V, W, H, X+Y,
   and measure each one joining (`stagger.py`). Step 6 is then timed from the
   last dial.
6. `check.py`: re-read every gateway's directory until it is complete or the
   settle runs out, timing each from the dial. Then, no sooner than 70 s after
   the dial, judge spheres, links, containment and delegation.
7. Archive every node's log and the results to `../runs/`, then scan every log
   for LAN traffic between spheres. A crossing fails the run, as a failed
   check does.

### Why one apply, before the daemons

meshnet-lab's `create_link` (`network.py`) brings a link up and puts it into
the node's bridge first. Only after that does it isolate the port and run the
filter hook. While a link is being added, its server's bridge briefly joins
all the server's links into one segment, with no filter on them.

The first version added the Internet links in a second apply, to running
daemons, as the star suite does. On 2026-11-12, both runs merged communities
in that moment:
- T2 with W1, V1 with X1, and others, for 10–35 s, until isolation cut them
  and the ping timeout closed them;
- W1 with H1 for good, since they share a link.

With no daemon running, nothing is there to send in that window. The star
suite's 33 v2 archives were scanned the same way and show no LAN traffic
between communities.

## Why the mDNS filter

A meshnet-lab link carries everything between two nodes. Two gateways with LAN
on, joined by a link, would find each other by mDNS and merge their communities
into one Local sphere. The star suite avoided that by switching the hub's LAN
off. In a world, W1 and H1 are both gateways of their own communities, so
`link_filter.sh` drops UDP 5353 on every Internet link instead. Only the
Internet transport can connect across them.

The filter drops mDNS two ways:
- **By port** (UDP 5353), for unfragmented packets, including any sent
  unicast.
- **By the mDNS multicast MAC** (`01:00:5e:00:00:fb`, `33:33:00:00:00:fb`).
  This catches fragments.

The port rules alone leak. libp2p-mdns puts every listen address in one reply,
each repeating a random 32–63 character name. Replies therefore vary in size
around the 1500-byte MTU, and the larger ones leave as IP fragments. tc flower
doesn't read UDP ports on a fragment, not even the first, so those pass the
port rules.

That is how W1 and H1 merged 14 minutes into the 2026-11-11 run. The one reply
W1's port rule caught on 2026-11-12 was 1457 bytes, 15 under the limit.
Tested in a private network namespace: a 2000-byte packet to
`224.0.0.251:5353` crossed the port rules as two fragments, and the MAC rule
stopped it.

qaul's mDNS asks every 30 s (`rust/libqaul/src/connections/lan.rs`). A clean
reading straight after the dial therefore proves nothing, which is why
`check.py` waits until 70 s after the dial before judging the spheres. A merge
can also come at any random later moment, which is why `sphere_scan.py` reads
the whole log.

## What `check.py` asserts and what it only reports

| Line | Held against | |
|---|---|---|
| spheres | no LAN neighbour outside a node's own Local sphere, at the moment of judging | asserted |
| sphere scan (`sphere_scan.py`) | no LAN traffic between spheres in any log line, bring-up to archive | asserted by `run.sh` |
| links | every planned Internet link up, nothing unplanned; ping time per end | asserted |
| gateway role | exactly the nodes with an Internet link (§2.3) | asserted on v2 |
| containment | members know no user outside their sphere (§2.3); gateways are reported, since they hold the world | asserted on v2 |
| delegation | which gateway each member subscribed to, and whether it was the lowest-metric eligible one (§10.3 SHOULD) | reported |
| directory | own manifest entries plus trusted entries held, at every gateway-role node, against the users reachable from anywhere | reported, with completion times |

## The hybrid run

In the hybrid run, S1 and r1 run on test.qaul.net, and the far gateways reach
them over the real Internet (plan steps 3 to 5).

### Step 3: a way out of the lab (built)

`gen_world.py` also writes `<name>-hybrid.json`, the lab part of the run: no
far nodes, and no link touching them. The lab ends of the far links (T1, T2,
V1, W1, r3 and S2) reach the server through `egress.sh` instead:

```
ns-<id> "egress" 198.18.0.(4k+2)/30  <-veth->  host "qe-<id>" 198.18.0.(4k+1)/30  --NAT--> eth0
```

- **Inside the node, the only route over that link is to the server (/32).**
  Everything else stays in the lab. Members get no link at all.
- **Every gateway has its own /30,** routed and not bridged, so far gateways
  never share a segment, and mDNS (link-local multicast) is never forwarded.
- **The host's nft table `qaul_egress` lets the links reach the server and
  nothing else.** It drops traffic between links, new connections from
  outside, and anything addressed to the host itself.
- **`down` restores `ip_forward` to what it was before,** and the run's
  teardown always calls it.
- **Address range:** `198.18.0.0/15` is reserved for testing (RFC 2544), clear
  of WSL's `172.31.0.0/20` and the lab's `10.0.0.0/8`.

Tested on 2026-11-12 in a private network namespace (`unshare -rnm`, no sudo),
with a stand-in Internet namespace that has no route back to the egress
range:
- 7 of 7 reachability cases came out as intended;
- the server saw only the host's address;
- `down` left nothing behind.

To check it in the real lab, with no qauld and nothing sent to the server but
ping:

```sh
sudo -v
sudo MESHNET_LAB=$HOME/meshnet-lab ./egress_check.sh -w world-1-half
```

### Step 4: S1 and r1 on test.qaul.net (done by hand, 2026-11-13)

The user runs every server command; no script touches the server.
- **Binary:** `~/bin/qauld` on the server is built from `~/qaul-v2` at the
  lab's commit.
- **Firewall:** ufw is installed but inactive, so a small nft table does the
  filtering. It touches only port 9229 and the `qaul` user's mDNS:

  ```
  table inet qaul_hybrid {
    chain in {
      type filter hook input priority filter; policy accept;
      iifname "lo" accept
      ip saddr <home address> udp dport 9229 counter accept
      udp dport 9229 counter drop
      tcp dport 9229 counter drop
    }
    chain out {
      type filter hook output priority filter; policy accept;
      meta skuid "qaul" udp dport 5353 counter drop
    }
  }
  ```

- **First test:**
  - S1 listened on 9229, and r1 connected to it over loopback;
  - both took the gateway role;
  - the table dropped 14 start-up mDNS packets and one stranger's probe of
    TCP 9229.

### Step 5: a hybrid run

**1. On the server, by hand: start S1 and r1 fresh.**

```sh
pgrep -a -x qauld                        # expect nothing
echo ${SSH_CLIENT%% *}                   # home address: must match the nft rule
RUN=~/hybrid/$(date -u +%Y%m%dT%H%M%SZ); mkdir -p $RUN/S1 $RUN/r1; echo $RUN
sudo nft list table inet qaul_hybrid > $RUN/nft-before.txt   # counters are cumulative
                                         # (`nft reset counters` left them as they were)
cd $RUN/S1 && RUST_LOG='libqaul::router_v2=debug,libqaul=info' ~/bin/qauld --name S1 --port 9229 --routing-v2 --daemonize
~/bin/qauld-ctl --socket $RUN/S1/qauld.sock transports disable --id lan
cd $RUN/r1 && RUST_LOG='libqaul::router_v2=debug,libqaul=info' ~/bin/qauld --name r1 --routing-v2 --daemonize
~/bin/qauld-ctl --socket $RUN/r1/qauld.sock transports disable --id lan
~/bin/qauld-ctl --socket $RUN/r1/qauld.sock connections nodes add --address /ip4/127.0.0.1/udp/9229/quic-v1 --name S1
~/bin/qauld-ctl --socket $RUN/S1/qauld.sock --json router neighbours   # r1 listed (give it ~10 s)
```

**2. On the laptop:**

```sh
sudo -v
sudo QAUL_ROUTING_V2=1 RUST_LOG='libqaul::router_v2=debug,libqaul=info' \
  MESHNET_LAB=$HOME/meshnet-lab ./run.sh -w world-1-half -H
```

`run.sh -H` runs `<name>-hybrid.json` and does the following:
1. Pings the server from the laptop. The result goes to
   `results/hybrid-ping-*.txt`.
2. Brings egress up for the far gateways.
3. Dials S1 at `/ip4/<test.qaul.net>/udp/9229/quic-v1`. Use `-S` for another
   address.
4. Checks everything from the lab side. A far node's identity is inferred, and
   all of its lab dialers must see the same peer. The directory is only
   complete once S1's and r1's users have come through S1. The r1–S1 link
   shows as "not visible from this lab".
5. Records the bytes each far gateway sent over the real Internet in
   `results/hybrid-egress-*.txt`, and pings again.
6. Archives the lab, takes egress down, and prints the commands for step 3.

**3. On the server, by hand, then `scp` on the laptop.** Save S1's and r1's
state, stop them, and copy back `qauld.err`, the JSON files and `nft.txt`
into `<archive>/far/`. Never copy `config.yaml`. `run.sh` prints the exact
commands.

## The ring world (ring-1): half the lab on the server

`worlds/ring-1.json` puts ten communities on one ring of gateways, with no
hub. Five run in the laptop's lab (A-E), five in a second lab on
test.qaul.net (F-J), and the ring crosses the real Internet at S1 and r1,
which run on the server's host:

```
A1 - B1 - C1 - D1 - E1 ==Internet== S1 - F1 - G1 - H1 - I1 - J1 - r1 ==Internet== A1
```

`diagrams/ring-1.svg` draws it (`./draw_world.py topologies/ring-1.roles.json`,
which picks the ring layout when the Internet links form one loop).

| File | Role |
|---|---|
| `run_far.sh` | The server half, as root on the server: `up` (archive, teardown, apply `ring-1-server.json`, start the daemons, host links), `dial` (wait until the laptop reaches S1, then dial and run `check.py --side server`), `finish` (archive, scan, remove) |
| `hostlink.sh` | Gives the server-lab nodes that dial S1 or r1 a point-to-point link to the host, at 198.19.0.1, with mDNS dropped |
| `nodecall.py` | Server end of one SSH session that `ring_traffic.py` keeps open, to reach server-lab nodes and read the server's clock |
| `ring_traffic.py` | Messages between pairs 0-6 ring hops apart, laptop and server; cross-machine times corrected by the measured clock offset. `run.sh -m` runs it for this world. |

`wire.py` and `check.py` take `--side server` for the server's lab. Each
side's gateways are held against the whole world's directory (296 users).

A run, in order:

1. Copy `lib/`, `lab/` and `world/` (with the generated topologies) to
   `~/ring/tests/integration/` on the server.
2. Once: the server's `qaul_hybrid` table accepts UDP 9230 (r1) from the
   home address, and 9229/9230 from `qh-*` (the host links).
3. Server: `run_far.sh up ring-1`. Bring-up kills every qauld, so start S1
   (`--port 9229`) and r1 (`--port 9230`) only after it, both with LAN off
   and no link between them.
4. Server: `run_far.sh dial ring-1 <S1's qauld.sock>` in the background. It
   waits for the laptop.
5. Laptop: `run.sh -w ring-1 -s 420 -H -m`. Its dial reaches S1 and starts
   the server half's dial within a second.
6. Server: save S1's and r1's state, stop them, `run_far.sh finish`, and
   copy the server archive and S1's and r1's logs into the laptop's archive.
