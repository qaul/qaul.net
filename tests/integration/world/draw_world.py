#!/usr/bin/env python3
"""Draw a world as an SVG: every node and link, from the generator's own files.

Communities are drawn at the positions gen_world.py gave their people, so the
radio meshes are the real ones. Servers and remote users sit in an Internet
tier above them. In the hybrid run, S1 and r1 run on test.qaul.net and
everything else in the lab, so links between the two cross the real Internet.

A world whose Internet links form one loop (ring-1) is drawn as a ring
instead: the stations on a circle, the server's half on the right and the
laptop's on the left, and each village's own radio mesh opening outward from
its gateway. --layout picks one; auto draws a ring when the links are a loop.

Usage:
    ./draw_world.py topologies/world-1.roles.json [--out diagrams/world-1.svg]
    ./draw_world.py topologies/ring-1.roles.json  [--layout auto|row|ring]
"""

import argparse
import json
import math
import os

SCALE = 26          # px per layout unit
PAD = 30
TOP_TIER = 210      # px reserved above the communities for the Internet tier

COL = {
    "radio": "#9aa3ad", "bridge": "#d97706", "near": "#2563eb", "far": "#dc2626",
    "member": "#6b7280", "gateway": "#111827", "server": "#7c3aed", "remote": "#0d9488",
    "island": "#ffffff", "text": "#111827", "muted": "#4b5563",
    "lab_box": "#f3f4f6", "far_box": "#fee2e2",
}


def ring_order(roles: dict) -> list[str] | None:
    """The Internet stations in ring order, starting at the first community's
    gateway and heading for the next community's, or None if the Internet
    links are not one loop through every node on them."""
    adj: dict[str, list[str]] = {}
    for l in roles["internet_links"]:
        adj.setdefault(l["dialer"], []).append(l["target"])
        adj.setdefault(l["target"], []).append(l["dialer"])
    if len(adj) < 3 or any(len(v) != 2 for v in adj.values()):
        return None
    gws = [g for c in roles["communities"] for g in c["gateways"] if g in adj]
    start = gws[0] if gws else sorted(adj)[0]
    nxt = gws[1] if len(gws) > 1 and gws[1] in adj[start] else sorted(adj[start])[0]
    order = [start]
    prev, cur = start, nxt
    while cur != start:
        order.append(cur)
        prev, cur = cur, next(x for x in adj[cur] if x != prev)
    return order if len(order) == len(adj) else None


def legend(add, width: int, y: int, lines, shapes) -> None:
    colx = [PAD, PAD + (width - 2 * PAD) // 2]
    for i, (color, dash, label) in enumerate(lines):
        x, yy = colx[i % 2], y + 22 * (i // 2)
        d = f' stroke-dasharray="{dash}"' if dash else ""
        add(f'<line x1="{x}" y1="{yy}" x2="{x + 28}" y2="{yy}" stroke="{color}" '
            f'stroke-width="2.5"{d}/>')
        add(f'<text x="{x + 36}" y="{yy + 4}" font-size="12" fill="{COL["text"]}">{label}</text>')
    ly2 = y + 22 * ((len(lines) + 1) // 2) + 6
    lx = PAD
    for shape, label in shapes:
        if shape in ("member", "island"):
            add(f'<circle cx="{lx + 8}" cy="{ly2}" r="5" fill="{COL[shape]}" '
                f'stroke="{COL["member"]}" stroke-width="1.2"/>')
        elif shape == "gateway":
            add(f'<circle cx="{lx + 8}" cy="{ly2}" r="8" fill="{COL["gateway"]}"/>')
        elif shape == "server":
            add(f'<rect x="{lx}" y="{ly2 - 8}" width="16" height="16" rx="3" fill="{COL["server"]}"/>')
        else:
            add(f'<polygon points="{lx + 8},{ly2 - 9} {lx + 17},{ly2} {lx + 8},{ly2 + 9} '
                f'{lx - 1},{ly2}" fill="{COL["remote"]}"/>')
        add(f'<text x="{lx + 24}" y="{ly2 + 4}" font-size="12" fill="{COL["text"]}">{label}</text>')
        lx += 24 + int(6.6 * len(label)) + 34


def station_mark(add, x: float, y: float, m: dict, big: bool = False) -> None:
    role = m["role"]
    if role == "server":
        h = 20 if big else 16
        add(f'<rect x="{x - h:.1f}" y="{y - h:.1f}" width="{2 * h}" height="{2 * h}" rx="4" '
            f'fill="{COL["server"]}"/>')
        add(f'<text x="{x:.1f}" y="{y + 5:.1f}" font-size="13" fill="#fff" '
            f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')
    elif role == "remote":
        h = 19 if big else 15
        add(f'<polygon points="{x:.1f},{y - h:.1f} {x + h:.1f},{y:.1f} {x:.1f},{y + h:.1f} '
            f'{x - h:.1f},{y:.1f}" fill="{COL["remote"]}"/>')
        add(f'<text x="{x:.1f}" y="{y + 4:.1f}" font-size="11" fill="#fff" '
            f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')
    else:
        r = 14 if big else 11
        add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="{COL["gateway"]}"/>')
        add(f'<text x="{x:.1f}" y="{y + 4:.1f}" font-size="{11 if big else 10}" fill="#fff" '
            f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')


def draw_ring(roles: dict, topo: dict, order: list[str], out: str) -> None:
    nodes = roles["nodes"]
    pos = {n["id"]: (n["x"], n["y"]) for n in topo["nodes"]}
    scale, ring_r, margin = 17, 300, 205
    size = 2 * (ring_r + margin) + 2 * PAD
    cx = cy = PAD + ring_r + margin
    width, height = size, size + 120

    # The server's stations on the right half, top to bottom, the laptop's on
    # the left, bottom to top: the ring then crosses between them at the top
    # and the bottom. Without two runs, evenly round.
    far = [nodes[s]["location"] == "far" for s in order]
    runs = sum(1 for i in range(len(order)) if far[i] != far[i - 1])
    angle: dict[str, float] = {}
    if runs == 2:
        i0 = next(i for i in range(len(order)) if far[i] and not far[i - 1])
        rot = order[i0:] + order[:i0]
        right = [s for s in rot if nodes[s]["location"] == "far"]
        left = [s for s in rot if nodes[s]["location"] != "far"]
        for k, s in enumerate(right):
            angle[s] = (k + 0.5) * 180 / len(right)
        for k, s in enumerate(left):
            angle[s] = 180 + (k + 0.5) * 180 / len(left)
    else:
        for k, s in enumerate(order):
            angle[s] = (k + 0.5) * 360 / len(order)

    def on_circle(a: float, r: float) -> tuple[float, float]:
        t = math.radians(a)
        return cx + r * math.sin(t), cy - r * math.cos(t)

    station = {s: on_circle(angle[s], ring_r) for s in order}

    # Each village's mesh, turned so it opens outward from its gateway.
    xy: dict[str, tuple[float, float]] = dict(station)
    village = {}
    for c in roles["communities"]:
        g = next((x for x in c["gateways"] if x in station), None)
        if g is None:
            continue
        gx, gy = pos[g]
        vec = {n: (pos[n][0] - gx, pos[n][1] - gy) for n in c["members"]}
        others = [v for n, v in vec.items() if n != g]
        mx = sum(v[0] for v in others) / len(others)
        my = sum(v[1] for v in others) / len(others)
        ux, uy = math.sin(math.radians(angle[g])), -math.cos(math.radians(angle[g]))
        turn = math.atan2(uy, ux) - math.atan2(my, mx)
        ct, st = math.cos(turn), math.sin(turn)
        sx, sy = station[g]
        for n, (vx, vy) in vec.items():
            if n != g:
                xy[n] = (sx + scale * (vx * ct - vy * st), sy + scale * (vx * st + vy * ct))
        pts = [xy[n] for n in c["members"]]
        ccx = sum(p[0] for p in pts) / len(pts)
        ccy = sum(p[1] for p in pts) / len(pts)
        rad = max(math.hypot(p[0] - ccx, p[1] - ccy) for p in pts) + 12
        village[c["name"]] = (ccx, ccy, rad, g, len(c["members"]), (ux, uy))

    o = []
    add = o.append
    add(f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}" font-family="Helvetica, Arial, sans-serif">')
    add(f'<rect width="{width}" height="{height}" fill="#ffffff"/>')

    if runs == 2:
        add(f'<rect x="{cx:.0f}" y="{PAD}" width="{width - PAD - cx:.0f}" height="{size - 2 * PAD}" '
            f'rx="14" fill="{COL["far_box"]}" stroke="{COL["far"]}" stroke-dasharray="4 3"/>')
        add(f'<text x="{width - PAD - 12:.0f}" y="{PAD + 22}" font-size="14" text-anchor="end" '
            f'fill="{COL["far"]}" font-weight="bold">test.qaul.net</text>')
        add(f'<text x="{width - PAD - 12:.0f}" y="{PAD + 40}" font-size="12" text-anchor="end" '
            f'fill="{COL["far"]}">a lab on the server, S1 and r1 on its host</text>')
        add(f'<text x="{PAD + 12}" y="{PAD + 22}" font-size="14" fill="{COL["near"]}" '
            f'font-weight="bold">laptop</text>')
        add(f'<text x="{PAD + 12}" y="{PAD + 40}" font-size="12" fill="{COL["muted"]}">'
            f'a lab on the laptop (WSL)</text>')

    for name, (vx, vy, rad, g, n, (ux, uy)) in village.items():
        add(f'<circle cx="{vx:.1f}" cy="{vy:.1f}" r="{rad:.1f}" fill="{COL["lab_box"]}" '
            f'stroke="#d1d5db"/>')

    for l in topo["links"]:
        if l.get("internet_only") or l["source"] not in xy or l["target"] not in xy:
            continue
        (x0, y0), (x1, y1) = xy[l["source"]], xy[l["target"]]
        add(f'<line x1="{x0:.1f}" y1="{y0:.1f}" x2="{x1:.1f}" y2="{y1:.1f}" '
            f'stroke="{COL["radio"]}" stroke-width="1.1"/>')
    for l in roles["internet_links"]:
        (x0, y0), (x1, y1) = station[l["dialer"]], station[l["target"]]
        color = COL["far"] if l["far"] else COL["near"]
        dash = ' stroke-dasharray="9 5"' if l["far"] else ""
        add(f'<line x1="{x0:.1f}" y1="{y0:.1f}" x2="{x1:.1f}" y2="{y1:.1f}" '
            f'stroke="{color}" stroke-width="{3.5 if l["far"] else 3}"{dash}/>')
        if l["far"]:
            mx, my = (x0 + x1) / 2, (y0 + y1) / 2
            dy = -10 if my < cy else 20
            add(f'<text x="{mx:.1f}" y="{my + dy:.1f}" font-size="12" text-anchor="middle" '
                f'fill="{COL["far"]}" font-weight="bold">real Internet</text>')

    for nid, (x, y) in xy.items():
        if nid in station:
            continue
        add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3.8" fill="{COL["member"]}" '
            f'stroke="{COL["member"]}" stroke-width="1"/>')
    for s in order:
        station_mark(add, *station[s], nodes[s], big=True)

    for name, (vx, vy, rad, g, n, (ux, uy)) in village.items():
        lx, ly = vx + ux * (rad + 16), vy + uy * (rad + 16)
        anchor = "middle" if abs(ux) < 0.35 else ("start" if ux > 0 else "end")
        add(f'<text x="{lx:.1f}" y="{ly + 4:.1f}" font-size="14" text-anchor="{anchor}" '
            f'fill="{COL["text"]}" font-weight="bold">{name}: {n}</text>')

    total = len(nodes)
    vill = len(village)
    add(f'<text x="{cx:.0f}" y="{cy - 14:.0f}" font-size="22" text-anchor="middle" '
        f'fill="{COL["text"]}" font-weight="bold">{roles["name"]}</text>')
    add(f'<text x="{cx:.0f}" y="{cy + 10:.0f}" font-size="13" text-anchor="middle" '
        f'fill="{COL["muted"]}">{total} devices, {vill} villages</text>')
    add(f'<text x="{cx:.0f}" y="{cy + 28:.0f}" font-size="13" text-anchor="middle" '
        f'fill="{COL["muted"]}">one ring of gateways, no central server</text>')

    legend(add, width, size + 18, [
        (COL["radio"], "", "radio link inside a village (at most 4 per person)"),
        (COL["near"], "", "Internet link inside one machine"),
        (COL["far"], "9 4", "Internet link across the real Internet, laptop to server"),
    ], [("member", "person"), ("gateway", "village gateway"), ("server", "server S1"),
        ("remote", "remote user r1")])
    add("</svg>")
    with open(out, "w") as f:
        f.write("\n".join(o))
    print(f"wrote {out}")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("roles")
    ap.add_argument("--topology", help="default: the world .json beside the roles file")
    ap.add_argument("--out", help="default: diagrams/<world>.svg beside this script")
    ap.add_argument("--layout", choices=("auto", "row", "ring"), default="auto",
                    help="auto: a ring when the Internet links form one loop")
    args = ap.parse_args()

    with open(args.roles) as f:
        roles = json.load(f)
    with open(args.topology or args.roles.replace(".roles.json", ".json")) as f:
        topo = json.load(f)
    here = os.path.dirname(os.path.abspath(__file__))
    out = args.out or os.path.join(here, "diagrams", f"{roles['name']}.svg")
    os.makedirs(os.path.dirname(out), exist_ok=True)

    order = ring_order(roles) if args.layout != "row" else None
    if args.layout == "ring" and order is None:
        raise SystemExit("the Internet links do not form one loop; use --layout row")
    if order:
        draw_ring(roles, topo, order, out)
        return

    nodes = roles["nodes"]
    pos = {n["id"]: (n["x"], n["y"]) for n in topo["nodes"]}
    comm_ids = [nid for nid, m in nodes.items() if m["community"]]
    xmin = min(pos[n][0] for n in comm_ids)
    xmax = max(pos[n][0] for n in comm_ids)
    ymax = max(pos[n][1] for n in comm_ids)
    width = int((xmax - xmin) * SCALE + 2 * PAD + 40)
    comm_top = PAD + TOP_TIER

    def P(nid):
        x, y = pos[nid]
        return PAD + 20 + (x - xmin) * SCALE, comm_top + y * SCALE

    # Internet tier: servers in a row, remote users beside the server they use.
    servers = roles["servers"]
    remotes = roles["remote_users"]
    by_name = roles["by_name"]
    tier = {}
    s1, s2 = by_name.get("S1"), by_name.get("S2")
    span = width - 2 * PAD
    if s1:
        tier[s1] = (PAD + span * 0.30, PAD + 70)
    if s2:
        tier[s2] = (PAD + span * 0.62, PAD + 70)
    for i, sid in enumerate(s for s in servers if s not in tier):
        tier[sid] = (PAD + span * (0.45 + 0.1 * i), PAD + 70)
    for rid in remotes:
        peers = [l["target"] for l in roles["internet_links"] if l["dialer"] == rid]
        if len(peers) == 1 and peers[0] in tier:
            sx, sy = tier[peers[0]]
            far = nodes[rid]["location"] != nodes[peers[0]]["location"]
            dx = -95 if nodes[rid]["location"] == "far" else 95
            tier[rid] = (sx + dx, sy + 55 if not far else sy + 55)
        else:
            xs = [tier[p][0] for p in peers if p in tier]
            tier[rid] = (sum(xs) / len(xs) if xs else PAD + span * 0.5, PAD + 135)
    # nudge a remote user that landed on another one
    for a in remotes:
        for b in remotes:
            if a < b and abs(tier[a][0] - tier[b][0]) < 40 and abs(tier[a][1] - tier[b][1]) < 20:
                tier[b] = (tier[b][0] + 70, tier[b][1])

    def XY(nid):
        return tier[nid] if nid in tier else P(nid)

    height = int(comm_top + ymax * SCALE + 150)
    o = []
    add = o.append
    add(f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}" font-family="Helvetica, Arial, sans-serif">')
    add(f'<rect width="{width}" height="{height}" fill="#ffffff"/>')

    # where things run (hybrid run)
    far_nodes = [n for n, m in nodes.items() if m["location"] == "far"]
    if far_nodes:
        fx = [XY(n)[0] for n in far_nodes]
        fy = [XY(n)[1] for n in far_nodes]
        x0, x1 = min(fx) - 45, max(fx) + 45
        y0, y1 = min(fy) - 42, max(fy) + 30
        add(f'<rect x="{x0:.0f}" y="{y0:.0f}" width="{x1 - x0:.0f}" height="{y1 - y0:.0f}" '
            f'rx="10" fill="{COL["far_box"]}" stroke="{COL["far"]}" stroke-dasharray="4 3"/>')
        add(f'<text x="{x0 + 8:.0f}" y="{y0 + 15:.0f}" font-size="12" fill="{COL["far"]}" '
            f'font-weight="bold">test.qaul.net (hybrid run)</text>')
    lab_y0 = PAD + 20
    add(f'<text x="{width - PAD:.0f}" y="{lab_y0:.0f}" font-size="12" text-anchor="end" '
        f'fill="{COL["muted"]}">everything outside the red box runs in the lab on the laptop</text>')

    # community backgrounds and labels
    for c in roles["communities"]:
        ms = c["members"]
        xs = [P(n)[0] for n in ms]
        ys = [P(n)[1] for n in ms]
        x0, x1, y0, y1 = min(xs) - 14, max(xs) + 14, min(ys) - 14, max(ys) + 14
        add(f'<rect x="{x0:.0f}" y="{y0:.0f}" width="{x1 - x0:.0f}" height="{y1 - y0:.0f}" '
            f'rx="12" fill="{COL["lab_box"]}" stroke="#d1d5db"/>')
        gws = ", ".join(nodes[g]["name"] for g in c["gateways"]) or "no gateway"
        add(f'<text x="{(x0 + x1) / 2:.0f}" y="{y1 + 18:.0f}" font-size="13" text-anchor="middle" '
            f'fill="{COL["text"]}" font-weight="bold">{c["name"]}: {len(ms)}</text>')
        add(f'<text x="{(x0 + x1) / 2:.0f}" y="{y1 + 33:.0f}" font-size="11" text-anchor="middle" '
            f'fill="{COL["muted"]}">{gws}</text>')

    # links: radio first, Internet on top
    bridges = {frozenset((b["a"], b["b"])) for b in roles.get("radio_bridges", [])}
    for l in topo["links"]:
        if l.get("internet_only"):
            continue
        (x0, y0), (x1, y1) = P(l["source"]), P(l["target"])
        br = frozenset((l["source"], l["target"])) in bridges
        add(f'<line x1="{x0:.1f}" y1="{y0:.1f}" x2="{x1:.1f}" y2="{y1:.1f}" '
            f'stroke="{COL["bridge"] if br else COL["radio"]}" '
            f'stroke-width="{3 if br else 1.2}"/>')
    for l in roles["internet_links"]:
        (x0, y0), (x1, y1) = XY(l["dialer"]), XY(l["target"])
        color = COL["far"] if l["far"] else COL["near"]
        dash = ' stroke-dasharray="7 4"' if l["far"] else ""
        add(f'<line x1="{x0:.1f}" y1="{y0:.1f}" x2="{x1:.1f}" y2="{y1:.1f}" '
            f'stroke="{color}" stroke-width="2"{dash} opacity="0.85"/>')

    # nodes
    for nid, m in nodes.items():
        x, y = XY(nid)
        role = m["role"]
        if role == "server":
            add(f'<rect x="{x - 16:.1f}" y="{y - 16:.1f}" width="32" height="32" rx="4" '
                f'fill="{COL["server"]}"/>')
            add(f'<text x="{x:.1f}" y="{y + 5:.1f}" font-size="13" fill="#fff" '
                f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')
        elif role == "remote":
            add(f'<polygon points="{x:.1f},{y - 15:.1f} {x + 15:.1f},{y:.1f} {x:.1f},{y + 15:.1f} '
                f'{x - 15:.1f},{y:.1f}" fill="{COL["remote"]}"/>')
            add(f'<text x="{x:.1f}" y="{y + 4:.1f}" font-size="11" fill="#fff" '
                f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')
        elif role == "gateway":
            add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="11" fill="{COL["gateway"]}"/>')
            add(f'<text x="{x:.1f}" y="{y + 4:.1f}" font-size="10" fill="#fff" '
                f'text-anchor="middle" font-weight="bold">{m["name"]}</text>')
        else:
            island = m["sphere"] and not next(
                (s for s in roles["spheres"] if s["name"] == m["sphere"]), {}).get("gateways")
            fill = COL["island"] if island else COL["member"]
            add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4.5" fill="{fill}" '
                f'stroke="{COL["member"]}" stroke-width="1.2"/>')

    # legend: two fixed columns, three rows
    ly = height - 78
    colx = [PAD, PAD + (width - 2 * PAD) // 2]
    lines = [
        (COL["radio"], "", "radio link (BLE-like, at most 4 per person)"),
        (COL["bridge"], "", "radio link joining twin villages X and Y"),
        (COL["near"], "", "Internet link inside the lab"),
        (COL["far"], "7 4", "Internet link that crosses the real Internet (hybrid run)"),
    ]
    for i, (color, dash, label) in enumerate(lines):
        x, y = colx[i % 2], ly + 22 * (i // 2)
        d = f' stroke-dasharray="{dash}"' if dash else ""
        add(f'<line x1="{x}" y1="{y}" x2="{x + 28}" y2="{y}" stroke="{color}" '
            f'stroke-width="2.5"{d}/>')
        add(f'<text x="{x + 36}" y="{y + 4}" font-size="12" fill="{COL["text"]}">{label}</text>')
    ly2 = ly + 50
    lx = PAD
    for shape, label in [("member", "member"), ("island", "member with no gateway in reach"),
                         ("gateway", "gateway"), ("server", "server"), ("remote", "remote user")]:
        if shape in ("member", "island"):
            add(f'<circle cx="{lx + 8}" cy="{ly2}" r="5" fill="{COL[shape]}" '
                f'stroke="{COL["member"]}" stroke-width="1.2"/>')
        elif shape == "gateway":
            add(f'<circle cx="{lx + 8}" cy="{ly2}" r="8" fill="{COL["gateway"]}"/>')
        elif shape == "server":
            add(f'<rect x="{lx}" y="{ly2 - 8}" width="16" height="16" rx="3" fill="{COL["server"]}"/>')
        else:
            add(f'<polygon points="{lx + 8},{ly2 - 9} {lx + 17},{ly2} {lx + 8},{ly2 + 9} '
                f'{lx - 1},{ly2}" fill="{COL["remote"]}"/>')
        add(f'<text x="{lx + 24}" y="{ly2 + 4}" font-size="12" fill="{COL["text"]}">{label}</text>')
        lx += 24 + int(6.6 * len(label)) + 34

    add("</svg>")
    with open(out, "w") as f:
        f.write("\n".join(o))
    print(f"wrote {out}")


if __name__ == "__main__":
    main()
