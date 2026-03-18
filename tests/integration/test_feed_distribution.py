# test_feed_distribution.py
#
# Verifies that a feed message sent from one node reaches ALL other nodes
# with no duplicates.
#
# Feed messages in qaul use libp2p floodsub — every connected peer forwards
# each message to all its peers. On a fully converged mesh, every node should
# eventually receive every feed message exactly once.
#
# Procedure:
#   1. Start the topology and wait for full convergence
#   2. Send one feed message from each node using a unique content string
#   3. Poll every node's feed list until all expected messages appear
#   4. Assert each message appears exactly once on every node (no duplicates)
#
# The test sends N messages (one per node) and expects every node to end up
# with all N messages. This exercises multi-hop propagation across the mesh.

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
PROPAGATION_WAIT = 120
POLL_INTERVAL = 5
PUBSUB_WARMUP = 30  # seconds to wait after routing convergence for the pubsub
                    # mesh to stabilise before sending — floodsub connections
                    # lag behind routing table convergence on cold topologies


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def test_feed_messages_reach_all_nodes(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    propagation_wait: int = PROPAGATION_WAIT,
    poll_interval: int = POLL_INTERVAL,
    pubsub_warmup: int = PUBSUB_WARMUP,
) -> dict:
    """
    Send one feed message from each node, then assert every node receives
    all messages exactly once.
    """
    sorted_ids = sorted(node_ids)
    nodes = [Node(nid) for nid in sorted_ids]

    # wait for routing convergence before sending — nodes must know their
    # neighbours for feed messages to propagate
    print(f"  waiting up to {discovery_wait}s for routing convergence...")
    t_start = time.time()
    deadline = t_start + discovery_wait
    expected_count = len(sorted_ids)

    # convergence check: first node must see all others in its routing table
    observer = nodes[0]
    remote_count = 0
    while time.time() < deadline:
        table = observer.router_table()
        # count non-local entries
        remote_count = sum(
            1
            for e in table
            if e["connections"] and e["connections"][0]["module"].lower() != "local"
        )
        if remote_count >= expected_count - 1:
            elapsed = round(time.time() - t_start, 1)
            print(
                f"  convergence reached after {elapsed}s ({remote_count} remote nodes)"
            )
            break
        time.sleep(poll_interval)
    else:
        raise AssertionError(
            f"routing convergence not reached within {discovery_wait}s — "
            f"observer sees {remote_count} remote nodes, expected {expected_count - 1}"
        )

    # wait for the pubsub (floodsub) mesh to stabilise — routing convergence
    # does not guarantee pubsub connections are established end-to-end yet
    print(f"  waiting {pubsub_warmup}s for pubsub mesh to stabilise...")
    time.sleep(pubsub_warmup)

    # send one unique message from each node
    stamp = int(time.time())
    messages = {}
    for node in nodes:
        content = f"feed-dist-{node.id}-{stamp}"
        node.send_feed_message(content)
        messages[node.id] = content
        print(f"  sent from {node.id}: '{content}'")

    expected_contents = set(messages.values())
    print(
        f"  waiting up to {propagation_wait}s for all {len(expected_contents)} messages to propagate..."
    )

    # poll each node until it has all messages
    t_send = time.time()
    deadline = t_send + propagation_wait

    # track per-node arrival times for reporting
    first_complete: dict[str, float] = {}

    while time.time() < deadline:
        time.sleep(poll_interval)
        elapsed = round(time.time() - t_send, 1)

        all_done = True
        for node in nodes:
            if node.id in first_complete:
                continue
            contents = node.feed_message_contents()
            received = set(contents)
            missing = expected_contents - received
            if not missing:
                first_complete[node.id] = elapsed
                print(
                    f"    +{elapsed}s  node {node.id} has all {len(expected_contents)} messages"
                )
            else:
                all_done = False

        if all_done:
            break

    # assert all nodes received all messages
    failures = []
    duplicate_failures = []

    for node in nodes:
        contents = node.feed_message_contents()
        received = set(contents)
        missing = expected_contents - received
        if missing:
            failures.append(
                f"  node {node.id}: missing {len(missing)} message(s): {missing}"
            )

        # check for duplicates
        for content in expected_contents:
            count = contents.count(content)
            if count > 1:
                duplicate_failures.append(
                    f"  node {node.id}: message '{content}' appears {count} times"
                )

    assert not failures, (
        f"feed propagation incomplete after {propagation_wait}s:\n"
        + "\n".join(failures)
    )
    assert not duplicate_failures, "duplicate feed messages detected:\n" + "\n".join(
        duplicate_failures
    )

    not_complete = [nid for nid in sorted_ids if nid not in first_complete]
    if not_complete:
        # they completed in the final poll but we didn't record the exact time
        for nid in not_complete:
            first_complete[nid] = round(time.time() - t_send, 1)

    max_time = max(first_complete.values())
    print(
        f"  PASS: all {len(sorted_ids)} nodes received all {len(expected_contents)} messages "
        f"within {max_time}s, no duplicates"
    )

    return {
        "passed": True,
        "node_count": len(sorted_ids),
        "message_count": len(expected_contents),
        "completion_times_s": first_complete,
        "max_propagation_s": max_time,
        "notes": (
            f"all {len(sorted_ids)} nodes received all {len(expected_contents)} feed messages "
            f"within {max_time}s, no duplicates"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        result = test_feed_messages_reach_all_nodes()
        print(f"  result: {result}")
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
