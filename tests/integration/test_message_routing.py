"""Verify that a feed message from one node reaches a distant node."""

import time

import pytest

from conftest import all_topology_files, discovery_wait_for
from lib.node import Node

TOPOLOGIES = all_topology_files()
TEST_MESSAGE = "hello from first node"


@pytest.mark.correctness
@pytest.mark.parametrize("topology_network", TOPOLOGIES, indirect=True)
class TestMessageRouting:

    def test_feed_message_reaches_far_node(self, topology_network):
        """A feed message from the first node must reach the last node."""
        ids = topology_network["node_ids"]
        wait = discovery_wait_for(topology_network["file"])

        sender = topology_network["nodes"][ids[0]]
        receiver = topology_network["nodes"][ids[-1]]

        time.sleep(wait)
        sender.send_feed_message(TEST_MESSAGE)
        time.sleep(30)

        contents = receiver.feed_message_contents()
        assert TEST_MESSAGE in contents, (
            f"message not found on node {ids[-1]} — seen: {contents}"
        )

    def test_feed_message_fields_are_present(self, topology_network):
        """Feed messages should have all expected fields."""
        first = topology_network["nodes"][topology_network["node_ids"][0]]
        messages = first.feed_messages()

        if not messages:
            pytest.skip("no feed messages yet")

        msg = messages[0]
        for field in ("index", "message_id", "sender_id", "content", "time_sent", "timestamp_sent"):
            assert field in msg, f"feed message missing field: {field}"
