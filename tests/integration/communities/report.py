#!/usr/bin/env python3
"""Tabulate compare.sh runs: v1 against v2, one cell per arm, one value per run.

Reads only the run archives (runs/<tag>-<UTC start>/), so a table can be
rebuilt at any time. The timeline is reduced again from the archived
recording rather than read from the archived times-*.json, so every run is
judged by the current reducer. Wire bytes and packets come straight from
the three counter samples (before bring-up, after the settle, after the
window), framing and all.

Usage:
    ./report.py ../runs/communities-3x10-v1-20261103T095029Z ../runs/communities-3x10-v2-...
    ./report.py --json out.json RUN...
"""

import argparse
import glob
import json
import os
import sys
from datetime import datetime

from timeline import Recording, load_events, load_roles, reduce


def read_kv(path: str) -> dict:
    out = {}
    with open(path) as f:
        for line in f:
            key, _, value = line.rstrip("\n").partition("=")
            if key:
                out[key] = value
    return out


def one(pattern: str) -> str | None:
    matches = [p for p in glob.glob(pattern) if not p.endswith("-events.jsonl")]
    return matches[0] if matches else None


def wire_window(pre_path: str, post_path: str, roles: dict) -> dict:
    """Bytes and packets sent per second over the window, all nodes and by role."""
    pre, post = (json.load(open(p)) for p in (pre_path, post_path))
    window = (datetime.fromisoformat(post["timestamp"])
              - datetime.fromisoformat(pre["timestamp"])).total_seconds()
    gateways = {c["gateway"] for c in roles["communities"]}

    def role(nid):
        return "hub" if nid == roles["hub"] else "gateway" if nid in gateways else "member"

    sent = {"hub": [], "gateway": [], "member": []}
    tx_bytes = tx_packets = 0
    for nid, after in post["counters"].items():
        before = pre["counters"].get(nid)
        if before is None:
            continue
        b = after["tx_bytes"] - before["tx_bytes"]
        tx_bytes += b
        tx_packets += after["tx_packets"] - before["tx_packets"]
        sent[role(nid)].append(b / window)
    return {
        "window_s": round(window, 1),
        "tx_bytes_per_s": tx_bytes / window,
        "tx_packets_per_s": tx_packets / window,
        "bytes_per_packet": tx_bytes / tx_packets if tx_packets else None,
        "per_node_tx_bytes_per_s": {r: sum(v) / len(v) for r, v in sent.items() if v},
    }


def load_run(path: str) -> dict:
    run = read_kv(os.path.join(path, "run.txt"))
    arm, topo = run["ARM"], run["TOPO"]
    res = os.path.join(path, "results")
    stem = os.path.join(res, f"timeline-{topo}-{arm}")
    roles = load_roles(os.path.join(path, "roles.json"))

    times = reduce(roles, Recording(stem + ".jsonl"), load_events(stem + "-events.jsonl"), arm)
    crossing = json.load(open(os.path.join(res, f"crossing-{topo}-{arm}.json")))
    steady = json.load(open(os.path.join(res, f"wire-steady-{topo}-{arm}.json")))
    params = read_kv(os.path.join(res, f"compare-{topo}-{arm}.txt"))
    wire_steady = wire_window(stem + "-wire-1.json", stem + "-wire-2.json", roles)
    return {
        "path": path,
        "name": os.path.basename(path.rstrip("/")),
        "arm": arm,
        "topo": topo,
        "git": run.get("GIT_REV"),
        "params": {k: v for k, v in params.items() if k not in ("ARM", "REPEAT")},
        "times": times,
        "crossing": crossing,
        "routing_by_type": steady.get("by_type", []),
        "wire_bringup": wire_window(stem + "-wire-0.json", stem + "-wire-1.json", roles),
        "wire_steady": wire_steady,
        "invalid": overran(times, params, wire_steady),
    }


# A phase that ran well past its configured length (the host slept, say) froze
# every node with it, so its numbers describe the pause, not the protocol.
SLACK_S = 30


def overran(times: dict, params: dict, wire_steady: dict) -> dict:
    """{section: why} for every phase that lasted longer than compare.sh asked."""
    out = {}
    heal = float(params.get("HEAL", "nan"))
    for label, cut_key in (("gateway_reconnect", "CUT_GATEWAY"), ("member_leave", "CUT_MEMBER")):
        x = times.get(label)
        if x is None or cut_key not in params:
            continue
        cut = float(params[cut_key])
        if x["cut_lasted_s"] > cut + SLACK_S:
            out[label] = f"cut lasted {x['cut_lasted_s']:.0f}s, not {cut:.0f}s"
        elif x["heal_window_s"] > heal + SLACK_S:
            out[label] = f"heal lasted {x['heal_window_s']:.0f}s, not {heal:.0f}s"
    if "WINDOW" in params and wire_steady["window_s"] > float(params["WINDOW"]) + SLACK_S:
        out["wire_steady"] = f"window lasted {wire_steady['window_s']:.0f}s, not {params['WINDOW']}s"
    return out


# --------------------------------------------------------------------------
# rows
# --------------------------------------------------------------------------


def s(v) -> str:
    return "—" if v is None else f"{v:.0f}" if v >= 10 else f"{v:.1f}"


def recovery(run: dict, label: str, d: str, part: str) -> str:
    x = run["times"].get(label)
    if x is None:
        return "—"
    if label in run["invalid"]:
        return "invalid"
    leg = x[d]
    # both cuts remove the only path, so a route that outlives one is stale
    stale = leg["reachable_at_cut"] and leg["unreachable_after_cut_s"] is None
    if part == "lost":
        if not leg["reachable_at_cut"]:
            return "down already"
        return "never (stale)" if stale else s(leg["unreachable_after_cut_s"])
    if stale:
        return "n/a"
    back = leg["reachable_after_heal_s"]
    return f">{x['heal_window_s']:.0f}" if back is None else s(back)


def knows_z(run: dict) -> str:
    t = run["times"]
    named = t["z_named_at_a_s"]
    if named is None:
        return "never"
    cross = t["events"].get("cross")
    dial = t["events"].get("dial")
    if cross is not None and dial is not None and named >= cross - dial:
        return f"on contact, +{s(run['crossing']['cross'].get('names_s'))}"
    return s(named)


def leg(run: dict, which: str, part: str) -> str:
    x = run["crossing"].get(which)
    if not x:
        return "—"
    # crossing.py leaves a leg unset when it timed out, and every leg after it
    return "failed" if x.get(part) is None else s(x[part])


def steady(run: dict, cell) -> str:
    return "invalid" if "wire_steady" in run["invalid"] else cell(run["wire_steady"])


def routing_bytes(run: dict, sphere: str) -> str:
    rows = [r for r in run["routing_by_type"]
            if r["sphere"] == sphere and r["type"] != "MANAGEMENT"]
    return "—" if not rows else f"{sum(r['bytes_per_s'] for r in rows):.0f}"


ROWS = [
    ("Bring-up (seconds)", None),
    ("Own community converged, slowest community (from bring-up)",
     lambda r: s(r["times"]["local_convergence_max_s"])),
    ("Hub routes every user (from dial)", lambda r: s(r["times"]["hub_reaches_all_s"])),
    ("Every gateway routes every user (from dial)",
     lambda r: s(r["times"]["gateway_directory_max_s"])),
    ("Z reachable from A (from dial)", lambda r: s(r["times"]["reachable_s"]["a_to_z"])),
    ("A reachable from Z (from dial)", lambda r: s(r["times"]["reachable_s"]["z_to_a"])),
    ("Every member routes every foreign user (from dial)",
     lambda r: s(r["times"]["every_member_reaches_all_s"]) if r["arm"] == "v1" else "contained"),
    ("Members holding foreign users before contact",
     lambda r: f"{len(r['times']['members_holding_foreign_users'])}"),
    ("A knows Z's name (from dial)", knows_z),
    ("Crossing, A to Z (seconds)", None),
    ("Invite", lambda r: leg(r, "cross", "invite_s")),
    ("Accept", lambda r: leg(r, "cross", "accept_s")),
    ("Deliver", lambda r: leg(r, "cross", "deliver_s")),
    ("Total", lambda r: leg(r, "cross", "total_s")),
    ("Local control total", lambda r: leg(r, "local", "total_s")),
    ("A's gateway loses its hub link (seconds)", None),
    ("A→Z lost after the cut", lambda r: recovery(r, "gateway_reconnect", "a_to_z", "lost")),
    ("A→Z back after the heal", lambda r: recovery(r, "gateway_reconnect", "a_to_z", "back")),
    ("Z→A lost after the cut", lambda r: recovery(r, "gateway_reconnect", "z_to_a", "lost")),
    ("Z→A back after the heal", lambda r: recovery(r, "gateway_reconnect", "z_to_a", "back")),
    ("Z leaves its community (seconds)", None),
    ("A→Z lost after the cut", lambda r: recovery(r, "member_leave", "a_to_z", "lost")),
    ("A→Z back after the heal", lambda r: recovery(r, "member_leave", "a_to_z", "back")),
    ("Z→A lost after the cut", lambda r: recovery(r, "member_leave", "z_to_a", "lost")),
    ("Z→A back after the heal", lambda r: recovery(r, "member_leave", "z_to_a", "back")),
    ("Wire, every node's uplink (sent)", None),
    ("Bring-up to settle end, KB/s", lambda r: f"{r['wire_bringup']['tx_bytes_per_s'] / 1000:.1f}"),
    ("Steady window, KB/s", lambda r: steady(r, lambda w: f"{w['tx_bytes_per_s'] / 1000:.1f}")),
    ("Steady window, packets/s", lambda r: steady(r, lambda w: f"{w['tx_packets_per_s']:.0f}")),
    ("Steady window, bytes per packet", lambda r: steady(r, lambda w: f"{w['bytes_per_packet']:.0f}")),
    ("Steady, per hub B/s", lambda r: steady(r, lambda w: f"{w['per_node_tx_bytes_per_s']['hub']:.0f}")),
    ("Steady, per gateway B/s",
     lambda r: steady(r, lambda w: f"{w['per_node_tx_bytes_per_s']['gateway']:.0f}")),
    ("Steady, per member B/s",
     lambda r: steady(r, lambda w: f"{w['per_node_tx_bytes_per_s']['member']:.0f}")),
    ("router_v2 frames, steady (logged payload, no framing)", None),
    ("Local sphere B/s", lambda r: routing_bytes(r, "Local")),
    ("Internet sphere (cross-membrane) B/s", lambda r: routing_bytes(r, "Internet")),
]


def table(runs: list[dict]) -> str:
    arms = [a for a in ("v1", "v2") if any(r["arm"] == a for r in runs)]
    by_arm = {a: [r for r in runs if r["arm"] == a] for a in arms}
    lines = [f"| Measure | {' | '.join(arms)} |", "|---|" + "---|" * len(arms)]
    for label, cell in ROWS:
        if cell is None:
            lines.append(f"| **{label}** |" + " |" * len(arms))
            continue
        cells = [" / ".join(cell(r) for r in by_arm[a]) for a in arms]
        lines.append(f"| {label} | {' | '.join(cells)} |")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("runs", nargs="+", help="run archive directories")
    ap.add_argument("--json", dest="json_out", default=None)
    args = ap.parse_args()

    runs = sorted((load_run(p) for p in args.runs), key=lambda r: (r["arm"], r["name"]))
    topos = {r["topo"] for r in runs}
    params = {json.dumps(r["params"], sort_keys=True) for r in runs}

    print(f"## {', '.join(sorted(topos))}: v1 against v2\n")
    for arm in ("v1", "v2"):
        names = [r["name"].rsplit("-", 1)[-1] for r in runs if r["arm"] == arm]
        if names:
            print(f"- {arm}: {len(names)} run(s), {', '.join(names)}")
    print(f"- git: {', '.join(sorted({r['git'] or '?' for r in runs}))}")
    if len(params) == 1:
        p = runs[0]["params"]
        print("- " + ", ".join(f"{k}={v}" for k, v in p.items() if k != "TOPO"))
    else:
        print("- WARNING: the runs were made with different compare.sh settings")
    print("\nOne value per run, in archive order, separated by ' / '.\n")
    print(table(runs))
    notes = [f"- {r['name']}: {section} invalid, {why}"
             for r in runs for section, why in r["invalid"].items()]
    if notes:
        print("\n\"invalid\": the phase ran past its configured length, so every node was"
              " paused with it.\n")
        print("\n".join(notes))

    if args.json_out:
        with open(args.json_out, "w") as f:
            json.dump(runs, f, indent=2)


if __name__ == "__main__":
    sys.exit(main())
