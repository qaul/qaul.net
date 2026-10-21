#!/usr/bin/env python3
"""Measure router_v2 protocol overhead across a community/hub topology.

Two instruments, deliberately kept separate:

  router_v2 logs    every frame is logged at exactly two choke points
                    (lib.rs SEND, receive/mod.rs RECV) with its exact
                    size, so overhead can be attributed per peer, per
                    transport and per message type. Counts protocol
                    frames only - no libp2p/QUIC/TCP framing.

  uplink counters   `ip -s link` per namespace gives total wire bytes,
                    framing included, and works identically for v1 and
                    v2 - which is the only way to compare the two arms,
                    since v1 logs no sizes at all. It also counts mDNS,
                    ping, manifest and feed traffic, so it is the cost of
                    running the protocol, not routing in isolation.

Transport maps to sphere: Lan/Ble are the Local sphere, Internet is the
Internet sphere, so "cross-membrane" is simply the Internet rows (spec
section 2.3).

Usage:
    ./overhead.py sample --roles topologies/communities-3x10.roles.json -o pre.json
    # ... let it run ...
    ./overhead.py sample --roles topologies/communities-3x10.roles.json -o post.json
    ./overhead.py reduce --roles topologies/communities-3x10.roles.json \
        --pre pre.json --post post.json [--json out.json]
"""

import argparse
import concurrent.futures
import datetime
import json
import os
import re
import subprocess
import time
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

SUDO = ["sudo", "-n"]

TYPE_NAMES = {
    0x01: "ROUTING_UPDATE",
    0x02: "INDEX_DUMP",
    0x03: "NODE_MANIFEST",
    0x04: "MANIFEST_DELTA",
    0x05: "MANIFEST_REQUEST",
}

LOCAL_SPHERE_TRANSPORTS = {"Lan", "Ble"}

# One origin cycle (10s, router_v2/init.rs) plus margin: long enough that
# a refreshed entry must have had a new seq by now.
REFRESH_WATCH_S = 13

# ' 2026-10-17T17:56:31.097Z INFO  libqaul > router_v2 SEND -> peer=12D3K... '
# Matched on the SEND/RECV keyword rather than the arrow glyphs, so the
# regex does not depend on how the log file is encoded.
FRAME_RE = re.compile(
    r"^\s*(?P<ts>\d{4}-\d{2}-\d{2}T[\d:.]+Z)\s+\w+\s+.*?"
    r"router_v2\s+(?P<direction>SEND|RECV)\b.*?"
    r"peer=(?P<peer>\S+)\s+"
    r"transport=(?P<transport>\w+)\s+"
    r"type=(?P<type>0x[0-9a-fA-F]+)\s+"
    r"bytes=(?P<bytes>\d+)"
)


def parse_ts(text: str) -> datetime.datetime:
    return datetime.datetime.fromisoformat(text.replace("Z", "+00:00"))


def now_iso() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


# --------------------------------------------------------------------------
# sample
# --------------------------------------------------------------------------


def iface_stats(node_id: str) -> dict:
    """RX/TX byte and packet counters for a node's only interface."""
    cmd = SUDO + [
        "ip", "netns", "exec", f"ns-{node_id}",
        "ip", "-s", "-j", "link", "show", "uplink",
    ]
    res = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
    if res.returncode != 0:
        raise RuntimeError(f"ip -s link failed for {node_id}: {res.stderr.strip()}")

    data = json.loads(res.stdout)
    stats = data[0].get("stats64") or {}
    rx, tx = stats.get("rx", {}), stats.get("tx", {})
    missing = not rx or not tx
    if missing:
        raise RuntimeError(
            f"no stats64 for {node_id}; iproute2 too old for -j stats"
        )
    return {
        "rx_bytes": rx["bytes"],
        "tx_bytes": tx["bytes"],
        "rx_packets": rx["packets"],
        "tx_packets": tx["packets"],
    }


def peer_id_of(node_id: str) -> str | None:
    """This node's libp2p PeerId, so log `peer=` fields can be attributed.

    Best effort: the byte totals do not depend on it.
    """
    try:
        from lib.node import Node

        return Node(node_id).node_info().get("id_base58")
    except Exception:
        return None


def hub_is_clean(hub_id: str) -> tuple[bool, int]:
    """True when the hub holds no user entry.

    A user entry at the hub means either the hub bridged the communities
    over LAN, or a gateway's attach-time origin leak has not yet aged out
    at the 35s route expiry. Measuring across either would inflate
    cross-membrane bytes - the one figure the report turns on.
    """
    from lib.node import Node

    try:
        table = Node(hub_id).router_v2_table()
    except Exception:
        return True, -1  # cannot tell; do not block the sample
    users = [e for e in table if e.get("space") == "user"]
    return not users, len(users)


def do_sample(node_ids: list[str], out_path: str) -> None:
    counters: dict[str, dict] = {}
    peer_ids: dict[str, str] = {}
    errors: dict[str, str] = {}

    with concurrent.futures.ThreadPoolExecutor(max_workers=min(len(node_ids), 32)) as pool:
        stat_futures = {pool.submit(iface_stats, nid): nid for nid in node_ids}
        peer_futures = {pool.submit(peer_id_of, nid): nid for nid in node_ids}

        for future in concurrent.futures.as_completed(stat_futures):
            nid = stat_futures[future]
            try:
                counters[nid] = future.result()
            except Exception as exc:
                errors[nid] = str(exc)

        for future in concurrent.futures.as_completed(peer_futures):
            nid = peer_futures[future]
            try:
                peer = future.result()
                if peer:
                    peer_ids[nid] = peer
            except Exception:
                pass

    sample = {
        "timestamp": now_iso(),
        "counters": counters,
        "peer_ids": peer_ids,
        "errors": errors,
    }
    with open(out_path, "w") as f:
        json.dump(sample, f, indent=1)

    print(f"sampled {len(counters)}/{len(node_ids)} nodes at {sample['timestamp']}")
    if peer_ids:
        print(f"  resolved {len(peer_ids)} peer ids")
    if errors:
        print(f"  {len(errors)} node(s) failed: {sorted(errors)[:5]}")
    print(f"wrote {out_path}")


# --------------------------------------------------------------------------
# reduce
# --------------------------------------------------------------------------


def read_frames(node_id: str, start, end) -> list[dict]:
    """router_v2 frames logged by one node inside [start, end]."""
    path = f"/tmp/qaul-{node_id}/qauld.err"
    if not os.path.exists(path):
        return []

    frames = []
    with open(path, errors="replace") as f:
        for line in f:
            if "router_v2" not in line:
                continue
            match = FRAME_RE.match(line)
            if not match:
                continue
            ts = parse_ts(match.group("ts"))
            if ts < start or ts > end:
                continue
            frames.append(
                {
                    "ts": ts,
                    "direction": match.group("direction"),
                    "peer": match.group("peer"),
                    "transport": match.group("transport"),
                    "type": int(match.group("type"), 16),
                    "bytes": int(match.group("bytes")),
                }
            )
    return frames


def fmt(value: float) -> str:
    return f"{value:,.1f}"


def do_reduce(roles: dict, pre: dict, post: dict, json_out: str | None) -> None:
    start, end = parse_ts(pre["timestamp"]), parse_ts(post["timestamp"])
    window_s = (end - start).total_seconds()
    if window_s <= 0:
        raise SystemExit("post sample is not after the pre sample")

    hub = roles["hub"]
    gateways = {c["gateway"] for c in roles["communities"]}
    community_of = {}
    for community in roles["communities"]:
        for member in community["members"]:
            community_of[member] = community["index"]

    node_ids = [hub] + [m for c in roles["communities"] for m in c["members"]]

    with concurrent.futures.ThreadPoolExecutor(max_workers=min(len(node_ids), 32)) as pool:
        futures = {pool.submit(read_frames, nid, start, end): nid for nid in node_ids}
        frames = {futures[f]: f.result() for f in concurrent.futures.as_completed(futures)}

    # (direction, transport, type) -> [frames, bytes]
    by_key: dict[tuple, list] = defaultdict(lambda: [0, 0])
    # node -> transport -> [frames, bytes]  (SEND only: bytes this node put out)
    sent_by_node: dict[str, dict] = defaultdict(lambda: defaultdict(lambda: [0, 0]))

    for nid, node_frames in frames.items():
        for frame in node_frames:
            key = (frame["direction"], frame["transport"], frame["type"])
            by_key[key][0] += 1
            by_key[key][1] += frame["bytes"]
            if frame["direction"] == "SEND":
                slot = sent_by_node[nid][frame["transport"]]
                slot[0] += 1
                slot[1] += frame["bytes"]

    def sent(nid: str, transports) -> tuple[int, int]:
        total_frames = total_bytes = 0
        for transport, (count, size) in sent_by_node.get(nid, {}).items():
            if transport in transports:
                total_frames += count
                total_bytes += size
        return total_frames, total_bytes

    internet = {"Internet"}
    report: dict = {
        "window_s": window_s,
        "window": {"start": pre["timestamp"], "end": post["timestamp"]},
        "topology": {
            "name": roles["name"],
            "communities": roles["community_count"],
            "members_per_community": roles["members_per_community"],
            "nodes": roles["totals"]["nodes"],
            "links": roles["totals"]["links"],
        },
    }

    print(f"\n=== {roles['name']} — {window_s:.0f}s window ===")
    print(f"{roles['community_count']} communities x {roles['members_per_community']} members, "
          f"{roles['totals']['nodes']} nodes, {roles['totals']['links']} links")

    # --- cross-membrane -------------------------------------------------
    hub_frames, hub_bytes = sent(hub, internet)
    gw_frames = gw_bytes = 0
    for gateway in gateways:
        f_count, f_bytes = sent(gateway, internet)
        gw_frames += f_count
        gw_bytes += f_bytes
    cross_bytes = hub_bytes + gw_bytes

    print("\n-- cross-membrane (Internet sphere, outbound) --")
    print(f"  hub {hub}        {fmt(hub_bytes / window_s)} B/s  "
          f"({fmt(hub_frames / window_s)} frames/s, {hub_bytes:,} B total)")
    print(f"  {len(gateways)} gateways      {fmt(gw_bytes / window_s)} B/s  "
          f"({fmt(gw_frames / window_s)} frames/s, {gw_bytes:,} B total)")
    print(f"  per gateway      {fmt(gw_bytes / window_s / max(len(gateways), 1))} B/s")
    print(f"  total crossing   {fmt(cross_bytes / window_s)} B/s")
    report["cross_membrane"] = {
        "hub_bytes_per_s": hub_bytes / window_s,
        "gateways_bytes_per_s": gw_bytes / window_s,
        "per_gateway_bytes_per_s": gw_bytes / window_s / max(len(gateways), 1),
        "total_bytes_per_s": cross_bytes / window_s,
        "hub_frames_per_s": hub_frames / window_s,
        "gateway_frames_per_s": gw_frames / window_s,
    }

    # --- intra-community ------------------------------------------------
    print("\n-- intra-community (Local sphere, outbound) --")
    print("  community  nodes  links      B/s    B/s/link   B/s/node")
    community_rows = []
    for community in roles["communities"]:
        members = community["members"]
        total_bytes = total_frames = 0
        for member in members:
            f_count, f_bytes = sent(member, LOCAL_SPHERE_TRANSPORTS)
            total_frames += f_count
            total_bytes += f_bytes
        per_s = total_bytes / window_s
        link_count = max(community["links"], 1)
        row = {
            "index": community["index"],
            "members": len(members),
            "links": community["links"],
            "bytes_per_s": per_s,
            "bytes_per_s_per_link": per_s / link_count,
            "bytes_per_s_per_node": per_s / len(members),
            "frames_per_s": total_frames / window_s,
        }
        community_rows.append(row)
        print(f"  {community['index']:>9}  {len(members):>5}  {community['links']:>5}  "
              f"{fmt(per_s):>9}  {fmt(row['bytes_per_s_per_link']):>9}  "
              f"{fmt(row['bytes_per_s_per_node']):>9}")
    report["communities"] = community_rows

    # --- by message type ------------------------------------------------
    print("\n-- outbound by message type --")
    print("  sphere     type                 frames/s        B/s")
    type_rows = []
    for (direction, transport, type_id), (count, size) in sorted(
        by_key.items(), key=lambda kv: -kv[1][1]
    ):
        if direction != "SEND":
            continue
        sphere = "Local" if transport in LOCAL_SPHERE_TRANSPORTS else "Internet"
        name = TYPE_NAMES.get(type_id, f"UNKNOWN(0x{type_id:02x})")
        type_rows.append(
            {
                "sphere": sphere,
                "transport": transport,
                "type": name,
                "frames_per_s": count / window_s,
                "bytes_per_s": size / window_s,
                "bytes_total": size,
            }
        )
        print(f"  {sphere:<9}  {name:<18} {fmt(count / window_s):>10} {fmt(size / window_s):>10}")
    report["by_type"] = type_rows

    # --- wire bytes -----------------------------------------------------
    print("\n-- wire bytes (uplink counters, all traffic incl. framing) --")
    wire_rows = []
    missing = []
    for nid in node_ids:
        before, after = pre["counters"].get(nid), post["counters"].get(nid)
        if not before or not after:
            missing.append(nid)
            continue
        tx = after["tx_bytes"] - before["tx_bytes"]
        rx = after["rx_bytes"] - before["rx_bytes"]
        wire_rows.append(
            {
                "node": nid,
                "role": "hub" if nid == hub else ("gateway" if nid in gateways else "member"),
                "community": community_of.get(nid),
                "tx_bytes_per_s": tx / window_s,
                "rx_bytes_per_s": rx / window_s,
            }
        )
    for role in ("hub", "gateway", "member"):
        rows = [r for r in wire_rows if r["role"] == role]
        if not rows:
            continue
        tx = sum(r["tx_bytes_per_s"] for r in rows) / len(rows)
        rx = sum(r["rx_bytes_per_s"] for r in rows) / len(rows)
        print(f"  {role:<8} n={len(rows):<4} tx {fmt(tx):>9} B/s   rx {fmt(rx):>9} B/s  (mean)")
    if missing:
        print(f"  no counter pair for {len(missing)} node(s): {sorted(missing)[:5]}")
    report["wire"] = wire_rows

    print("\nThe membrane is not a byte count - the logs carry frame sizes,"
          "\nnot contents. Assert it separately:  ./overhead.py membrane --roles ...")

    if json_out:
        parent = os.path.dirname(json_out)
        if parent:
            os.makedirs(parent, exist_ok=True)
        with open(json_out, "w") as f:
            json.dump(report, f, indent=1)
        print(f"\nwrote {json_out}")


# --------------------------------------------------------------------------
# membrane
# --------------------------------------------------------------------------


def do_membrane(roles: dict) -> None:
    """Assert the section 2.3 membrane held, by inspecting routing tables.

    Byte totals cannot show this - the logs carry frame sizes, not
    contents - so it is checked where it is observable: no user entry may
    cross to the Internet sphere, so the hub must hold none at all, and no
    community may know a user belonging to another community.

    The user-space checks are assertions. Node space is reported rather
    than asserted: "only gateways cross" needs each node's 8-byte node id
    to compare against, which the table exposes only as base58 targets.
    """
    from lib.node import Node

    hub_id = roles["hub"]
    community_of_user: dict[str, int] = {}
    own_user: dict[str, str] = {}

    print("resolving local user ids")
    for community in roles["communities"]:
        for member in community["members"]:
            try:
                user_id = Node(member).local_user_id()
            except Exception as exc:
                print(f"  {member}: could not resolve ({exc})")
                continue
            community_of_user[user_id] = community["index"]
            own_user[member] = user_id

    failures = 0

    # --- the hub holds no user entry at all -----------------------------
    hub_table = Node(hub_id).router_v2_table()
    hub_users = [e for e in hub_table if e.get("space") == "user"]
    hub_nodes = [e for e in hub_table if e.get("space") == "node"]
    print(f"\nhub {hub_id}: {len(hub_users)} user entries, "
          f"{len(hub_nodes)} node entries")
    if hub_users:
        failures += 1
        transports = {e.get("transport") for e in hub_users}
        print("  FAIL  no user entry may cross to the Internet sphere")
        print(f"        transports={transports} "
              f"hops={sorted({e.get('hop_count') for e in hub_users})} "
              f"age_ms={sorted(e.get('age_ms', 0) for e in hub_users)}")
        if transports == {"LAN"}:
            print("        transport is LAN: the hub bridged - rerun from stage 1")
        else:
            # A gateway should originate node entries, not user entries
            # (desired_propagation_form -> Node once an internet transport
            # is active). So an Internet-borne user entry is most likely the
            # single tick where the internet mirror already existed but the
            # form had not flipped yet. Refreshing entries would mean
            # something worse: a sender that stays in User form.
            print(f"        re-reading in {REFRESH_WATCH_S}s to tell a stale "
                  "entry from one being refreshed")
            before = {e.get("index"): (e.get("seq_num"), e.get("age_ms", 0))
                      for e in hub_users}
            time.sleep(REFRESH_WATCH_S)
            after_users = [
                e for e in Node(hub_id).router_v2_table()
                if e.get("space") == "user"
            ]
            if not after_users:
                print("        -> GONE: transient, cleared by the 35s route "
                      "expiry. Still a spec violation, but self-healing.")
            else:
                refreshed = [
                    e for e in after_users
                    if e.get("index") in before
                    and e.get("seq_num") != before[e["index"]][0]
                ]
                if refreshed:
                    print(f"        -> REFRESHING: {len(refreshed)} entry(ies) "
                          "got a new seq. A sender keeps emitting user entries "
                          "over the Internet sphere - persistent leak.")
                else:
                    print(f"        -> still present, not refreshed "
                          f"(age_ms={sorted(e.get('age_ms',0) for e in after_users)}); "
                          "aging out toward the 35s expiry.")
    else:
        print("  ok    user space empty")
    print(f"  node space targets: "
          f"{sorted({e.get('target_id') for e in hub_nodes})[:6]} "
          f"(expect gateways only; reported, not asserted)")

    # --- no community knows another community's users -------------------
    print("\nforeign users per community (expect 0):")
    for community in roles["communities"]:
        idx = community["index"]
        worst = 0
        worst_node = None
        for member in community["members"]:
            try:
                table = Node(member).router_v2_table()
            except Exception:
                continue
            foreign = 0
            for entry in table:
                if entry.get("space") != "user":
                    continue
                target = entry.get("target_id")
                owner = community_of_user.get(target)
                if owner is not None and owner != idx:
                    foreign += 1
            if foreign > worst:
                worst, worst_node = foreign, member
        mark = "ok" if worst == 0 else "FAIL"
        if worst:
            failures += 1
        print(f"  community {idx}: worst node {worst_node or '-'} knows "
              f"{worst} foreign user(s) [{mark}]")

    unresolved = sum(
        1 for c in roles["communities"] for m in c["members"] if m not in own_user
    )
    if unresolved:
        print(f"\nnote: {unresolved} node(s) had no resolvable local user, so "
              "their users could not be attributed")

    print(f"\n{'MEMBRANE HELD' if failures == 0 else f'{failures} membrane failure(s)'}")
    if failures:
        raise SystemExit(1)


def load_roles(path: str) -> dict:
    with open(path) as f:
        return json.load(f)


def all_node_ids(roles: dict) -> list[str]:
    return [roles["hub"]] + [m for c in roles["communities"] for m in c["members"]]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="mode", required=True)

    p_sample = sub.add_parser("sample", help="snapshot interface counters")
    p_sample.add_argument("--roles", required=True)
    p_sample.add_argument("-o", "--out", required=True)
    p_sample.add_argument("--force", action="store_true",
                          help="sample even if the hub still holds user entries")

    p_reduce = sub.add_parser("reduce", help="summarise overhead over a window")
    p_reduce.add_argument("--roles", required=True)
    p_reduce.add_argument("--pre", required=True)
    p_reduce.add_argument("--post", required=True)
    p_reduce.add_argument("--json", dest="json_out", default=None)

    p_membrane = sub.add_parser("membrane", help="assert the 2.3 membrane held")
    p_membrane.add_argument("--roles", required=True)

    args = ap.parse_args()
    roles = load_roles(args.roles)

    if args.mode == "sample":
        clean, count = hub_is_clean(roles["hub"])
        if not clean and not args.force:
            raise SystemExit(
                f"hub {roles['hub']} holds {count} user entry(ies), so the "
                "membrane is not clean yet and cross-membrane bytes would be "
                "inflated.\n"
                "  wait out the 35s route expiry and re-check with:\n"
                f"    ./overhead.py membrane --roles {args.roles}\n"
                "  or pass --force to sample anyway."
            )
        do_sample(all_node_ids(roles), args.out)
    elif args.mode == "membrane":
        do_membrane(roles)
    else:
        with open(args.pre) as f:
            pre = json.load(f)
        with open(args.post) as f:
            post = json.load(f)
        do_reduce(roles, pre, post, args.json_out)


if __name__ == "__main__":
    main()
