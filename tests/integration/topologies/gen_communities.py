#!/usr/bin/env python3
"""Generate a community/hub topology for the protocol-overhead tests.

Shape:

    community 0              community 1          ...
    +------------+
    | m  m  m  m |
    | m  m  m  m |           (same)
    | g          |             g
    +--|---------+             |
       +--------- INTERNET --> [ hub ] <-- ...

Every community is a degree-<=4 lattice, filled row-major into a
ceil(sqrt(N))-wide grid, so no member ever needs more than the 4
simultaneous connections a BLE radio can hold.

The gateway is the lattice origin, which has at most 2 community links, so
its link to the hub leaves it at 3 - still inside the BLE budget. An
interior gateway would already sit at 4 and the hub link would make the
community unbuildable on a radio, which is why the corner is not just a
tidy default.

The hub is node 0000 and is the only node whose degree is unbounded: it is
an internet relay, not a radio peer, so the 4-connection cap does not
apply to it.

Two files are written:
  <name>.json        meshnet-lab topology (nodes + links)
  <name>.roles.json  which id is the hub, which are gateways, and the
                     per-community member lists - so the runner and the
                     reducer never infer roles from the shape.

Usage:
    ./gen_communities.py --communities 3 --members 10
    ./gen_communities.py -c 10 -m 10 --name communities-10x10
"""

import argparse
import json
import math
import os
from collections import deque

# A BLE radio holds at most 4 simultaneous connections.
BLE_MAX_LINKS = 4


def ceil_sqrt(n: int) -> int:
    """Smallest w with w*w >= n."""
    return math.isqrt(n - 1) + 1 if n > 1 else 1


def lattice(n: int) -> tuple[int, list[tuple[int, int]]]:
    """Row-major degree-<=4 lattice over n nodes.

    Returns (width, links) with links as local-index pairs. A partial last
    row is normal: the right/down neighbour is simply absent.
    """
    w = ceil_sqrt(n)
    links: list[tuple[int, int]] = []
    for p in range(n):
        if (p % w) + 1 < w and p + 1 < n:
            links.append((p, p + 1))
        if p + w < n:
            links.append((p, p + w))
    return w, links


def degrees(n: int, links: list[tuple[int, int]]) -> list[int]:
    d = [0] * n
    for a, b in links:
        d[a] += 1
        d[b] += 1
    return d


def diameter(n: int, links: list[tuple[int, int]]) -> int:
    """Exact diameter by BFS from every node.

    Computed rather than derived from the lattice dimensions, because a
    partial last row shifts it.
    """
    if n <= 1:
        return 0
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
        if len(seen) != n:
            raise AssertionError("lattice is disconnected - generator bug")
        worst = max(worst, max(seen.values()))
    return worst


def build(communities: int, members: int, name: str) -> tuple[dict, dict]:
    nodes: list[dict] = []
    links: list[dict] = []
    hub_links: list[dict] = []
    next_index = 0

    def new_id() -> str:
        nonlocal next_index
        node_id = f"{next_index:04x}"
        next_index += 1
        return node_id

    hub = new_id()
    nodes.append({"id": hub, "x": 0, "y": 0})

    community_records = []
    x_cursor = 0
    community_link_count = 0

    for c in range(communities):
        width, local_links = lattice(members)
        local_degrees = degrees(members, local_links)

        ids = [new_id() for _ in range(members)]
        for p, node_id in enumerate(ids):
            row, col = divmod(p, width)
            nodes.append({"id": node_id, "x": x_cursor + col, "y": 3 + row})

        for a, b in local_links:
            links.append({"source": ids[a], "target": ids[b]})
        community_link_count += len(local_links)

        gateway = ids[0]
        hub_links.append({"source": hub, "target": gateway})

        member_max = max(local_degrees) if members else 0
        if member_max > BLE_MAX_LINKS:
            raise AssertionError(
                f"community {c}: a member needs {member_max} links, "
                f"BLE allows {BLE_MAX_LINKS}"
            )
        gateway_total = local_degrees[0] + 1 if members else 1
        if gateway_total > BLE_MAX_LINKS:
            raise AssertionError(
                f"community {c}: gateway needs {gateway_total} links "
                f"(including the hub), BLE allows {BLE_MAX_LINKS}"
            )

        community_records.append(
            {
                "index": c,
                "gateway": gateway,
                "members": ids,
                "lattice_width": width,
                "links": len(local_links),
                "max_member_degree": member_max,
                "gateway_degree_with_hub": gateway_total,
                "diameter": diameter(members, local_links),
            }
        )
        x_cursor += width + 2

    # Stage 1 omits the hub's links. The hub boots with no neighbour at
    # all, so it cannot form a LAN neighbourship and cannot bridge the
    # communities while they converge; its LAN is switched off before the
    # links are added in stage 2. `network.py apply` is incremental
    # (_get_task diffs the states), so stage 2 only creates those links and
    # leaves every running daemon alone.
    stage1 = {"nodes": nodes, "links": list(links)}
    topology = {"nodes": nodes, "links": links + hub_links}
    roles = {
        "name": name,
        "hub": hub,
        "community_count": communities,
        "members_per_community": members,
        "communities": community_records,
        "totals": {
            "nodes": len(nodes),
            "links": len(topology["links"]),
            "community_links": community_link_count,
            "hub_links": communities,
        },
    }
    return topology, stage1, roles


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("-c", "--communities", type=int, required=True)
    ap.add_argument("-m", "--members", type=int, required=True)
    ap.add_argument("--name", default=None, help="base filename (no extension)")
    ap.add_argument("--out-dir", default=os.path.dirname(os.path.abspath(__file__)))
    args = ap.parse_args()

    if args.communities < 1 or args.members < 1:
        ap.error("--communities and --members must both be >= 1")

    name = args.name or f"communities-{args.communities}x{args.members}"
    topology, stage1, roles = build(args.communities, args.members, name)

    topo_path = os.path.join(args.out_dir, f"{name}.json")
    stage1_path = os.path.join(args.out_dir, f"{name}-stage1.json")
    roles_path = os.path.join(args.out_dir, f"{name}.roles.json")
    with open(topo_path, "w") as f:
        json.dump(topology, f)
    with open(stage1_path, "w") as f:
        json.dump(stage1, f)
    with open(roles_path, "w") as f:
        json.dump(roles, f, indent=1)

    first = roles["communities"][0]
    print(f"{name}: {roles['totals']['nodes']} nodes, {roles['totals']['links']} links")
    print(f"  hub                  {roles['hub']} (degree {args.communities}, LAN to be disabled)")
    print(f"  communities          {args.communities} x {args.members} members")
    print(f"  per community        {first['links']} links, "
          f"{first['lattice_width']}-wide lattice, diameter {first['diameter']}")
    print(f"  max member degree    {first['max_member_degree']} (BLE cap {BLE_MAX_LINKS})")
    print(f"  gateway degree       {first['gateway_degree_with_hub']} including the hub link")
    print(f"  community links      {roles['totals']['community_links']}")
    print(f"  hub links            {roles['totals']['hub_links']}")
    print(f"wrote {topo_path}       (full: community + hub links)")
    print(f"wrote {stage1_path}  (stage 1: no hub links)")
    print(f"wrote {roles_path}")


if __name__ == "__main__":
    main()
