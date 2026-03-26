"""Verify that each node's LAN neighbours match the topology links exactly."""

import json
import time

import pytest

from conftest import all_topology_files
from lib.node import Node

TOPOLOGIES = all_topology_files()


def _load_expected_neighbours(topology_file: str) -> dict[str, set[str]]:
    with open(topology_file) as f:
        data = json.load(f)
    adj: dict[str, set[str]] = {n["id"]: set() for n in data["nodes"]}
    for link in data["links"]:
        adj[link["source"]].add(link["target"])
        adj[link["target"]].add(link["source"])
    return adj


@pytest.mark.routing
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
def test_neighbours_match_topology(topology_network):
    """Each node's LAN neighbours must exactly match topology links."""
    time.sleep(30)  # mDNS + ping settle time

    nodes = topology_network["nodes"]
    topo_file = topology_network["file"]

    # build hex_id <-> peer_id mapping
    peer_map = {nid: node.node_info()["node_id"] for nid, node in nodes.items()}
    reverse = {v: k for k, v in peer_map.items()}

    expected = _load_expected_neighbours(topo_file)
    failures = []
    rtt_violations = []

    for hex_id in sorted(nodes.keys()):
        result = nodes[hex_id].router_neighbours()
        lan = result.get("lan", [])

        actual_hex = {reverse.get(n["node_id"], n["node_id"]) for n in lan}
        expected_hex = expected[hex_id]

        missing = expected_hex - actual_hex
        unexpected = actual_hex - expected_hex
        if missing:
            failures.append(f"node {hex_id}: missing {missing}")
        if unexpected:
            failures.append(f"node {hex_id}: unexpected {unexpected}")

        for n in lan:
            if n["rtt"] == 0:
                failures.append(f"node {hex_id}: neighbour {reverse.get(n['node_id'], n['node_id'])} RTT=0")

    assert not failures, "neighbour mismatches:\n  " + "\n  ".join(failures)
