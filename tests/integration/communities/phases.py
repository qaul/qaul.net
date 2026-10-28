#!/usr/bin/env python3
"""Wire a generated community/hub topology into its two spheres.

Everything here is a shipped qauld-ctl command. No config file is edited
and no port is pinned, so the lab node runs exactly the binary and the
defaults a real deployment would.

Runs in two phases, because the order matters. If the hub boots with its
links already present, it forms LAN neighbourships with every gateway and
bridges the communities into one flat Local sphere before anything can
switch LAN off - and those entries then live until the 35 s route expiry
(route_expiry_ms, configuration.rs). Nothing in the membrane stops
Local->Local propagation; it only gates Local->Internet. So the hub must
never have a LAN neighbour at all:

  phase 1  (topology: <name>-stage1.json, hub has no links)
     hub: `transports disable lan`, then assert the hub has no LAN
     neighbour and an empty routing table - it cannot have learned
     anything, because it has no links yet.

  phase 2  (topology: <name>.json, hub links now added)
     read the hub's resolved multiaddress from `node info` (the internet
     listener binds an ephemeral port, so this is discovered at runtime
     rather than pinned in config), then
     `connections nodes add --address <hub> --name hub` on each gateway -
     libqaul stores the peer enabled and dials it immediately - and
     verify every gateway reports the gateway role, which is derived from
     having an active internet transport and never configured.

Usage:
    ./phases.py --roles ... --phase 1
    ./phases.py --roles ... --phase 2 [--dump-tables]
    ./phases.py --roles ... --verify-only
"""

import argparse
import json
import os
import sys
import time

# lib/ is one level up, shared with the rest of tests/integration.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.node import Node


def pick_hub_address(addresses: list[str]) -> tuple[str | None, list[str]]:
    """Choose the multiaddress a gateway can actually dial.

    Drops loopback and wildcard, prefers QUIC over TCP, and prefers the
    lab's 10.0.0.0/8 uplink address over anything else.
    """
    usable = [
        a
        for a in addresses
        if "/ip4/" in a
        and "/ip4/127." not in a
        and "/ip4/0.0.0.0" not in a
    ]

    def rank(address: str) -> tuple[int, int]:
        lab = 0 if address.startswith("/ip4/10.") else 1
        quic = 0 if "quic" in address else 1
        return (lab, quic)

    usable.sort(key=rank)
    return (usable[0] if usable else None), usable


def gateway_role(status: dict) -> bool | None:
    """The gateway flag out of `router v2 status`, whatever it is called.

    Looked up rather than hardcoded so a field rename shows up as
    "unknown" instead of a silently wrong pass.
    """
    for key, value in status.items():
        if "gateway" in key.lower() and isinstance(value, bool):
            return value
    return None


def verify(roles: dict, quiet: bool = False, v1: bool = False) -> bool:
    hub_id = roles["hub"]
    gateways = [c["gateway"] for c in roles["communities"]]
    ok = True

    hub_neighbours = Node(hub_id).router_neighbours()
    internet_neighbours = hub_neighbours.get("internet") or []
    expected = roles["community_count"]
    if len(internet_neighbours) != expected:
        ok = False
    if not quiet:
        mark = "ok" if len(internet_neighbours) == expected else "MISMATCH"
        print(f"  hub {hub_id}: {len(internet_neighbours)}/{expected} "
              f"internet neighbours [{mark}]")

    # is_gateway is a v2 concept (sync_gateway_role). v1 has no sphere model
    # and so no gateway role to assert: on that arm the internet neighbour
    # count above is the whole check.
    if v1:
        if not quiet:
            print("  gateway role: n/a on v1 (no sphere model)")
        return ok

    unknown_field = False
    for gateway in gateways:
        status = Node(gateway).router_v2_status()
        role = gateway_role(status)
        if role is None:
            unknown_field = True
            ok = False
            if not quiet:
                print(f"  gateway {gateway}: no boolean gateway field in status")
                print(f"    raw: {json.dumps(status)[:300]}")
        else:
            if not role:
                ok = False
            if not quiet:
                print(f"  gateway {gateway}: is_gateway={role} "
                      f"[{'ok' if role else 'NOT A GATEWAY'}]")

    if unknown_field and not quiet:
        print("  note: the gateway flag is derived from an active internet"
              " transport (sync_gateway_role), so a false reading means the"
              " peering never connected.")
    return ok


def dump_tables(roles: dict, v1: bool = False) -> None:
    """Print the hub's v2 table - the direct membrane check.

    No user entry may cross to the Internet sphere, so the hub's user
    space must be empty and its node space must hold gateways only.
    """
    hub_id = roles["hub"]
    if v1:
        print(f"\n-- hub {hub_id} router table (v1) --")
        print(json.dumps(Node(hub_id).router_table(), indent=1)[:2000])
        return
    print(f"\n-- hub {hub_id} router v2 table (membrane check) --")
    print(json.dumps(Node(hub_id).router_v2_table(), indent=1)[:2000])


def phase1(roles: dict, v1: bool = False) -> bool:
    """Switch the hub's LAN off while it still has no links.

    Asserting the table is empty here is the whole point: it proves the
    hub never bridged, rather than hoping stale entries expired later.
    """
    hub_id = roles["hub"]
    hub = Node(hub_id)

    print(f"hub {hub_id}: disabling LAN")
    hub.disable_transport("lan")

    neighbours = hub.router_neighbours()
    lan = neighbours.get("lan") or []
    # `router table` is the v1 view and answers on both arms; the v2 table
    # additionally tags entries with their index space, so user entries can
    # be counted exactly rather than inferred.
    if v1:
        users = hub.router_table()
    else:
        users = [e for e in hub.router_v2_table() if e.get("space") == "user"]

    ok = not lan and not users
    print(f"  LAN neighbours   {len(lan)} [{'ok' if not lan else 'SHOULD BE 0'}]")
    print(f"  user entries     {len(users)} [{'ok' if not users else 'ALREADY BRIDGED'}]")
    if users:
        print("  the hub already learned users over LAN, so it has bridged the")
        print("  communities. Re-apply the stage-1 topology and bring up again;")
        print("  these entries only clear after the 35s route expiry.")
    else:
        print("\nphase 1 done. now add the hub links and run phase 2:")
        print(f"  sudo $MESHNET_LAB/network.py apply "
              f"topologies/{roles['name']}.json")
        print(f"  ./phases.py --roles ... --phase 2")
    return ok


def setup(roles: dict, settle: int, v1: bool = False,
          reverse: bool = False) -> bool:
    hub_id = roles["hub"]
    hub = Node(hub_id)
    gateways = [c["gateway"] for c in roles["communities"]]
    # The first gateway to dial is the one whose node binding the hub loses to
    # the reserved-index sweep, so dial order is the independent variable when
    # testing that claim.
    if reverse:
        gateways = list(reversed(gateways))

    # Phase 1 already switched LAN off. Re-asserted here because a phase-2
    # run against a hub that still has LAN up would measure a bridge.
    lan = (hub.router_neighbours().get("lan") or [])
    if lan:
        raise SystemExit(
            f"hub {hub_id} has {len(lan)} LAN neighbour(s): it is bridging the "
            "communities. Run phase 1 on the stage-1 topology first."
        )

    address, candidates = pick_hub_address(hub.addresses())
    if not address:
        raise SystemExit(
            f"hub {hub_id} advertises no dialable ip4 address; saw: {candidates}"
        )
    print(f"hub {hub_id}: dialable at {address}")
    if len(candidates) > 1:
        print(f"  other candidates: {candidates[1:]}")

    print(f"dial order: {' -> '.join(gateways)}"
          + ("  (reversed)" if reverse else ""))
    for position, gateway in enumerate(gateways):
        Node(gateway).add_internet_peer(address, "hub")
        print(f"gateway {gateway}: peered to hub"
              + ("   <- dialed first" if position == 0 else ""))

    print(f"waiting {settle}s for the internet dials to settle")
    time.sleep(settle)

    print("verifying:")
    return verify(roles, v1=v1)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--settle", type=int, default=15,
                    help="seconds to wait for internet dials (default 15)")
    ap.add_argument("--phase", type=int, choices=(1, 2), default=2)
    ap.add_argument("--verify-only", action="store_true")
    ap.add_argument("--dump-tables", action="store_true")
    ap.add_argument("--reverse", action="store_true",
                    help="dial the gateways in reverse order (phase 2)")
    ap.add_argument("--v1", action="store_true",
                    help="v1 arm: skip the v2-only gateway-role assertion and "
                         "read the v1 routing table")
    args = ap.parse_args()

    with open(args.roles) as f:
        roles = json.load(f)

    if args.verify_only:
        print("verifying:")
        ok = verify(roles, v1=args.v1)
    elif args.phase == 1:
        ok = phase1(roles, v1=args.v1)
    else:
        ok = setup(roles, args.settle, v1=args.v1, reverse=args.reverse)

    if args.dump_tables:
        dump_tables(roles, v1=args.v1)

    if not ok:
        raise SystemExit("setup incomplete - see above")
    if not args.verify_only and args.phase == 2:
        print("\nspheres wired: communities on Local, hub on Internet")


if __name__ == "__main__":
    main()
