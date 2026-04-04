"""Verify that all nodes start up and respond to basic queries."""

import pytest

from conftest import all_topology_files
from lib.node import Node

TOPOLOGIES = all_topology_files()


@pytest.mark.correctness
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
class TestNodeStartup:

    def test_all_nodes_respond(self, topology_network):
        """Every node must respond to node info without error."""
        for nid, node in topology_network["nodes"].items():
            info = node.node_info()
            assert info is not None, f"node {nid} returned no info"

    def test_node_info_fields(self, topology_network):
        """node info must return node_id and a non-empty addresses list."""
        for nid, node in topology_network["nodes"].items():
            info = node.node_info()
            assert "node_id" in info, f"node {nid}: missing 'node_id'"
            assert "addresses" in info, f"node {nid}: missing 'addresses'"
            assert len(info["addresses"]) > 0, f"node {nid}: addresses empty"

    def test_node_ids_are_distinct(self, topology_network):
        """Each node must have a unique node_id."""
        ids = [node.node_id() for node in topology_network["nodes"].values()]
        assert len(ids) == len(set(ids)), f"duplicate node IDs: {ids}"
