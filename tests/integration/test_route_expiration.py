"""Verify that a dead node's route expires within the expected window."""

import time

import pytest

from conftest import line_topology_files, discovery_wait_for
from lib.network import kill_node
from lib.node import Node

LINE_TOPOLOGIES = line_topology_files()
EXPIRATION_MARGIN = 1.5


@pytest.mark.routing
@pytest.mark.parametrize("topology_network", LINE_TOPOLOGIES, indirect=True)
def test_dead_node_route_expires(topology_network):
    """
    Kill the last node and assert its route disappears from the observer
    within (hop_count + 1) * 10s * 1.5.

    Only runs on line topologies — grids have redundant paths.
    """
    ids = topology_network["node_ids"]
    wait = discovery_wait_for(topology_network["file"])

    observer_id = ids[0]
    target_id = ids[-1]
    observer = topology_network["nodes"][observer_id]
    target = topology_network["nodes"][target_id]

    target_user_id = target.local_user_id()
    print(f"  target {target_id} user_id={target_user_id}")

    # poll until target appears in observer's routing table
    t0 = time.time()
    deadline = t0 + wait
    observer_table = {}

    while time.time() < deadline:
        observer_table = {
            e["user_id"]: e["connections"][0]
            for e in observer.router_table()
            if e["connections"]
        }
        if target_user_id in observer_table:
            print(f"  target found after {round(time.time() - t0, 1)}s")
            break
        time.sleep(2)

    assert target_user_id in observer_table, (
        f"target not in routing table after {wait}s"
    )

    hop_count = observer_table[target_user_id]["hop_count"]
    threshold = (hop_count + 1) * 10
    limit = threshold * EXPIRATION_MARGIN

    kill_node(target_id)
    print(f"  killed {target_id}, expecting expiration within {limit:.0f}s")
    t_kill = time.time()

    disappeared_at = None
    while time.time() < t_kill + limit:
        time.sleep(2)
        current = {e["user_id"] for e in observer.router_table() if e["connections"]}
        if target_user_id not in current:
            disappeared_at = round(time.time() - t_kill, 1)
            break

    assert disappeared_at is not None, (
        f"route still present {limit:.0f}s after kill (hop_count={hop_count})"
    )
    print(f"  expired in {disappeared_at}s (threshold={threshold}s)")
