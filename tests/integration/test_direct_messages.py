"""
Verify that a direct chat message is delivered to the intended recipient
and is NOT received by an intermediate node.
"""

import time

import pytest

from conftest import all_topology_files
from lib.node import Node
from lib.topology import load_node_ids

TOPOLOGIES = all_topology_files()

# defaults for standalone / run_network_tests.py usage
TOPOLOGY = "topologies/line-5.json"
NODE_IDS = load_node_ids(TOPOLOGY)
DISCOVERY_WAIT = 200
DELIVERY_WAIT = 60
POLL_INTERVAL = 5


def setup():
    from lib.network import apply_topology, start_qaul, wait_for_nodes
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    from lib.network import stop_qaul, clear_topology
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

    # wait until sender knows the recipient
    recipient_name = f"test-{recipient_id}"
    print(f"  waiting for {recipient_name} (timeout {discovery_wait}s)...")
    t_start = time.time()
    deadline = t_start + discovery_wait
    group_id = None

    while time.time() < deadline:
        users = sender.known_users()
        user_by_name = {u["name"]: u for u in users}
        if recipient_name in user_by_name:
            group_id = user_by_name[recipient_name]["group_id"]
            print(f"  found after {round(time.time() - t_start, 1)}s, group_id={group_id}")
            break
        time.sleep(poll_interval)
    else:
        known = [u["name"] for u in sender.known_users()]
        raise AssertionError(f"{recipient_name} not found after {discovery_wait}s — known: {known}")

    recipient = Node(recipient_id)
    content = f"direct-msg-test-{int(time.time())}"
    sender.send_chat_message(group_id, content)
    print(f"  sent: '{content}'")
    t_send = time.time()

    # poll recipient
    delivered_at = None
    while time.time() < t_send + delivery_wait:
        time.sleep(poll_interval)
        try:
            conv = recipient.conversation(group_id)
            msgs = [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]
            if content in msgs:
                delivered_at = round(time.time() - t_send, 1)
                break
        except Exception:
            pass

    assert delivered_at is not None, f"not delivered to {recipient_id} within {delivery_wait}s"

    # snooper must NOT have it
    snooper_msgs = []
    try:
        conv = snooper.conversation(group_id)
        snooper_msgs = [c for msg in conv.get("messages", []) for c in (msg.get("content") or [])]
    except Exception:
        pass

    assert content not in snooper_msgs, f"snooper {snooper_id} received the message"
    print(f"  PASS: delivered in {delivered_at}s, not on {snooper_id}")

    return {
        "passed": True,
        "sender": sender_id,
        "recipient": recipient_id,
        "snooper": snooper_id,
        "delivery_time_s": delivered_at,
        "notes": f"delivered in {delivered_at}s, not seen on {snooper_id}",
    }


# --- pytest entry point ---

@pytest.mark.network
@pytest.mark.parametrize("converged_network", TOPOLOGIES, indirect=True)
def test_direct_message_unicast_pytest(converged_network):
    """pytest wrapper using the converged_network fixture."""
    test_direct_message_unicast(
        node_ids=converged_network["node_ids"],
        discovery_wait=1,  # already converged
    )


if __name__ == "__main__":
    try:
        setup()
        test_direct_message_unicast()
    finally:
        teardown()
