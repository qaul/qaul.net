#!/usr/bin/env python3
"""Write a ring world's description: villages whose gateways form a ring.

The backbone is ring-1's. Each village's first gateway links to the next
village's, and the ring crosses between the laptop's half (A-E) and the
server's (F-J) at S1 and r1, which run on the server's host.

With --gateways auto, each village gets about one gateway per 10 people,
2 to 4. Every extra gateway then gets --extra links (default 1-2) to
gateways of other villages, drawn at random with --seed, at most --max-degree
Internet links per gateway. A link between the laptop and the server's lab is
dialled from the laptop, the end behind NAT; gen_world.py gives the server
end a forwarded port.

The radio meshes come from the description's "seed" (2, as in ring-1) and
gateway placement is deterministic, so every ring world has ring-1's villages
and ring-1's first gateways. Only the extra gateways and links differ.

Usage:
    ./make_ring.py --name ring-1 > worlds/ring-1.json
    ./make_ring.py --name ring-2 --gateways auto --seed 7 > worlds/ring-2.json
"""

import argparse
import json
import random

LAPTOP = [("A", 40), ("B", 34), ("C", 30), ("D", 24), ("E", 20)]
SERVER = [("F", 40), ("G", 34), ("H", 30), ("I", 24), ("J", 18)]
MESH_SEED = 2


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--name", required=True)
    ap.add_argument("--gateways", default="1", help="'auto' (2-4 by size) or a number per village")
    ap.add_argument("--extra", default="1-2", help="random links per extra gateway, e.g. 1-2")
    ap.add_argument("--max-degree", type=int, default=4)
    ap.add_argument("--seed", type=int, default=7, help="for the random links only")
    args = ap.parse_args()
    rng = random.Random(args.seed)
    lo, hi = (int(x) for x in args.extra.split("-"))

    villages = [(n, k, "laptop") for n, k in LAPTOP] + [(n, k, "server") for n, k in SERVER]
    where = {}
    gws: dict[str, list[str]] = {}
    for v, size, side in villages:
        count = (min(4, max(2, round(size / 10))) if args.gateways == "auto"
                 else int(args.gateways))
        gws[v] = [f"{v}{i + 1}" for i in range(count)]
        for g in gws[v]:
            where[g] = side
    village_of = {g: v for v, gs in gws.items() for g in gs}

    peers: dict[str, list[str]] = {g: [] for g in where}
    degree = {g: 0 for g in where}
    linked: set[frozenset] = set()

    def link(dialer: str, target: str) -> None:
        peers[dialer].append(target)
        linked.add(frozenset((dialer, target)))
        for x in (dialer, target):
            if x in degree:
                degree[x] += 1

    # The backbone, exactly as ring-1 dials it.
    firsts = [gws[v][0] for v, _, _ in villages]
    link("A1", "r1")
    for a, b in zip(firsts[:5], firsts[1:5]):
        link(b, a)                      # B1->A1 ... E1->D1
    link("E1", "S1")
    link("F1", "S1")
    for a, b in zip(firsts[5:], firsts[6:]):
        link(b, a)                      # G1->F1 ... J1->I1
    link("J1", "r1")

    # Random shortcuts from every extra gateway.
    shortcuts = []
    for v, _, _ in villages:
        for g in gws[v][1:]:
            want = rng.randint(lo, hi)
            pool = [t for t in where if village_of[t] != v and frozenset((g, t)) not in linked
                    and degree[t] < args.max_degree]
            rng.shuffle(pool)
            for t in pool[:want]:
                if degree[g] >= args.max_degree:
                    break
                # The laptop end dials: nothing outside can open a connection into the lab.
                d, tt = (t, g) if where[g] == "server" and where[t] == "laptop" else (g, t)
                link(d, tt)
                shortcuts.append((d, tt))

    desc = {
        "name": args.name,
        "seed": MESH_SEED,
        "description": (
            "Ten villages whose gateways form one ring over the Internet, half on the "
            "laptop and half on test.qaul.net; the ring crosses the real Internet at S1 "
            "and r1." + (f" Extra gateways with {len(shortcuts)} random shortcuts "
                         f"(make_ring.py --gateways {args.gateways} --extra {args.extra} "
                         f"--seed {args.seed})." if shortcuts else "")
            + " See tests/integration/REPORT-ring-world.md."),
        "servers": [{"name": "S1", "location": "far", "port": 9229,
                     "stands_in_for": "a server on the ring, where it crosses to the laptop"}],
        "server_links": [],
        "communities": [],
        "radio_bridges": [],
        "remote_users": [{"name": "r1", "location": "far", "port": 9230, "peers": [],
                          "situation": "a user on the server, on the ring where it crosses "
                                       "back to the laptop"}],
    }
    for v, size, side in villages:
        c = {"name": v, "members": size}
        if side == "server":
            c["location"] = "far"
        c["situation"] = side
        c["gateways"] = [{"name": g, "place": "edge", "peers": peers[g]} for g in gws[v]]
        desc["communities"].append(c)
    print(json.dumps(desc, indent=1))


if __name__ == "__main__":
    main()
