#!/usr/bin/env python3
"""Inspect a wired world: spheres, links, containment, delegation, directory.

Run it straight after wire.py phase2, with --t0 set to the dial time and
--poll to the settle. It reads the directory first and keeps re-reading it, so
each gateway's completion is timed from the dial. Everything else is judged
no sooner than 70 s after the dial: qaul's mDNS asks only every 30 s, so a LAN
neighbourship that should never have formed may not have formed *yet*.

What it reports, and what each line is held against:

  spheres      no node has a LAN neighbour outside its own Local sphere. A
               crossing means the mDNS filter on an Internet link failed and two
               communities merged (link_filter.sh). Asserted.
  links        every planned Internet link is up, nothing unplanned, and the
               ping round trip each end measures (rtt, microseconds). Asserted.
  role         the right nodes hold the gateway role (§2.3). Asserted on v2.
  containment  members know no user outside their own Local sphere (§2.3).
               Gateways are reported, not judged: they are supposed to know
               the world. Asserted on v2.
  delegation   which gateway each member subscribed to, and whether it was the
               lowest-metric eligible one (§10.3 SHOULD). Reported.
  directory    at every node in the gateway role, its own manifest entries plus
               the trusted entries of every manifest it holds, against the
               users reachable from anywhere (§2.3: the union of gateway
               manifests is the global directory). With --poll, re-read until
               complete and record when each node got there. Reported.

Usage:
    ./check.py --roles topologies/world-1-half.roles.json [--poll 300 --t0 EPOCH]
               [--json ../results/world-check.json] [--v1]
"""

import argparse
import json
import time

from worldlib import (Node, batch_query, identities, lab_nodes, load_roles, name,
                      parse_far, reverse, sphere_members)
from wire import verify

# qaul's mDNS asks every 30 s (connections/lan.rs). Two rounds plus margin
# must pass after the dial before "no LAN neighbour across spheres" means
# anything.
MDNS_GUARD_S = 70


def check_spheres(roles, neighbours, by_peer) -> list[dict]:
    spheres = sphere_members(roles)
    crossings = []
    for nid, nb in neighbours.items():
        for transport in ("lan", "ble"):
            for n in nb.get(transport) or []:
                other = by_peer.get(n["node_id"])
                if other is None or other not in spheres[nid]:
                    crossings.append({
                        "node": name(roles, nid),
                        "neighbour": name(roles, other) if other else n["node_id"],
                        "transport": transport,
                    })
    return crossings


def link_rtts(roles, neighbours, ids, far_peers=None) -> list[dict]:
    """Each Internet link's ping time as each end measures it.

    In a hybrid run only the lab end is visible; a far node's peer id comes
    from far_peers (inferred by wire.verify). A link with both ends on the
    server (r1-S1) gets up=None: the lab cannot see it.
    """
    far_peers = far_peers or {}
    by_name = roles["by_name"]
    peer_of = {nid: v["peer"] for nid, v in ids.items()}
    peer_of.update({by_name[n]: p for n, p in far_peers.items()})
    rows = []
    for link in roles["internet_links"]:
        d, t = link["dialer"], link["target"]
        row = {"link": f"{name(roles, d)}->{name(roles, t)}", "far": link["far"]}
        if d not in neighbours and t not in neighbours:
            rows.append({**row, "up": None, "rtt_us": {}, "seen": "not visible from this lab"})
            continue
        rtt = {}
        for here, there in ((d, t), (t, d)):
            if here not in neighbours or there not in peer_of:
                continue
            for n in neighbours[here].get("internet") or []:
                if n["node_id"] == peer_of[there]:
                    rtt[name(roles, here)] = n.get("rtt")
        rows.append({**row, "up": bool(rtt), "rtt_us": rtt})
    return rows


def containment(roles, nodes, ids) -> dict:
    spheres = sphere_members(roles)
    by_user = reverse(ids, "user")
    known = batch_query(nodes, "known_user_ids")
    members, gateways = {}, {}
    for nid, users in known.items():
        own = {ids[m]["user"] for m in spheres[nid] if m in ids}
        foreign = [u for u in users if u not in own]
        row = {"known": len(users), "foreign": len(foreign),
               "foreign_names": sorted(name(roles, by_user[u]) if u in by_user else u[:10]
                                       for u in foreign)[:10]}
        role = roles["nodes"][nid]["role"]
        (members if role == "member" else gateways)[name(roles, nid)] = row
    leaks = {k: v for k, v in members.items() if v["foreign"]}
    return {"members_checked": len(members), "members_with_foreign_users": leaks,
            "gateway_role_nodes": gateways}


def delegation(roles, nodes, ids) -> dict:
    by_v2 = reverse(ids, "v2")
    spheres = {s["name"]: s for s in roles["spheres"]}
    members = [nid for nid in nodes if roles["nodes"][nid]["role"] == "member"
               and spheres[roles["nodes"][nid]["sphere"]]["gateways"]]
    delegations = batch_query({m: nodes[m] for m in members}, "router_v2_delegations")
    tables = batch_query({m: nodes[m] for m in members}, "router_v2_table")

    split: dict[str, dict[str, int]] = {}
    unsubscribed, not_lowest, details = [], [], {}
    for m in members:
        sphere = spheres[roles["nodes"][m]["sphere"]]
        counts = split.setdefault(sphere["name"], {name(roles, g): 0 for g in sphere["gateways"]})
        subs = delegations[m].get("subscriptions") or []
        target = by_v2.get(subs[0]["target_node_id"]) if subs else None

        # §10.3 eligibility as the member sees it: local_only, and a gateway.
        metric = {}
        for row in tables[m]:
            if str(row.get("space")).lower() != "node":
                continue
            node = by_v2.get(row.get("target_id"))
            if node in sphere["gateways"] and row.get("local_only"):
                metric[node] = row.get("metric")
        best = min(metric.values()) if metric else None

        details[name(roles, m)] = {
            "gateway": name(roles, target) if target else None,
            "metric_to_gateways": {name(roles, g): v for g, v in metric.items()},
            "outstanding": len(delegations[m].get("outstanding") or []),
            "declined": len(delegations[m].get("declined") or []),
        }
        if target is None:
            unsubscribed.append(name(roles, m))
            continue
        counts[name(roles, target)] = counts.get(name(roles, target), 0) + 1
        if best is not None and metric.get(target) is not None and metric[target] > best:
            not_lowest.append(f"{name(roles, m)}->{name(roles, target)} "
                              f"({metric[target]} vs best {best})")
    return {"split": split, "unsubscribed": unsubscribed,
            "not_lowest_metric": not_lowest, "members": details}


def directory(roles, nodes) -> dict:
    expected = len(roles["expected"]["globally_reachable"])
    holders = [nid for nid in roles["expected"]["gateway_role"] if nid in nodes]
    sub = {nid: nodes[nid] for nid in holders}
    manifests = batch_query(sub, "router_v2_manifests")
    own = batch_query(sub, "router_v2_delegations")
    out = {}
    for nid in holders:
        origins = manifests[nid]
        trusted = sum(o.get("trusted_users") or 0 for o in origins)
        untrusted = sum((o.get("delegated_users") or 0) - (o.get("trusted_users") or 0)
                        for o in origins)
        stale = sum(1 for o in origins
                    if (o.get("advertised_version") or 0) > (o.get("committed_version") or 0))
        mine = len(own[nid].get("entries") or [])
        out[name(roles, nid)] = {
            "entries": mine + trusted, "expected": expected,
            "own": mine, "from_others": trusted, "untrusted": untrusted,
            "origins": len(origins), "stale_origins": stale,
            "complete": mine + trusted >= expected,
        }
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--poll", type=int, default=0,
                    help="re-read the directory until complete, up to this many "
                         "seconds after --t0")
    ap.add_argument("--interval", type=int, default=10)
    ap.add_argument("--t0", type=float, default=None,
                    help="epoch seconds of the dial; times are measured from it "
                         "(default: now)")
    ap.add_argument("--json", help="write the full result here")
    ap.add_argument("--v1", action="store_true", help="v1 arm: skip v2-only parts")
    ap.add_argument("--hybrid", action="store_true",
                    help="the far nodes run on test.qaul.net: check from the lab side only")
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR",
                    help="hybrid: where a far node listens (as given to wire.py)")
    ap.add_argument("--side", choices=("laptop", "server"), default="laptop",
                    help="hybrid: which machine's lab to check (default laptop). Each "
                         "side's gateways are held against the whole world's directory.")
    args = ap.parse_args()
    if args.side == "server" and not args.hybrid:
        raise SystemExit("--side server is part of a hybrid run: add --hybrid")

    roles = load_roles(args.roles)
    if args.hybrid:
        parse_far(roles, args.far, args.side)  # same validation as wire.py; addresses unused
    t0 = args.t0 if args.t0 is not None else time.time()
    node_ids = lab_nodes(roles, args.hybrid, args.side)
    nodes = {nid: Node(nid) for nid in node_ids}
    result = {"world": roles["name"], "arm": "v1" if args.v1 else "v2", "t0": t0,
              "hybrid": args.hybrid, "side": args.side}
    ok = True
    print(f"=== {roles['name']} ({result['arm']}"
          f"{', ' + args.side + ' side' if args.hybrid else ''}) ===")

    # 1. The directory first, so its completion times are measured from the
    #    dial rather than from whenever the slower checks finish.
    if not args.v1:
        deadline = t0 + args.poll
        completed_at: dict[str, float] = {}
        while True:
            dirs = directory(roles, nodes)
            now = round(time.time() - t0, 1)
            for g, d in dirs.items():
                if d["complete"] and g not in completed_at:
                    completed_at[g] = now
            if all(d["complete"] for d in dirs.values()) or time.time() >= deadline:
                break
            time.sleep(args.interval)
        result["directory"] = dirs
        result["directory_complete_at_s"] = completed_at
        expected = len(roles["expected"]["globally_reachable"])
        print(f"\n-- directory at each gateway-role node (expected {expected} users), "
              f"read at +{now}s --")
        for g, d in dirs.items():
            when = f"  complete at +{completed_at[g]}s" if g in completed_at else ""
            print(f"  {g:<4} {d['entries']:>4}  (own {d['own']}, from {d['origins']} "
                  f"origin(s) {d['from_others']}, untrusted {d['untrusted']}, "
                  f"stale origins {d['stale_origins']}){when}")
        incomplete = [g for g, d in dirs.items() if not d["complete"]]
        print(f"  incomplete: {len(incomplete)}"
              f"{'' if not incomplete else '  ' + ', '.join(incomplete)}")

    # 2. mDNS asks every 30 s, so a LAN neighbourship across an Internet link
    #    might not have formed yet. Two rounds plus margin before judging.
    wait = MDNS_GUARD_S - (time.time() - t0)
    if wait > 0:
        print(f"\nwaiting {wait:.0f}s more: spheres are judged no sooner than "
              f"{MDNS_GUARD_S}s after the dial (two mDNS rounds)")
        time.sleep(wait)
    result["judged_at_s"] = round(time.time() - t0, 1)

    ids = identities(roles, node_ids, v2=not args.v1)
    by_peer = reverse(ids, "peer")
    neighbours = batch_query(nodes, "router_neighbours")

    crossings = check_spheres(roles, neighbours, by_peer)
    result["sphere_crossings"] = crossings
    ok &= not crossings
    print(f"\n-- spheres: LAN neighbours outside their own sphere: {len(crossings)}"
          f" [{'ok' if not crossings else 'COMMUNITIES MERGED'}]")
    for c in crossings[:10]:
        print(f"  {c['node']} - {c['neighbour']} over {c['transport']}")

    print("\n-- Internet links and gateway role --")
    wired = verify(roles, args.v1, hybrid=args.hybrid, side=args.side)
    result["wiring"] = wired
    ok &= wired["ok"]
    if wired.get("far_peers"):
        print("  far nodes as the lab sees them: "
              + ", ".join(f"{n} {p[:16]}..." for n, p in wired["far_peers"].items()))
    rtts = link_rtts(roles, neighbours, ids, wired.get("far_peers"))
    result["links"] = rtts
    for r in rtts:
        if r["up"] is None:
            rtt = "not visible from this lab"
        else:
            rtt = ", ".join(f"{k} {v}us" for k, v in r["rtt_us"].items()) or "down"
        print(f"  {r['link']:<10} {'far ' if r['far'] else 'near'}  {rtt}")

    if not args.v1:
        cont = containment(roles, nodes, ids)
        result["containment"] = cont
        leaks = cont["members_with_foreign_users"]
        ok &= not leaks
        print(f"\n-- containment: {cont['members_checked']} members, "
              f"{len(leaks)} know users outside their sphere "
              f"[{'ok' if not leaks else 'MEMBRANE LEAK'}]")
        for k, v in list(leaks.items())[:10]:
            print(f"  {k}: {v['foreign']} foreign, e.g. {', '.join(v['foreign_names'][:5])}")

        dele = delegation(roles, nodes, ids)
        result["delegation"] = dele
        print("\n-- delegation: members per gateway --")
        for sphere, counts in dele["split"].items():
            print(f"  {sphere:<5} " + ", ".join(f"{g} {n}" for g, n in counts.items()))
        print(f"  not subscribed yet       {len(dele['unsubscribed'])}"
              f"{'' if not dele['unsubscribed'] else '  ' + ', '.join(dele['unsubscribed'][:12])}")
        print(f"  not the lowest metric    {len(dele['not_lowest_metric'])}"
              f"{'' if not dele['not_lowest_metric'] else '  ' + '; '.join(dele['not_lowest_metric'][:6])}")

    result["ok"] = ok
    if args.json:
        with open(args.json, "w") as f:
            json.dump(result, f, indent=1)
        print(f"\nwrote {args.json}")
    if not ok:
        raise SystemExit("check failed - see above")
    print("\nchecks passed (the directory and delegation lines are reported, not asserted)")


if __name__ == "__main__":
    main()
