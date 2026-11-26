#!/usr/bin/env python3
"""Generate a world topology from a description file.

A world is communities of different sizes, each a radio mesh with any number
of gateways (none included), joined over the Internet sphere through servers,
direct gateway-to-gateway peerings and remote users. Why it is shaped this way
is in tests/integration/PLAN-real-internet.md.

Where each node runs, in a hybrid run:
  near                       the laptop's lab (the default)
  far server / remote user   test.qaul.net itself, started by hand (S1, r1):
                             "host" in the roles file
  far community              a second lab on test.qaul.net (ring-1's F-J)
A laptop node reaches the server by dialling a host node, or a server-lab
node through a forwarded port on the server ("fwd_port" in the roles file,
from 9240 up; hostlink.sh sets the forwarding up). Either way it goes out
through egress.sh. A server-lab node reaches a host node through its own
link to the host (hostlink.sh). Nothing outside can dial into the laptop's
lab, so a link from the server to the laptop is refused.

Files written to topologies/:

  <name>.json         the full network: radio links and Internet links,
                      applied once, before any daemon starts
  <name>-hybrid.json  the laptop's part of a hybrid run: every far node and
                      every link touching one left out. Their laptop ends
                      reach test.qaul.net through egress.sh instead.
  <name>-server.json  only when a community is far: the server lab's part,
                      its nodes and the links among them
  <name>.roles.json   every node's name, role, community and sphere, the
                      Internet dial plan, and what the checks expect

There is no stage-1 variant, unlike the star suite. See below.

Inside a community, people are placed at random in an area and linked to
whoever is in radio range, nearest first, with at most 4 links each - the
number of connections a BLE radio holds. Pieces left apart are joined by
their closest pair, as if someone stood a little closer. Internet links are
not radio links, so they do not count toward the 4.

Every Internet link carries "internet_only": "1". link_filter.sh, run by
meshnet-lab for each link, drops mDNS on those links only. Without it, two
gateways joined by one (W1-H1) would find each other over LAN and merge
their communities into one Local sphere.

Why the whole network goes up before any daemon starts: meshnet-lab's
create_link (network.py) brings a link up and puts it into the node's bridge
first. Only then does it isolate the port and run the filter hook. While a
link is being created, its server's bridge briefly joins all the server's
links into one segment, with no filter on them yet. Lab runs on 2026-11-12
that added the Internet links to running daemons merged communities that
way: T2-W1, V1-X1 and others for 10-35 s, and W1-H1 for good. With no
daemon running, nothing is there to send in that window. Servers then boot
with links but behind the filter, and phase 1 switches their LAN off and
asserts they joined no Local sphere, which tests the filter too.

The layout is fixed by the seed in the description, so every run of a world
uses the same network.

Usage:
    ./gen_world.py worlds/world-1-half.json
"""

import argparse
import json
import math
import os
import random
from collections import deque

# A BLE radio holds at most 4 simultaneous connections.
BLE_MAX_LINKS = 4

# First public UDP port forwarded into the server's lab (S1 is 9229, r1 9230).
FWD_PORT_BASE = 9240

# Radio range in units where people stand at a density of one per unit area.
# Wide enough that most people have several candidates; the 4-link cap then
# trims the list, nearest first.
RADIO_RANGE = 1.5

# Space between communities on the drawing. Coordinates only matter for
# reading the layout; meshnet-lab does not use them.
COMMUNITY_GAP = 3.0


def dist(p, q) -> float:
    return math.hypot(p[0] - q[0], p[1] - q[1])


def components(n: int, links: list[tuple[int, int]]) -> list[list[int]]:
    adj: list[list[int]] = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        adj[b].append(a)
    seen = [False] * n
    out = []
    for start in range(n):
        if seen[start]:
            continue
        comp, queue = [], deque([start])
        seen[start] = True
        while queue:
            cur = queue.popleft()
            comp.append(cur)
            for nxt in adj[cur]:
                if not seen[nxt]:
                    seen[nxt] = True
                    queue.append(nxt)
        out.append(comp)
    return out


def diameter(n: int, links: list[tuple[int, int]]) -> int:
    adj: list[list[int]] = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        adj[b].append(a)
    worst = 0
    for src in range(n):
        seen = {src: 0}
        queue = deque([src])
        while queue:
            cur = queue.popleft()
            for nxt in adj[cur]:
                if nxt not in seen:
                    seen[nxt] = seen[cur] + 1
                    queue.append(nxt)
        worst = max(worst, max(seen.values()))
    return worst


def radio_mesh(n: int, rng: random.Random):
    """People at random in a sqrt(n)-wide square, linked within radio range.

    Returns (points, links) with links as local index pairs. Retries with
    fresh positions in the rare case the cap leaves pieces that cannot be
    joined, so the result is always connected and within the cap.
    """
    if n == 1:
        return [(0.0, 0.0)], []
    side = math.sqrt(n)
    for _ in range(100):
        pts = [(rng.uniform(0, side), rng.uniform(0, side)) for _ in range(n)]
        pairs = sorted(
            (dist(pts[a], pts[b]), a, b) for a in range(n) for b in range(a + 1, n)
        )
        degree = [0] * n
        links: list[tuple[int, int]] = []

        def add(a: int, b: int) -> None:
            links.append((a, b))
            degree[a] += 1
            degree[b] += 1

        for d, a, b in pairs:
            if d > RADIO_RANGE:
                break
            if degree[a] < BLE_MAX_LINKS and degree[b] < BLE_MAX_LINKS:
                add(a, b)

        while True:
            comps = components(n, links)
            if len(comps) == 1:
                return pts, links
            comp_of = {node: i for i, comp in enumerate(comps) for node in comp}
            joint = next(
                (
                    (a, b)
                    for _, a, b in pairs
                    if comp_of[a] != comp_of[b]
                    and degree[a] < BLE_MAX_LINKS
                    and degree[b] < BLE_MAX_LINKS
                ),
                None,
            )
            if joint is None:
                break
            add(*joint)
    raise AssertionError(f"no connected mesh of {n} within {BLE_MAX_LINKS} links")


def place_gateways(pts, specs: list[dict]) -> list[int]:
    """Pick which people hold the Internet connections.

    "centre" is the person nearest the middle. "edge" is the person farthest
    from the middle and from every gateway already placed, so several edge
    gateways spread out rather than bunch in one corner.
    """
    cx = sum(p[0] for p in pts) / len(pts)
    cy = sum(p[1] for p in pts) / len(pts)
    chosen: list[int] = []
    for spec in specs:
        free = [i for i in range(len(pts)) if i not in chosen]
        if not free:
            raise AssertionError(f"more gateways than people: {spec['name']}")
        place = spec.get("place", "edge")
        if place == "centre":
            pick = min(free, key=lambda i: dist(pts[i], (cx, cy)))
        elif place == "edge":
            anchors = [pts[c] for c in chosen] + [(cx, cy)]
            pick = max(free, key=lambda i: min(dist(pts[i], a) for a in anchors))
        else:
            raise SystemExit(f"gateway {spec['name']}: unknown place {place!r}")
        chosen.append(pick)
    return chosen


def build(desc: dict, source: str) -> tuple[dict, dict, dict]:
    rng = random.Random(desc.get("seed", 1))
    next_index = 0

    def new_id() -> str:
        nonlocal next_index
        node_id = f"{next_index:04x}"
        next_index += 1
        return node_id

    names: dict[str, str] = {}  # name -> id
    nodes_meta: dict[str, dict] = {}
    topo_nodes: list[dict] = []

    def register(name: str, meta: dict, x: float, y: float) -> str:
        if name in names:
            raise SystemExit(f"duplicate name {name!r}")
        node_id = new_id()
        names[name] = node_id
        nodes_meta[node_id] = {"name": name, **meta}
        topo_nodes.append({"id": node_id, "x": round(x, 2), "y": round(y, 2)})
        return node_id

    # Servers and remote users first, on a row above the communities.
    servers = []
    for i, s in enumerate(desc.get("servers", [])):
        loc = s.get("location", "near")
        servers.append(register(s["name"], {
            "role": "server", "community": None, "sphere": None,
            "location": loc, "host": loc == "far", "port": s.get("port", 9229),
            "lan": False, "stands_in_for": s.get("stands_in_for", ""),
        }, 6.0 * i, -8.0))
    remotes = []
    for i, r in enumerate(desc.get("remote_users", [])):
        loc = r.get("location", "near")
        remotes.append(register(r["name"], {
            "role": "remote", "community": None, "sphere": None,
            "location": loc, "host": loc == "far", "port": r.get("port", 9229),
            "lan": True, "situation": r.get("situation", ""),
        }, 6.0 * i + 3.0, -12.0))

    # Communities: each its own radio mesh, laid out left to right.
    radio_links: list[tuple[str, str]] = []
    community_records = []
    community_points: dict[str, dict[str, tuple[float, float]]] = {}
    degree: dict[str, int] = {}
    x_cursor = 0.0

    for c in desc["communities"]:
        n = c["members"]
        gw_specs = c.get("gateways", [])
        pts, local_links = radio_mesh(n, rng)
        gw_local = place_gateways(pts, gw_specs) if gw_specs else []
        gw_name_of = {idx: spec["name"] for idx, spec in zip(gw_local, gw_specs)}

        ids = []
        for idx, (px, py) in enumerate(pts):
            is_gateway = idx in gw_name_of
            name = gw_name_of[idx] if is_gateway else f"{c['name']}.{idx + 1:02d}"
            ids.append(register(name, {
                "role": "gateway" if is_gateway else "member",
                "community": c["name"], "sphere": None,
                "location": c.get("location", "near"), "host": False, "lan": True,
            }, x_cursor + px, py))
        community_points[c["name"]] = {
            ids[i]: (x_cursor + pts[i][0], pts[i][1]) for i in range(n)
        }

        for a, b in local_links:
            radio_links.append((ids[a], ids[b]))
        local_degree = [0] * n
        for a, b in local_links:
            local_degree[a] += 1
            local_degree[b] += 1
        for i, node_id in enumerate(ids):
            degree[node_id] = local_degree[i]

        community_records.append({
            "name": c["name"],
            "situation": c.get("situation", ""),
            "members": ids,
            "gateways": [ids[i] for i in gw_local],
            "radio_links": len(local_links),
            "max_degree": max(local_degree) if n else 0,
            "mean_degree": round(sum(local_degree) / n, 2) if n else 0,
            "diameter": diameter(n, local_links),
        })
        x_cursor += math.sqrt(n) + COMMUNITY_GAP

    # Radio bridges: two communities within range of each other, joined by
    # their closest pair of people who still have a free radio link.
    bridges = []
    for bridge in desc.get("radio_bridges", []):
        a_name, b_name = bridge["between"]
        if a_name not in community_points or b_name not in community_points:
            raise SystemExit(f"radio bridge names an unknown community: {bridge}")
        candidates = sorted(
            (dist(pa, pb), a, b)
            for a, pa in community_points[a_name].items()
            for b, pb in community_points[b_name].items()
            if degree[a] < BLE_MAX_LINKS and degree[b] < BLE_MAX_LINKS
        )
        if not candidates:
            raise SystemExit(f"no free radio link for the bridge {a_name}-{b_name}")
        if nodes_meta[candidates[0][1]]["location"] != nodes_meta[candidates[0][2]]["location"]:
            raise SystemExit(f"radio bridge {a_name}-{b_name}: one is on the laptop, "
                             "the other on the server")
        _, a, b = candidates[0]
        radio_links.append((a, b))
        degree[a] += 1
        degree[b] += 1
        bridges.append({"between": [a_name, b_name], "a": a, "b": b})

    # Local spheres: whatever the radio links join, whatever the labels say.
    member_ids = [m for c in community_records for m in c["members"]]
    index_of = {m: i for i, m in enumerate(member_ids)}
    sphere_parts = components(
        len(member_ids), [(index_of[a], index_of[b]) for a, b in radio_links]
    )
    community_of = {m: nodes_meta[m]["community"] for m in member_ids}
    spheres = []
    for part in sphere_parts:
        members = sorted(member_ids[i] for i in part)
        comm_names = sorted({community_of[m] for m in members},
                            key=[c["name"] for c in community_records].index)
        sphere_name = "+".join(comm_names)
        for m in members:
            nodes_meta[m]["sphere"] = sphere_name
        spheres.append({
            "name": sphere_name,
            "communities": comm_names,
            "members": members,
            "gateways": [g for c in community_records if c["name"] in comm_names
                         for g in c["gateways"]],
        })

    # Internet links: who dials whom. The dialer runs `connections nodes add`.
    def resolve(name: str, context: str) -> str:
        if name not in names:
            raise SystemExit(f"{context}: unknown peer {name!r}")
        return names[name]

    plan: list[tuple[str, str]] = []
    for c in desc["communities"]:
        for g in c.get("gateways", []):
            for peer in g.get("peers", []):
                plan.append((names[g["name"]], resolve(peer, g["name"])))
    for r in desc.get("remote_users", []):
        for peer in r.get("peers", []):
            plan.append((names[r["name"]], resolve(peer, r["name"])))
    for s in desc.get("server_links", []):
        plan.append((resolve(s["dialer"], "server link"), resolve(s["target"], "server link")))

    seen_pairs = set()
    internet_links = []
    for dialer, target in plan:
        pair = frozenset((dialer, target))
        if dialer == target or pair in seen_pairs:
            raise SystemExit(
                f"Internet link {nodes_meta[dialer]['name']}-{nodes_meta[target]['name']} "
                "is listed twice or loops"
            )
        seen_pairs.add(pair)
        # A link crosses the real Internet only when its two ends are in
        # different places: r1-S1 both sit on the server and never leave it.
        far = nodes_meta[dialer]["location"] != nodes_meta[target]["location"]
        dm, tm = nodes_meta[dialer], nodes_meta[target]
        if dm["host"] and tm["location"] == "far" and not tm["host"]:
            raise SystemExit(
                f"{dm['name']} would dial {tm['name']} in the server's lab; the lab end "
                "of a host link must be the dialer")
        if far and nodes_meta[dialer]["location"] == "far":
            # In a hybrid run the lab sits behind Windows' and the home
            # router's NAT, so nothing outside can open a connection into it.
            raise SystemExit(
                f"{nodes_meta[dialer]['name']} would dial "
                f"{nodes_meta[target]['name']} from the far side; the lab end "
                "of a far link must be the dialer"
            )
        internet_links.append({"dialer": dialer, "target": target, "far": far})

    # A server-lab node dialled from the laptop is reached through a port
    # forwarded on the server's public address, one per node.
    fwd = sorted({l["target"] for l in internet_links
                  if l["far"] and not nodes_meta[l["target"]]["host"]})
    for k, t in enumerate(fwd):
        nodes_meta[t]["fwd_port"] = FWD_PORT_BASE + k

    # Checks on the result, so a bad description fails here and not in the lab.
    for node_id, d in degree.items():
        if d > BLE_MAX_LINKS:
            raise AssertionError(f"{nodes_meta[node_id]['name']} has {d} radio links")
    for c in community_records:
        if not c["members"]:
            raise SystemExit(f"community {c['name']} has no members")
    for g in (m for m, meta in nodes_meta.items() if meta["role"] == "gateway"):
        if not any(g in (l["dialer"], l["target"]) for l in internet_links):
            raise SystemExit(f"gateway {nodes_meta[g]['name']} has no Internet link")

    # What the checks assert. Any node with an Internet link holds the gateway
    # role (§2.3), servers and remote users included.
    internet_nodes = sorted({x for l in internet_links for x in (l["dialer"], l["target"])})
    inet_index = {x: i for i, x in enumerate(internet_nodes)}
    internet_components = components(
        len(internet_nodes),
        [(inet_index[l["dialer"]], inet_index[l["target"]]) for l in internet_links],
    )
    reachable = set(servers) | set(remotes)
    local_only = set()
    for s in spheres:
        (reachable if s["gateways"] else local_only).update(s["members"])

    topo_links = [{"source": a, "target": b} for a, b in radio_links]
    inet_topo_links = [
        {"source": l["dialer"], "target": l["target"], "internet_only": "1"}
        for l in internet_links
    ]

    topology = {"nodes": topo_nodes, "links": topo_links + inet_topo_links}
    roles = {
        "name": desc["name"],
        "kind": "world",
        "seed": desc.get("seed", 1),
        "source": source,
        "description": desc.get("description", ""),
        "nodes": nodes_meta,
        "by_name": names,
        "servers": servers,
        "remote_users": remotes,
        "communities": community_records,
        "spheres": spheres,
        "radio_bridges": bridges,
        "internet_links": internet_links,
        "expected": {
            "gateway_role": internet_nodes,
            "internet_components": len(internet_components),
            "globally_reachable": sorted(reachable),
            "local_only": sorted(local_only),
        },
        "totals": {
            "nodes": len(topo_nodes),
            "lab_nodes": sum(1 for m in nodes_meta.values() if m["location"] != "far"),
            "far_nodes": sum(1 for m in nodes_meta.values() if m["location"] == "far"),
            "radio_links": len(topo_links),
            "internet_links": len(internet_links),
            "far_internet_links": sum(1 for l in internet_links if l["far"]),
            "server_lab_nodes": sum(1 for m in nodes_meta.values()
                                    if m["location"] == "far" and not m["host"]),
        },
    }
    return topology, roles


def summary(roles: dict) -> None:
    meta = roles["nodes"]
    name = lambda i: meta[i]["name"]
    t = roles["totals"]
    print(f"{roles['name']}: {t['nodes']} nodes "
          f"({t['lab_nodes']} in the lab, {t['far_nodes']} far"
          f"{', of them ' + str(t['server_lab_nodes']) + ' in the server lab' if t.get('server_lab_nodes') else ''}), "
          f"{t['radio_links']} radio links, {t['internet_links']} Internet links "
          f"({t['far_internet_links']} far)")
    print(f"  servers        {', '.join(name(s) for s in roles['servers'])}")
    print(f"  remote users   {', '.join(name(r) for r in roles['remote_users'])}")
    print("  communities    name  people  gateways        links  max/mean degree  diameter")
    for c in roles["communities"]:
        gws = ", ".join(name(g) for g in c["gateways"]) or "-"
        print(f"                 {c['name']:<4}  {len(c['members']):>6}  {gws:<14}  "
              f"{c['radio_links']:>5}  {c['max_degree']}/{c['mean_degree']:<13}  {c['diameter']:>8}")
    for b in roles["radio_bridges"]:
        print(f"  radio bridge   {b['between'][0]}-{b['between'][1]}: {name(b['a'])} - {name(b['b'])}")
    print("  local spheres  " + "; ".join(
        f"{s['name']} ({len(s['members'])} people, gateways: "
        f"{', '.join(name(g) for g in s['gateways']) or 'none'})"
        for s in roles["spheres"]))
    print("  Internet links " + ", ".join(
        f"{name(l['dialer'])}->{name(l['target'])}{' (far)' if l['far'] else ''}"
        for l in roles["internet_links"]))
    fwd = [(m["name"], m["fwd_port"]) for m in meta.values() if "fwd_port" in m]
    if fwd:
        print("  forwarded      " + ", ".join(f"{n} :{p}" for n, p in sorted(fwd, key=lambda x: x[1])))
    e = roles["expected"]
    print(f"  expected       {len(e['gateway_role'])} nodes in the gateway role, "
          f"{e['internet_components']} Internet component(s), "
          f"{len(e['globally_reachable'])} users reachable from anywhere, "
          f"{len(e['local_only'])} local only")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("description", help="world description, e.g. worlds/world-1.json")
    here = os.path.dirname(os.path.abspath(__file__))
    ap.add_argument("--out-dir", default=os.path.join(here, "topologies"))
    args = ap.parse_args()

    with open(args.description) as f:
        desc = json.load(f)
    source = os.path.relpath(os.path.abspath(args.description), here)
    topology, roles = build(desc, source)

    # The hybrid run's lab part: the far nodes run on test.qaul.net, and the
    # lab ends of far links reach them through egress.sh, not a lab link.
    far = {nid for nid, m in roles["nodes"].items() if m["location"] == "far"}
    hybrid = {
        "nodes": [n for n in topology["nodes"] if n["id"] not in far],
        "links": [l for l in topology["links"]
                  if l["source"] not in far and l["target"] not in far],
    }

    # The server lab's part: far community nodes, and the links among them.
    # Their links to S1 and r1 go through hostlink.sh, not the lab.
    server_lab = {nid for nid, m in roles["nodes"].items()
                  if m["location"] == "far" and not m["host"]}
    server = {
        "nodes": [n for n in topology["nodes"] if n["id"] in server_lab],
        "links": [l for l in topology["links"]
                  if l["source"] in server_lab and l["target"] in server_lab],
    }

    os.makedirs(args.out_dir, exist_ok=True)
    base = os.path.join(args.out_dir, desc["name"])
    with open(f"{base}.json", "w") as f:
        json.dump(topology, f)
    with open(f"{base}-hybrid.json", "w") as f:
        json.dump(hybrid, f)
    written = [f"{base}.json", f"{base}-hybrid.json"]
    if server_lab:
        with open(f"{base}-server.json", "w") as f:
            json.dump(server, f)
        written.append(f"{base}-server.json")
    with open(f"{base}.roles.json", "w") as f:
        json.dump(roles, f, indent=1)
    written.append(f"{base}.roles.json")

    summary(roles)
    print("wrote " + ", ".join(written))


if __name__ == "__main__":
    main()
