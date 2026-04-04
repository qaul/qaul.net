"""
Verify that feed messages from each node reach ALL other nodes, no duplicates.
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
PROPAGATION_WAIT = 180
POLL_INTERVAL = 5
PUBSUB_WARMUP = 30


def setup():
    from lib.network import apply_topology, start_qaul, wait_for_nodes
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    from lib.network import stop_qaul, clear_topology
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
    observer = nodes[0]

    # convergence wait
    print(f"  waiting up to {discovery_wait}s for all {len(sorted_ids)} users...")
    t_start = time.time()
    deadline = t_start + discovery_wait
    while time.time() < deadline:
        if len(observer.known_users()) >= len(sorted_ids):
            print(f"  all users known after {round(time.time() - t_start, 1)}s")
            break
        time.sleep(poll_interval)
    else:
        raise AssertionError(
            f"convergence not reached within {discovery_wait}s — "
            f"observer knows {len(observer.known_users())} of {len(sorted_ids)}"
        )

    if pubsub_warmup > 0:
        print(f"  pubsub warmup {pubsub_warmup}s...")
        time.sleep(pubsub_warmup)

    # send one message per node
    stamp = int(time.time())
    messages = {}
    for node in nodes:
        content = f"feed-dist-{node.id}-{stamp}"
        node.send_feed_message(content)
        messages[node.id] = content

    expected = set(messages.values())
    print(f"  waiting up to {propagation_wait}s for {len(expected)} messages...")

    t_send = time.time()
    deadline = t_send + propagation_wait
    first_complete: dict[str, float] = {}

    while time.time() < deadline:
        time.sleep(poll_interval)
        elapsed = round(time.time() - t_send, 1)
        all_done = True
        for node in nodes:
            if node.id in first_complete:
                continue
            received = set(node.feed_message_contents())
            if not (expected - received):
                first_complete[node.id] = elapsed
            else:
                all_done = False
        if all_done:
            break

    # final check
    failures = []
    for node in nodes:
        contents = node.feed_message_contents()
        missing = expected - set(contents)
        if missing:
            failures.append(f"node {node.id}: missing {len(missing)}")
        for c in expected:
            if contents.count(c) > 1:
                failures.append(f"node {node.id}: duplicate '{c}'")

    assert not failures, f"propagation failed:\n  " + "\n  ".join(failures)

    for nid in sorted_ids:
        if nid not in first_complete:
            first_complete[nid] = round(time.time() - t_send, 1)

    max_time = max(first_complete.values())
    print(f"  PASS: all nodes got all messages within {max_time}s")

    return {
        "passed": True,
        "node_count": len(sorted_ids),
        "message_count": len(expected),
        "max_propagation_s": max_time,
        "notes": f"all {len(sorted_ids)} nodes received all messages within {max_time}s",
    }


# --- pytest entry point ---

@pytest.mark.network
@pytest.mark.parametrize("converged_network", TOPOLOGIES, indirect=True)
def test_feed_distribution_pytest(converged_network):
    """pytest wrapper using the converged_network fixture."""
    test_feed_messages_reach_all_nodes(
        node_ids=converged_network["node_ids"],
        discovery_wait=1,
        pubsub_warmup=0,
    )


if __name__ == "__main__":
    try:
        setup()
        test_feed_messages_reach_all_nodes()
    finally:
        teardown()
