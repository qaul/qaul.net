#!/usr/bin/env python3
"""
Run network behaviour tests across topologies.

Warms the network ONCE per topology (apply -> start -> wait for convergence),
then runs the Python test functions back to back without tearing down between them.

Results saved to results/<date>-network-<topology>.json.

Usage:
    python3 run_network_tests.py                          # all topologies
    python3 run_network_tests.py topologies/line-5.json  # one topology
"""

import datetime
import json
import os
import sys
import time
import traceback

sys.path.insert(0, ".")

from lib.network import apply_topology, clear_topology, start_qaul, stop_qaul, wait_for_nodes
from lib.node import Node
from lib.topology import load_node_ids
from test_direct_messages import test_direct_message_unicast
from test_feed_distribution import test_feed_messages_reach_all_nodes
from test_degraded_links import test_feed_delivery_under_packet_loss

TOPOLOGIES = [
    {"file": "topologies/line-5.json",   "discovery_wait": 200, "degraded_loss_percents": [30, 50, 80]},
    {"file": "topologies/line-10.json",  "discovery_wait": 400, "degraded_loss_percents": [30, 50, 80]},
    {"file": "topologies/grid4-3x3.json","discovery_wait": 200, "degraded_loss_percents": []},
    {"file": "topologies/grid4-5x5.json","discovery_wait": 250, "degraded_loss_percents": []},
    {"file": "topologies/grid8-4x4.json","discovery_wait": 200, "degraded_loss_percents": []},
]


def wait_for_convergence(node_ids: list[str], timeout: int = 200):
    """Wait until the observer node sees all peers in its users list."""
    sorted_ids = sorted(node_ids)
    observer = Node(sorted_ids[0])
    t_start = time.time()
    deadline = t_start + timeout

    print(f"  waiting for all {len(node_ids)} users (timeout {timeout}s)...")
    while time.time() < deadline:
        if len(observer.known_users()) >= len(node_ids):
            elapsed = round(time.time() - t_start, 1)
            print(f"  all users known after ~{elapsed}s")
            return
        time.sleep(5)

    known = len(observer.known_users())
    raise TimeoutError(f"only {known}/{len(node_ids)} users after {timeout}s")


def run_test(name: str, fn, kwargs: dict) -> dict:
    """Run a single test function, catching errors and returning a result dict."""
    t0 = time.time()
    try:
        result = fn(**kwargs)
        result["elapsed_s"] = round(time.time() - t0, 1)
        return result
    except Exception as e:
        traceback.print_exc()
        return {
            "passed": False,
            "notes": f"ERROR: {e}",
            "elapsed_s": round(time.time() - t0, 1),
        }


def run_topology(config: dict) -> dict:
    topo_file = config["file"]
    node_ids = load_node_ids(topo_file)
    topo_name = os.path.splitext(os.path.basename(topo_file))[0]
    sorted_ids = sorted(node_ids)

    print(f"\n{'=' * 50}")
    print(f"topology: {topo_name}  ({len(node_ids)} nodes)")
    print(f"{'=' * 50}")

    record = {
        "topology": topo_name,
        "node_count": len(node_ids),
        "tests": {},
        "overall_passed": False,
        "timestamp": datetime.datetime.now(datetime.UTC).isoformat(),
    }

    try:
        apply_topology(topo_file)
        start_qaul()
        wait_for_nodes(node_ids, timeout=60)
        wait_for_convergence(node_ids, timeout=config["discovery_wait"])

        suite_passed = True

        # direct messages
        print(f"\n  [test_direct_messages]")
        result = run_test("test_direct_messages", test_direct_message_unicast, {
            "node_ids": sorted_ids,
            "discovery_wait": 1,  # already converged
        })
        record["tests"]["test_direct_messages"] = result
        if result.get("passed"):
            print(f"    pass ({result.get('elapsed_s', '?')}s)")
        else:
            print(f"    FAIL: {result.get('notes', '')}")
            suite_passed = False

        # feed distribution
        print(f"\n  [test_feed_distribution]")
        result = run_test("test_feed_distribution", test_feed_messages_reach_all_nodes, {
            "node_ids": sorted_ids,
            "discovery_wait": 1,  # already converged
            "pubsub_warmup": 0,   # already warmed
        })
        record["tests"]["test_feed_distribution"] = result
        if result.get("passed"):
            print(f"    pass ({result.get('elapsed_s', '?')}s)")
        else:
            print(f"    FAIL: {result.get('notes', '')}")
            suite_passed = False

        # degraded links
        loss_percents = config.get("degraded_loss_percents", [])
        if loss_percents:
            for pct in loss_percents:
                test_name = f"test_degraded_links_{pct}pct"
                print(f"\n  [{test_name}]")
                result = run_test(test_name, test_feed_delivery_under_packet_loss, {
                    "node_ids": sorted_ids,
                    "discovery_wait": 1,
                    "pubsub_warmup": 0,
                    "loss_percent": pct,
                })
                record["tests"][test_name] = result
                if result.get("passed"):
                    print(f"    pass ({result.get('elapsed_s', '?')}s)")
                else:
                    print(f"    FAIL: {result.get('notes', '')}")
                    suite_passed = False
        else:
            record["tests"]["test_degraded_links"] = {
                "passed": None,
                "notes": "skipped — grid topology",
            }

        record["overall_passed"] = suite_passed

    except Exception as e:
        record["tests"]["_setup"] = {"passed": False, "notes": str(e)}
        print(f"  ERROR during setup/convergence: {e}")
    finally:
        try:
            stop_qaul()
            clear_topology()
        except Exception as e:
            print(f"  TEARDOWN ERROR: {e}")

    return record


def save_result(record: dict):
    os.makedirs("results", exist_ok=True)
    date = datetime.datetime.now(datetime.UTC).strftime("%Y-%m-%d")
    filename = f"results/{date}-network-{record['topology']}.json"
    with open(filename, "w") as f:
        json.dump(record, f, indent=2)
    print(f"\n  result saved to {filename}")


if __name__ == "__main__":
    if len(sys.argv) == 2:
        topo_arg = sys.argv[1]
        configs = [c for c in TOPOLOGIES if os.path.basename(c["file"]) == os.path.basename(topo_arg)]
        if not configs:
            print(f"unknown topology: {topo_arg}")
            print(f"known: {[c['file'] for c in TOPOLOGIES]}")
            sys.exit(1)
    else:
        configs = TOPOLOGIES

    all_results = []
    total_passed = total_failed = 0

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
    for r in all_results:
        status = "PASS" if r["overall_passed"] else "FAIL"
        print(f"  [{status}] {r['topology']} ({r['node_count']} nodes)")
        for test_name, result in r["tests"].items():
            if result.get("passed") is None:
                indicator = "SKIP"
            elif result["passed"]:
                indicator = "pass"
            else:
                indicator = "FAIL"
            print(f"         {indicator}  {test_name}")

    sys.exit(0 if total_failed == 0 else 1)
