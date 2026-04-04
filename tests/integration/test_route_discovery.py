"""Measure routing table convergence time."""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = all_topology_files()


@pytest.mark.routing
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
def test_all_nodes_appear_in_routing_table(topology_network):
    """
    Poll router table on the first node every 5s.
    Assert all nodes discovered within discovery_wait.
    """
    ids = topology_network["node_ids"]
    wait = discovery_wait_for(topology_network["file"])

    observer = topology_network["nodes"][ids[0]]
    expected = len(ids)

    first_seen: dict[str, float] = {}
    t0 = time.time()
    deadline = t0 + wait

    while time.time() < deadline:
        elapsed = time.time() - t0
        for entry in observer.router_table():
            uid = entry["user_id"]
            if uid not in first_seen:
                first_seen[uid] = round(elapsed, 1)
                hops = entry["connections"][0]["hop_count"] if entry["connections"] else "?"
                print(f"    +{elapsed:.0f}s  discovered {uid} (hops={hops})")

        if len(first_seen) >= expected:
            break
        time.sleep(5)

    missing = expected - len(first_seen)
    assert missing == 0, (
        f"{missing} of {expected} nodes not in routing table after {round(time.time() - t0, 1)}s"
    )
