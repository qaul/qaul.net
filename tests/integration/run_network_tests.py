#!/usr/bin/env python3
"""
Run all three network behaviour tests across every topology in TOPOLOGIES.

For each topology:
  1. test_direct_messages    — chat message delivered to recipient, not to snooper
  2. test_feed_distribution  — all nodes receive all feed messages, no duplicates
  3. test_degraded_links     — feed delivery rate under 30% packet loss on a central link

Results are written to results/<date>-network-<topology_name>.json.

Usage:
    python3 run_network_tests.py                          # all topologies, all tests
    python3 run_network_tests.py topologies/line-5.json  # one topology only
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
from test_degraded_links import test_feed_delivery_under_packet_loss
from test_direct_messages import test_direct_message_unicast
from test_feed_distribution import test_feed_messages_reach_all_nodes

# ---------------------------------------------------------------------------
# Topology configs
#
# discovery_wait: time to wait for full routing convergence before the test
#   starts sending messages. Must be long enough for multi-hop routes to form.
#
# propagation_wait: time to wait after sending before checking delivery.
#   Feed messages travel via floodsub — on well-converged networks this is
#   fast, but the first message on a cold topology can take a full RouterInfo
#   cycle.
#
# run_degraded: False for topologies where tc netem interface naming may not
#   match the expected pattern, or where the topology is too small to be
#   interesting for packet loss tests.
# ---------------------------------------------------------------------------
TOPOLOGIES = [
    {
        "file": "topologies/line-5.json",
        "discovery_wait": 200,
        "propagation_wait": 180,
        "pubsub_warmup": 30,
        "run_degraded": True,
    },
    {
        "file": "topologies/line-10.json",
        "discovery_wait": 200,
        "propagation_wait": 120,
        "pubsub_warmup": 30,
        "run_degraded": True,
    },
    {
        "file": "topologies/grid4-3x3.json",
        "discovery_wait": 120,
        "propagation_wait": 90,
        "pubsub_warmup": 20,
        "run_degraded": False,
    },
    {
        "file": "topologies/grid4-5x5.json",
        "discovery_wait": 180,
        "propagation_wait": 120,
        "pubsub_warmup": 30,
        "run_degraded": False,
    },
    {
        "file": "topologies/grid8-4x4.json",
        "discovery_wait": 90,
        "propagation_wait": 90,
        "pubsub_warmup": 20,
        "run_degraded": False,
    },
]


def run_topology(config: dict) -> dict:
    topo_file = config["file"]
    discovery_wait = config["discovery_wait"]
    propagation_wait = config["propagation_wait"]
    pubsub_warmup = config["pubsub_warmup"]
    run_degraded = config["run_degraded"]
    topo_name = os.path.splitext(os.path.basename(topo_file))[0]

    node_ids = load_node_ids(topo_file)
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

    suite_passed = True

    tests = [
        (
            "test_direct_messages",
            test_direct_message_unicast,
            {
                "node_ids": node_ids,
                "discovery_wait": discovery_wait,
            },
        ),
        (
            "test_feed_distribution",
            test_feed_messages_reach_all_nodes,
            {
                "node_ids": node_ids,
                "discovery_wait": discovery_wait,
                "propagation_wait": propagation_wait,
                "pubsub_warmup": pubsub_warmup,
            },
        ),
    ]

    if run_degraded:
        tests.append(
            (
                "test_degraded_links",
                test_feed_delivery_under_packet_loss,
                {
                    "node_ids": node_ids,
                    "discovery_wait": discovery_wait,
                    "propagation_wait": propagation_wait,
                    "pubsub_warmup": pubsub_warmup,
                },
            )
        )
    else:
        print("  skipping test_degraded_links (grid topology — interface naming not standardised)")
        record["tests"]["test_degraded_links"] = {
            "passed": None,
            "notes": "skipped — grid topology tc netem interface naming not standardised",
        }

    for test_name, test_fn, kwargs in tests:
        print(f"\n  [{test_name}]")

        # fresh network for each test to avoid state leaking between tests
        try:
            apply_topology(topo_file)
            start_qaul()
            wait_for_nodes(node_ids, timeout=60)
        except Exception as e:
            record["tests"][test_name] = {
                "passed": False,
                "notes": f"setup failed: {e}",
            }
            print(f"    SETUP FAILED: {e}")
            suite_passed = False
            try:
                stop_qaul()
                clear_topology()
            except Exception:
                pass
            continue

        t0 = time.time()
        try:
            result = test_fn(**kwargs)
            elapsed = round(time.time() - t0, 1)
            result["elapsed_s"] = elapsed
            record["tests"][test_name] = result
        except AssertionError as e:
            elapsed = round(time.time() - t0, 1)
            record["tests"][test_name] = {
                "passed": False,
                "notes": str(e),
                "elapsed_s": elapsed,
            }
            print(f"    FAIL: {e}")
            suite_passed = False
        except Exception as e:
            elapsed = round(time.time() - t0, 1)
            record["tests"][test_name] = {
                "passed": False,
                "notes": f"ERROR: {e}",
                "elapsed_s": elapsed,
            }
            print(f"    ERROR: {e}")
            traceback.print_exc()
            suite_passed = False
        finally:
            try:
                stop_qaul()
                clear_topology()
            except Exception as e:
                print(f"    TEARDOWN ERROR: {e}")

    record["overall_passed"] = suite_passed
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
        configs = [c for c in TOPOLOGIES if c["file"] == topo_arg]
        if not configs:
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
