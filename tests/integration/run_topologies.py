#!/usr/bin/env python3
"""
Run all three correctness tests across every topology in TOPOLOGIES.

For each topology:
  1. test_node_startup    — all nodes respond, node info fields present
  2. test_user_discovery  — farthest node's user is known after discovery_wait
  3. test_message_routing — feed message from first node reaches last node

Results are written to results/<date>-<topology_name>.json.

Usage:
    ./run_topologies.py
    ./run_topologies.py topologies/line-5.json   # run one topology only
"""

import datetime
import json
import os
import sys
import time
import traceback

sys.path.insert(0, ".")

from lib.network import (
    apply_topology,
    clear_topology,
    start_qaul,
    stop_qaul,
    wait_for_nodes,
)
from lib.topology import load_node_ids
from lib.node import Node

# ---------------------------------------------------------------------------
# Topology configs
# discovery_wait: seconds from cold start until all nodes are expected to know
#                 each other's user accounts. Formula: diameter * 10s * 2 + margin.
# propagation_wait: seconds after sending a feed message for it to reach all nodes.
# ---------------------------------------------------------------------------
TOPOLOGIES = [
    {
        "file": "topologies/line-5.json",
        "discovery_wait": 120,
        "propagation_wait": 30,
    },
    {
        "file": "topologies/line-10.json",
        "discovery_wait": 200,
        "propagation_wait": 30,
    },
    {
        "file": "topologies/grid4-3x3.json",
        "discovery_wait": 120,
        "propagation_wait": 30,
    },
    {
        "file": "topologies/grid4-5x5.json",
        "discovery_wait": 180,
        "propagation_wait": 30,
    },
    {
        "file": "topologies/grid8-4x4.json",
        "discovery_wait": 90,
        "propagation_wait": 30,
    },
]


def local_user_id(node: Node) -> str:
    """Return the qaul user ID of the node's own local account."""
    for user in node.known_users():
        if any(c["module"] == "LOCAL" for c in user["connections"]):
            return user["id"]
    raise ValueError(f"No LOCAL user found on node {node.id}")


def distant_pair(node_ids: list[str]) -> tuple[str, str]:
    """Return the (first, last) node IDs when sorted — topologically the farthest pair."""
    sorted_ids = sorted(node_ids)
    return sorted_ids[0], sorted_ids[-1]


def test_node_startup(node_ids: list[str]) -> dict:
    """
    All nodes must respond to node info and return a dict with expected fields.
    """
    result = {"passed": False, "notes": ""}
    seen_ids = []

    for nid in node_ids:
        info = Node(nid).node_info()
        for field in ("node_id", "addresses"):
            assert field in info, f"node {nid}: node info missing field '{field}'"
        seen_ids.append(info["node_id"])

    assert len(set(seen_ids)) == len(node_ids), (
        f"node IDs are not all distinct: {seen_ids}"
    )

    result["passed"] = True
    result["notes"] = f"all {len(node_ids)} nodes responded with distinct IDs"
    return result


def test_user_discovery(node_ids: list[str], discovery_wait: int) -> dict:
    """
    After discovery_wait seconds, the first node must know the farthest node's user.
    """
    result = {"passed": False, "notes": "", "actual_wait_s": discovery_wait}

    first_id, last_id = distant_pair(node_ids)
    node_first = Node(first_id)
    node_last = Node(last_id)

    print(f"    waiting {discovery_wait}s for user discovery...")
    time.sleep(discovery_wait)

    id_last = local_user_id(node_last)
    known = node_first.known_user_ids()

    assert id_last in known, (
        f"node {first_id} does not know about node {last_id} after {discovery_wait}s\n"
        f"  known user IDs: {known}"
    )

    result["passed"] = True
    result["notes"] = (
        f"node {first_id} knows node {last_id}'s user after {discovery_wait}s"
    )
    return result


def test_message_routing(
    node_ids: list[str], discovery_wait: int, propagation_wait: int
) -> dict:
    """
    After discovery_wait, send a feed message from the first node.
    After propagation_wait more seconds it must appear on the last node.
    """
    result = {
        "passed": False,
        "notes": "",
        "actual_wait_s": discovery_wait + propagation_wait,
    }
    test_message = f"hello from {sorted(node_ids)[0]}"

    first_id, last_id = distant_pair(node_ids)
    node_sender = Node(first_id)
    node_receiver = Node(last_id)

    # User discovery must have already happened before we send — the feed
    # service drops messages from unknown senders. If test_user_discovery ran
    # first in this suite the wait has already elapsed; we sleep again here
    # only as a safety buffer.
    print(f"    waiting {discovery_wait}s before sending (user discovery)...")
    time.sleep(discovery_wait)

    node_sender.send_feed_message(test_message)

    print(f"    waiting {propagation_wait}s for message propagation...")
    time.sleep(propagation_wait)

    contents = node_receiver.feed_message_contents()
    assert test_message in contents, (
        f"message not found on node {last_id} after {propagation_wait}s\n"
        f"  messages seen: {contents}"
    )

    result["passed"] = True
    result["notes"] = (
        f"message from {first_id} reached {last_id} "
        f"after {discovery_wait}s discovery + {propagation_wait}s propagation"
    )
    return result


def run_topology(config: dict) -> dict:
    topo_file = config["file"]
    discovery_wait = config["discovery_wait"]
    propagation_wait = config["propagation_wait"]
    topo_name = os.path.splitext(os.path.basename(topo_file))[0]

    node_ids = load_node_ids(topo_file)
    print(f"\n{'=' * 50}")
    print(f"topology: {topo_name}  ({len(node_ids)} nodes)")
    print(f"{'=' * 50}")

    record = {
        "topology": topo_name,
        "node_count": len(node_ids),
        "discovery_wait_s": discovery_wait,
        "propagation_wait_s": propagation_wait,
        "tests": {},
        "overall_passed": False,
        "timestamp": datetime.datetime.now(datetime.UTC).isoformat(),
    }

    try:
        apply_topology(topo_file)
        start_qaul()
        wait_for_nodes(node_ids, timeout=60)
        print(f"  all {len(node_ids)} nodes up")
    except Exception as e:
        record["tests"]["setup"] = {"passed": False, "notes": str(e)}
        print(f"  SETUP FAILED: {e}")
        try:
            stop_qaul()
            clear_topology()
        except Exception:
            pass
        return record

    suite_passed = True

    for test_name, test_fn, kwargs in [
        ("test_node_startup", test_node_startup, {"node_ids": node_ids}),
        (
            "test_user_discovery",
            test_user_discovery,
            {"node_ids": node_ids, "discovery_wait": discovery_wait},
        ),
        (
            "test_message_routing",
            test_message_routing,
            {
                "node_ids": node_ids,
                "discovery_wait": discovery_wait,
                "propagation_wait": propagation_wait,
            },
        ),
    ]:
        print(f"  running {test_name}...")
        t0 = time.time()
        try:
            result = test_fn(**kwargs)
            elapsed = time.time() - t0
            result["elapsed_s"] = round(elapsed, 1)
            record["tests"][test_name] = result
            status = "PASS" if result["passed"] else "FAIL"
            print(f"    {status}: {result['notes']}")
        except AssertionError as e:
            elapsed = time.time() - t0
            record["tests"][test_name] = {
                "passed": False,
                "notes": str(e),
                "elapsed_s": round(elapsed, 1),
            }
            print(f"    FAIL: {e}")
            suite_passed = False
        except Exception as e:
            elapsed = time.time() - t0
            record["tests"][test_name] = {
                "passed": False,
                "notes": f"ERROR: {e}",
                "elapsed_s": round(elapsed, 1),
            }
            print(f"    ERROR: {e}")
            traceback.print_exc()
            suite_passed = False

    try:
        stop_qaul()
        clear_topology()
    except Exception as e:
        print(f"  TEARDOWN ERROR: {e}")

    record["overall_passed"] = suite_passed
    return record


def save_result(record: dict):
    os.makedirs("results", exist_ok=True)
    date = datetime.datetime.now(datetime.UTC).strftime("%Y-%m-%d")
    filename = f"results/{date}-{record['topology']}.json"
    with open(filename, "w") as f:
        json.dump(record, f, indent=2)
    print(f"  result saved to {filename}")


if __name__ == "__main__":
    # Optional: single topology passed as argument
    if len(sys.argv) == 2:
        topo_arg = sys.argv[1]
        configs = [c for c in TOPOLOGIES if c["file"] == topo_arg]
        if not configs:
            # Allow passing just the filename without the directory prefix
            configs = [
                c
                for c in TOPOLOGIES
                if os.path.basename(c["file"]) == os.path.basename(topo_arg)
            ]
        if not configs:
            print(f"unknown topology: {topo_arg}")
            print(f"known topologies: {[c['file'] for c in TOPOLOGIES]}")
            sys.exit(1)
    else:
        configs = TOPOLOGIES

    all_results = []
    total_passed = 0
    total_failed = 0

    for config in configs:
        record = run_topology(config)
        save_result(record)
        all_results.append(record)
        if record["overall_passed"]:
            total_passed += 1
        else:
            total_failed += 1

    print(f"\n{'=' * 50}")
    print(f"Topologies: {total_passed} passed, {total_failed} failed")
    print(f"{'=' * 50}")
        else:
            total_failed += 1

    print(f"\n{'='*50}")
    print(f"Topologies: {total_passed} passed, {total_failed} failed")
    print(f"{'='*50}")
    for r in all_results:
        status = "PASS" if r["overall_passed"] else "FAIL"
        print(f"  [{status}] {r['topology']} ({r['node_count']} nodes)")

    sys.exit(0 if total_failed == 0 else 1)
