"""Helpers shared by the world suite's scripts.

Everything reads the roles file gen_world.py writes; nothing infers a role
from the shape of the network.
"""

import json
import os
import sys

# lib/ is one level up, shared with the rest of tests/integration.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.node import Node, batch_query  # noqa: E402


def load_roles(path: str) -> dict:
    with open(path) as f:
        roles = json.load(f)
    if roles.get("kind") != "world":
        raise SystemExit(f"{path} is not a world roles file (made by gen_world.py)")
    return roles


def name(roles: dict, node_id: str) -> str:
    return roles["nodes"][node_id]["name"]


SIDES = ("laptop", "server")


def on_host(meta: dict) -> bool:
    """A far server or remote user runs on test.qaul.net itself, started by
    hand (S1, r1), not in a lab. Roles files written before "host" existed
    only had far servers and remote users, so the default says the same."""
    return meta.get("host", meta["location"] == "far" and meta["role"] in ("server", "remote"))


def lab_nodes(roles: dict, hybrid: bool = False, side: str = "laptop") -> list[str]:
    """The nodes running in this machine's lab.

    In a lab-only run every node is in the laptop's lab, the far ones
    included. In a hybrid run the laptop's lab holds the near nodes, and the
    server's lab (side="server") the far communities; the host nodes (S1, r1)
    are in neither and can't be reached through a lab socket.
    """
    if side not in SIDES:
        raise ValueError(f"side must be one of {SIDES}, got {side!r}")
    if not hybrid:
        return list(roles["nodes"]) if side == "laptop" else []
    if side == "laptop":
        return [nid for nid, m in roles["nodes"].items() if m["location"] != "far"]
    return [nid for nid, m in roles["nodes"].items()
            if m["location"] == "far" and not on_host(m)]


def egress_nodes(roles: dict) -> list[str]:
    """Lab nodes that need a way out to test.qaul.net in a hybrid run.

    These are the lab ends of the far Internet links (T1, T2, V1, W1, r3 and
    S2 in world-1). gen_world.py guarantees the lab end is the dialer, since
    nothing outside can open a connection into the lab. Members never get one.
    """
    far = {nid for nid, m in roles["nodes"].items() if m["location"] == "far"}
    out = set()
    for link in roles["internet_links"]:
        if link["far"]:
            out.update(x for x in (link["dialer"], link["target"]) if x not in far)
    return sorted(out)


def parse_far(roles: dict, specs: list[str] | None, side: str = "laptop") -> dict[str, str]:
    """--far NAME=MULTIADDR, repeated -> {far node id: the address to dial}.

    In a hybrid run the lab cannot query the nodes outside it: S1 and r1 run
    on test.qaul.net and are driven by hand. All the lab is told is where each
    such dial target listens: from the laptop, the server's public address;
    from the server's lab, the host end of hostlink.sh's links. Every target
    outside this side's lab that one of its nodes dials must have one.
    """
    out: dict[str, str] = {}
    for spec in specs or []:
        nm, sep, addr = spec.partition("=")
        if not sep or not addr:
            raise SystemExit(f"--far wants NAME=MULTIADDR, got '{spec}'")
        nid = roles["by_name"].get(nm)
        meta = roles["nodes"].get(nid) if nid else None
        if meta is None or not (on_host(meta) or "fwd_port" in meta):
            raise SystemExit(f"--far {nm}: not a node the laptop can dial on the server "
                             f"in {roles['name']}")
        out[nid] = addr
    lab = set(lab_nodes(roles, True, side))
    needed = {l["target"] for l in roles["internet_links"]
              if l["dialer"] in lab and l["target"] not in lab}
    missing = sorted(name(roles, n) for n in needed if n not in out)
    if missing:
        raise SystemExit(f"a hybrid run needs --far NAME=MULTIADDR for {', '.join(missing)}")
    return out


def infer_far_peers(roles: dict, neighbours: dict, by_peer: dict) -> dict[str, str]:
    """far node id -> the peer id its lab dialers see it as.

    A far node gets a fresh identity every run and the lab cannot ask it, so
    its id is read off the lab side: a lab node with exactly one Internet link
    leaving the lab votes, with each of its neighbours that is not a lab node,
    for that link's other end, and the most voted id wins. A node with several
    links leaving the lab (ring-2's shortcuts) can't tell which is which, so it
    doesn't vote; wire.verify checks those links by count instead.
    """
    from collections import Counter
    leaving: dict[str, list[str]] = {}
    for link in roles["internet_links"]:
        d, t = link["dialer"], link["target"]
        if (d in neighbours) != (t in neighbours):
            here, there = (d, t) if d in neighbours else (t, d)
            leaving.setdefault(here, []).append(there)
    votes: dict[str, Counter] = {}
    for here, there in leaving.items():
        if len(there) != 1:
            continue
        for n in neighbours[here].get("internet") or []:
            if n["node_id"] not in by_peer:
                votes.setdefault(there[0], Counter())[n["node_id"]] += 1
    return {t: c.most_common(1)[0][0] for t, c in votes.items()}


def sphere_members(roles: dict) -> dict[str, set[str]]:
    """node id -> every node in its Local sphere, itself included.

    Servers and remote users belong to no sphere: their set is themselves.
    """
    out = {nid: {nid} for nid in roles["nodes"]}
    for sphere in roles["spheres"]:
        members = set(sphere["members"])
        for m in members:
            out[m] = members
    return out


def pick_lab_address(addresses: list[str]) -> str | None:
    """The multiaddress another lab node can dial.

    Same rule as communities/phases.py: no loopback or wildcard, the lab's
    10.0.0.0/8 uplink first, QUIC before TCP.
    """
    usable = [a for a in addresses
              if "/ip4/" in a and "/ip4/127." not in a and "/ip4/0.0.0.0" not in a]
    usable.sort(key=lambda a: (0 if a.startswith("/ip4/10.") else 1,
                               0 if "quic" in a else 1))
    return usable[0] if usable else None


def gateway_flag(status: dict) -> bool | None:
    """The gateway flag out of `router v2 status`, or None if it is missing,
    so a renamed field shows up as unknown rather than a wrong pass."""
    value = status.get("is_gateway")
    return value if isinstance(value, bool) else None


def identities(roles: dict, node_ids: list[str], v2: bool = True) -> dict:
    """Every way the tools name a node, keyed by lab id.

    peer: the libp2p PeerId that `router neighbours` reports.
    user: the node's own account, as `users list` reports it.
    v2:   the short v2 node id that the v2 table, manifests and delegations
          use (v2 arm only).
    """
    nodes = {nid: Node(nid) for nid in node_ids}
    peer = batch_query(nodes, "node_id")
    user = batch_query(nodes, "local_user_id")
    out = {nid: {"peer": peer[nid], "user": user[nid]} for nid in node_ids}
    if v2:
        status = batch_query(nodes, "router_v2_status")
        for nid in node_ids:
            out[nid]["v2"] = status[nid].get("node_id")
    return out


def reverse(ids: dict, key: str) -> dict[str, str]:
    """identity value -> lab id, for one kind of identity."""
    return {v[key]: nid for nid, v in ids.items() if v.get(key)}
