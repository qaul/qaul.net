#!/usr/bin/env python3
"""Time how a community lab converges, the same way for both arms.

  record  polls every node's `users list` once a second and appends a line
          whenever, for some observer, a user appears, gets its name, or
          gains or loses a route. Runs until it is sent SIGTERM.
  mark    appends a named timestamp (bringup, dial, cut_gateway ...) to the
          events file.
  reduce  turns the recording and the events into the comparison's times.

`users list` is the one instrument both arms share: v1 logs none of the
events v2 does. Its `connections` list is non-empty exactly when the node
holds a route to the user (router/users.rs, build_user_entry), so a user
counts as reachable from an observer when it is listed with connections.
The observer's own account is the one with a LOCAL connection.

"Z reachable from A" (A the first community's last member, Z the last
community's last member) is read where each arm resolves it: in v1 at A,
which holds every user; in v2 at A's gateway, because under section 2.3 a
member never holds a foreign user and forwards to its gateway instead. In
v2 it also needs A's own leg to that gateway, read as A routing the
gateway's user, so cutting a member breaks its side as it does in v1.

A recovery time only counts inside its heal window, which ends at the next
cut or when the recording stops. A direction that was already down when a
cut began is reported as such rather than as lost at once.

Every time comes from the poll that first saw the change, so it carries up
to one poll interval of error, equally in both arms.

Usage:
    ./timeline.py record --roles R --out T.jsonl
    ./timeline.py mark --events E.jsonl dial
    ./timeline.py reduce --roles R --recording T.jsonl --events E.jsonl --arm v2 [--json out.json]
"""

import argparse
import concurrent.futures
import json
import os
import signal
import sys
import time

# lib/ is one level up, shared with the rest of tests/integration.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.node import Node

POLL = 1.0


def load_roles(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def all_node_ids(roles: dict) -> list[str]:
    return [roles["hub"]] + [m for c in roles["communities"] for m in c["members"]]


# --------------------------------------------------------------------------
# record
# --------------------------------------------------------------------------


def poll_once(nodes: dict) -> dict:
    """{node: (response time, users list)} for every node that answered."""
    def ask(node: Node):
        users = node.known_users()
        return time.time(), users

    out = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=min(len(nodes), 32)) as pool:
        futures = {pool.submit(ask, node): nid for nid, node in nodes.items()}
        for future in concurrent.futures.as_completed(futures):
            try:
                out[futures[future]] = future.result()
            except Exception:
                # not up yet, or mid-restart: the next poll asks again
                pass
    return out


def record(roles: dict, out_path: str, interval: float) -> None:
    nodes = {nid: Node(nid) for nid in all_node_ids(roles)}
    state: dict[tuple[str, str], tuple[bool, bool]] = {}
    selfs: dict[str, str] = {}
    stop = False

    def on_signal(*_):
        nonlocal stop
        stop = True

    signal.signal(signal.SIGTERM, on_signal)
    signal.signal(signal.SIGINT, on_signal)

    with open(out_path, "a", buffering=1) as f:
        f.write(json.dumps({"t": time.time(), "started": True}) + "\n")
        while not stop:
            began = time.time()
            for nid, (t, users) in poll_once(nodes).items():
                listed = set()
                for user in users:
                    uid = user["id"]
                    listed.add(uid)
                    connections = user.get("connections") or []
                    if nid not in selfs and any(c.get("module") == "LOCAL" for c in connections):
                        selfs[nid] = uid
                        f.write(json.dumps({"t": t, "node": nid, "self": uid}) + "\n")
                    now = (bool(user.get("name")), bool(connections))
                    if state.get((nid, uid)) != now:
                        state[(nid, uid)] = now
                        f.write(json.dumps({"t": t, "node": nid, "user": uid,
                                            "named": now[0], "routed": now[1]}) + "\n")
                for key in [k for k in state if k[0] == nid and k[1] not in listed]:
                    del state[key]
                    f.write(json.dumps({"t": t, "node": nid, "user": key[1],
                                        "listed": False}) + "\n")
            time.sleep(max(0.0, interval - (time.time() - began)))
        f.write(json.dumps({"t": time.time(), "stopped": True}) + "\n")


# --------------------------------------------------------------------------
# mark
# --------------------------------------------------------------------------


def mark(events_path: str, name: str) -> None:
    with open(events_path, "a") as f:
        f.write(json.dumps({"event": name, "t": time.time()}) + "\n")


# --------------------------------------------------------------------------
# reduce
# --------------------------------------------------------------------------


class Recording:
    def __init__(self, path: str):
        self.selfs: dict[str, str] = {}
        # (observer, user) -> [(t, routed)] in time order; first listed / named
        self.routed: dict[tuple[str, str], list[tuple[float, bool]]] = {}
        self.listed: dict[tuple[str, str], float] = {}
        self.named: dict[tuple[str, str], float] = {}
        self.end = 0.0
        with open(path) as f:
            for line in f:
                rec = json.loads(line)
                self.end = max(self.end, rec["t"])
                if "self" in rec:
                    self.selfs[rec["node"]] = rec["self"]
                    continue
                if "user" not in rec:
                    continue
                key = (rec["node"], rec["user"])
                if rec.get("listed") is False:
                    self.routed.setdefault(key, []).append((rec["t"], False))
                    continue
                self.listed.setdefault(key, rec["t"])
                if rec["named"]:
                    self.named.setdefault(key, rec["t"])
                self.routed.setdefault(key, []).append((rec["t"], rec["routed"]))

    def routed_at(self, obs: str, uid: str, t: float) -> bool:
        """Whether obs held a route to uid at time t."""
        state = False
        for at, routed in self.routed.get((obs, uid), []):
            if at > t:
                break
            state = routed
        return state

    def holds(self, chain: list[tuple[str, str]], t: float) -> bool:
        """Whether every (observer, user) leg of the chain was routed at t."""
        return all(self.routed_at(obs, uid, t) for obs, uid in chain)

    def first(self, chain: list[tuple[str, str]], after: float, routed: bool) -> float | None:
        """First time at or after `after` that the whole chain was routed
        (routed=True), or that some leg of it was not (routed=False)."""
        changes = sorted({t for leg in chain for t, _ in self.routed.get(leg, []) if t > after})
        return next((t for t in [after] + changes if self.holds(chain, t) == routed), None)

    def first_routed(self, obs: str, uid: str, after: float = 0.0) -> float | None:
        """First time at or after `after` that obs held a route to uid."""
        return self.first([(obs, uid)], after, True)


def latest(times: list[float | None]) -> float | None:
    """When the last of them happened, or None if any never did."""
    return None if not times or any(t is None for t in times) else max(times)


def since(t: float | None, origin: float | None) -> float | None:
    return None if t is None or origin is None else round(t - origin, 1)


def reduce(roles: dict, rec: Recording, events: dict, arm: str) -> dict:
    communities = roles["communities"]
    member_users = {m: rec.selfs.get(m) for c in communities for m in c["members"]}
    missing = [m for m, uid in member_users.items() if uid is None]
    community_users = [uid for uid in member_users.values() if uid]
    community_of = {m: c["index"] for c in communities for m in c["members"]}

    bringup, dial = events.get("bringup"), events.get("dial")
    first, last = communities[0], communities[-1]
    a, z = first["members"][-1], last["members"][-1]
    a_uid, z_uid = member_users.get(a), member_users.get(z)
    # where each end's peer is resolved (see the module docstring)
    a_resolver = a if arm == "v1" else first["gateway"]
    z_resolver = z if arm == "v1" else last["gateway"]
    if arm == "v1":
        directions = {"a_to_z": [(a, z_uid)], "z_to_a": [(z, a_uid)]}
    else:
        directions = {
            "a_to_z": [(a, member_users.get(a_resolver)), (a_resolver, z_uid)],
            "z_to_a": [(z, member_users.get(z_resolver)), (z_resolver, a_uid)],
        }

    out: dict = {"arm": arm, "a": a, "z": z,
                 "resolvers": {"a_to_z": a_resolver, "z_to_a": z_resolver},
                 "missing_self": missing, "events": events}

    # 1. local convergence: every member routes to every user of its own community
    local = {}
    for c in communities:
        own = [member_users[m] for m in c["members"] if member_users.get(m)]
        times = [rec.first_routed(o, u) for o in c["members"] for u in own
                 if u != member_users.get(o)]
        local[c["index"]] = since(latest(times), bringup)
    out["local_convergence_s"] = local
    out["local_convergence_max_s"] = (None if any(v is None for v in local.values())
                                      else max(local.values()))

    # 2. the hub routes to every community user
    out["hub_reaches_all_s"] = since(
        latest([rec.first_routed(roles["hub"], u) for u in community_users]), dial)

    # 3. every gateway routes to every community user: the global directory
    gateways = {}
    for c in communities:
        g = c["gateway"]
        times = [rec.first_routed(g, u) for u in community_users if u != member_users.get(g)]
        gateways[g] = since(latest(times), dial)
    out["gateway_directory_s"] = gateways
    out["gateway_directory_max_s"] = (None if any(v is None for v in gateways.values())
                                      else max(gateways.values()))

    # 4. Z reachable from A, and A from Z for the replies
    out["reachable_s"] = {d: since(rec.first(chain, 0.0, True), dial)
                          for d, chain in directions.items()}
    out["a_itself_lists_z_s"] = since(rec.listed.get((a, z_uid)), dial)

    # 5. every member routes to every community user, foreign ones included
    owner = {uid: m for m, uid in member_users.items() if uid}
    gateway_ids = {c["gateway"] for c in communities}
    plain = [m for m in member_users if m not in gateway_ids]
    foreign_pairs = [(o, u) for o in plain for u in community_users
                     if community_of[owner[u]] != community_of[o]]
    out["every_member_reaches_all_s"] = since(
        latest([rec.first_routed(o, u) for o, u in foreign_pairs]), dial)
    # containment, read before the crossing's first contact adds a profile
    before_cross = events.get("cross", float("inf"))
    out["members_holding_foreign_users"] = sorted(
        {o for o, u in foreign_pairs if rec.listed.get((o, u), float("inf")) < before_cross})

    # 8. Z's name at A, and where the crossing first contacted it
    out["z_named_at_a_s"] = since(rec.named.get((a, z_uid)), dial)

    # 9. A's gateway loses and regains its hub link
    # 10. Z leaves its community and comes back
    for label, cut, heal, after in (("gateway_reconnect", "cut_gateway", "heal_gateway", "cut_member"),
                                    ("member_leave", "cut_member", "heal_member", None)):
        if cut not in events or heal not in events:
            continue
        until = events.get(after, rec.end) if after else rec.end
        out[label] = {"cut_lasted_s": round(events[heal] - events[cut], 1),
                      "heal_window_s": round(until - events[heal], 1)}
        for d, chain in directions.items():
            held = rec.holds(chain, events[cut])
            lost = rec.first(chain, events[cut], False) if held else None
            if lost is not None and lost > events[heal]:
                lost = None
            back = rec.first(chain, events[heal], True)
            if back is not None and back > until:
                back = None
            out[label][d] = {
                "reachable_at_cut": held,
                "unreachable_after_cut_s": since(lost, events[cut]),
                "reachable_after_heal_s": since(back, events[heal]),
            }
    return out


def load_events(path: str) -> dict:
    events = {}
    with open(path) as f:
        for line in f:
            rec = json.loads(line)
            events[rec["event"]] = rec["t"]
    return events


def show(r: dict) -> None:
    def fmt(v):
        return "never" if v is None else f"{v}s"

    print(f"=== timeline, arm {r['arm']} (A={r['a']}, Z={r['z']}; resolved at "
          f"{r['resolvers']['a_to_z']} and {r['resolvers']['z_to_a']}) ===")
    if r["missing_self"]:
        print(f"  no own account seen for {r['missing_self']}")
    print(f"  1 local convergence        max {fmt(r['local_convergence_max_s'])}"
          f"  per community {r['local_convergence_s']}  (from bring-up)")
    print(f"  2 hub reaches every user   {fmt(r['hub_reaches_all_s'])}  (from dial)")
    print(f"  3 gateway directory        max {fmt(r['gateway_directory_max_s'])}"
          f"  {r['gateway_directory_s']}")
    print(f"  4 Z reachable from A       {fmt(r['reachable_s']['a_to_z'])}"
          f"  A from Z {fmt(r['reachable_s']['z_to_a'])}"
          f"  (A itself lists Z: {fmt(r['a_itself_lists_z_s'])})")
    print(f"  5 every member reaches all {fmt(r['every_member_reaches_all_s'])}"
          f"  ({len(r['members_holding_foreign_users'])} members hold foreign users)")
    print(f"  8 Z named at A             {fmt(r['z_named_at_a_s'])}")
    for label, n in (("gateway_reconnect", 9), ("member_leave", 10)):
        if label not in r:
            continue
        x = r[label]
        print(f"  {n} {label} (cut lasted {x['cut_lasted_s']}s,"
              f" heal window {x['heal_window_s']}s)")
        for d in ("a_to_z", "z_to_a"):
            if not x[d]["reachable_at_cut"]:
                lost = "already unreachable at the cut"
            elif x[d]["unreachable_after_cut_s"] is None:
                lost = "stayed reachable through the cut"
            else:
                lost = f"unreachable {x[d]['unreachable_after_cut_s']}s after the cut"
            back = x[d]["reachable_after_heal_s"]
            back = f"not back within {x['heal_window_s']}s" if back is None else f"back {back}s after the heal"
            print(f"      {d}: {lost}, {back}")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="mode", required=True)

    p_rec = sub.add_parser("record")
    p_rec.add_argument("--roles", required=True)
    p_rec.add_argument("--out", required=True)
    p_rec.add_argument("--interval", type=float, default=POLL)

    p_mark = sub.add_parser("mark")
    p_mark.add_argument("--events", required=True)
    p_mark.add_argument("name")

    p_red = sub.add_parser("reduce")
    p_red.add_argument("--roles", required=True)
    p_red.add_argument("--recording", required=True)
    p_red.add_argument("--events", required=True)
    p_red.add_argument("--arm", choices=("v1", "v2"), required=True)
    p_red.add_argument("--json", dest="json_out", default=None)

    args = ap.parse_args()
    if args.mode == "record":
        record(load_roles(args.roles), args.out, args.interval)
    elif args.mode == "mark":
        mark(args.events, args.name)
    else:
        result = reduce(load_roles(args.roles), Recording(args.recording),
                        load_events(args.events), args.arm)
        show(result)
        if args.json_out:
            with open(args.json_out, "w") as f:
                json.dump(result, f, indent=2)


if __name__ == "__main__":
    main()
