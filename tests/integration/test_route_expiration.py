# test_route_expiration.py
#
# Verifies that a dead node's route expires from the routing table within
# the expected window after it stops sending RouterInfo broadcasts.
#
# The expiration formula from the qaul routing protocol is:
#   threshold = (hop_count + 1) × 10 seconds
#
# So a node at hop 4 should disappear from the routing table within 50s
# of going silent. This test asserts disappearance within that window
# plus a 50% safety margin.
#
# Procedure:
#   1. Start the topology and wait for full convergence
#   2. Pick the last node (highest sorted ID) as the target to kill
#   3. Record the target's user_id and hop_count from the observer's routing table
#   4. Kill the target node's qauld process
#   5. Poll the observer's routing table every 2s
#   6. Assert the target's user_id disappears within the expected window
#
# This test is best run on line topologies where hop counts are predictable.
# On dense grid topologies the target node may have many alternative routes
# via other nodes, so expiration takes longer or may not happen at all if
# any neighbour of the target is still alive.

import sys
import time

sys.path.insert(0, ".")

from lib.network import (
    apply_topology,
    clear_topology,
    kill_node,
    start_qaul,
    stop_qaul,
    wait_for_nodes,
)
from lib.node import Node
from lib.topology import load_node_ids

TOPOLOGY = "topologies/line-5.json"
NODE_IDS = load_node_ids(TOPOLOGY)
DISCOVERY_WAIT = 120
POLL_INTERVAL = 2
EXPIRATION_MARGIN = 1.5  # assert within threshold × this multiplier


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def _local_user_id(node: Node) -> str:
    """Return the qaul user ID of the node's own local account."""
    for entry in node.router_table():
        if entry["connections"] and entry["connections"][0]["module"] == "Local":
            return entry["user_id"]
    raise ValueError(f"No Local entry in router table for node {node.id}")


def test_dead_node_route_expires(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    poll_interval: int = POLL_INTERVAL,
    expiration_margin: float = EXPIRATION_MARGIN,
) -> dict:
    """
    Kill the last node and assert its route disappears from the observer
    within (hop_count + 1) × 10s × expiration_margin seconds.
    """
    sorted_ids = sorted(node_ids)
    observer_id = sorted_ids[0]
    target_id = sorted_ids[-1]

    observer = Node(observer_id)
    target = Node(target_id)

    print(f"  waiting {discovery_wait}s for convergence...")
    time.sleep(discovery_wait)

    # get target's qaul user_id before killing it
    target_user_id = _local_user_id(target)
    print(f"  target node {target_id} has user_id {target_user_id}")

    # find that user in the observer's routing table and record hop_count
    observer_table = {
        entry["user_id"]: entry["connections"][0]
        for entry in observer.router_table()
        if entry["connections"]
    }

    assert target_user_id in observer_table, (
        f"target user {target_user_id} not found in observer {observer_id}'s "
        f"routing table after {discovery_wait}s — convergence incomplete"
    )

    hop_count = observer_table[target_user_id]["hop_count"]
    expiration_threshold = (hop_count + 1) * 10
    expiration_limit = expiration_threshold * expiration_margin

    print(
        f"  target is {hop_count} hops away — "
        f"expected expiration within {expiration_threshold}s "
        f"(asserting within {expiration_limit:.0f}s)"
    )

    # kill the target node
    kill_node(target_id)
    print(f"  killed node {target_id}")
    t_kill = time.time()

    # poll until the target's route disappears
    disappeared_at = None
    deadline = t_kill + expiration_limit

    while time.time() < deadline:
        time.sleep(poll_interval)
        elapsed = round(time.time() - t_kill, 1)

        current_ids = {
            entry["user_id"]
            for entry in observer.router_table()
            if entry["connections"]
        }

        if target_user_id not in current_ids:
            disappeared_at = elapsed
            print(
                f"  route for {target_user_id} disappeared at +{elapsed}s "
                f"(threshold {expiration_threshold}s)"
            )
            break

    assert disappeared_at is not None, (
        f"route for node {target_id} (user {target_user_id}) still present "
        f"in routing table {expiration_limit:.0f}s after node was killed\n"
        f"  expected expiration within {expiration_threshold}s "
        f"(hop_count={hop_count})"
    )

    print(
        f"  PASS: route expired in {disappeared_at}s "
        f"(threshold {expiration_threshold}s, limit {expiration_limit:.0f}s)"
    )

    return {
        "passed": True,
        "hop_count": hop_count,
        "expiration_threshold_s": expiration_threshold,
        "actual_expiration_s": disappeared_at,
        "notes": (
            f"node {target_id} at hop {hop_count}: route expired in "
            f"{disappeared_at}s (threshold {expiration_threshold}s)"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        test_dead_node_route_expires()
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
