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


def lab_nodes(roles: dict, hybrid: bool = False) -> list[str]:
    """The nodes running in the local lab.

    In a lab-only run every node is local, the far ones included. In a hybrid
    run the far nodes (S1, r1) run on test.qaul.net and are not reachable
    through a lab socket.
    """
    return [nid for nid, meta in roles["nodes"].items()
            if not (hybrid and meta["location"] == "far")]


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


def parse_far(roles: dict, specs: list[str] | None) -> dict[str, str]:
    """--far NAME=MULTIADDR, repeated -> {far node id: the address to dial}.

    In a hybrid run the lab cannot query the far nodes: they run on
    test.qaul.net and the user drives them by hand. All the lab is told is
    where each far dial target listens. Every far target with a lab dialer
    must have one.
    """
    out: dict[str, str] = {}
    for spec in specs or []:
        nm, sep, addr = spec.partition("=")
        if not sep or not addr:
            raise SystemExit(f"--far wants NAME=MULTIADDR, got '{spec}'")
        nid = roles["by_name"].get(nm)
        if nid is None or roles["nodes"][nid]["location"] != "far":
            raise SystemExit(f"--far {nm}: not a far node of {roles['name']}")
        out[nid] = addr
    needed = {l["target"] for l in roles["internet_links"]
              if l["far"] and roles["nodes"][l["dialer"]]["location"] != "far"}
    missing = sorted(name(roles, n) for n in needed if n not in out)
    if missing:
        raise SystemExit(f"a hybrid run needs --far NAME=MULTIADDR for {', '.join(missing)}")
    return out


def infer_far_peers(roles: dict, neighbours: dict, by_peer: dict) -> dict[str, str]:
    """far node id -> the peer id its lab dialers see it as.

    A far node gets a fresh identity every run and the lab cannot ask it, so
    its id is read off the lab side: each Internet neighbour of a far link's
    lab dialer that is not a lab node votes for that link's far target, and
    the most voted id wins. With one far target (S1), every far gateway must
    see the same id; any other unknown neighbour shows up as an unplanned link.
    """
    from collections import Counter
    votes: dict[str, Counter] = {}
    for link in roles["internet_links"]:
        d, t = link["dialer"], link["target"]
        if not link["far"] or d not in neighbours:
            continue
        for n in neighbours[d].get("internet") or []:
            if n["node_id"] not in by_peer:
                votes.setdefault(t, Counter())[n["node_id"]] += 1
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
