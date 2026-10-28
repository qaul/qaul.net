# Community/hub suite

Lab tests for router_v2's sphere model on its intended deployment shape: separate
communities, each a Local-sphere mesh behind its own gateway, joined only through an
Internet-sphere hub. They measure what §2.3 promises — that the membrane holds,
and that users can still reach each other across it — and the protocol overhead of
running it.

Findings so far, and the fix plan they led to, are in
[`docs/research/router-v2-cross-community-messaging.md`](../../../docs/research/router-v2-cross-community-messaging.md).

## What is here

| File | Role |
|---|---|
| `run.sh` | Runs the whole sequence below in one invocation, as root |
| `bringup.py` | Starts every node's `qauld` on an applied topology and waits until each answers with its local account |
| `phases.py` | Phase 1: switch the hub's LAN off and assert it learned nothing. Phase 2: peer each gateway to the hub over Internet |
| `manifests.py` | Which manifest origins each node holds, by sphere and role — the directory's preconditions |
| `crossing.py` | Containment (members hold no foreign users) and reachability (a cross-community invite and message arrive) |
| `overhead.py` | Protocol bytes per peer, transport and message type, plus uplink counters for a v1/v2 comparison |
| `topologies/` | `gen_communities.py` and the topologies it writes |

Shared code stays one level up: `lib/` for the node and network helpers, and
`lab/` for the tools that work on any lab run — `archive_lab.sh` and
`log_counters.py`.

Every topology comes as three files:

- `<name>.json` — the full network.
- `<name>-stage1.json` — the same network without the hub's links, so the hub
  cannot form a LAN neighbourship before phase 1 switches its LAN off.
- `<name>.roles.json` — which node is the hub, which are gateways, and each
  community's members, so no script infers roles from the shape.

To make another size: `topologies/gen_communities.py -c 10 -m 10 --name
communities-10x10`.

## Running it

Needs meshnet-lab and a `qauld` built from the branch, run as root.

```sh
sudo -v
sudo QAUL_ROUTING_V2=1 RUST_LOG='libqaul::router_v2=debug,libqaul=info' \
  MESHNET_LAB=$HOME/meshnet-lab ./run.sh -c
```

Options:

| Flag | Effect |
|---|---|
| `-t NAME` | Topology base name; default `communities-3x10` |
| `-r` | Dial the gateways in reverse order in phase 2 |
| `-c` | Also run `crossing.py` |
| `-T SECS` | Crossing timeout; default 60 |
| `-s SECS` | Settle time before inspecting; default 180 |
| `-1` | v1 arm |

One invocation matters: a sudo timestamp expiring mid-sequence has corrupted a
measurement before.

**For the v1 arm**, erase `QAUL_ROUTING_V2` as well as passing `-1`. `software.py`
starts under `sudo -E`, so a surviving variable runs v2 while the output says v1.
`run.sh` refuses the combination.

### The same thing by hand

Run these from this directory if you want to stop between stages:

1. Apply the stage-1 topology, which omits the hub's links:
   `sudo $MESHNET_LAB/network.py apply topologies/communities-3x10-stage1.json`
2. Start the daemons:
   `./bringup.py --roles topologies/communities-3x10.roles.json`
3. Switch the hub's LAN off, and assert its table is empty — proof it never
   bridged, rather than hoping entries expired:
   `./phases.py --roles ... --phase 1`
4. Add the hub's links. `network.py apply` is incremental, so this creates the hub
   links and leaves every daemon running:
   `sudo $MESHNET_LAB/network.py apply topologies/communities-3x10.json`
5. Peer each gateway to the hub over the Internet sphere:
   `./phases.py --roles ... --phase 2`
6. **Wait at least 180 seconds.** The directory assembles over roughly 3 minutes.
   Sampling earlier reads as a defect that is really just convergence.
7. Check the preconditions: `./manifests.py --roles ...`. On `communities-3x10`,
   expect 3 of 3 node entries in the hub's node space, and each gateway holding 3
   Internet-learned origins at v4 with 10 manifest entries.
8. Run the crossing test: `./crossing.py --roles ... --arm v2 --timeout 60`

By hand, nothing archives the logs. Run `sudo ../lab/archive_lab.sh <tag>` before
anything tears the lab down.

## Where the output goes

- **`../results/`** — the JSON reports from `manifests.py` and `crossing.py`.
- **`../runs/<tag>-<UTC start>/`** — written by `run.sh` through
  `lab/archive_lab.sh`. It holds every node's log (gzipped), the run's results and
  roles, the node and user PeerIds, and a `counters.txt` summary. It is archived
  before each teardown and again when a run ends, so the evidence survives the next
  bring-up.
- **The live lab** — read the same counters with `../lab/log_counters.py
  --roles topologies/communities-3x10.roles.json`.

Both directories are git-ignored.
