"""Verify routing table stability — no flaps after convergence."""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = all_topology_files()
STABILITY_DURATION = 120
POLL_INTERVAL = 10


@pytest.mark.routing
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
def test_routing_table_is_stable(topology_network):
    """
    After convergence, poll the routing table repeatedly and assert no flapping.
    """
    ids = topology_network["node_ids"]
    wait = discovery_wait_for(topology_network["file"])
    observer = topology_network["nodes"][ids[0]]

    print(f"  waiting {wait}s for convergence...")
    time.sleep(wait)

    baseline = {
        entry["user_id"]: entry["connections"][0]
        for entry in observer.router_table()
        if entry["connections"]
    }
    assert len(baseline) > 0, "routing table empty after discovery wait"

    print(f"  baseline: {len(baseline)} users, polling for {STABILITY_DURATION}s...")

    flaps: dict[str, int] = {}
    absent_in_last: set[str] = set()

    t0 = time.time()
    deadline = t0 + STABILITY_DURATION
    poll_count = 0

    while time.time() < deadline:
        time.sleep(POLL_INTERVAL)
        poll_count += 1

        present_now = {
            entry["user_id"]: entry["connections"][0]
            for entry in observer.router_table()
            if entry["connections"]
        }

        for uid in baseline:
            if uid in absent_in_last and uid in present_now:
                flaps[uid] = flaps.get(uid, 0) + 1

        absent_in_last = set(baseline.keys()) - set(present_now.keys())

    total_flaps = sum(flaps.values())
    assert total_flaps == 0, (
        f"route flapping detected across {poll_count} polls:\n"
        + "\n".join(f"  {uid}: {count} flap(s)" for uid, count in flaps.items())
    )
