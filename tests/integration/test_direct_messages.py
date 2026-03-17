# test_direct_messages.py
#
# Verifies that a direct chat message is delivered to the intended recipient
# and is NOT received by an intermediate node that is not the recipient.
#
# In qaul, direct messages use a group_id derived from the two participants'
# user IDs (GroupId::from_peers). The message is routed unicast through the
# mesh — only the two participants store it. Intermediate nodes relay the
# encrypted payload but never store it in their own conversation list.
#
# Procedure:
#   1. Start the topology and wait for full convergence
#   2. On the sender node (0000), look up the recipient node's user entry
#      from `users list` to get its group_id
#   3. Send a chat message from sender to recipient
#   4. Poll the recipient's conversation until the message appears (or timeout)
#   5. Assert an intermediate node (the middle node) has no messages in
#      its conversation for the same group_id
#
# Uses line-5 (0000—0001—0002—0003—0004):
#   sender    = 0000 (one end)
#   recipient = 0004 (other end, 4 hops away)
#   snooper   = 0002 (middle node, relays messages but should not store them)

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
DISCOVERY_WAIT = 120
DELIVERY_WAIT = 60
POLL_INTERVAL = 5


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def _get_group_id_for_user(sender: Node, target_user_id: str) -> str:
    """
    Look up the group_id (direct-message conversation ID) that sender uses
    to address target_user_id.  This comes from `users list --json`.
    """
    for user in sender.known_users():
        if user["id"] == target_user_id:
            gid = user.get("group_id")
            if gid:
                return gid
    raise ValueError(
        f"user {target_user_id} not found in {sender.id}'s users list "
        f"or missing group_id field"
    )


def _local_user_id(node: Node) -> str:
    """Return the qaul user ID of the node's own local account."""
    for entry in node.router_table():
        if entry["connections"] and entry["connections"][0]["module"] == "Local":
            return entry["user_id"]
    raise ValueError(f"No Local entry in router table for node {node.id}")


def test_direct_message_unicast(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    delivery_wait: int = DELIVERY_WAIT,
    poll_interval: int = POLL_INTERVAL,
) -> dict:
    """
    Send a direct message from the first node to the last node and assert:
      - the recipient receives the message
      - the middle node does not receive the message
    """
    sorted_ids = sorted(node_ids)
    sender_id = sorted_ids[0]
    recipient_id = sorted_ids[-1]
    snooper_id = sorted_ids[len(sorted_ids) // 2]

    sender = Node(sender_id)
    recipient = Node(recipient_id)
    snooper = Node(snooper_id)

    # wait for routing convergence: sender must know all other nodes
    print(
        f"  waiting for sender {sender_id} to discover all nodes "
        f"(timeout {discovery_wait}s)..."
    )
    t_start = time.time()
    deadline = t_start + discovery_wait
    recipient_user_id = _local_user_id(recipient)

    while time.time() < deadline:
        known = sender.known_user_ids()
        if recipient_user_id in known:
            elapsed = round(time.time() - t_start, 1)
            print(f"  recipient {recipient_id} discovered by sender after {elapsed}s")
            break
        time.sleep(poll_interval)
    else:
        raise AssertionError(
            f"sender {sender_id} did not discover recipient {recipient_id} "
            f"(user {recipient_user_id}) within {discovery_wait}s"
        )

    # get the direct-message group_id from sender's perspective
    group_id = _get_group_id_for_user(sender, recipient_user_id)
    print(f"  direct message group_id: {group_id}")

    # send the message
    test_content = f"direct-msg-test-{int(time.time())}"
    sender.send_chat_message(group_id, test_content)
    print(f"  sent message: '{test_content}'")
    t_send = time.time()

    # poll recipient until the message appears
    delivered_at = None
    deadline = t_send + delivery_wait

    while time.time() < deadline:
        time.sleep(poll_interval)
        elapsed = round(time.time() - t_send, 1)
        try:
            conv = recipient.conversation(group_id)
            messages = conv.get("messages", [])
            contents = [
                c
                for msg in messages
                for c in (msg.get("content") or [])
            ]
            if test_content in contents:
                delivered_at = elapsed
                print(f"  message delivered to recipient after {elapsed}s")
                break
        except Exception as e:
            print(f"    +{elapsed}s  recipient conversation error: {e}")

    assert delivered_at is not None, (
        f"message '{test_content}' was not delivered to recipient {recipient_id} "
        f"within {delivery_wait}s"
    )

    # check that snooper (middle node) does NOT have the message
    snooper_messages = []
    try:
        conv = snooper.conversation(group_id)
        snooper_messages = [
            c
            for msg in conv.get("messages", [])
            for c in (msg.get("content") or [])
        ]
    except Exception:
        # conversation not found on snooper — expected
        pass

    assert test_content not in snooper_messages, (
        f"intermediate node {snooper_id} received the direct message — "
        f"message should only be stored at sender and recipient"
    )

    print(
        f"  PASS: message delivered to {recipient_id} in {delivered_at}s, "
        f"not present on intermediate node {snooper_id}"
    )

    return {
        "passed": True,
        "sender": sender_id,
        "recipient": recipient_id,
        "snooper": snooper_id,
        "group_id": group_id,
        "delivery_time_s": delivered_at,
        "notes": (
            f"direct message delivered to {recipient_id} in {delivered_at}s, "
            f"not seen on {snooper_id}"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        result = test_direct_message_unicast()
        print(f"  result: {result}")
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
