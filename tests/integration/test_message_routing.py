# confirms that a feed message sent from a node reaches
# anoher node

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

TOPOLOGY = "topologies/line-5.json"
NODE_IDS = [f"{i:04x}" for i in range(5)]
TEST_MESSAGE = "hello from node 0"


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=30)


def teardown():
    stop_qaul()
    clear_topology()


def test_feed_message_reaches_far_node(wait_seconds=90):
    node_sender = Node("0000")
    node_receiver = Node("0004")

    node_sender.send_feed_message(TEST_MESSAGE)

    print(f"  waiting {wait_seconds}s for message propagation...")
    time.sleep(wait_seconds)

    contents = node_receiver.feed_message_contents()

    assert TEST_MESSAGE in contents, (
        f"message '{TEST_MESSAGE}' not found on node 0004 after {wait_seconds}s\n"
        f"  messages seen: {contents}"
    )
    print("  PASS: feed message reached node 0004")


def test_feed_message_fields_are_present():
    """
    Feed messages should have all expected fields in JSON output.
    """
    node = Node("0000")
    messages = node.feed_messages()

    if not messages:
        print("  SKIP: no feed messages yet")
        return

    msg = messages[0]
    for field in (
        "index",
        "message_id",
        "sender_id",
        "content",
        "time_sent",
        "timestamp_sent",
    ):
        assert field in msg, f"feed message missing field: {field}"

    print("  PASS: feed message entries contain all expected fields")


if __name__ == "__main__":
    try:
        setup()
        test_feed_message_reaches_far_node()
        test_feed_message_fields_are_present()
    finally:
        teardown()
