# test_route_discovery.py
#
# Measures how long it takes for a node to discover all other nodes in the
# routing table after a cold start.
#
# The test polls `router table` on the first node every 5 seconds and records
# the timestamp when each new user first appears. It asserts that all nodes are
# discovered within discovery_wait seconds.
#
# This test is parameterised by topology file and discovery_wait so it can
# be called from run_routing_tests.py across all topologies.
# It can also be run standalone (see __main__ at the bottom).

import sys
import time

sys.path.insert(0, ".")

from lib.network import (
    apply_topology,
    clear_topology,
    start_qaul,
    stop_qaul,
    wait_for_nodes,
)
from lib.node import Node
from lib.topology import load_node_ids

TOPOLOGY = "topologies/line-5.json"
NODE_IDS = load_node_ids(TOPOLOGY)
DISCOVERY_WAIT = 120
POLL_INTERVAL = 5


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def test_all_nodes_appear_in_routing_table(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    poll_interval: int = POLL_INTERVAL,
) -> dict:
    """
    Poll router table on the first node every poll_interval seconds.
    Record the elapsed time when each new user first appears.
    Assert all nodes discovered within discovery_wait seconds.

    Returns a dict of timing observations for recording in results.
    """
    observer = Node(sorted(node_ids)[0])
    expected_count = len(node_ids)

    # track which user_ids have been seen and when they first appeared
    first_seen: dict[str, float] = {}
    t_start = time.time()
    deadline = t_start + discovery_wait

    print(
        f"  polling router table on node {observer.id} every {poll_interval}s "
        f"(expecting {expected_count} users, timeout {discovery_wait}s)..."
    )

    while time.time() < deadline:
        elapsed = time.time() - t_start
        table = observer.router_table()

        for entry in table:
            uid = entry["user_id"]
            if uid not in first_seen:
                first_seen[uid] = round(elapsed, 1)
                hops = (
                    entry["connections"][0]["hop_count"]
                    if entry["connections"]
                    else "?"
                )
                print(f"    +{elapsed:.0f}s  discovered user {uid}  (hop_count={hops})")

        if len(first_seen) >= expected_count:
            break

        time.sleep(poll_interval)

    total_elapsed = round(time.time() - t_start, 1)

    missing = expected_count - len(first_seen)
    assert missing == 0, (
        f"{missing} of {expected_count} nodes still not in routing table "
        f"after {total_elapsed}s\n"
        f"  discovered: {len(first_seen)}  expected: {expected_count}"
    )

    convergence_time = max(first_seen.values())
    print(
        f"  PASS: full convergence at {convergence_time}s "
        f"(all {expected_count} users in routing table, limit {discovery_wait}s)"
    )

    return {
        "passed": True,
        "convergence_time_s": convergence_time,
        "discovery_timeline": first_seen,
        "notes": (
            f"all {expected_count} nodes discovered in {convergence_time}s "
            f"(limit {discovery_wait}s)"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        result = test_all_nodes_appear_in_routing_table()
        print(f"  PASS: convergence timeline: {result['discovery_timeline']}")
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
