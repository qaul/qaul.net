#!/usr/bin/env python3
"""
Run all three correctness tests across every topology in TOPOLOGIES.

For each topology:
  1. test_node_startup    — all nodes respond, node info fields present
  2. test_user_discovery  — farthest node's user becomes known, timed
  3. test_message_routing — feed message from first node reaches last node

Each timed test polls until its condition holds and records how long that took,
so the result files carry a measurement rather than the length of a sleep.

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
    wait_for_local_accounts,
    wait_for_nodes,
)
from lib.node import Node
from lib.topology import load_node_ids

# ---------------------------------------------------------------------------
# Topology configs
#
# These are *ceilings*, not sleeps. Each timed test polls its condition once a
# second and stops as soon as it holds, so a generous ceiling costs nothing on
# a healthy run and the recorded `measured_s` is the actual convergence time.
#
# Calibration: measured user discovery on this harness is 4-10s at 9 nodes,
# 12-13s at 16-20, and 14s at 25, with per-hop propagation ~0.5s (matching
# spec §7.1). 60s is therefore >4x the worst measured case — loose enough not
# to flake, tight enough that the 90-200s waits these replaced would now fail.
# Watch `measured_s` in results/ for drift long before it reaches the ceiling.
# ---------------------------------------------------------------------------
DISCOVERY_TIMEOUT_S = 60
PROPAGATION_TIMEOUT_S = 30

TOPOLOGIES = [
    {
        "file": "topologies/line-5.json",
        "discovery_timeout": DISCOVERY_TIMEOUT_S,
        "propagation_timeout": PROPAGATION_TIMEOUT_S,
    },
    {
        "file": "topologies/line-10.json",
        "discovery_timeout": DISCOVERY_TIMEOUT_S,
        "propagation_timeout": PROPAGATION_TIMEOUT_S,
    },
    {
        "file": "topologies/grid4-3x3.json",
        "discovery_timeout": DISCOVERY_TIMEOUT_S,
        "propagation_timeout": PROPAGATION_TIMEOUT_S,
    },
    {
        "file": "topologies/grid4-5x5.json",
        "discovery_timeout": DISCOVERY_TIMEOUT_S,
        "propagation_timeout": PROPAGATION_TIMEOUT_S,
    },
    {
        "file": "topologies/grid8-4x4.json",
        "discovery_timeout": DISCOVERY_TIMEOUT_S,
        "propagation_timeout": PROPAGATION_TIMEOUT_S,
    },
]

POLL_INTERVAL_S = 1.0


def poll_until(predicate, timeout: int, interval: float = POLL_INTERVAL_S):
    """Poll `predicate` until it returns truthy. Returns (ok, elapsed_seconds).

    The elapsed time is the measurement these tests exist to produce, so it is
    returned on failure too — "not converged after 60s" and "converged at 58s"
    are different problems and the caller should be able to tell them apart.
    """
    t0 = time.time()
    deadline = t0 + timeout
    while True:
        if predicate():
            return True, round(time.time() - t0, 1)
        if time.time() >= deadline:
            return False, round(time.time() - t0, 1)
        time.sleep(interval)


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


def test_user_discovery(node_ids: list[str], discovery_timeout: int) -> dict:
    """
    The first node must come to know the farthest node's user. Polls until it
    does and records how long it took.
    """
    first_id, last_id = distant_pair(node_ids)
    node_first = Node(first_id)
    node_last = Node(last_id)

    id_last = node_last.local_user_id()
    print(f"    polling {first_id} for {last_id}'s user (ceiling {discovery_timeout}s)...")

    ok, measured = poll_until(
        lambda: id_last in node_first.known_user_ids(), discovery_timeout
    )

    result = {
        "passed": ok,
        "notes": "",
        "measured_s": measured,
        "timeout_s": discovery_timeout,
    }
    if not ok:
        # A user-id dump is unreadable at 25 nodes; a count localises the
        # failure to "knows nothing" vs "knows most of the mesh".
        known = node_first.known_user_ids()
        raise AssertionError(
            f"node {first_id} does not know node {last_id}'s user after {measured}s "
            f"(knows {len(known)} of {len(node_ids)} expected users)"
        )

    result["notes"] = f"node {first_id} knew node {last_id}'s user after {measured}s"
    return result


def test_message_routing(
    node_ids: list[str], discovery_timeout: int, propagation_timeout: int
) -> dict:
    """
    Once the farthest node's user is known, send a feed message from the first
    node and poll the last node for it.
    """
    test_message = f"hello from {sorted(node_ids)[0]}"

    first_id, last_id = distant_pair(node_ids)
    node_sender = Node(first_id)
    node_receiver = Node(last_id)

    # The feed service drops messages from unknown senders, so discovery has to
    # have happened before we send. test_user_discovery normally established
    # this already; polling the same condition costs nothing when it holds and
    # keeps this test standalone when it is run on its own.
    id_sender = node_sender.local_user_id()
    ok, discovery_measured = poll_until(
        lambda: id_sender in node_receiver.known_user_ids(), discovery_timeout
    )
    if not ok:
        raise AssertionError(
            f"node {last_id} does not know the sender {first_id} after "
            f"{discovery_measured}s, so a feed message would be dropped unsent"
        )

    node_sender.send_feed_message(test_message)
    print(f"    polling {last_id} for the message (ceiling {propagation_timeout}s)...")

    ok, measured = poll_until(
        lambda: test_message in node_receiver.feed_message_contents(),
        propagation_timeout,
    )

    result = {
        "passed": ok,
        "notes": "",
        "discovery_measured_s": discovery_measured,
        "measured_s": measured,
        "timeout_s": propagation_timeout,
    }
    if not ok:
        contents = node_receiver.feed_message_contents()
        raise AssertionError(
            f"message not found on node {last_id} after {measured}s "
            f"({len(contents)} message(s) present)"
        )

    result["notes"] = (
        f"message from {first_id} reached {last_id} in {measured}s "
        f"(sender known after {discovery_measured}s)"
    )
    return result


def run_topology(config: dict) -> dict:
    topo_file = config["file"]
    discovery_timeout = config["discovery_timeout"]
    propagation_timeout = config["propagation_timeout"]
    topo_name = os.path.splitext(os.path.basename(topo_file))[0]

    node_ids = load_node_ids(topo_file)
    print(f"\n{'=' * 50}")
    print(f"topology: {topo_name}  ({len(node_ids)} nodes)")
    print(f"{'=' * 50}")

    record = {
        "topology": topo_name,
        "node_count": len(node_ids),
        "discovery_timeout_s": discovery_timeout,
        "propagation_timeout_s": propagation_timeout,
        "tests": {},
        "overall_passed": False,
        "timestamp": datetime.datetime.now(datetime.UTC).isoformat(),
    }

    try:
        apply_topology(topo_file)
        start_qaul()
        wait_for_nodes(node_ids, timeout=60)
        # "node info answers" is not "ready": the own account and its Local
        # connection land about a second later, and the tests resolve it.
        wait_for_local_accounts(node_ids, timeout=60)
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
            {"node_ids": node_ids, "discovery_timeout": discovery_timeout},
        ),
        (
            "test_message_routing",
            test_message_routing,
            {
                "node_ids": node_ids,
                "discovery_timeout": discovery_timeout,
                "propagation_timeout": propagation_timeout,
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
    for r in all_results:
        status = "PASS" if r["overall_passed"] else "FAIL"
        timings = []
        for name, key in (
            ("discovery", "test_user_discovery"),
            ("routing", "test_message_routing"),
        ):
            measured = r["tests"].get(key, {}).get("measured_s")
            if measured is not None:
                timings.append(f"{name} {measured}s")
        suffix = f"  [{', '.join(timings)}]" if timings else ""
        print(f"  [{status}] {r['topology']} ({r['node_count']} nodes){suffix}")

    sys.exit(0 if total_failed == 0 else 1)
