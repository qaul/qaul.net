#!/usr/bin/env python3
"""
Run network behaviour tests across topologies.

Warms the network ONCE per topology (apply → start → wait for convergence →
send sentinel and wait for all nodes to receive it), then runs the three bash
test scripts back to back without tearing down between them.

Results saved to results/<date>-network-<topology>.json.

Usage:
    python3 run_network_tests.py                          # all topologies
    python3 run_network_tests.py topologies/line-5.json  # one topology
"""

import datetime
import json
import os
import subprocess
import sys
import time

sys.path.insert(0, ".")

from lib.network import apply_topology, clear_topology, start_qaul, stop_qaul, wait_for_nodes
from lib.node import Node
from lib.topology import load_node_ids

TOPOLOGIES = [
    {"file": "topologies/line-5.json",   "discovery_wait": 200, "run_degraded": True},
    {"file": "topologies/line-10.json",  "discovery_wait": 250, "run_degraded": True},
    {"file": "topologies/grid4-3x3.json","discovery_wait": 150, "run_degraded": False},
    {"file": "topologies/grid4-5x5.json","discovery_wait": 200, "run_degraded": False},
    {"file": "topologies/grid8-4x4.json","discovery_wait": 120, "run_degraded": False},
]


def wait_for_convergence(node_ids: list[str], timeout: int = 200):
    """
    Block until the mesh is provably ready to run tests:
      1. All nodes appear in the observer's users list
      2. A sentinel feed message sent from the observer reaches every node

    Step 2 proves the pubsub (floodsub) mesh is end-to-end ready — no fixed
    sleep, no guessing.
    """
    sorted_ids = sorted(node_ids)
    observer = Node(sorted_ids[0])

    print(f"  waiting for all {len(node_ids)} users in users list (timeout {timeout}s)...")
    deadline = time.time() + timeout
    while time.time() < deadline:
        if len(observer.known_users()) >= len(node_ids):
            elapsed = round(time.time() - (deadline - timeout), 1)
            print(f"  all users known after ~{elapsed}s")
            break
        time.sleep(5)
    else:
        known = len(observer.known_users())
        raise TimeoutError(f"only {known}/{len(node_ids)} users after {timeout}s")

    # prove pubsub mesh is ready with a sentinel message
    sentinel = f"sentinel-{int(time.time())}"
    observer.send_feed_message(sentinel)
    print(f"  sent sentinel '{sentinel}', waiting for all nodes to receive it...")

    deadline = time.time() + 120
    while time.time() < deadline:
        if all(sentinel in Node(nid).feed_message_contents() for nid in sorted_ids):
            print("  sentinel received by all nodes — mesh is ready")
            return
        time.sleep(5)

    missing = [nid for nid in sorted_ids if sentinel not in Node(nid).feed_message_contents()]
    raise TimeoutError(f"sentinel not received by {missing} within 120s")


def run_bash_test(script: str, node_ids: list[str]) -> dict:
    """
    Run a bash test script with node IDs as arguments.
    Progress messages go to the terminal (stderr is inherited).
    The last JSON line on stdout is the test result.
    """
    t0 = time.time()
    script_path = os.path.join(os.path.dirname(__file__), script)
    try:
        result = subprocess.run(
            ["bash", script_path] + node_ids,
            stdout=subprocess.PIPE,
            text=True,
            timeout=300,
        )
        elapsed = round(time.time() - t0, 1)
        json_lines = [l for l in result.stdout.splitlines() if l.strip().startswith("{")]
        if not json_lines:
            return {"passed": False, "notes": f"no JSON output from {script}", "elapsed_s": elapsed}
        data = json.loads(json_lines[-1])
        data["elapsed_s"] = elapsed
        return data
    except Exception as e:
        return {"passed": False, "notes": f"ERROR: {e}", "elapsed_s": round(time.time() - t0, 1)}


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

        tests = [
            ("test_direct_messages",   "test_direct_messages.sh"),
            ("test_feed_distribution", "test_feed_distribution.sh"),
        ]
        if config["run_degraded"]:
            tests.append(("test_degraded_links", "test_degraded_links.sh"))
        else:
            record["tests"]["test_degraded_links"] = {
                "passed": None,
                "notes": "skipped — grid topology",
            }

        suite_passed = True
        for test_name, script in tests:
            print(f"\n  [{test_name}]")
            result = run_bash_test(script, sorted_ids)
            record["tests"][test_name] = result
            if result.get("passed"):
                print(f"    pass ({result.get('elapsed_s', '?')}s)")
            else:
                print(f"    FAIL: {result.get('notes', '')}")
                suite_passed = False

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
