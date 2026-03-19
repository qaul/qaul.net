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
#   1. Start the topology and wait for all nodes to appear in sender's users list
#   2. Find the recipient's entry by name ("test-<node_id>") and read group_id
#   3. Send a chat message from sender to recipient
#   4. Poll the recipient's conversation until the message appears (or timeout)
#   5. Assert the middle node has no messages for the same group_id
#
# Uses line-5 (0000—0001—0002—0003—0004):
#   sender    = 0000 (one end)
#   recipient = 0004 (other end, 4 hops away)
#   snooper   = 0002 (middle node, relays but should not store)

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
DISCOVERY_WAIT = 200
DELIVERY_WAIT = 60
POLL_INTERVAL = 5


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


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
    snooper = Node(snooper_id)

    # wait until sender's users list contains all nodes — users list and
    # routing table converge together, and users list has the group_id we need.
    # nodes are named "test-<node_id>" by meshnet-lab.
    print(f"  waiting for all {len(sorted_ids)} users in sender's users list (timeout {discovery_wait}s)...")
    t_start = time.time()
    deadline = t_start + discovery_wait
    group_id = None

    while time.time() < deadline:
        users = sender.known_users()
        user_by_name = {u["name"]: u for u in users}
        recipient_name = f"test-{recipient_id}"
        if recipient_name in user_by_name:
            elapsed = round(time.time() - t_start, 1)
            group_id = user_by_name[recipient_name]["group_id"]
            print(f"  recipient {recipient_id} found after {elapsed}s, group_id: {group_id}")
            break
        time.sleep(poll_interval)
    else:
        known_names = [u["name"] for u in sender.known_users()]
        raise AssertionError(
            f"recipient {recipient_id} (test-{recipient_id}) not found in sender's "
            f"users list after {discovery_wait}s — known: {known_names}"
        )

    recipient = Node(recipient_id)

    # send the message
    test_content = f"direct-msg-test-{int(time.time())}"
    sender.send_chat_message(group_id, test_content)
    print(f"  sent: '{test_content}'")
    t_send = time.time()

    # poll recipient until the message appears
    delivered_at = None
    deadline = t_send + delivery_wait

    while time.time() < deadline:
        time.sleep(poll_interval)
        elapsed = round(time.time() - t_send, 1)
        try:
            conv = recipient.conversation(group_id)
            contents = [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]
            if test_content in contents:
                delivered_at = elapsed
                print(f"  delivered to {recipient_id} after {elapsed}s")
                break
        except Exception as e:
            print(f"    +{elapsed}s  conversation error: {e}")

    assert delivered_at is not None, (
        f"message not delivered to {recipient_id} within {delivery_wait}s"
    )

    # check snooper does NOT have the message
    snooper_contents = []
    try:
        conv = snooper.conversation(group_id)
        snooper_contents = [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]
    except Exception:
        pass  # conversation not found on snooper — expected

    assert test_content not in snooper_contents, (
        f"intermediate node {snooper_id} received the direct message"
    )

    print(f"  PASS: delivered to {recipient_id} in {delivered_at}s, not present on {snooper_id}")

    return {
        "passed": True,
        "sender": sender_id,
        "recipient": recipient_id,
        "snooper": snooper_id,
        "group_id": group_id,
        "delivery_time_s": delivered_at,
        "notes": f"direct message delivered to {recipient_id} in {delivered_at}s, not seen on {snooper_id}",
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
