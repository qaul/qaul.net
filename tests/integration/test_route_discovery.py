"""Measure routing table convergence time."""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = all_topology_files()


def test_all_nodes_appear_in_routing_table(
    node_ids: list[str],
    discovery_wait: int,
    poll_interval: float = 1.0,
) -> dict:
    """
    Poll the first node's router table until every node appears, recording when
    each one was first seen.

    Returns the measured convergence time, so the result file carries a
    measurement rather than the length of a wait. `convergence_time_s` is the
    moment the last node appeared, which is directly comparable to the
    pre-v2 baseline in results/2026-03-17-routing-*.json.
    """
    observer = Node(node_ids[0])
    expected = len(node_ids)

    first_seen: dict[str, float] = {}
    t0 = time.time()
    deadline = t0 + discovery_wait

    while True:
        elapsed = time.time() - t0
        for entry in observer.router_table():
            uid = entry["user_id"]
            if uid not in first_seen:
                first_seen[uid] = round(elapsed, 1)
                hops = entry["connections"][0]["hop_count"] if entry["connections"] else "?"
                print(f"    +{elapsed:.0f}s  discovered {uid} (hops={hops})")

        if len(first_seen) >= expected:
            break
        if time.time() >= deadline:
            break
        # Convergence is a seconds-scale event on this harness; a 5s poll
        # reported it to the nearest 5s, which is coarser than the thing
        # being measured.
        time.sleep(poll_interval)

    missing = expected - len(first_seen)
    assert missing == 0, (
        f"{missing} of {expected} nodes not in routing table after "
        f"{round(time.time() - t0, 1)}s (seen: {sorted(first_seen.values())})"
    )

    convergence_time_s = max(first_seen.values())
    return {
        "passed": True,
        "convergence_time_s": convergence_time_s,
        "discovery_timeline": first_seen,
        "notes": (
            f"all {expected} nodes discovered in {convergence_time_s}s "
            f"(limit {discovery_wait}s)"
        ),
    }


# --- pytest entry point ---

@pytest.mark.routing
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
def test_all_nodes_appear_in_routing_table_pytest(topology_network):
    """pytest wrapper using the topology_network fixture."""
    test_all_nodes_appear_in_routing_table(
        node_ids=topology_network["node_ids"],
        discovery_wait=discovery_wait_for(topology_network["file"]),
    )
