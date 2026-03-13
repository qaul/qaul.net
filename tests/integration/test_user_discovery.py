# confirm's that nodes can see each other
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

TOPOLOGY_FILE = "topologies/line-5.json"
# since we're using a five node line topology
NODE_IDS = [f"{i:04x}" for i in range(5)]


def setup():
    apply_topology(TOPOLOGY_FILE)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=10)


def teardown():
    stop_qaul()
    clear_topology()


def test_nodes_discover_neighbours(interval=60):
    print(f"  waiting {interval}s for node discovery...")
    time.sleep(interval)

    node_a = Node("0000")
    node_b = Node("0001")

    id_b = local_user_id(node_b) 
    known_ids = node_a.known_user_ids()

    assert id_b in known_ids, (
        f"node 0000 does not know about node 0001 after {interval}s\n"
        f"  known user ids: {known_ids}"
    )
    print("  PASS: node 0000 knows about node 0001")


def test_user_fields_are_present():
    """
    check if the returned res by users list has expected fields
    """
    node = Node("0000")
    users = node.known_users()

    assert len(users) > 0, "node 0000 has no known users"

    user = users[0]
    for field in (
        "id",
        "name",
        "verified",
        "blocked",
        "connectivity",
        "group_id",
        "public_key",
    ):
        assert field in user, f"user entry missing field: {field}"

    print("  PASS: user entries contain all expected fields")

def local_user_id(node: Node) -> str:
    """"returns the id of the node's own local user account"""
    for user in node.known_users():
        if any(c["module"] == "LOCAL" for c in user["connections"]):
            return user["id"]
    raise ValueError(f"No LOCAL user found on node {node.id}")

if __name__ == "__main__":
    try:
        setup()
        test_nodes_discover_neighbours()
        test_user_fields_are_present()
    finally:
        teardown()

