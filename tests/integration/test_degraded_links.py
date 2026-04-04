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
DISCOVERY_WAIT = 200
PROPAGATION_WAIT = 120
POLL_INTERVAL = 5
PUBSUB_WARMUP = 30  # seconds after routing convergence before applying loss
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


def _node_pid(node_id: str) -> str:
    """Return the PID of the qauld process for a node.

    meshnet-lab starts qauld with --name=test-<node_id>, so we find it
    via pgrep rather than a PID file (meshnet-lab does not write PID files).
    """
    result = subprocess.run(
        ["pgrep", "-f", f"qauld --name=test-{node_id}"],
        capture_output=True, text=True, check=True,
    )
    return result.stdout.strip()


def _apply_packet_loss(node_id: str, loss_percent: int):
    """Apply packet loss on a node's uplink interface.

    Each node has a single 'uplink' interface connecting it to the meshnet-lab
    switch. Uses nsenter via /proc/<pid>/ns/net to enter the node's namespace.
    """
    pid = _node_pid(node_id)
    subprocess.run(
        [
            "sudo", "nsenter", f"--net=/proc/{pid}/ns/net",
            "tc", "qdisc", "add", "dev", "uplink",
            "root", "netem", "loss", f"{loss_percent}%",
        ],
        check=True,
    )


def _remove_packet_loss(node_id: str):
    """Remove tc qdisc from a node's uplink interface."""
    pid = _node_pid(node_id)
    subprocess.run(
        [
            "sudo", "nsenter", f"--net=/proc/{pid}/ns/net",
            "tc", "qdisc", "del", "dev", "uplink", "root",
        ],
        check=False,  # don't fail if qdisc wasn't set
    )


def test_feed_delivery_under_packet_loss(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    propagation_wait: int = PROPAGATION_WAIT,
    poll_interval: int = POLL_INTERVAL,
    pubsub_warmup: int = PUBSUB_WARMUP,
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

    # apply packet loss on the middle node's uplink — this degrades all traffic
    # passing through the centre of the line topology
    loss_node = sorted_ids[len(sorted_ids) // 2]

    # wait until sender's users list contains all nodes
    print(f"  waiting up to {discovery_wait}s for all {len(sorted_ids)} users to appear...")
    t_start = time.time()
    deadline = t_start + discovery_wait

    while time.time() < deadline:
        known = sender.known_users()
        if len(known) >= len(sorted_ids):
            elapsed = round(time.time() - t_start, 1)
            print(f"  all {len(known)} users known after {elapsed}s")
            break
        time.sleep(poll_interval)
    else:
        raise AssertionError(f"convergence not reached within {discovery_wait}s")

    print(f"  waiting {pubsub_warmup}s for pubsub mesh to stabilise before degrading link...")
    time.sleep(pubsub_warmup)

    print(f"  applying {loss_percent}% packet loss on node {loss_node}'s uplink")
    _apply_packet_loss(loss_node, loss_percent)

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
    _remove_packet_loss(loss_node)

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
        "degraded_node": loss_node,
        "delivery_rates": delivery_rates,
        "notes": (
            f"{loss_percent}% packet loss on node {loss_node} uplink: "
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
