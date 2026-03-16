#!/usr/bin/env python3
"""
Run all four routing protocol tests across every topology in TOPOLOGIES.

For each topology:
  1. test_route_discovery    — polls router table, records when each node first appears
  2. test_route_stability    — polls for stability_duration, asserts no flapping or RTT drift
  3. test_route_expiration   — kills last node, asserts route expires within protocol window
                               (line topologies only — grids have redundant paths)
  4. test_neighbour_detection — asserts each node's LAN neighbours match topology links exactly

Results are written to results/<date>-routing-<topology_name>.json.

Usage:
    python3 run_routing_tests.py                          # all topologies, all tests
    python3 run_routing_tests.py topologies/line-5.json  # one topology only
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
from test_neighbour_detection import test_neighbours_match_topology
from test_route_discovery import test_all_nodes_appear_in_routing_table
from test_route_expiration import test_dead_node_route_expires
from test_route_stability import test_routing_table_is_stable

# ---------------------------------------------------------------------------
# Topology configs
#
# stability_duration: how long to poll for stability after convergence.
#   300s (5 min) in standalone mode, shorter here to keep total runtime
#   reasonable when running all topologies. Increase if you want a thorough
#   stability check.
#
# run_expiration: False for grid topologies — killing one node in a dense
#   grid may not remove its route from the observer because alternative
#   paths exist via other nodes. Expiration tests are most meaningful on
#   line topologies where each route has exactly one path.
# ---------------------------------------------------------------------------
TOPOLOGIES = [
    {
        "file": "topologies/line-5.json",
        "discovery_wait": 120,
        "stability_duration": 120,
        "neighbour_wait": 30,
        "run_expiration": True,
    },
    {
        "file": "topologies/line-10.json",
        "discovery_wait": 200,
        "stability_duration": 120,
        "neighbour_wait": 30,
        "run_expiration": True,
    },
    {
        "file": "topologies/grid4-3x3.json",
        "discovery_wait": 120,
        "stability_duration": 120,
        "neighbour_wait": 30,
        "run_expiration": False,
    },
    {
        "file": "topologies/grid4-5x5.json",
        "discovery_wait": 180,
        "stability_duration": 120,
        "neighbour_wait": 30,
        "run_expiration": False,
    },
    {
        "file": "topologies/grid8-4x4.json",
        "discovery_wait": 90,
        "stability_duration": 120,
        "neighbour_wait": 30,
        "run_expiration": False,
    },
]


def run_topology(config: dict) -> dict:
    topo_file = config["file"]
    discovery_wait = config["discovery_wait"]
    stability_duration = config["stability_duration"]
    neighbour_wait = config["neighbour_wait"]
    run_expiration = config["run_expiration"]
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

    # each routing test does its own convergence wait internally, so the
    # topology is set up fresh for each test and torn down after
    suite_passed = True

    tests = [
        (
            "test_route_discovery",
            test_all_nodes_appear_in_routing_table,
            {"node_ids": node_ids, "discovery_wait": discovery_wait},
        ),
        (
            "test_route_stability",
            test_routing_table_is_stable,
            {
                "node_ids": node_ids,
                "discovery_wait": discovery_wait,
                "stability_duration": stability_duration,
            },
        ),
        (
            "test_neighbour_detection",
            test_neighbours_match_topology,
            {
                "node_ids": node_ids,
                "topology_file": topo_file,
                "neighbour_wait": neighbour_wait,
            },
        ),
    ]

    if run_expiration:
        tests.append(
            (
                "test_route_expiration",
                test_dead_node_route_expires,
                {"node_ids": node_ids, "discovery_wait": discovery_wait},
            )
        )
    else:
        print(
            "  skipping test_route_expiration "
            "(grid topology — redundant paths make expiration unpredictable)"
        )
        record["tests"]["test_route_expiration"] = {
            "passed": None,
            "notes": "skipped — grid topology has redundant paths",
        }

    for test_name, test_fn, kwargs in tests:
        print(f"\n  [{test_name}]")

        # fresh network for each test
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
    filename = f"results/{date}-routing-{record['topology']}.json"
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
