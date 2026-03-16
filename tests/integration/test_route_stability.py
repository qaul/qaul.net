# test_route_stability.py
#
# After full convergence, polls the routing table every 10 seconds for
# stability_duration seconds and checks that:
#   - No user disappears and reappears (route flapping)
#   - Hop counts do not change for stable routes
#   - RTT does not drift by more than rtt_tolerance_percent
#
# A flap is defined as a user that was present in one poll, absent in the
# next, then present again. This indicates the route is expiring and
# being re-discovered on every RouterInfo cycle, which wastes bandwidth
# and causes intermittent delivery failures.

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
STABILITY_DURATION = 300  # 5 minutes
POLL_INTERVAL = 10
RTT_TOLERANCE_PERCENT = 20


def setup():
    apply_topology(TOPOLOGY)
    start_qaul()
    wait_for_nodes(NODE_IDS, timeout=60)


def teardown():
    stop_qaul()
    clear_topology()


def test_routing_table_is_stable(
    node_ids: list[str] = NODE_IDS,
    discovery_wait: int = DISCOVERY_WAIT,
    stability_duration: int = STABILITY_DURATION,
    poll_interval: int = POLL_INTERVAL,
    rtt_tolerance_percent: int = RTT_TOLERANCE_PERCENT,
) -> dict:
    """
    After waiting for full convergence, poll the routing table repeatedly
    and assert it does not flap or drift.
    """
    observer = Node(sorted(node_ids)[0])

    # wait for convergence before starting stability checks
    print(f"  waiting {discovery_wait}s for convergence before stability check...")
    time.sleep(discovery_wait)

    # take a baseline snapshot
    baseline = {
        entry["user_id"]: entry["connections"][0]
        for entry in observer.router_table()
        if entry["connections"]
    }

    assert len(baseline) > 0, (
        "routing table is empty after discovery wait — cannot check stability"
    )

    print(f"  baseline: {len(baseline)} users in routing table")
    print(f"  polling every {poll_interval}s for {stability_duration}s...")

    # tracking state
    flaps: dict[str, int] = {}  # user_id -> flap count
    absent_in_last: set[str] = set()  # user_ids missing in the previous poll
    hop_changes: dict[str, list] = {}  # user_id -> list of observed hop_counts
    rtt_observations: dict[str, list] = {}  # user_id -> list of observed RTTs

    t_start = time.time()
    deadline = t_start + stability_duration
    poll_count = 0

    while time.time() < deadline:
        time.sleep(poll_interval)
        poll_count += 1
        elapsed = round(time.time() - t_start)

        table = observer.router_table()
        present_now = {
            entry["user_id"]: entry["connections"][0]
            for entry in table
            if entry["connections"]
        }

        for uid in baseline:
            if uid in absent_in_last and uid in present_now:
                # was gone, now back — that is a flap
                flaps[uid] = flaps.get(uid, 0) + 1
                print(
                    f"    +{elapsed}s  FLAP detected for user {uid} "
                    f"(flap #{flaps[uid]})"
                )

            if uid in present_now:
                conn = present_now[uid]

                # track hop count changes
                if uid not in hop_changes:
                    hop_changes[uid] = [conn["hop_count"]]
                elif conn["hop_count"] != hop_changes[uid][-1]:
                    print(
                        f"    +{elapsed}s  hop count changed for {uid}: "
                        f"{hop_changes[uid][-1]} -> {conn['hop_count']}"
                    )
                    hop_changes[uid].append(conn["hop_count"])

                # track RTT observations
                if uid not in rtt_observations:
                    rtt_observations[uid] = [conn["rtt"]]
                else:
                    rtt_observations[uid].append(conn["rtt"])

        absent_in_last = set(baseline.keys()) - set(present_now.keys())
        if absent_in_last:
            print(f"    +{elapsed}s  users absent from table: {absent_in_last}")

    # assertions
    total_flaps = sum(flaps.values())
    assert total_flaps == 0, (
        f"route flapping detected across {poll_count} polls:\n"
        + "\n".join(f"  {uid}: {count} flap(s)" for uid, count in flaps.items())
    )

    rtt_violations = []
    for uid, rtts in rtt_observations.items():
        if len(rtts) < 2:
            continue
        baseline_rtt = rtt_observations[uid][0]
        if baseline_rtt == 0:
            continue
        for rtt in rtts[1:]:
            drift = abs(rtt - baseline_rtt) / baseline_rtt * 100
            if drift > rtt_tolerance_percent:
                rtt_violations.append(
                    f"  {uid}: baseline {baseline_rtt}ms, observed {rtt}ms "
                    f"({drift:.0f}% drift, limit {rtt_tolerance_percent}%)"
                )

    assert not rtt_violations, (
        f"RTT drift exceeded {rtt_tolerance_percent}% tolerance:\n"
        + "\n".join(rtt_violations)
    )

    hop_change_count = sum(1 for changes in hop_changes.values() if len(changes) > 1)

    print(
        f"  PASS: {poll_count} polls over {stability_duration}s — "
        f"0 flaps, 0 RTT violations, {hop_change_count} hop count change(s)"
    )

    return {
        "passed": True,
        "poll_count": poll_count,
        "flaps": flaps,
        "hop_changes": {uid: v for uid, v in hop_changes.items() if len(v) > 1},
        "notes": (
            f"{poll_count} polls over {stability_duration}s: "
            f"0 flaps, {hop_change_count} hop count change(s)"
        ),
    }


if __name__ == "__main__":
    try:
        setup()
        test_routing_table_is_stable()
    except AssertionError as e:
        print(f"  FAIL: {e}")
    finally:
        teardown()
