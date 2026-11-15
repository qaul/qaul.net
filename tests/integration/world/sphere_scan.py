#!/usr/bin/env python3
"""Scan an archived world run for LAN traffic that crossed a sphere boundary.

check.py looks at the spheres once, at the end of its settle. A merge after
that is invisible to it: on 2026-11-11 W1 and H1 became LAN neighbours 14
minutes after the dial, long after the check had passed. This reads every
node's whole log instead, from bring-up to the moment it was archived, so it
also covers a lab left running after its check. That is the case when
run.sh archives the outgoing lab at the start of the next run.

Two nodes may exchange LAN traffic only if both keep LAN on and they sit in
the same Local sphere. Anything else (two communities, a server, a remote
user) is a crossing. The evidence is router_v2's per-message log lines
("peer=<PeerId> transport=Lan"), so a v1 archive has nothing to judge.

Usage:
    ./sphere_scan.py ../runs/world-1-half-v2-20261112T095741Z [--json out.json]

Exits 1 if any crossing is found, 0 otherwise. An archive that is not a
world run is skipped with 0.
"""

import argparse
import glob
import gzip
import json
import os
import re
import sys
from datetime import datetime, timezone

LINE = re.compile(r"^\s*(\S+Z)\s.*?peer=(12D3\w+)\s+transport=Lan\b")


def parse_ts(ts: str) -> float | None:
    try:
        return datetime.strptime(ts, "%Y-%m-%dT%H:%M:%S.%fZ").replace(
            tzinfo=timezone.utc).timestamp()
    except ValueError:
        return None


def read_kv(path: str) -> dict:
    out = {}
    if os.path.isfile(path):
        with open(path) as f:
            for line in f:
                if "=" in line:
                    k, v = line.rstrip("\n").split("=", 1)
                    out[k] = v
    return out


def dial_time(archive: str) -> float | None:
    """t0 from this run's check result, if the archive kept it."""
    for path in glob.glob(os.path.join(archive, "results", "world-check-*.json")):
        try:
            with open(path) as f:
                t0 = json.load(f).get("t0")
            if isinstance(t0, (int, float)):
                return float(t0)
        except (OSError, ValueError):
            pass
    return None


def allowed(meta_a: dict, meta_b: dict) -> bool:
    return (meta_a.get("lan") and meta_b.get("lan")
            and meta_a.get("sphere") is not None
            and meta_a.get("sphere") == meta_b.get("sphere"))


def scan(archive: str) -> dict | None:
    roles_path = os.path.join(archive, "roles.json")
    if not os.path.isfile(roles_path):
        return None
    with open(roles_path) as f:
        roles = json.load(f)
    if roles.get("kind") != "world":
        return None
    nodes = roles["nodes"]

    peer_of = {}
    for nid in nodes:
        ids = read_kv(os.path.join(archive, nid, "ids.txt"))
        if ids.get("node_id"):
            peer_of[ids["node_id"]] = nid

    lan_lines = 0
    crossings: dict[tuple[str, str], dict] = {}
    for nid, meta in nodes.items():
        log = os.path.join(archive, nid, "qauld.err.gz")
        opener = gzip.open
        if not os.path.isfile(log):
            # A running lab's log is plain; an archive's is gzipped.
            log, opener = os.path.join(archive, nid, "qauld.err"), open
        if not os.path.isfile(log):
            continue  # a far node in a hybrid run; its log lives elsewhere
        with opener(log, "rt", errors="replace") as f:
            for line in f:
                if "transport=Lan" not in line:
                    continue
                m = LINE.match(line)
                if not m:
                    continue
                lan_lines += 1
                other = peer_of.get(m.group(2))
                if other is None or allowed(meta, nodes.get(other, {})):
                    continue
                key = tuple(sorted((nid, other)))
                ts = parse_ts(m.group(1))
                c = crossings.setdefault(key, {"first": ts, "last": ts, "lines": 0,
                                               "seen_by": set()})
                c["lines"] += 1
                c["seen_by"].add(nid)
                if ts is not None:
                    c["first"] = ts if c["first"] is None else min(c["first"], ts)
                    c["last"] = ts if c["last"] is None else max(c["last"], ts)

    run = read_kv(os.path.join(archive, "run.txt"))
    started = parse_ts(run.get("STARTED", "").replace("Z", "") + "Z") if run.get("STARTED") else None
    t0 = dial_time(archive)

    def name(nid: str) -> str:
        return nodes.get(nid, {}).get("name", nid)

    def where(nid: str) -> str:
        m = nodes.get(nid, {})
        return m.get("sphere") or m.get("role", "?")

    out = []
    for (a, b), c in sorted(crossings.items(), key=lambda kv: kv[1]["first"] or 0):
        out.append({
            "pair": f"{name(a)}-{name(b)}",
            "spheres": f"{where(a)} / {where(b)}",
            "first_utc": datetime.fromtimestamp(c["first"], timezone.utc).isoformat()
            if c["first"] else None,
            "first_after_dial_s": round(c["first"] - t0, 1) if c["first"] and t0 else None,
            "first_after_start_s": round(c["first"] - started, 1)
            if c["first"] and started else None,
            "last_utc": datetime.fromtimestamp(c["last"], timezone.utc).isoformat()
            if c["last"] else None,
            "log_lines": c["lines"],
            "seen_by": sorted(name(x) for x in c["seen_by"]),
        })
    return {"archive": os.path.basename(os.path.normpath(archive)),
            "tag": run.get("TAG"), "lan_lines": lan_lines, "t0": t0,
            "crossings": out, "ok": not out}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("archive")
    ap.add_argument("--json", help="also write the result here")
    args = ap.parse_args()

    res = scan(args.archive)
    if res is None:
        print(f"sphere scan: {args.archive} is not a world archive, skipped")
        return 0
    if args.json:
        with open(args.json, "w") as f:
            json.dump(res, f, indent=2)

    print(f"sphere scan: {res['archive']} ({res['lan_lines']} LAN log lines)")
    if res["lan_lines"] == 0:
        print("  no per-peer LAN lines in any log, so nothing to judge (a v1 run?)")
        return 0
    if res["ok"]:
        print("  no LAN traffic crossed a sphere boundary, from bring-up to archive")
        return 0
    for c in res["crossings"]:
        when = (f"{c['first_after_dial_s']:+.1f} s from the dial"
                if c["first_after_dial_s"] is not None else
                f"{c['first_after_start_s']:.1f} s after the lab started"
                if c["first_after_start_s"] is not None else "time unknown")
        print(f"  CROSSING {c['pair']} ({c['spheres']}): first {c['first_utc']} ({when}), "
              f"last {c['last_utc']}, {c['log_lines']} lines")
    return 1


if __name__ == "__main__":
    sys.exit(main())
