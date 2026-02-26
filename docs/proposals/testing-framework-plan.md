# qaul Multi-Node Testing Framework

The goal is to be able to spin up N qaul nodes in a simulated mesh network, run commands against them programmatically, make assertions, and tear everything down. The whole thing should be scriptable and repeatable so it can eventually run in CI.

The two pieces that make this possible are meshnet-lab (for the simulated network) and qauld-ctl (for talking to individual nodes). meshnet-lab creates isolated Linux network namespaces that behave like real mesh nodes — each one has its own network stack, and the links between them can have bandwidth limits, latency, and packet loss applied. qauld-ctl gives us a programmable socket interface to each running qauld instance.

The server has qauld and qaul installed at `/home/qaul/bin`. meshnet-lab is at `/home/qaul/meshnet-lab` and needs to be configured for qaul.

---

## Step 0: Build qauld-ctl on the server

Since qauld-ctl isn't installed yet, the first thing is to build it from the qaul.net source:

```sh
cd qaul.net/rust
cargo build --release -p qauld-ctl
```

The binary ends up at `rust/target/release/qauld-ctl`. Putting it in `/home/qaul/bin/` is cleaner so the start/stop scripts and test runner don't need to know its location explicitly.

---

## Step 1: Fix the existing qaul profile in meshnet-lab

meshnet-lab already ships a qaul profile (`protocols/qaul_start.sh` and `protocols/qaul_stop.sh`). The stop script is fine. The start script has a bug — the PID guard is broken:

```sh
# this is always true — it's testing if the string is non-empty, not if the file exists
if test "/tmp/qaul-${id}.pid"; then
```

It should be:

```sh
if [ ! -f "/tmp/qaul-${id}.pid" ]; then
```

The fixed start script should look like this:

```sh
#!/bin/sh

address="$1"
id="$2"

if [ ! -f "/tmp/qaul-${id}.pid" ]; then
  echo "starting qauld on ${address} in ns ${id}"

  # derive an IPv4 address from the uplink MAC address
  addr4() {
    local mac=$(cat "/sys/class/net/$1/address")
    IFS=':'; set $mac; unset IFS
    [ "$6" = "ff" -o "$6" = "00" ] && set $1 $2 $3 $4 $5 "01"
    printf "10.%d.%d.%d" 0x$4 0x$5 0x$6
  }

  ip link set "uplink" down
  ip link set "uplink" up
  ip a a $(addr4 "uplink")/8 dev "uplink"

  DIR="/tmp/qaul-${id}"
  rm -rf "$DIR"
  mkdir "$DIR"
  cd "$DIR"

  nohup /home/qaul/bin/qauld --name="node-${id}" > /dev/null 2>&1 < /dev/null &
  echo $! > "/tmp/qaul-${id}.pid"
else
  echo "qauld already running in ns ${id}"
fi
```

Using the absolute path `/home/qaul/bin/qauld` avoids any PATH issues inside network namespaces, which don't inherit the user's shell environment.

---

## Step 2: Structure of the test framework

The framework lives inside the qaul.net repository under `tests/integration/`. This makes sense because qauld-ctl is also built here, and the tests are fundamentally about testing qauld and qauld-ctl together. It also means the tests travel with the code and can eventually be wired into CI.

```
qaul.net/
└── tests/
    └── integration/
        ├── lib/
        │   ├── node.py          # wrapper around qauld-ctl for a single node
        │   └── network.py       # setup/teardown helpers
        ├── topologies/
        │   ├── line-5.json
        │   ├── grid4-3x3.json
        │   └── ...
        ├── test_node_startup.py
        ├── test_user_discovery.py
        ├── test_message_routing.py
        └── run.py
```

---

## Step 3: The node wrapper (lib/node.py)

Every test needs to talk to individual nodes. The way that works: each qauld instance runs inside a network namespace, but its working directory (`/tmp/qaul-<id>/`) is on the host filesystem. So `qauld-ctl --socket /tmp/qaul-<id>/qauld.sock` reaches it from the host without entering the namespace.

The node wrapper just shells out to qauld-ctl and parses stdout:

```python
import subprocess

QAULD_CTL = "/home/qaul/bin/qauld-ctl"

class Node:
    def __init__(self, node_id: str):
        self.id = node_id
        self.socket = f"/tmp/qaul-{node_id}/qauld.sock"

    def _run(self, *args) -> str:
        """Run a qauld-ctl command against this node, return stdout."""
        cmd = [QAULD_CTL, "--socket", self.socket] + list(args)
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode != 0:
            raise RuntimeError(
                f"qauld-ctl failed on node {self.id}: {result.stderr.strip()}"
            )
        return result.stdout.strip()

    def is_alive(self) -> bool:
        """Check if qauld is reachable on this node."""
        try:
            self._run("node", "info")
            return True
        except Exception:
            return False

    def node_id(self) -> str:
        """Get the qaul node ID (libp2p peer ID)."""
        output = self._run("node", "info")
        # parse "Node ID: <id>" from output
        for line in output.splitlines():
            if line.startswith("Node ID:"):
                return line.split(":", 1)[1].strip()
        raise ValueError(f"Could not parse node ID from: {output}")

    def known_users(self) -> list[str]:
        """Return list of known user IDs visible from this node."""
        output = self._run("users", "list")
        users = []
        for line in output.splitlines():
            # each line has format: N | name | <base58-id> | ...
            parts = [p.strip() for p in line.split("|")]
            if len(parts) >= 3 and parts[2]:
                users.append(parts[2])
        return users

    def send_feed_message(self, message: str):
        """Send a feed message into the network."""
        self._run("feed", "send", "--message", message)

    def feed_messages(self) -> list[str]:
        """Return feed message contents visible from this node."""
        output = self._run("feed", "list")
        messages = []
        for line in output.splitlines():
            if line.startswith("  "):  # indented content lines
                messages.append(line.strip())
        return messages

    def create_account(self, name: str):
        self._run("account", "create", "--username", name)

    def default_account(self) -> str:
        return self._run("account", "default")
```

The parsing logic here depends on the exact output format of each `qauld-ctl` command — you'll need to adjust these as you run them and see what comes out. The structure above is intentionally simple: just string parsing on the CLI output, no JSON mode needed for now.

---

## Step 4: Network setup helpers (lib/network.py)

This wraps the meshnet-lab scripts so tests can set up and tear down without caring about the details:

```python
import subprocess
import time
import os

MESHNET_LAB = os.environ.get("MESHNET_LAB", "/home/qaul/meshnet-lab")

def apply_topology(topology_file: str):
    """Create the network from a topology JSON file."""
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/network.py", "apply", topology_file],
        check=True
    )

def clear_topology():
    """Remove all network namespaces."""
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/network.py", "apply", "none"],
        check=True
    )

def start_qaul():
    """Start qauld in all namespaces."""
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/software.py", "start", "qaul"],
        check=True
    )

def stop_qaul():
    """Stop all qauld instances."""
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/software.py", "stop", "qaul"],
        check=True
    )

def wait_for_nodes(node_ids: list[str], timeout: int = 30):
    """Block until all nodes are reachable via qauld-ctl, or raise on timeout."""
    from lib.node import Node

    deadline = time.time() + timeout
    remaining = list(node_ids)

    while remaining and time.time() < deadline:
        still_down = []
        for nid in remaining:
            if not Node(nid).is_alive():
                still_down.append(nid)
        remaining = still_down
        if remaining:
            time.sleep(1)

    if remaining:
        raise TimeoutError(
            f"Nodes still not reachable after {timeout}s: {remaining}"
        )
```

The default meshnet-lab path is `/home/qaul/meshnet-lab` but can be overridden with the `MESHNET_LAB` environment variable if needed.

---

## Step 5: Writing a test

Tests are just Python scripts. Here's what a node discovery test looks like:

```python
# test_user_discovery.py
#
# Checks that after startup, nodes on a line topology can eventually
# see each other's user accounts.

import time
import sys
sys.path.insert(0, ".")

from lib.network import apply_topology, clear_topology, start_qaul, stop_qaul, wait_for_nodes
from lib.node import Node

TOPOLOGY = "topologies/line-5.json"

# node IDs in a 5-node line are: 0000, 0001, 0002, 0003, 0004
NODE_IDS = [f"{i:04x}" for i in range(5)]

def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=30)

def teardown():
    stop_qaul()
    clear_topology()

def test_nodes_discover_neighbors(wait_seconds=60):
    """
    After waiting for the gossip protocol to propagate,
    the first node should be able to see at least the second node's user.
    """
    print(f"  waiting {wait_seconds}s for node discovery...")
    time.sleep(wait_seconds)

    node_a = Node("0000")
    node_b = Node("0001")

    id_b = node_b.node_id()
    known_by_a = node_a.known_users()

    assert id_b in known_by_a, (
        f"node 0000 does not know about node 0001 after {wait_seconds}s\n"
        f"  known users: {known_by_a}"
    )
    print("  PASS: node 0000 knows about node 0001")

if __name__ == "__main__":
    try:
        setup()
        test_nodes_discover_neighbors()
    finally:
        teardown()
```

The `finally` block in teardown is important — if a test fails mid-run, you still want the network cleaned up so the next test can start fresh.

---

## Step 6: Pre-generate topology files

Rather than generating topologies at runtime, pre-generate them and commit the JSON files. This makes test runs reproducible and removes the meshnet-lab Python dependency from the test runner itself:

```sh
# run from /home/qaul/meshnet-lab
./topology.py line 5 > /path/to/qaul.net/tests/integration/topologies/line-5.json
./topology.py grid4 3 3 > /path/to/qaul.net/tests/integration/topologies/grid4-3x3.json
./topology.py grid4 5 5 > /path/to/qaul.net/tests/integration/topologies/grid4-5x5.json
./topology.py line 10 > /path/to/qaul.net/tests/integration/topologies/line-10.json
```

Start small. A 5-node line is enough to test multi-hop routing (node 0 → node 4 is 4 hops). A 3×3 grid adds branching paths.

---

## Step 7: The test runner (run.py)

```python
#!/usr/bin/env python3

import sys
import traceback

tests = [
    ("node startup", "test_node_startup"),
    ("user discovery", "test_user_discovery"),
    ("feed message routing", "test_message_routing"),
]

passed = 0
failed = 0

for name, module_name in tests:
    print(f"\n[{name}]")
    try:
        mod = __import__(module_name)
        mod.setup()
        try:
            # run all functions starting with "test_"
            for fn_name in dir(mod):
                if fn_name.startswith("test_"):
                    print(f"  running {fn_name}...")
                    getattr(mod, fn_name)()
            passed += 1
        finally:
            mod.teardown()
    except AssertionError as e:
        print(f"  FAIL: {e}")
        failed += 1
    except Exception as e:
        print(f"  ERROR: {e}")
        traceback.print_exc()
        failed += 1

print(f"\n{'='*40}")
print(f"Results: {passed} passed, {failed} failed")
sys.exit(0 if failed == 0 else 1)
```

---

## Step 8: Traffic control for realistic conditions

The basic setup uses no bandwidth or latency limits. To test under more realistic wireless conditions, pass a link command when applying the topology:

```python
def apply_topology_with_tc(topology_file: str, bandwidth_mbit=10, latency_ms=20, loss_percent=0):
    """Apply topology with traffic control on each link."""
    tc_cmd = (
        f'tc qdisc replace dev "{{ifname}}" root netem '
        f'delay {latency_ms}ms loss {loss_percent}% rate {bandwidth_mbit}mbit'
    )
    subprocess.run(
        ["sudo", f"{MESHNET_LAB}/network.py",
         "--link-command", tc_cmd,
         "apply", topology_file],
        check=True
    )
```

With this you can write tests that specifically check behaviour under degraded links — e.g. 50% packet loss should still allow message delivery via alternate paths in a grid topology.

---

## Implementation order

1. Build qauld-ctl from source on the server (`cargo build --release -p qauld-ctl`), copy binary to `/home/qaul/bin/`
2. Fix `qaul_start.sh` in `/home/qaul/meshnet-lab/protocols/` — update the PID guard and use the absolute path `/home/qaul/bin/qauld`
3. Manually verify meshnet-lab works: create a 3-node line, start qaul, confirm sockets appear at `/tmp/qaul-*/qauld.sock`
4. Manually run `qauld-ctl --socket /tmp/qaul-0000/qauld.sock node info` from the host — confirm it works
5. Create `tests/integration/` in the qaul.net repo, write `lib/node.py`, test each method manually against a live node
6. Write `lib/network.py`
7. Write the first test (`test_node_startup.py`) — just checks all nodes come up and respond
8. Write `test_user_discovery.py`
9. Write `test_message_routing.py` (feed messages)
10. Write `run.py`

The hard part is steps 7 and 8 — figuring out how long to wait for qaul's gossip protocol to propagate across N hops. This needs empirical measurement on the server before you can set realistic timeouts in the tests. The meshnet-lab `convergence1` test scenario is a good reference for how to approach this: start nodes, then poll for connectivity at increasing intervals.
