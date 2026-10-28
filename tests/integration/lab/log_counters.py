#!/usr/bin/env python3
"""Count router_v2's failure signatures in every node's log.

Each counter is a log line that the research doc
(docs/research/router-v2-cross-community-messaging.md) ties to a defect, so a
run is judged by numbers rather than by re-reading 31 logs:

  stale_user    index_dump: dropped stale user mirror       defect 3: a late dump
  stale_node    index_dump: dropped stale node mirror       pruned a newer binding
  unknown_user  receive-loop drop: no mapping (space=User)  a binding was lost,
  unknown_node  receive-loop drop: no mapping (space=Node)  whatever lost it
  no_mirror     index_dump: no mirror for                   defect 7: a frame beat
  vanished      neighbour vanished between mapping steps    our own first ping
  redump        peers that sent this node more than one INDEX_DUMP, i.e. that
                reconnected - the stranded-gateway precondition
  dials         connection established!  (Internet dials; LAN logs at trace)
  retired       missed N consecutive pings, closing the connection
  gw_role       gateway role changes
  form          propagation form changes
  mgmt_noroute  management: no route to                     defect 2
  prof_timeout  management: profile fetch ... timed out
  deferred      node_manifest ... deferred: origin key unknown
  no_key        No key found for user                       defect 1

Several are logged at debug. A node whose log holds no DEBUG line reports them
as "-" rather than as a misleading 0; run the lab with
RUST_LOG='libqaul::router_v2=debug,libqaul=info'.

A dump split into several chunks counts once per chunk. At lab sizes every dump
is one chunk.

Reads plain logs or the gzipped copies archive_lab.sh keeps. Read-only, so it
is safe against a running lab.

Usage:
    lab/log_counters.py                     # the live lab, /tmp/qaul-*
    lab/log_counters.py runs/<run>          # an archive from archive_lab.sh
    lab/log_counters.py --roles communities/topologies/communities-3x10.roles.json \
        --json c.json
"""

import argparse
import gzip
import json
import os
import re
import sys
from collections import Counter

# (counter, pattern, logged at debug)
SIGNATURES = [
    ("stale_user", re.compile(r"index_dump: dropped stale user mirror"), True),
    ("stale_node", re.compile(r"index_dump: dropped stale node mirror"), True),
    ("unknown_user", re.compile(r"no mapping for incoming idx=\d+ \(space=User"), False),
    ("unknown_node", re.compile(r"no mapping for incoming idx=\d+ \(space=Node"), False),
    ("no_mirror", re.compile(r"index_dump: no mirror for"), True),
    ("vanished", re.compile(r"neighbour vanished between mapping steps"), True),
    ("dials", re.compile(r"connection established! peer="), False),
    ("retired", re.compile(r"consecutive pings, closing the connection"), False),
    ("gw_role", re.compile(r"router_v2: gateway role"), False),
    ("form", re.compile(r"router_v2: propagation form"), False),
    ("mgmt_noroute", re.compile(r"management: no route to"), True),
    ("prof_timeout", re.compile(r"management: profile fetch for .* timed out"), True),
    ("deferred", re.compile(r"deferred: origin key unknown"), True),
    ("no_key", re.compile(r"No key found for user"), False),
]
DEBUG_ONLY = {name for name, _, debug in SIGNATURES if debug}
COLUMNS = [name for name, _, _ in SIGNATURES[:6]] + ["redump"] + [
    name for name, _, _ in SIGNATURES[6:]
]

DUMP_IN = re.compile(r"INDEX_DUMP ← peer=(\S+)")
TIMESTAMP = re.compile(r"^\s*(\d{4}-\d\d-\d\dT[\d:.]+Z)")


def discover(root: str) -> list[tuple[str, str]]:
    """(node id, log path) for every node under root, live or archived."""
    found = []
    for entry in sorted(os.listdir(root)):
        node_dir = os.path.join(root, entry)
        if not os.path.isdir(node_dir):
            continue
        for name in ("qauld.err", "qauld.err.gz"):
            path = os.path.join(node_dir, name)
            if os.path.isfile(path):
                node = entry[len("qaul-"):] if entry.startswith("qaul-") else entry
                found.append((node, path))
                break
    return found


def roles_of(path: str | None) -> dict[str, str]:
    if not path or not os.path.isfile(path):
        return {}
    with open(path) as f:
        spec = json.load(f)
    roles = {}
    for community in spec.get("communities", []):
        for member in community.get("members", []):
            roles[member] = "member"
        if community.get("gateway"):
            roles[community["gateway"]] = "gateway"
    if spec.get("hub"):
        roles[spec["hub"]] = "hub"
    return roles


def scan(path: str) -> dict:
    counts = Counter()
    dumps_from = Counter()
    debug = False
    first = last = None

    opener = gzip.open if path.endswith(".gz") else open
    with opener(path, "rt", encoding="utf-8", errors="replace") as f:
        for line in f:
            stamp = TIMESTAMP.match(line)
            if stamp:
                first = first or stamp.group(1)
                last = stamp.group(1)
            if not debug and " DEBUG " in line:
                debug = True
            for name, pattern, _ in SIGNATURES:
                if pattern.search(line):
                    counts[name] += 1
            dump = DUMP_IN.search(line)
            if dump:
                dumps_from[dump.group(1)] += 1

    repeat = {peer: n for peer, n in dumps_from.items() if n > 1}
    counts["redump"] = len(repeat)
    return {
        "first": first,
        "last": last,
        "debug": debug,
        "counts": {name: counts[name] for name in COLUMNS},
        "redump_peers": repeat,
    }


def cell(node: dict, name: str) -> str:
    if name in DEBUG_ONLY and not node["debug"]:
        return "-"
    return str(node["counts"][name])


def report(root: str, nodes: dict[str, dict]) -> None:
    firsts = [n["first"] for n in nodes.values() if n["first"]]
    lasts = [n["last"] for n in nodes.values() if n["last"]]
    span = f"{min(firsts)} .. {max(lasts)}" if firsts and lasts else "no timestamps"
    print(f"{root}: {len(nodes)} node log(s), {span}")

    no_debug = [node for node, n in nodes.items() if not n["debug"]]
    if no_debug:
        print(f"no DEBUG lines in {len(no_debug)} log(s), so their debug-only "
              f"counters read '-': {', '.join(no_debug)}")

    totals = {name: sum(n["counts"][name] for n in nodes.values()) for name in COLUMNS}
    shown = [name for name in COLUMNS if totals[name]]
    zero = [name for name in COLUMNS if not totals[name]]

    if shown:
        widths = {name: max(len(name), 4) for name in shown}
        print()
        print(f"{'node':<6}{'role':<9}" + " ".join(f"{name:>{widths[name]}}" for name in shown))
        for node_id in sorted(nodes):
            node = nodes[node_id]
            print(f"{node_id:<6}{node['role']:<9}"
                  + " ".join(f"{cell(node, name):>{widths[name]}}" for name in shown))
        print(f"{'total':<15}" + " ".join(f"{totals[name]:>{widths[name]}}" for name in shown))

    if zero:
        print(f"\nzero on every node: {', '.join(zero)}")

    reconnected = {node: n["redump_peers"] for node, n in nodes.items() if n["redump_peers"]}
    for node_id, peers in sorted(reconnected.items()):
        for peer, count in sorted(peers.items()):
            print(f"redump: {node_id} received {count} INDEX_DUMPs from {peer}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("root", nargs="?", default="/tmp",
                    help="directory holding the node dirs (default: /tmp, the live lab)")
    ap.add_argument("--roles", help="roles file; defaults to <root>/roles.json if present")
    ap.add_argument("--json", help="also write the counters to this file")
    args = ap.parse_args()

    logs = discover(args.root)
    if args.root == "/tmp":
        logs = [(node, path) for node, path in logs if "/qaul-" in path]
    if not logs:
        print(f"no node logs (qauld.err or qauld.err.gz) under {args.root}", file=sys.stderr)
        return 1

    roles = roles_of(args.roles or os.path.join(args.root, "roles.json"))
    nodes = {}
    for node_id, path in logs:
        nodes[node_id] = scan(path)
        nodes[node_id]["role"] = roles.get(node_id, "")

    report(args.root, nodes)

    if args.json:
        with open(args.json, "w") as f:
            json.dump({"root": args.root, "nodes": nodes}, f, indent=2)
    return 0


if __name__ == "__main__":
    sys.exit(main())
