# test_neighbour_detection.py
#
# Verifies that each node's direct LAN neighbours exactly match the links
# defined in the topology file — no more, no less.
#
# The topology JSON defines which nodes are directly connected via links.
# After startup, each qauld instance should detect exactly those direct
# neighbours via mDNS peer discovery and RTT pings. This test asserts:
#
#   - Every node listed as a direct link in the topology appears as a
#     LAN neighbour in `router neighbours`
#   - No node that is NOT a direct link appears as a LAN neighbour
#   - RTT is non-zero for every detected neighbour (the link is active)
#
# The topology uses hex node IDs (0000, 0001, ...) but `router neighbours`
# returns libp2p peer IDs (base58). The test resolves this by querying
# `node info` on every node to build a hex_id → peer_id mapping first.

import json
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
# neighbour detection is driven by mDNS and RTT pings (every 5s),
# so it settles much faster than full routing convergence.
# 30s is sufficient for direct neighbours to appear.
NEIGHBOUR_WAIT = 30


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def _load_expected_neighbours(topology_file: str) -> dict[str, set[str]]:
    """
    Read the topology JSON and return a dict mapping each node hex_id
    to the set of hex_ids it is directly connected to via links.
    """
    with open(topology_file) as f:
        data = json.load(f)

    adjacency: dict[str, set[str]] = {n["id"]: set() for n in data["nodes"]}

    for link in data["links"]:
        src = link["source"]
        tgt = link["target"]
        adjacency[src].add(tgt)
        adjacency[tgt].add(src)

    return adjacency


def _build_peer_id_map(node_ids: list[str]) -> dict[str, str]:
    """
    Query node info on every node and return a mapping of hex_id → peer_id.
    """
    mapping = {}
    for nid in node_ids:
        info = Node(nid).node_info()
        mapping[nid] = info["node_id"]
    return mapping


def test_neighbours_match_topology(
    node_ids: list[str] = NODE_IDS,
    topology_file: str = TOPOLOGY,
    neighbour_wait: int = NEIGHBOUR_WAIT,
) -> dict:
    """
    For every node, assert that its LAN neighbours exactly match the
    direct links in the topology file.
    """
    print(f"  waiting {neighbour_wait}s for neighbour detection to settle...")
    time.sleep(neighbour_wait)

    print("  building hex_id → peer_id mapping...")
    peer_id_map = _build_peer_id_map(node_ids)
    # reverse map: peer_id → hex_id (for readable error messages)
    reverse_map = {v: k for k, v in peer_id_map.items()}

    print("  loading expected neighbours from topology file...")
    expected_neighbours = _load_expected_neighbours(topology_file)

    failures = []
    rtt_violations = []
    total_checked = 0

    for hex_id in sorted(node_ids):
        node = Node(hex_id)
        result = node.router_neighbours()
        lan_neighbours = result.get("lan", [])

        actual_peer_ids = {n["node_id"] for n in lan_neighbours}
        actual_hex_ids = {reverse_map[pid] if pid in reverse_map else pid for pid in actual_peer_ids}

        expected_hex_ids = expected_neighbours[hex_id]
        expected_peer_ids = {peer_id_map[n] for n in expected_hex_ids}

        missing = expected_hex_ids - actual_hex_ids
        unexpected = actual_hex_ids - expected_hex_ids

        if missing:
            failures.append(
                f"  node {hex_id}: missing neighbours {missing} "
                f"(expected from topology)"
            )
        if unexpected:
            failures.append(
                f"  node {hex_id}: unexpected neighbours {unexpected} "
                f"(not in topology links)"
            )

        # check RTT is non-zero for all detected neighbours
        for n in lan_neighbours:
            if n["rtt"] == 0:
                rtt_violations.append(
                    f"  node {hex_id}: neighbour {reverse_map[n['node_id']] if n['node_id'] in reverse_map else n['node_id']} "
                    f"has RTT=0 (link not active)"
                )

        total_checked += 1
        status = "ok" if not missing and not unexpected else "MISMATCH"
        print(
            f"    node {hex_id}: expected {sorted(expected_hex_ids)} "
            f"got {sorted(actual_hex_ids)}  [{status}]"
        )

    assert not failures, (
        f"neighbour mismatch on {len(failures)} node(s):\n" + "\n".join(failures)
    )
    assert not rtt_violations, (
        f"zero RTT on {len(rtt_violations)} link(s):\n" + "\n".join(rtt_violations)
    )

    print(f"  PASS: all {total_checked} nodes have correct LAN neighbours")

    return {
        "passed": True,
        "nodes_checked": total_checked,
        "notes": f"all {total_checked} nodes have correct LAN neighbours matching topology",
    }


if __name__ == "__main__":
    try:
        setup()
        test_neighbours_match_topology()
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
