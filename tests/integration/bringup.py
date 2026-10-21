#!/usr/bin/env python3
"""Start qauld on an already-applied topology and wait until it is usable.

The step between `network.py apply` and `setup_communities.py`: without it
every qauld-ctl call hits a stale socket from the previous run and fails
with ConnectionRefused (the socket file survives, the daemon does not).

Does not apply or clear the topology - that stays an explicit sudo command
so a long sweep cannot tear down the wrong network.

Usage:
    ./bringup.py --roles topologies/communities-3x10.roles.json
    ./bringup.py --roles ... --stop
"""

import argparse
import json
import os
import subprocess
import sys

sys.path.insert(0, ".")

from lib.network import start_qaul, stop_qaul, wait_for_local_accounts, wait_for_nodes


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--timeout", type=int, default=90)
    ap.add_argument("--stop", action="store_true", help="stop qauld and exit")
    ap.add_argument("--v1", action="store_true",
                    help="measure the v1 arm deliberately (skips the v2 guard)")
    args = ap.parse_args()

    with open(args.roles) as f:
        roles = json.load(f)
    node_ids = [roles["hub"]] + [m for c in roles["communities"] for m in c["members"]]

    if args.stop:
        stop_qaul()
        print(f"stopped qauld on {len(node_ids)} nodes")
        return

    if not os.environ.get("MESHNET_LAB"):
        raise SystemExit("MESHNET_LAB is not set; lib.network needs it")

    # Overhead measured on v1 would look like a valid result, so this is a
    # hard stop rather than a warning.
    if not args.v1 and not os.environ.get("QAUL_ROUTING_V2"):
        raise SystemExit(
            "QAUL_ROUTING_V2 is not set, so qauld would start on v1 routing "
            "and the overhead numbers would silently be the wrong arm.\n"
            "  fish:  set -x QAUL_ROUTING_V2 1\n"
            "To measure the v1 arm on purpose, pass --v1."
        )

    arm = "v1" if args.v1 else "v2"
    print(f"starting qauld on {len(node_ids)} nodes ({arm} routing)")
    start_qaul()

    print(f"waiting for all {len(node_ids)} nodes to answer qauld-ctl")
    wait_for_nodes(node_ids, timeout=args.timeout)

    print("waiting for every node to publish its own account")
    wait_for_local_accounts(node_ids, timeout=args.timeout)

    stale = [
        d
        for d in os.listdir("/tmp")
        if d.startswith("qaul-")
        and os.path.isdir(f"/tmp/{d}")
        and d.replace("qaul-", "") not in set(node_ids)
    ]
    if stale:
        print(f"\nnote: {len(stale)} node dir(s) in /tmp belong to an earlier run "
              f"and are ignored (the reducer keys off --roles), e.g. {sorted(stale)[:3]}")

    print(f"\n{len(node_ids)} nodes up. next: ./setup_communities.py --roles {args.roles}")


if __name__ == "__main__":
    main()
