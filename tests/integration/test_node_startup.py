# confirm that all nodes startup

import sys

sys.path.insert(0, ".")

from lib.network import (
    apply_topology,
    clear_topology,
    start_qaul,
    stop_qaul,
    wait_for_nodes,
)
from lib.node import Node

TOPOLOGY = "topologies/line-5.json"
NODE_IDS = [f"{i:04x}" for i in range(5)]


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=30)


def teardown():
    stop_qaul()
    clear_topology()


def test_all_nodes_respond():
    """node must respond to node info without error"""
    for nid in NODE_IDS:
        node = Node(nid)
        info = node.node_info()
        assert info is not None, f"node {nid} returned no info"
    print("  PASS: all nodes responded")


def test_node_info_fields():
    """node info must return node_id and a non-empty addresses list."""
    for nid in NODE_IDS:
        info = Node(nid).node_info()
        assert "node_id" in info, f"node {nid}: missing field 'node_id'"
        assert "addresses" in info, f"node {nid}: missing field 'addresses'"
        assert len(info["addresses"]) > 0, f"node {nid}: addresses list is empty"
    print("  PASS: node info fields are well-formed on all nodes")


def test_node_ids_are_distinct():
    """Each node must have a unique node_id."""
    ids = [Node(nid).node_id() for nid in NODE_IDS]
    assert len(ids) == len(set(ids)), f"duplicate node IDs found: {ids}"
    print("  PASS: all node IDs are distinct")


if __name__ == "__main__":
    try:
        setup()
        test_all_nodes_respond()
        test_node_info_fields()
        test_node_ids_are_distinct()
    finally:
        teardown()
