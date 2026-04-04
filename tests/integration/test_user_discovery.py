"""Verify that nodes discover each other's user accounts."""

import time

import pytest

from conftest import all_topology_files
from lib.node import Node

TOPOLOGIES = all_topology_files()


@pytest.mark.correctness
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
class TestUserDiscovery:

    def test_nodes_discover_neighbours(self, topology_network):
        """First node should discover its immediate neighbour within 30s."""
        time.sleep(30)

        ids = topology_network["node_ids"]
        node_a = topology_network["nodes"][ids[0]]
        node_b = topology_network["nodes"][ids[1]]

        id_b = node_b.local_user_id()
        known_ids = node_a.known_user_ids()

        assert id_b in known_ids, (
            f"node {ids[0]} does not know about node {ids[1]} after 30s\n"
            f"  known user ids: {known_ids}"
        )

    def test_user_fields_are_present(self, topology_network):
        """User entries must contain all expected fields."""
        first = topology_network["nodes"][topology_network["node_ids"][0]]
        users = first.known_users()

        assert len(users) > 0, "no known users"

        user = users[0]
        for field in ("id", "name", "verified", "blocked", "connectivity", "group_id", "public_key"):
            assert field in user, f"user entry missing field: {field}"
