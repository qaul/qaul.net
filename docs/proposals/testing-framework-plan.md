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

The binary ends up at `rust/target/release/qauld-ctl`. Rather than copying it, create a symlink from `~/bin/` pointing to the release binary — the same pattern used for qauld:

```sh
ln -s /home/qaul/qaul.net/rust/target/release/qauld-ctl ~/bin/qauld-ctl
```

After that, `qauld-ctl` is on PATH everywhere, and rebuilding just requires `cargo build --release -p qauld-ctl` — the symlink doesn't need to be touched again.

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

  nohup qauld --name="node-${id}" > /dev/null 2>&1 < /dev/null &
  echo $! > "/tmp/qaul-${id}.pid"
else
  echo "qauld already running in ns ${id}"
fi
```

`qauld` is already on PATH on the server, so no absolute path is needed in the start script.

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

Now that qauld-ctl supports `--json`, the node wrapper uses it for all data-returning commands. This gives us two internal methods:

- `_run(*args)` — for fire-and-forget commands that produce no structured output (e.g. `feed send`)
- `_run_json(*args)` — for all data-returning commands; passes `--json` and returns a parsed Python dict or list

This eliminates all string parsing. Every method that returns data works directly with Python objects.

```python
import subprocess
import json

class Node:
    def __init__(self, node_id: str):
        self.id = node_id
        self.socket = f"/tmp/qaul-{node_id}/qauld.sock"

    def _run(self, *args) -> str:
        """Run a qauld-ctl command, return raw stdout."""
        cmd = ["qauld-ctl", "--socket", self.socket] + list(args)
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode != 0:
            raise RuntimeError(
                f"qauld-ctl failed on node {self.id}: {result.stderr.strip()}"
            )
        return result.stdout.strip()

    def _run_json(self, *args):
        """Run a qauld-ctl command with --json, return parsed output."""
        output = self._run("--json", *args)
        return json.loads(output)

    def is_alive(self) -> bool:
        """Check if qauld is reachable on this node."""
        try:
            self._run_json("node", "info")
            return True
        except Exception:
            return False

    def node_info(self) -> dict:
        """Return the full node info dict: node_id, addresses."""
        return self._run_json("node", "info")

    def node_id(self) -> str:
        """Get the libp2p peer ID of this node."""
        return self.node_info()["node_id"]

    def known_users(self) -> list[dict]:
        """Return list of all known users as dicts."""
        return self._run_json("users", "list")

    def known_user_ids(self) -> list[str]:
        """Return just the IDs of all known users."""
        return [u["id"] for u in self.known_users()]

    def get_user(self, user_id: str) -> dict:
        """Get a single user by ID."""
        return self._run_json("users", "get", "--user-id", user_id)

    def send_feed_message(self, message: str):
        """Send a feed message (fire-and-forget, no JSON needed)."""
        self._run("feed", "send", "--message", message)

    def feed_messages(self) -> list[dict]:
        """Return all feed messages as a list of dicts."""
        return self._run_json("feed", "list")

    def feed_message_contents(self) -> list[str]:
        """Return just the content strings of all feed messages."""
        return [m["content"] for m in self.feed_messages()]

    def default_account(self) -> dict:
        """Return the default account info dict."""
        return self._run_json("account", "default")

    def create_account(self, name: str, password: str = None) -> dict:
        """Create a new user account, return the created account dict."""
        args = ["account", "create", "--username", name]
        if password:
            args += ["--password", password]
        return self._run_json(*args)

    def groups(self) -> list[dict]:
        """Return all groups this node is a member of."""
        return self._run_json("group", "list")

    def group_info(self, group_id: str) -> dict:
        """Return info for a specific group."""
        return self._run_json("group", "info", "--id", group_id)

    def conversation(self, group_id: str, index: int = 0) -> dict:
        """Return conversation history for a group."""
        return self._run_json("chat", "conversation", "--group-id", group_id, "--index", str(index))
```

The key properties of this design:
- No string parsing anywhere — all structured data comes back as Python dicts/lists via `json.loads`
- `_run_json` is a one-liner that just prepends `--json` to any command
- Methods that return data always return typed Python objects, making assertions in tests direct and readable
- Fire-and-forget commands (`feed send`, `chat send`) still use `_run` since they produce no output

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

Tests are just Python scripts. With JSON output, assertions are direct comparisons on Python objects — no regex, no line splitting, no fragile string matching.

Here's what a node discovery test looks like:

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
    node 0000 should be able to see node 0001's user.
    """
    print(f"  waiting {wait_seconds}s for node discovery...")
    time.sleep(wait_seconds)

    node_a = Node("0000")
    node_b = Node("0001")

    id_b = node_b.node_id()
    known_ids = node_a.known_user_ids()

    assert id_b in known_ids, (
        f"node 0000 does not know about node 0001 after {wait_seconds}s\n"
        f"  known user ids: {known_ids}"
    )
    print("  PASS: node 0000 knows about node 0001")

def test_user_fields_are_present():
    """
    Users returned by users list should have all expected fields.
    """
    node = Node("0000")
    users = node.known_users()

    assert len(users) > 0, "node 0000 has no known users"

    user = users[0]
    for field in ("id", "name", "verified", "blocked", "connectivity", "group_id", "public_key"):
        assert field in user, f"user entry missing field: {field}"

    print("  PASS: user entries contain all expected fields")

if __name__ == "__main__":
    try:
        setup()
        test_nodes_discover_neighbors()
        test_user_fields_are_present()
    finally:
        teardown()
```

And a feed message routing test:

```python
# test_message_routing.py
#
# Checks that a feed message sent from node 0000 eventually
# appears on node 0004 (4 hops away).

import time
import sys
sys.path.insert(0, ".")

from lib.network import apply_topology, clear_topology, start_qaul, stop_qaul, wait_for_nodes
from lib.node import Node

TOPOLOGY = "topologies/line-5.json"
NODE_IDS = [f"{i:04x}" for i in range(5)]
TEST_MESSAGE = "hello from node 0"

def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=30)

def teardown():
    stop_qaul()
    clear_topology()

def test_feed_message_reaches_far_node(wait_seconds=90):
    node_sender = Node("0000")
    node_receiver = Node("0004")

    node_sender.send_feed_message(TEST_MESSAGE)

    print(f"  waiting {wait_seconds}s for message propagation...")
    time.sleep(wait_seconds)

    contents = node_receiver.feed_message_contents()

    assert TEST_MESSAGE in contents, (
        f"message '{TEST_MESSAGE}' not found on node 0004 after {wait_seconds}s\n"
        f"  messages seen: {contents}"
    )
    print("  PASS: feed message reached node 0004")

def test_feed_message_fields_are_present():
    """
    Feed messages should have all expected fields in JSON output.
    """
    node = Node("0000")
    messages = node.feed_messages()

    if not messages:
        print("  SKIP: no feed messages yet")
        return

    msg = messages[0]
    for field in ("index", "message_id", "sender_id", "content", "time_sent", "timestamp_sent"):
        assert field in msg, f"feed message missing field: {field}"

    print("  PASS: feed message entries contain all expected fields")

if __name__ == "__main__":
    try:
        setup()
        test_feed_message_reaches_far_node()
        test_feed_message_fields_are_present()
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

1. ✅ Build qauld-ctl from source on the server, create symlink at `~/bin/qauld-ctl`
2. ✅ Fix `qaul_start.sh` PID guard
3. ✅ Manually verify meshnet-lab + qaul works (3-node line topology)
4. ✅ Manually verify qauld-ctl reaches a node via socket
5. ✅ Add `--json` flag to qauld-ctl (all commands updated)
6. Create `tests/integration/` in the qaul.net repo, write `lib/node.py` using `_run_json`
7. Write `lib/network.py`
8. Write `test_node_startup.py` — checks all nodes come up and `node info` returns valid JSON with expected fields
9. Write `test_user_discovery.py` — checks node ID propagation and user entry field structure
10. Write `test_message_routing.py` — checks feed message propagation across hops
11. Write `run.py`

The hard part is steps 9 and 10 — figuring out how long to wait for qaul's gossip protocol to propagate across N hops. This needs empirical measurement on the server before you can set realistic timeouts in the tests. The meshnet-lab `convergence1` test scenario is a good reference for how to approach this: start nodes, then poll for connectivity at increasing intervals rather than sleeping a fixed duration.
