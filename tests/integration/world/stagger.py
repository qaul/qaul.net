#!/usr/bin/env python3
"""Scenario 1: communities come online minutes apart.

Instead of wire.py phase2's single dial, the world's Internet side is dialled
in steps: first the backbone (the links between servers and remote users),
then one community at a time, --gap seconds apart, in --order. Every
community has been running on its own radio links since the bring-up, so a
step is a village getting its Internet, not a village being built.

Every --interval seconds it samples, at every lab node in the gateway role:
  gateway   whether it holds the role, and its manifest version
            (router v2 status)
  inet      its Internet neighbours (router neighbours)
  own       how many users its own delegation list holds (router v2 delegations)
  origins   how many trusted users it holds per origin (router v2 manifests)
and which gateway each member is subscribed to.

For each community it reports, timed from that community's dial:
  links      when each dialled link was first seen up
  role       when each of its gateways took the gateway role (§10.1)
  bumps      its gateways' manifest versions as they were published (§10.8)
  members    when its members registered with one of its gateways (§10.3)
  seen       when each gateway already online listed all of its users
  sees       when each of its gateways listed every user already online
and, across the whole run, any directory that lost users after holding them
all.

The predictions are printed beside the results. Afterwards run.sh runs
check.py on the whole world, timed from the last dial.

Usage, as root, after wire.py bringup and phase1 (run.sh -j does it all):
    ./stagger.py --roles topologies/world-1.roles.json [--hybrid --far S1=...]
                 [--gap 180] [--first 90] [--after 240] [--order T,V,W,H,X+Y]
                 [--json ../results/stagger.json]
"""

import argparse
import json
import statistics
import time
import traceback

from scenario import gather
from wire import address_of, assert_servers_off_lan, dial
from worldlib import Node, identities, lab_nodes, load_roles, name, parse_far, reverse

BACKBONE = "backbone"

PREDICT = [
    ("links and role",
     "a joining gateway takes the role once its Internet link is up, within about 10 s "
     "of the dial (§10.1: is_gateway is set while the host has an active Internet "
     "connection)"),
    ("first bump",
     "it publishes is_gateway=1 at once if it hasn't published in the last 60 s, "
     "otherwise when its 60 s window opens (§10.8: an is_gateway change is rate-limited)"),
    ("members",
     "its members register within seconds of hearing that version, a few seconds per "
     "hop (§10.3, advertise-and-pull §10.8)"),
    ("seen",
     "its members' entries wait for the next 60 s windows (§10.8), so the community "
     "appears at the other gateways in steps 60 s apart; the farthest member decides "
     "when it is complete, as in the all-at-once formation (about +190 s for T)"),
    ("sees",
     "the newcomer pulls the directory that is already online within seconds: an "
     "INDEX_DUMP on connect advertises every committed manifest version (§8.2, §8.4), "
     "and it pulls each one (§10.8)"),
    ("no loss",
     "a community joining changes nothing that is already online: no directory loses "
     "users it held"),
]


def plan_steps(roles: dict, order: list[str]) -> list[dict]:
    """The backbone first, then one sphere per step. Every Internet link must
    land in exactly one step: a link belongs to its dialer's sphere, and a
    dialer outside every sphere (a server or remote user) is backbone."""
    spheres = {s["name"]: s for s in roles["spheres"]}
    unknown = [o for o in order if o not in spheres]
    if unknown:
        raise SystemExit(f"--order: no sphere {', '.join(unknown)} in {roles['name']} "
                         f"(it has {', '.join(spheres)})")
    sphere_of = {g: s["name"] for s in roles["spheres"] for g in s["gateways"]}
    steps = [{"name": BACKBONE, "sphere": None, "links": []}]
    by_sphere = {o: {"name": o, "sphere": o, "links": []} for o in order}
    for link in roles["internet_links"]:
        sp = sphere_of.get(link["dialer"])
        if sp is None:
            if link["target"] in sphere_of:
                raise SystemExit(f"{name(roles, link['dialer'])} -> "
                                 f"{name(roles, link['target'])}: a backbone node dials a "
                                 "community gateway, which this scenario doesn't model")
            steps[0]["links"].append(link)
        elif sp in by_sphere:
            by_sphere[sp]["links"].append(link)
        else:
            raise SystemExit(f"--order leaves out {sp}, which has Internet links")
    steps += [by_sphere[o] for o in order]
    empty = [s["name"] for s in steps if not s["links"]]
    if empty:
        raise SystemExit(f"no Internet links to dial for {', '.join(empty)}")
    return steps


class View:
    """The lab as the sampler sees it: names, groups, expected counts."""

    def __init__(self, roles: dict, hybrid: bool):
        self.roles = roles
        self.lab = sorted(lab_nodes(roles, hybrid))
        labset = set(self.lab)
        self.spheres = {s["name"]: s for s in roles["spheres"]}
        self.gw_role = set(roles["expected"]["gateway_role"])
        ids = identities(roles, self.lab)
        self.by_peer = reverse(ids, "peer")
        self.by_v2 = reverse(ids, "v2")
        self.gateways = {g: Node(g) for g in self.lab if g in self.gw_role}
        self.members = {m: Node(m) for m in self.lab
                        if roles["nodes"][m]["role"] == "member"
                        and self.spheres[roles["nodes"][m]["sphere"]]["gateways"]}
        self.expected: dict[str, int] = {}  # users each group should add to a directory
        for nid in roles["expected"]["globally_reachable"]:
            g = roles["nodes"][nid]["sphere"] or BACKBONE
            self.expected[g] = self.expected.get(g, 0) + 1
        self.labset = labset

    def n(self, nid: str) -> str:
        return name(self.roles, nid)

    def group(self, nm: str) -> str | None:
        """The sphere a node's users belong to, BACKBONE for a server or remote
        user, including a far one the lab can't name ("far:..."). None for an
        origin that isn't in the gateway role: a member's own manifest, which
        lists no trusted users anyway."""
        if nm.startswith("far:"):
            return BACKBONE
        nid = self.roles["by_name"].get(nm)
        if nid is None:
            return None
        meta = self.roles["nodes"][nid]
        if meta["sphere"] is None:
            return BACKBONE
        return meta["sphere"] if nid in self.gw_role else None

    def count(self, origins: dict, group: str) -> int:
        return sum(v for o, v in origins.items() if self.group(o) == group)


def sample(v: View) -> dict:
    status = gather(v.gateways, "router_v2_status")
    nbs = gather(v.gateways, "router_neighbours")
    own = gather(v.gateways, "router_v2_delegations")
    man = gather(v.gateways, "router_v2_manifests")
    subs = gather(v.members, "router_v2_delegations")
    out = {"gateway": {}, "version": {}, "inet": {}, "own": {}, "origins": {}, "subs": {}}
    for g in v.gateways:
        gn = v.n(g)
        st = status.get(g)
        out["gateway"][gn] = st.get("is_gateway") if st else "?"
        out["version"][gn] = st.get("manifest_version") if st else "?"
        nb = nbs.get(g)
        out["inet"][gn] = (sorted(v.n(v.by_peer[x["node_id"]]) if x["node_id"] in v.by_peer
                                  else "far" for x in nb.get("internet") or [])
                           if nb else "?")
        d = own.get(g)
        out["own"][gn] = len(d.get("entries") or []) if d else "?"
        rows = man.get(g)
        out["origins"][gn] = ({(v.n(v.by_v2[r["node_id"]]) if r["node_id"] in v.by_v2
                                else "far:" + str(r["node_id"])[-4:]): r.get("trusted_users") or 0
                               for r in rows} if rows is not None else "?")
    for m in v.members:
        d = subs.get(m)
        if d is None:
            out["subs"][v.n(m)] = "?"
            continue
        s = d.get("subscriptions") or []
        tgt = v.by_v2.get(s[0]["target_node_id"]) if s else None
        out["subs"][v.n(m)] = v.n(tgt) if tgt else None
    return out


# ---------------------------------------------------------------- analysis


def first_at(samples, since, pred):
    """Seconds after `since` of the first sample at or after it where pred holds."""
    for s in samples:
        if s["t"] >= since and pred(s):
            return round(s["t"] - since, 1)
    return None


def spread(xs):
    xs = [x for x in xs if x is not None]
    if not xs:
        return None
    return {"min": min(xs), "median": round(statistics.median(xs), 1), "max": max(xs)}


def analyse(v: View, steps: list[dict], events: list[dict], samples: list[dict]) -> dict:
    roles = v.roles
    out = {"steps": [], "drops": []}
    online_groups: list[str] = []      # groups whose users should be visible by now
    online_gws: list[str] = []         # lab gateway-role nodes dialled so far
    for st, ev in zip(steps, events):
        td = ev["t"]
        if st["sphere"] is None:
            joining = sorted({x for l in st["links"] for x in (l["dialer"], l["target"])
                              if x in v.gateways})
            group = BACKBONE
        else:
            joining = [g for g in v.spheres[st["sphere"]]["gateways"] if g in v.gateways]
            group = st["sphere"]
        jn = [v.n(g) for g in joining]
        r = {"step": st["name"], "t": td, "expected_users": v.expected.get(group, 0)}

        links = {}
        for l in st["links"]:
            d, t = l["dialer"], l["target"]
            label = f"{v.n(d)}-{v.n(t)}"
            if d not in v.labset:
                links[label] = "on the server"
                continue
            want = v.n(t) if t in v.labset else "far"
            links[label] = first_at(samples, td, lambda s, d=v.n(d), w=want:
                                    isinstance(s["inet"].get(d), list) and w in s["inet"][d])
        r["links"] = links
        r["role"] = {g: first_at(samples, td, lambda s, g=g: s["gateway"].get(g) is True)
                     for g in jn}

        bumps = {}
        for g in jn:
            seq, prev = [], None
            for s in samples:
                ver = s["version"].get(g)
                if ver == "?":
                    continue
                if prev is not None and ver != prev and s["t"] >= td:
                    seq.append([ver, round(s["t"] - td, 1)])
                prev = ver
            bumps[g] = seq
        r["bumps"] = bumps

        if st["sphere"] is not None:
            ms = [v.n(m) for m in v.members if roles["nodes"][m]["sphere"] == st["sphere"]]
            reg = {m: first_at(samples, td, lambda s, m=m: s["subs"].get(m) in jn)
                   for m in ms}
            r["members"] = {"n": len(ms), "registered": sum(x is not None for x in reg.values()),
                            "spread": spread(reg.values()),
                            "never": sorted(m for m, x in reg.items() if x is None),
                            "each": reg}
            observers = [v.n(o) for o in online_gws]
            exp = v.expected.get(group, 0)
            seen = {}
            for o in observers:
                seen[o] = {
                    "first": first_at(samples, td, lambda s, o=o: isinstance(s["origins"].get(o), dict)
                                      and v.count(s["origins"][o], group) > 0),
                    "all": first_at(samples, td, lambda s, o=o: isinstance(s["origins"].get(o), dict)
                                    and v.count(s["origins"][o], group) >= exp),
                }
            r["seen"] = seen
            r["seen_all_spread"] = spread(x["all"] for x in seen.values())
            r["seen_never"] = sorted(o for o, x in seen.items() if x["all"] is None)
            last = samples[-1]["origins"] if samples else {}
            r["seen_never_end"] = {o: (v.count(last[o], group) if isinstance(last.get(o), dict)
                                       else "?") for o in r["seen_never"]}

            earlier = list(online_groups)
            r["sees_groups"] = {gr: v.expected.get(gr, 0) for gr in earlier}
            r["sees"] = {g: first_at(samples, td, lambda s, g=g: isinstance(s["origins"].get(g), dict)
                                     and all(v.count(s["origins"][g], gr) >= v.expected.get(gr, 0)
                                             for gr in earlier))
                         for g in jn}
            r["sees_short_end"] = {
                g: {gr: v.count(last[g], gr) for gr in earlier
                    if v.count(last[g], gr) < v.expected.get(gr, 0)}
                for g in jn if r["sees"][g] is None and isinstance(last.get(g), dict)}
        out["steps"].append(r)
        online_groups.append(group)
        online_gws += [g for g in joining if g not in online_gws]

    # A directory that held all of a group's users and later held fewer.
    complete: set[tuple[str, str]] = set()
    for s in samples:
        for o, origins in s["origins"].items():
            if not isinstance(origins, dict):
                continue
            for gr, exp in v.expected.items():
                if v.group(o) == gr:
                    continue  # its own group: partly in its own entries, not in origins
                c = v.count(origins, gr)
                if c >= exp:
                    complete.add((o, gr))
                elif (o, gr) in complete:
                    out["drops"].append({"t": s["t"], "observer": o, "group": gr,
                                         "had": exp, "now": c})
                    complete.discard((o, gr))
    return out


# ---------------------------------------------------------------- report


def fmt(x) -> str:
    return "never" if x is None else (x if isinstance(x, str) else f"+{x:g}s")


def report(result: dict, args) -> None:
    print("\n========== scenario 1: communities come online "
          f"{args.gap:g} s apart ==========")
    print("predicted (spec):")
    for k, p in PREDICT:
        print(f"  {k:<14} {p}")
    for r in result["steps"]:
        print(f"\n-- {r['step']} ({r['expected_users']} users), dialled at +{r['t']:g}s; "
              "times after its dial --")
        print("  links up        " + ", ".join(f"{k} {fmt(x)}" for k, x in r["links"].items()))
        print("  gateway role    " + ", ".join(f"{g} {fmt(x)}" for g, x in r["role"].items()))
        print("  manifest bumps  " + "; ".join(
            f"{g} " + (", ".join(f"v{ver} +{t:g}s" for ver, t in seq[:5]) or "none")
            for g, seq in r["bumps"].items()))
        if "members" not in r:
            continue
        m = r["members"]
        sp = m["spread"]
        print(f"  members         {m['registered']}/{m['n']} registered"
              + (f", first +{sp['min']:g}s, median +{sp['median']:g}s, last +{sp['max']:g}s"
                 if sp else "")
              + (f"; never: {', '.join(m['never'])}" if m["never"] else ""))
        if r["seen"]:
            sa = r["seen_all_spread"]
            print("  seen by others  all of them at every gateway already online: "
                  + (f"+{sa['min']:g}s to +{sa['max']:g}s (median +{sa['median']:g}s)"
                     if sa else "never")
                  + (f"; never at {', '.join(r['seen_never'])}" if r["seen_never"] else ""))
            print("                  " + ", ".join(
                f"{o} {fmt(x['first'])}/{fmt(x['all'])}" for o, x in r["seen"].items())
                + "   (first one / all)")
            if r["seen_never_end"]:
                print("                  never complete; held at the end: " + ", ".join(
                    f"{o} {c}/{r['expected_users']}" for o, c in r["seen_never_end"].items()))
        groups = ", ".join(f"{g} {n}" for g, n in r["sees_groups"].items())
        print(f"  newcomer sees   everyone already online ({groups}): "
              + ", ".join(f"{g} {fmt(x)}" for g, x in r["sees"].items()))
        for g, short in r["sees_short_end"].items():
            print(f"                  {g} at the end: "
                  + ", ".join(f"{gr} {c}/{r['sees_groups'][gr]}" for gr, c in short.items()))
    d = result["drops"]
    print(f"\n-- directories that lost users after holding them all: {len(d)}")
    for x in d[:20]:
        print(f"  +{x['t']:g}s {x['observer']}: {x['group']} {x['had']} -> {x['now']}")


# ---------------------------------------------------------------- main


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--hybrid", action="store_true")
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR")
    ap.add_argument("--order", default="T,V,W,H,X+Y",
                    help="spheres in the order they come online (default T,V,W,H,X+Y)")
    ap.add_argument("--first", type=float, default=90,
                    help="seconds from the backbone's dial to the first community's "
                         "(default 90: the backbone's own first bump must be out)")
    ap.add_argument("--gap", type=float, default=180,
                    help="seconds between two communities' dials (default 180)")
    ap.add_argument("--after", type=float, default=240,
                    help="seconds to keep sampling after the last dial (default 240)")
    ap.add_argument("--interval", type=float, default=5)
    ap.add_argument("--json")
    args = ap.parse_args()

    roles = load_roles(args.roles)
    far = parse_far(roles, args.far) if args.hybrid else {}
    v = View(roles, args.hybrid)
    assert_servers_off_lan(roles, v.labset)
    steps = plan_steps(roles, [o.strip() for o in args.order.split(",") if o.strip()])
    addresses: dict[str, str] = {}
    for st in steps:  # read every target's address now, so a step's dial is quick
        for link in st["links"]:
            if link["dialer"] in v.labset:
                address_of(roles, link["target"], far, addresses)

    schedule = [0.0] + [args.first + i * args.gap for i in range(len(steps) - 1)]
    end = schedule[-1] + args.after
    print(f"=== scenario 1: {len(steps) - 1} communities, {args.gap:g} s apart, after the "
          f"backbone; about {end / 60:.0f} min ===")
    for st, at in zip(steps, schedule):
        print(f"  +{at:5.0f}s  {st['name']:<9} "
              + ", ".join(f"{name(roles, l['dialer'])}->{name(roles, l['target'])}"
                          f"{' (far)' if l['far'] else ''}" for l in st["links"]))
    print(f"sampling {len(v.gateways)} gateway-role nodes and {len(v.members)} members "
          f"every {args.interval:g}s")

    samples, events = [], []
    t0 = time.time()
    i = 0
    try:
        while True:
            now = time.time() - t0
            if i < len(steps) and now >= schedule[i]:
                st = steps[i]
                td = time.time()
                print(f"[+{td - t0:6.1f}s] {st['name']} comes online", flush=True)
                dial(roles, st["links"], v.labset, far, addresses)
                events.append({"step": st["name"], "t": round(td - t0, 1), "t_abs": td,
                               "links": [f"{name(roles, l['dialer'])}-{name(roles, l['target'])}"
                                         for l in st["links"]]})
                i += 1
                continue
            if now >= end:
                break
            tick = time.time()
            s = sample(v)
            s["t"] = round(tick - t0, 1)
            samples.append(s)
            nxt = tick + args.interval
            if i < len(steps):
                nxt = min(nxt, t0 + schedule[i])
            time.sleep(max(0.0, nxt - time.time()))
    finally:
        result = {}
        if events:
            try:
                result = analyse(v, steps[:len(events)], events, samples)
                report(result, args)
            except Exception:  # still write the samples: they are the evidence
                traceback.print_exc()
        if args.json:
            with open(args.json, "w") as f:
                json.dump({"scenario": "stagger", "world": roles["name"], "hybrid": args.hybrid,
                           "order": args.order, "first": args.first, "gap": args.gap,
                           "after": args.after, "expected": v.expected,
                           "predict": dict(PREDICT), "t0": t0,
                           "t_last_dial": events[-1]["t_abs"] if events else None,
                           "events": events, "samples": samples, "result": result},
                          f, indent=1)
            print(f"\nwrote {args.json}")

    down = [k for r in result.get("steps", []) for k, x in r["links"].items() if x is None]
    if down:
        raise SystemExit(f"links never seen up: {', '.join(down)}")


if __name__ == "__main__":
    main()
