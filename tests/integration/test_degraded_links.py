# test_degraded_links.py
#
# Verifies feed message delivery under degraded link conditions using
# Linux `tc netem` to inject packet loss on a specific link.
#
# The test applies packet loss to the interface connecting two nodes, then
# sends feed messages and measures how many arrive at all nodes. A healthy
# qaul mesh should handle moderate packet loss through retransmission and
# multi-path routing (on topologies with redundant paths).
#
# On line topologies there is only one path — packet loss on the critical
# link will degrade delivery. The test records actual delivery rate rather
# than asserting a specific threshold, allowing observation of how qaul
# handles degraded conditions.
#
# Procedure:
#   1. Start the topology and wait for full convergence
#   2. Apply packet loss (LOSS_PERCENT%) to the link between the two middle
#      nodes using `tc qdisc add dev <iface> root netem loss X%`
#   3. Send SEND_COUNT feed messages from the sender node
#   4. Wait for propagation, then count how many arrived at each node
#   5. Record delivery rates — assert at least MIN_DELIVERY_RATE on the
#      receiver directly adjacent to the degraded link
#   6. Remove the packet loss (cleanup)
#
# Interface naming: meshnet-lab uses veth pairs named after the link.
# In a line-5 topology, the interface on node 0002 facing node 0003 is
# typically named `br-0002-0003` or `veth-0002-0003`. The exact name
# depends on meshnet-lab's naming convention — this test reads it from
# the topology file and constructs the name accordingly.

import subprocess
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
PROPAGATION_WAIT = 90
POLL_INTERVAL = 5
LOSS_PERCENT = 30  # packet loss to apply on the degraded link
SEND_COUNT = 10  # number of feed messages to send
# minimum delivery rate expected even under packet loss (retransmission helps)
MIN_DELIVERY_RATE = 0.5


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def _get_link_interface(node_id: str, neighbour_id: str) -> str:
    """
    Return the name of the veth interface on node_id that connects to neighbour_id.
    meshnet-lab names these: veth-<node_id>-<neighbour_id>
    """
    return f"veth-{node_id}-{neighbour_id}"


def _apply_packet_loss(node_id: str, iface: str, loss_percent: int):
    """Apply packet loss to an interface inside a network namespace."""
    subprocess.run(
        [
            "sudo",
            "ip",
            "netns",
            "exec",
            node_id,
            "tc",
            "qdisc",
            "add",
            "dev",
            iface,
            "root",
            "netem",
            "loss",
            f"{loss_percent}%",
        ],
        check=True,
    )


def _remove_packet_loss(node_id: str, iface: str):
    """Remove tc qdisc from an interface inside a network namespace."""
    subprocess.run(
        [
            "sudo",
            "ip",
            "netns",
            "exec",
            node_id,
            "tc",
            "qdisc",
            "del",
            "dev",
            iface,
            "root",
        ],
        check=False,  # don't fail if qdisc wasn't set
    )


def test_feed_delivery_under_packet_loss(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    propagation_wait: int = PROPAGATION_WAIT,
    poll_interval: int = POLL_INTERVAL,
    loss_percent: int = LOSS_PERCENT,
    send_count: int = SEND_COUNT,
    min_delivery_rate: float = MIN_DELIVERY_RATE,
) -> dict:
    """
    Apply packet loss on a central link, send feed messages, and record
    delivery rates across all nodes.
    """
    sorted_ids = sorted(node_ids)
    nodes = [Node(nid) for nid in sorted_ids]
    sender = nodes[0]

    # pick the two middle nodes as the degraded link
    mid = len(sorted_ids) // 2
    loss_node_a = sorted_ids[mid - 1]
    loss_node_b = sorted_ids[mid]
    iface_a = _get_link_interface(loss_node_a, loss_node_b)
    iface_b = _get_link_interface(loss_node_b, loss_node_a)

    # wait for routing convergence
    print(f"  waiting up to {discovery_wait}s for convergence...")
    t_start = time.time()
    deadline = t_start + discovery_wait
    expected_remote = len(sorted_ids) - 1

    while time.time() < deadline:
        table = sender.router_table()
        remote_count = sum(
            1
            for e in table
            if e["connections"] and e["connections"][0]["module"].lower() != "local"
        )
        if remote_count >= expected_remote:
            elapsed = round(time.time() - t_start, 1)
            print(f"  convergence after {elapsed}s ({remote_count} remote nodes)")
            break
        time.sleep(poll_interval)
    else:
        raise AssertionError(f"convergence not reached within {discovery_wait}s")

    # apply packet loss on both sides of the degraded link
    print(
        f"  applying {loss_percent}% packet loss on link "
        f"{loss_node_a}↔{loss_node_b} "
        f"(ifaces: {iface_a}, {iface_b})"
    )
    _apply_packet_loss(loss_node_a, iface_a, loss_percent)
    _apply_packet_loss(loss_node_b, iface_b, loss_percent)

    # send messages
    stamp = int(time.time())
    sent_contents = []
    for i in range(send_count):
        content = f"degraded-{stamp}-{i:03d}"
        sender.send_feed_message(content)
        sent_contents.append(content)
        time.sleep(0.2)  # brief gap between sends

    print(f"  sent {send_count} messages from {sender.id}")

    # wait for propagation
    print(
        f"  waiting {propagation_wait}s for propagation under {loss_percent}% loss..."
    )
    time.sleep(propagation_wait)

    # remove packet loss before measuring results
    print("  removing packet loss...")
    _remove_packet_loss(loss_node_a, iface_a)
    _remove_packet_loss(loss_node_b, iface_b)

    # measure delivery on each node
    delivery_rates = {}
    for node in nodes:
        contents = node.feed_message_contents()
        received = sum(1 for c in sent_contents if c in contents)
        rate = received / send_count
        delivery_rates[node.id] = {
            "received": received,
            "sent": send_count,
            "rate": round(rate, 3),
        }
        status = "ok" if rate >= min_delivery_rate else "LOW"
        print(
            f"    node {node.id}: {received}/{send_count} "
            f"({rate * 100:.0f}%)  [{status}]"
        )

    # the node on the far side of the degraded link (nodes[-1]) is most affected
    far_node_id = sorted_ids[-1]
    far_rate = delivery_rates[far_node_id]["rate"]

    assert far_rate >= min_delivery_rate, (
        f"delivery rate to far node {far_node_id} is {far_rate * 100:.0f}% "
        f"— below minimum {min_delivery_rate * 100:.0f}% "
        f"under {loss_percent}% packet loss"
    )

    print(
        f"  PASS: {far_node_id} received {far_rate * 100:.0f}% of messages "
        f"under {loss_percent}% packet loss "
        f"(min required: {min_delivery_rate * 100:.0f}%)"
    )

    return {
        "passed": True,
        "loss_percent": loss_percent,
        "send_count": send_count,
        "degraded_link": f"{loss_node_a}↔{loss_node_b}",
        "delivery_rates": delivery_rates,
        "notes": (
            f"{loss_percent}% packet loss on {loss_node_a}↔{loss_node_b}: "
            f"far node delivery rate {far_rate * 100:.0f}%"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        result = test_feed_delivery_under_packet_loss()
        print(f"  result: {result}")
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
