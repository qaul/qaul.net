#!/usr/bin/env python3
"""Scenario 2: send messages between chosen pairs and time each one.

Run on a formed world (after check.py). Pairs are chosen from where each
member actually delegated, and grouped by the path the message must take
(PLAN-real-internet.md, section 10):

  local       two members of one community, far apart in its radio mesh
  bridge      a Y member to an X member, across the twin villages' radio link
  far-N       across communities, by how many far (real-Internet) links lie on
              the gateway path: sender's nearest gateway -> recipient's gateway.
              In world-1 that is far-0 (T3 -> S2 -> V2: all in the lab) or far-2
              (T1 -> S1 -> V1, T3 -> S2 -> S1 -> V1: out to S1 and back), since
              every path through S1 enters and leaves it.
  transit     an H member to elsewhere: H1 -> W1 -> ..., W1 relaying
  no-gateway  a Y member (no gateway of its own) to another community, via X1
  remote      a member to a remote user (r2, r3; r1 too in a lab-only run)
  island      a member to an I member: expected NOT to arrive, I has no gateway

Each pair gets a group of its own, because a member never learns another
community's users (§2.3) and qaul resolves a direct chat through the sender's
user list. So the sender creates a group and invites the recipient by user
id, and the recipient accepts. Then the sender sends --messages messages.

Timing comes from qaul itself: the recipient stores the sender's sent_at and
its own received_at (ms), and both ends are lab nodes on one clock. So each
message's delivery time is received_at - sent_at, exact to the millisecond.
The first message of a pair is reported apart, since it may carry session
set-up. In a hybrid run r1 is on the server: it could not accept an invite,
so it is left out.

Usage:
    ./traffic.py --roles topologies/world-1.roles.json [--hybrid --far S1=...]
                 [--per-class 3] [--messages 5] [--interval 2] [--timeout 180]
                 [--seed 1] [--json ../results/traffic.json]
"""

import argparse
import json
import random
import statistics
import time
from collections import deque
from concurrent.futures import ThreadPoolExecutor

from check import delegation
from worldlib import (Node, identities, lab_nodes, load_roles, name, parse_far)

POLL = 1.0


def wait_for(predicate, timeout: float, poll: float = POLL):
    """predicate()'s first truthy value, or None on timeout. Errors count as
    not yet: a node busy for a moment must not end the pair."""
    end = time.time() + timeout
    while True:
        try:
            value = predicate()
        except Exception:
            value = None
        if value:
            return value
        if time.time() >= end:
            return None
        time.sleep(poll)


# ---------------------------------------------------------------- topology


def radio_graph(topo: dict) -> dict[str, set[str]]:
    g: dict[str, set[str]] = {}
    for link in topo["links"]:
        if link.get("internet_only"):
            continue
        g.setdefault(link["source"], set()).add(link["target"])
        g.setdefault(link["target"], set()).add(link["source"])
    return g


def hops_from(graph: dict, src: str) -> dict[str, int]:
    seen = {src: 0}
    q = deque([src])
    while q:
        cur = q.popleft()
        for nxt in sorted(graph.get(cur, ())):
            if nxt not in seen:
                seen[nxt] = seen[cur] + 1
                q.append(nxt)
    return seen


def internet_path(roles: dict, a: str, b: str) -> list[str] | None:
    """Fewest-hop path between two gateways over the planned Internet links.

    Every Internet hop costs the same (§5.1: weight 15, no quality penalty),
    so fewest hops is what the routing metric prefers too. Ties are broken by
    node id, which makes the label deterministic; the real choice may differ
    on a tie, which the report notes."""
    g: dict[str, set[str]] = {}
    for link in roles["internet_links"]:
        g.setdefault(link["dialer"], set()).add(link["target"])
        g.setdefault(link["target"], set()).add(link["dialer"])
    prev = {a: None}
    q = deque([a])
    while q:
        cur = q.popleft()
        if cur == b:
            break
        for nxt in sorted(g.get(cur, ())):
            if nxt not in prev:
                prev[nxt] = cur
                q.append(nxt)
    if b not in prev:
        return None
    path = [b]
    while path[-1] != a:
        path.append(prev[path[-1]])
    return path[::-1]


def far_hops(roles: dict, path: list[str]) -> int:
    far = {frozenset((l["dialer"], l["target"])) for l in roles["internet_links"] if l["far"]}
    return sum(1 for x, y in zip(path, path[1:]) if frozenset((x, y)) in far)


# ---------------------------------------------------------------- pairs


def choose_pairs(roles, topo, deleg, lab, ids, per_class, rng) -> list[dict]:
    radio = radio_graph(topo)
    by_name = roles["by_name"]
    members = {}  # id -> {community, sphere, nearest, delegated}
    for nm, d in deleg["members"].items():
        nid = by_name[nm]
        metrics = d["metric_to_gateways"]
        # The gateway forwarding.rs uses for an unknown recipient: the lowest
        # metric. On a tie it keeps whichever it meets first in a hash map, so
        # a tied sender's path cannot be labelled: such senders get no
        # nearest gateway and are not used where the path matters.
        best = min(metrics.values()) if metrics else None
        tied = [g for g, v in metrics.items() if v == best]
        nearest = tied[0] if len(tied) == 1 else None
        members[nid] = {"community": roles["nodes"][nid]["community"],
                        "sphere": roles["nodes"][nid]["sphere"],
                        "nearest": by_name[nearest] if nearest else None,
                        "delegated": by_name[d["gateway"]] if d["gateway"] else None}
    island = [nid for nid, m in roles["nodes"].items()
              if m["role"] == "member" and nid in lab and nid not in members]
    by_comm: dict[str, list[str]] = {}
    for nid, m in members.items():
        by_comm.setdefault(m["community"], []).append(nid)
    for c in by_comm.values():
        c.sort()

    pairs: list[dict] = []
    used: set[tuple[str, str]] = set()

    def add(cls, s, r, path=None, note=""):
        if (s, r) in used or s == r:
            return False
        used.add((s, r))
        pairs.append({"class": cls, "sender": s, "recipient": r,
                      "sender_name": name(roles, s), "recipient_name": name(roles, r),
                      "recipient_user": ids[r]["user"],
                      "path": [name(roles, x) for x in path] if path else None,
                      "note": note})
        return True

    # local: the farthest-apart pairs in each multi-gateway community
    for comm in ("T", "V", "W"):
        nodes = by_comm.get(comm, [])
        cands = []
        for s in nodes:
            h = hops_from(radio, s)
            for r in nodes:
                if r > s and r in h:
                    cands.append((h[r], s, r))
        cands.sort(reverse=True)
        for hops, s, r in cands[:max(1, per_class // 2)]:
            add("local", s, r, note=f"{comm}, {hops} radio hops")

    # bridge and no-gateway: from the Y members
    ys = by_comm.get("Y", [])
    xs = by_comm.get("X", [])
    ts = by_comm.get("T", [])
    for s in rng.sample(ys, min(per_class, len(ys))):
        add("bridge", s, rng.choice(xs), note="Y -> X across the radio bridge")
    for s in rng.sample(ys, min(per_class, len(ys))):
        r = rng.choice([t for t in ts if members[t]["delegated"]])
        path = internet_path(roles, members[s]["nearest"], members[r]["delegated"])
        add("no-gateway", s, r, path, note="Y has no gateway of its own")

    # across communities, by far hops on the gateway path
    senders = [s for c in ("T", "V", "X") for s in by_comm.get(c, []) if members[s]["nearest"]]
    recips = [r for c in ("T", "V", "X", "W") for r in by_comm.get(c, []) if members[r]["delegated"]]
    buckets: dict[int, list[tuple]] = {}
    for s in senders:
        for r in recips:
            if members[s]["sphere"] == members[r]["sphere"]:
                continue
            path = internet_path(roles, members[s]["nearest"], members[r]["delegated"])
            if path:
                buckets.setdefault(far_hops(roles, path), []).append((s, r, path))
    for k in sorted(buckets):
        cands = buckets[k]
        rng.shuffle(cands)
        # spread over sender communities and recipient gateways
        seen_kind, picked = set(), 0
        for s, r, path in cands:
            kind = (members[s]["community"], members[r]["delegated"])
            if kind in seen_kind:
                continue
            if add(f"far-{k}", s, r, path):
                seen_kind.add(kind)
                picked += 1
            if picked >= per_class:
                break

    # transit: H's members, through W1
    for s in rng.sample(by_comm.get("H", []), min(per_class, len(by_comm.get("H", [])))):
        r = rng.choice([v for v in by_comm.get("V", []) if members[v]["delegated"]])
        path = internet_path(roles, members[s]["nearest"], members[r]["delegated"])
        add("transit", s, r, path, note="H1 -> W1: W1 relays H's traffic")

    # remote users, as recipients
    for rid in roles["remote_users"]:
        if rid not in lab:
            continue  # r1 on the server cannot accept an invite
        s = rng.choice([t for t in ts if members[t]["nearest"]])
        # A remote user is its own gateway (§10.2), so the path ends at it.
        path = internet_path(roles, members[s]["nearest"], rid)
        add("remote", s, rid, path, note=f"to remote user {name(roles, rid)}")

    # island: expected not to arrive
    for r in rng.sample(island, min(per_class, len(island))):
        add("island", rng.choice(ts), r, note="I has no gateway: expected NOT to arrive")
    return pairs


# ---------------------------------------------------------------- one pair


def run_pair(i: int, p: dict, args, stamp: str, make_node=Node) -> dict:
    """make_node(id) gives the node object; ring_traffic.py passes one that
    reaches server-lab nodes over SSH."""
    out = dict(p)
    s, r = make_node(p["sender"]), make_node(p["recipient"])
    gname = f"probe-{stamp}-{i:02d}"
    timeout = args.timeout if p.get("class") != "island" else args.island_timeout
    try:
        gid = s.create_group(gname)
    except Exception as e:
        return {**out, "failed_at": "create", "error": str(e)}
    out["group_id"] = gid

    t_inv = int(time.time() * 1000)
    s.invite_to_group(gid, p["recipient_user"])
    inv = wait_for(lambda: next((x for x in r.invitations()
                                 if x.get("group_id") == gid), None), timeout)
    if not inv:
        return {**out, "failed_at": "invite"}
    out["invite_ms"] = inv.get("received_at", 0) - t_inv

    t_acc = time.time()
    r.accept_invitation(gid)
    ok = wait_for(lambda: any(m.get("user_id") == p["recipient_user"]
                              and m.get("state") == "Activated"
                              for m in s.group_info(gid).get("members", [])), timeout)
    if not ok:
        return {**out, "failed_at": "accept"}
    out["accept_s"] = round(time.time() - t_acc, 1)

    contents = []
    for j in range(args.messages):
        c = f"{gname}-m{j}"
        s.send_chat_message(gid, c)
        contents.append(c)
        if j + 1 < args.messages:
            time.sleep(args.interval)

    def arrived():
        got = {}
        for msg in r.conversation(gid).get("messages", []):
            for c in msg.get("content") or []:
                if c in contents:
                    got[c] = (msg.get("sent_at"), msg.get("received_at"))
        return got if len(got) == len(contents) else None

    got = wait_for(arrived, timeout) or {}
    if len(got) < len(contents):
        # one last read, to record what did arrive
        try:
            for msg in r.conversation(gid).get("messages", []):
                for c in msg.get("content") or []:
                    if c in contents:
                        got[c] = (msg.get("sent_at"), msg.get("received_at"))
        except Exception:
            pass
    out["sent"] = len(contents)
    out["delivered"] = len(got)
    out["latency_ms"] = [(got[c][1] - got[c][0]) if c in got and None not in got[c] else None
                         for c in contents]
    try:
        mine = [m for m in s.conversation(gid).get("messages", [])
                if any(c in contents for c in (m.get("content") or []))]
        out["confirmed"] = sum(1 for m in mine if m.get("status") == "Confirmed")
    except Exception:
        out["confirmed"] = None
    if out["delivered"] < out["sent"]:
        out["failed_at"] = "deliver"
    return out


# ---------------------------------------------------------------- report


def summarise(results: list[dict]) -> dict:
    classes: dict[str, dict] = {}
    for r in results:
        c = classes.setdefault(r["class"], {"pairs": 0, "groups": 0, "sent": 0, "delivered": 0,
                                            "first_ms": [], "rest_ms": [], "invite_ms": []})
        c["pairs"] += 1
        if "accept_s" in r:
            c["groups"] += 1
        c["sent"] += r.get("sent", 0)
        c["delivered"] += r.get("delivered", 0)
        lat = r.get("latency_ms") or []
        if lat and lat[0] is not None:
            c["first_ms"].append(lat[0])
        c["rest_ms"] += [x for x in lat[1:] if x is not None]
        if "invite_ms" in r:
            c["invite_ms"].append(r["invite_ms"])

    def stats(xs):
        if not xs:
            return None
        xs = sorted(xs)
        return {"n": len(xs), "min": xs[0], "median": statistics.median(xs),
                "p90": xs[min(len(xs) - 1, int(round(0.9 * (len(xs) - 1))))], "max": xs[-1]}

    return {k: {"pairs": v["pairs"], "groups": v["groups"], "sent": v["sent"],
                "delivered": v["delivered"], "first_ms": stats(v["first_ms"]),
                "rest_ms": stats(v["rest_ms"]), "invite_ms": stats(v["invite_ms"])}
            for k, v in classes.items()}


ORDER = ["local", "bridge", "far-0", "far-1", "far-2", "far-3", "transit",
         "no-gateway", "remote", "island"]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--topology", help="default: the roles file's world .json beside it")
    ap.add_argument("--hybrid", action="store_true")
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR")
    ap.add_argument("--per-class", type=int, default=3)
    ap.add_argument("--messages", type=int, default=5)
    ap.add_argument("--interval", type=float, default=2.0)
    ap.add_argument("--timeout", type=float, default=180,
                    help="per stage of each pair: invite, accept, delivery (s)")
    ap.add_argument("--island-timeout", type=float, default=60,
                    help="how long to wait before calling an island invite undelivered")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--json")
    args = ap.parse_args()

    roles = load_roles(args.roles)
    if args.hybrid:
        parse_far(roles, args.far)
    topo_path = args.topology or args.roles.replace(".roles.json", ".json")
    with open(topo_path) as f:
        topo = json.load(f)
    lab = set(lab_nodes(roles, args.hybrid))
    nodes = {nid: Node(nid) for nid in sorted(lab)}
    ids = identities(roles, sorted(lab))
    deleg = delegation(roles, nodes, ids)
    rng = random.Random(args.seed)
    pairs = choose_pairs(roles, topo, deleg, lab, ids, args.per_class, rng)
    pairs.sort(key=lambda p: (ORDER.index(p["class"]) if p["class"] in ORDER else 99,
                              p["sender_name"]))

    print(f"=== traffic: {len(pairs)} pairs, {args.messages} messages each, "
          f"{args.interval}s apart ===")
    for i, p in enumerate(pairs):
        path = " -> ".join(p["path"]) if p["path"] else ""
        print(f"  {i:2d} {p['class']:<10} {p['sender_name']:>5} -> {p['recipient_name']:<5} "
              f"{path:<28} {p['note']}")

    stamp = time.strftime("%H%M%S")
    t_start = time.time()
    with ThreadPoolExecutor(max_workers=len(pairs) or 1) as pool:
        results = list(pool.map(lambda ip: run_pair(ip[0], ip[1], args, stamp),
                                enumerate(pairs)))
    summary = summarise(results)

    print("\n-- per pair --")
    for r in results:
        lat = r.get("latency_ms")
        lat_s = ", ".join("-" if x is None else str(x) for x in lat) if lat else ""
        state = r.get("failed_at", "ok")
        print(f"  {r['class']:<10} {r['sender_name']:>5} -> {r['recipient_name']:<5} "
              f"invite {r.get('invite_ms', '-')!s:>6} ms  "
              f"delivered {r.get('delivered', 0)}/{r.get('sent', args.messages)}  "
              f"[{lat_s}] ms  {'' if state == 'ok' else 'FAILED at ' + state}")

    print("\n-- per class: delivery time in ms (received_at - sent_at) --")
    print(f"  {'class':<10} {'pairs':>5} {'groups':>6} {'delivered':>10}  "
          f"{'first msg median':>16}  {'later msgs median / p90 / max':>30}")
    ok = True
    for k in sorted(summary, key=lambda c: ORDER.index(c) if c in ORDER else 99):
        v = summary[k]
        rest, first = v["rest_ms"], v["first_ms"]
        first_s = str(first["median"]) if first else "-"
        rest_s = f"{rest['median']} / {rest['p90']} / {rest['max']}" if rest else "-"
        print(f"  {k:<10} {v['pairs']:>5} {v['groups']:>6} {v['delivered']:>4}/{v['sent']:<5}  "
              f"{first_s:>16}  {rest_s:>30}")
        if k == "island":
            ok &= v["groups"] == 0
        else:
            ok &= v["groups"] == v["pairs"] and v["delivered"] == v["sent"]
    print(f"\n  {'every expected message arrived, nothing reached the island' if ok else 'SOME EXPECTED MESSAGES DID NOT ARRIVE (or the island was reached)'}")

    if args.json:
        with open(args.json, "w") as f:
            json.dump({"world": roles["name"], "hybrid": args.hybrid, "t_start": t_start,
                       "settings": {k: getattr(args, k) for k in
                                    ("per_class", "messages", "interval", "timeout", "seed")},
                       "pairs": results, "summary": summary, "ok": ok}, f, indent=1)
        print(f"wrote {args.json}")
    if not ok:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
