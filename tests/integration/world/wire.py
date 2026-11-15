#!/usr/bin/env python3
"""Start a world's daemons and connect its Internet side.

The world counterpart of communities/bringup.py and communities/phases.py,
for any number of servers, gateways per community and remote users.

  bringup  (topology: <name>.json, every link already present and filtered)
     start qauld on every node and wait until each answers and publishes
     its own account.

  phase1   switch LAN off on every server, then assert each has no LAN
     neighbour and an empty user space. Servers boot with their Internet
     links in place, and only link_filter.sh stands between their LAN and
     the gateways, so this also tests the filter. It proves no server ever
     joined a Local sphere, rather than hoping stale entries expired later.

  phase2   for every Internet link in the roles file's dial plan, read the
     target's dialable address from `node info` and run
     `connections nodes add` on the dialer. Then wait and verify: every
     planned link is up, no unplanned Internet link exists, and the right
     nodes hold the gateway role.

  verify   re-run phase 2's checks without dialling.

  --hybrid --far S1=/ip4/<server>/udp/9229/quic-v1
     S1 and r1 run on test.qaul.net, started by the user. Only lab nodes are
     started and queried; far targets are dialled at the given address; a far
     node's peer id is inferred from what its lab dialers see; links between
     two far nodes (r1-S1) are the user's to check on the server.

Whether any LAN neighbourship crossed between spheres is NOT judged here: qaul's
mDNS asks every 30 s (connections/lan.rs), so a clean reading straight after
the dial proves nothing. check.py judges it after the settle, and
sphere_scan.py reads every log afterwards for a crossing at any moment.

Usage:
    ./wire.py bringup --roles topologies/world-1-half.roles.json
    ./wire.py phase1  --roles ...
    ./wire.py phase2  --roles ... [--settle 15]
    ./wire.py verify  --roles ...
Add --v1 to any of them for the v1 arm.
"""

import argparse
import json
import os
import time

from worldlib import (Node, batch_query, gateway_flag, identities, infer_far_peers,
                      lab_nodes, load_roles, name, parse_far, pick_lab_address,
                      reverse)
from lib.network import start_qaul, wait_for_local_accounts, wait_for_nodes


def bringup(roles: dict, timeout: int, v1: bool, hybrid: bool = False) -> bool:
    if not os.environ.get("MESHNET_LAB"):
        raise SystemExit("MESHNET_LAB is not set; lib.network needs it")
    # A v1 measurement labelled v2 would look like a valid result, so this is
    # a hard stop rather than a warning.
    if not v1 and not os.environ.get("QAUL_ROUTING_V2"):
        raise SystemExit(
            "QAUL_ROUTING_V2 is not set, so qauld would start on v1 routing.\n"
            "Set it, or pass --v1 to run the v1 arm on purpose."
        )
    node_ids = lab_nodes(roles, hybrid)
    print(f"starting qauld on {len(node_ids)} nodes ({'v1' if v1 else 'v2'} routing)")
    start_qaul()
    print(f"waiting for all {len(node_ids)} nodes to answer qauld-ctl")
    wait_for_nodes(node_ids, timeout=timeout)
    print("waiting for every node to publish its own account")
    wait_for_local_accounts(node_ids, timeout=timeout)
    print(f"{len(node_ids)} nodes up")
    return True


def phase1(roles: dict, v1: bool, hybrid: bool = False) -> bool:
    ok = True
    lab = set(lab_nodes(roles, hybrid))
    for sid in roles["servers"]:
        if sid not in lab:
            print(f"server {name(roles, sid)}: on test.qaul.net, its LAN is the user's to switch off")
            continue
        server = Node(sid)
        print(f"server {name(roles, sid)} ({sid}): disabling LAN")
        server.disable_transport("lan")
        lan = server.router_neighbours().get("lan") or []
        if v1:
            # v1 lists the node's own user as a Local connection.
            users = [e for e in server.router_table()
                     if not any(c["module"].lower() == "local" for c in e["connections"])]
        else:
            users = [e for e in server.router_v2_table() if e.get("space") == "user"]
        good = not lan and not users
        ok &= good
        print(f"  LAN neighbours {len(lan)}, user entries {len(users)} "
              f"[{'ok' if good else 'ALREADY JOINED A LOCAL SPHERE'}]")
    if not ok:
        print("A server learned users over LAN before its LAN went off, so mDNS got")
        print("past link_filter.sh. Check the filters on the server's links")
        print("(tc -s filter show) before running again.")
    return ok


def internet_pairs(neighbours: dict, by_peer: dict) -> set[str]:
    return {by_peer.get(n["node_id"], f"?{n['node_id'][:8]}")
            for n in (neighbours.get("internet") or [])}


def verify(roles: dict, v1: bool, quiet: bool = False, hybrid: bool = False) -> dict:
    """Planned Internet links up, nothing unplanned, gateway roles right.

    In a hybrid run only the lab end of each link can be seen. A far node's
    identity is inferred from its lab dialers (infer_far_peers), and a link
    between two far nodes (r1-S1) is the user's to check on the server.
    """
    node_ids = lab_nodes(roles, hybrid)
    lab = set(node_ids)
    ids = identities(roles, node_ids, v2=not v1)
    by_peer = reverse(ids, "peer")
    nodes = {nid: Node(nid) for nid in node_ids}
    neighbours = batch_query(nodes, "router_neighbours")
    far_peers = infer_far_peers(roles, neighbours, by_peer) if hybrid else {}
    by_peer = {**by_peer, **{p: t for t, p in far_peers.items()}}

    planned = {frozenset((l["dialer"], l["target"])) for l in roles["internet_links"]
               if l["dialer"] in lab or l["target"] in lab}
    seen = set()
    for nid in node_ids:
        for other in internet_pairs(neighbours[nid], by_peer):
            seen.add(frozenset((nid, other)))

    missing = sorted(planned - seen, key=sorted)
    unplanned = sorted(seen - planned, key=sorted)

    def label(pair) -> str:
        return "-".join(name(roles, x) if x in roles["nodes"] else x for x in sorted(pair))

    result = {"missing_links": [label(p) for p in missing],
              "unplanned_links": [label(p) for p in unplanned],
              "far_peers": {name(roles, t): p for t, p in far_peers.items()}}
    if not quiet:
        print(f"  Internet links up      {len(planned & seen)}/{len(planned)}"
              f"{'' if not missing else '  MISSING: ' + ', '.join(result['missing_links'])}")
        print(f"  unplanned links        {len(unplanned)}"
              f"{'' if not unplanned else '  ' + ', '.join(result['unplanned_links'])}")

    servers_on_lan = [name(roles, s) for s in roles["servers"]
                      if s in lab and neighbours[s].get("lan")]
    result["servers_with_lan_neighbours"] = servers_on_lan
    if not quiet:
        print(f"  servers with LAN peers {len(servers_on_lan)}"
              f"{'' if not servers_on_lan else '  ' + ', '.join(servers_on_lan)}")

    ok = not missing and not unplanned and not servers_on_lan
    if not v1:
        status = batch_query(nodes, "router_v2_status")
        expected = set(roles["expected"]["gateway_role"]) & lab
        wrong = []
        for nid in node_ids:
            flag = gateway_flag(status[nid])
            if flag is None or flag != (nid in expected):
                wrong.append(f"{name(roles, nid)}={flag}")
        result["wrong_gateway_role"] = wrong
        ok &= not wrong
        if not quiet:
            print(f"  gateway role           {len(expected)} expected"
                  f"{', all right' if not wrong else '; WRONG: ' + ', '.join(wrong)}")
    result["ok"] = ok
    return result


def phase2(roles: dict, settle: int, v1: bool, hybrid: bool = False,
           far: dict[str, str] | None = None) -> bool:
    far = far or {}
    lab = set(lab_nodes(roles, hybrid))
    for sid in roles["servers"]:
        if sid not in lab:
            continue
        lan = Node(sid).router_neighbours().get("lan") or []
        if lan:
            raise SystemExit(f"server {name(roles, sid)} has {len(lan)} LAN neighbour(s): "
                             "run phase1 first")

    addresses: dict[str, str] = {}
    for link in roles["internet_links"]:
        dialer, target = link["dialer"], link["target"]
        if dialer not in lab:
            print(f"  {name(roles, dialer):>4} -> {name(roles, target):<4} "
                  "on test.qaul.net, dialled by the user")
            continue
        if target in far:
            addresses[target] = far[target]
        elif target not in addresses:
            address = pick_lab_address(Node(target).addresses())
            if not address:
                raise SystemExit(f"{name(roles, target)} advertises no dialable ip4 address")
            addresses[target] = address
        Node(dialer).add_internet_peer(addresses[target], name(roles, target))
        print(f"  {name(roles, dialer):>4} -> {name(roles, target):<4} "
              f"{'(far) ' if link['far'] else ''}{addresses[target]}")

    print(f"waiting {settle}s for the dials to connect")
    time.sleep(settle)
    print("verifying:")
    return verify(roles, v1, hybrid=hybrid)["ok"]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("step", choices=("bringup", "phase1", "phase2", "verify"))
    ap.add_argument("--roles", required=True)
    ap.add_argument("--settle", type=int, default=15,
                    help="phase2: seconds to wait for the dials (default 15)")
    ap.add_argument("--timeout", type=int, default=120,
                    help="bringup: seconds to wait for the nodes (default 120)")
    ap.add_argument("--v1", action="store_true", help="v1 arm: skip v2-only checks")
    ap.add_argument("--hybrid", action="store_true",
                    help="the far nodes run on test.qaul.net, not in the lab")
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR",
                    help="hybrid: where a far node listens, e.g. "
                         "S1=/ip4/37.27.63.85/udp/9229/quic-v1 (repeat per far target)")
    args = ap.parse_args()

    roles = load_roles(args.roles)
    far = parse_far(roles, args.far) if args.hybrid else {}
    if args.step == "bringup":
        ok = bringup(roles, args.timeout, args.v1, args.hybrid)
    elif args.step == "phase1":
        ok = phase1(roles, args.v1, args.hybrid)
    elif args.step == "phase2":
        ok = phase2(roles, args.settle, args.v1, args.hybrid, far)
    else:
        print("verifying:")
        result = verify(roles, args.v1, hybrid=args.hybrid)
        print(json.dumps(result, indent=1))
        ok = result["ok"]
    if not ok:
        raise SystemExit(f"{args.step}: incomplete - see above")


if __name__ == "__main__":
    main()
