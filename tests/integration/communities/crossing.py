#!/usr/bin/env python3
"""Does a message cross between communities, and how long does it take?

Runs identically on v1 and v2, so the two arms can be diffed. Everything
here is a shipped qauld-ctl command and nothing is v2-specific, which is
the point: a metric that only one arm can satisfy cannot compare them.

Two properties, and both are needed. The membrane check in overhead.py
asserts only the first:

  containment   a community must not learn another community's users.
                Under section 2.3 user entries never cross an Internet
                transport, so a member's user list holds its own community
                and nothing else. v1 has no sphere concept and relays the
                full table through the hub, so it should bridge.

  reachability  a message addressed to a user in another community must
                still arrive, via the gateway manifests that section 2.3
                makes the cross-sphere directory.

A membrane that blocks everything passes containment perfectly, so
containment alone cannot tell "working as specified" from "cross-community
messaging is broken". Reachability is the half that distinguishes them.

Addressing: the recipient is invited to a group by user id. Under v2 a
foreign user is never in `users list`, so the discovery-by-listing path
that test_direct_messages.py uses does not exist here; the test supplies
the id out of band, from each node's own account.

A local control pair inside one community runs the same sequence without
crossing a sphere boundary, so a slow crossing can be told apart from a
slow topology.

Usage:
    ./crossing.py --roles topologies/communities-3x10.roles.json --arm v2
    ./crossing.py --roles ... --arm v1 --json ../results/crossing-3x10-v1.json
"""

import argparse
import json
import os
import sys
import time

# lib/ is one level up, shared with the rest of tests/integration.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.node import Node, batch_query

POLL = 1.0


def leg(seconds: float | None) -> str:
    """Format one timed leg. 0.0s is a success, not a falsy miss."""
    return "TIMEOUT" if seconds is None else f"+{seconds}s"


def wait_until(predicate, timeout: float, poll: float = POLL):
    """Seconds until predicate() is truthy, or None on timeout."""
    start = time.time()
    deadline = start + timeout
    while time.time() < deadline:
        try:
            if predicate():
                return round(time.time() - start, 1)
        except Exception:
            # A node mid-convergence can refuse a query; that is not a
            # failure of the thing being timed.
            pass
        time.sleep(poll)
    return None


def collect_user_ids(roles: dict) -> dict:
    """node id -> its own account's user id, hub included.

    The hub hosts an account like any other node. Left out of the universe
    it reads as an unattributable foreign user on whichever node learns it,
    which is the difference between "a gateway knows the hub" and "the
    membrane leaked".
    """
    members = [m for c in roles["communities"] for m in c["members"]]
    nodes = {nid: Node(nid) for nid in members + [roles["hub"]]}
    return batch_query(nodes, "local_user_id")


def containment(roles: dict, user_ids: dict, settle: float = 90,
                poll: float = 10) -> dict:
    """How many users each node knows, split into own community vs foreign.

    Polled until two consecutive reads agree rather than snapshotted, because
    a snapshot taken while the network is still filling reads low on both
    arms. v1 relays its whole table every 10s (sending_table_period), so an
    early read on the bridging arm shows few foreign users and looks exactly
    like a membrane holding. The time to stabilise is itself the answer to
    how fast each arm spreads its directory.
    """
    own_of = {}
    for community in roles["communities"]:
        for member in community["members"]:
            own_of[member] = community["index"]
    # A gateway is *supposed* to know every foreign user: section 2.3 makes the
    # union of the gateway manifests the global directory, and §11.5 fetches a
    # profile for each delegated user it learns. Scoring gateways in the same
    # pool as members makes a working directory read as a membrane breach, so
    # the verdict is taken on members only and gateways are reported as what
    # they are - the directory holders.
    gateway_ids = {c["gateway"] for c in roles["communities"]}

    hub_uid = user_ids.get(roles["hub"])
    by_community: dict[int, set] = {c["index"]: set() for c in roles["communities"]}
    for nid, uid in user_ids.items():
        if nid in own_of:
            by_community[own_of[nid]].add(uid)

    nodes = {nid: Node(nid) for nid in own_of}

    start = time.time()
    deadline = start + settle
    known = batch_query(nodes, "known_user_ids")
    previous = {nid: len(ids) for nid, ids in known.items()}
    stable_at = None
    history = [{"t": 0.0, "total_known": sum(previous.values())}]
    print(f"\n-- waiting for user discovery to settle "
          f"(<= {settle:.0f}s, {poll:.0f}s apart) --")
    print(f"  +0s     {sum(previous.values())} user entries across "
          f"{len(previous)} nodes")
    while time.time() < deadline:
        time.sleep(poll)
        known = batch_query(nodes, "known_user_ids")
        current = {nid: len(ids) for nid, ids in known.items()}
        elapsed = round(time.time() - start, 1)
        total = sum(current.values())
        history.append({"t": elapsed, "total_known": total})
        changed = sum(1 for nid in current if current[nid] != previous.get(nid))
        print(f"  +{elapsed:<6.0f} {total} user entries"
              + (f", {changed} node(s) changed" if changed else ", stable"))
        if not changed:
            stable_at = elapsed
            break
        previous = current
    if stable_at is None:
        print(f"  still changing at {settle:.0f}s; reporting the last read")

    rows = []
    for nid, ids in known.items():
        index = own_of[nid]
        ids = set(ids)
        knows_hub = hub_uid in ids
        foreign = ids - by_community[index] - ({hub_uid} if hub_uid else set())
        rows.append({
            "node": nid,
            "community": index,
            "known_total": len(ids),
            "known_own": len(ids & by_community[index]),
            "knows_hub": knows_hub,
            "known_foreign": len(foreign),
        })

    rows.sort(key=lambda r: (r["community"], r["node"]))
    for row in rows:
        row["is_gateway"] = row["node"] in gateway_ids
    member_rows = [r for r in rows if not r["is_gateway"]]
    gateway_rows = [r for r in rows if r["is_gateway"]]
    worst = max(member_rows or rows, key=lambda r: r["known_foreign"])
    expected_own = roles["members_per_community"]
    expected_foreign = (roles["community_count"] - 1) * expected_own

    print("\n-- containment (users known per node) --")
    print(f"  {'community':>9} {'nodes':>6} {'own/' + str(expected_own):>12} "
          f"{'foreign/' + str(expected_foreign):>16}")
    for community in roles["communities"]:
        index = community["index"]
        group = [r for r in rows if r["community"] == index]
        own = [r["known_own"] for r in group]
        foreign = [r["known_foreign"] for r in group]
        print(f"  {index:>9} {len(group):>6} "
              f"{f'{min(own)}-{max(own)}':>12} {f'{min(foreign)}-{max(foreign)}':>16}")
    hub_knowers = [r["node"] for r in rows if r["knows_hub"]]
    if gateway_rows:
        gw_foreign = [r["known_foreign"] for r in gateway_rows]
        complete = sum(1 for f in gw_foreign if f >= expected_foreign)
        print(f"  gateways ({len(gateway_rows)}): {min(gw_foreign)}-{max(gw_foreign)}"
              f" of {expected_foreign} foreign users — {complete}/"
              f"{len(gateway_rows)} hold a complete directory")
        print("    expected: a gateway holds the global directory (§2.3), so"
              " this should be full.\n    It is excluded from the containment"
              " verdict, which is about members.")
    print(f"  worst member {worst['node']} knows {worst['known_foreign']} "
          f"foreign user(s) of {expected_foreign} possible")
    if hub_knowers:
        print(f"  hub's own user known by {len(hub_knowers)} node(s): "
              f"{sorted(hub_knowers)[:6]}")
        print("    counted separately: the hub is infrastructure, not a"
              " community member. A gateway holding it over an Internet"
              " transport is the 2.3 origin-path leak, not bridging.")

    return {
        "expected_own": expected_own,
        "expected_foreign_if_bridged": expected_foreign,
        "max_foreign_known": worst["known_foreign"],
        "gateway_foreign_known": {r["node"]: r["known_foreign"]
                                  for r in gateway_rows},
        "gateways_with_complete_directory": sum(
            1 for r in gateway_rows if r["known_foreign"] >= expected_foreign),
        "foreign_fraction": (worst["known_foreign"] / expected_foreign
                             if expected_foreign else 0.0),
        "hub_user_known_by": hub_knowers,
        # A single stray entry and a fully relayed directory are different
        # faults, so the threshold separates them rather than collapsing
        # both into one boolean.
        "bridged": worst["known_foreign"] >= max(2, expected_foreign // 2),
        "leaking": 0 < worst["known_foreign"] < max(2, expected_foreign // 2),
        "settled_after_s": stable_at,
        "discovery_history": history,
        "per_node": rows,
    }


def pick_pair(roles: dict) -> tuple[str, str, str | None, str]:
    """(sender, cross recipient, local control recipient or None, label).

    The sender is the community's last member, which is the lattice corner
    furthest from the gateway, so the message traverses the community as
    well as the membrane. Neither endpoint is a gateway: a gateway-to-
    gateway hop would skip the part of the path that is in question.

    With one member per community that is unavoidable - the only member is
    the gateway - and there is no second node to use as a local control.
    """
    if roles["community_count"] < 2:
        raise SystemExit(
            f"{roles['name']} has one community, so there is no boundary to "
            "cross. Use a topology with --communities >= 2."
        )

    first, last = roles["communities"][0], roles["communities"][-1]
    sender = first["members"][-1]
    cross = last["members"][-1]
    local = first["members"][1] if len(first["members"]) > 1 else None
    label = f"{sender} (c{first['index']}) -> {cross} (c{last['index']})"
    return sender, cross, local, label


def exchange(sender_id: str, recipient_id: str, user_ids: dict,
             timeout: float, tag: str) -> dict:
    """Group invite, accept, and one message. Each leg timed separately.

    Timed in legs because a failure localises: no invite means the routing
    layer never carried it, an invite that lands but never accepts means
    the reverse path is missing, and a delivered accept with no message
    means chat forwarding is the problem.
    """
    sender, recipient = Node(sender_id), Node(recipient_id)
    recipient_uid = user_ids[recipient_id]
    name = f"crossing-{tag}-{int(time.time())}"

    result: dict = {"tag": tag, "sender": sender_id, "recipient": recipient_id,
                    "group": name}
    print(f"\n-- {tag}: {sender_id} -> {recipient_id} --")

    group_id = sender.create_group(name)
    result["group_id"] = group_id
    print(f"  group {group_id}")

    sender.invite_to_group(group_id, recipient_uid)
    t_invite = wait_until(
        lambda: any(i.get("group_id") == group_id or i.get("id") == group_id
                    for i in recipient.invitations()),
        timeout)
    result["invite_s"] = t_invite
    print(f"  invite      {leg(t_invite)}")
    if t_invite is None:
        result["failed_at"] = "invite"
        return result

    recipient.accept_invitation(group_id)
    # An invited member is already listed, with state Invited (group_rpc.proto:
    # Invited = 0, Activated = 1), so a member count cannot distinguish an
    # invite that arrived from one that was accepted.
    t_accept = wait_until(
        lambda: any(m.get("user_id") == recipient_uid
                    and m.get("state") == "Activated"
                    for m in sender.group_info(group_id).get("members", [])),
        timeout)
    result["accept_s"] = t_accept
    print(f"  accept      {leg(t_accept)}")
    if t_accept is None:
        result["failed_at"] = "accept"
        return result

    content = f"{name}-payload"
    sender.send_chat_message(group_id, content)
    t_deliver = wait_until(
        lambda: content in [
            c for msg in recipient.conversation(group_id).get("messages", [])
            for c in (msg.get("content") or [])
        ],
        timeout)
    result["deliver_s"] = t_deliver
    print(f"  deliver     {leg(t_deliver)}")
    if t_deliver is None:
        result["failed_at"] = "deliver"
        return result

    total = round(t_invite + t_accept + t_deliver, 1)
    result["total_s"] = total
    print(f"  total       {total}s")
    return result


def diagnose(roles: dict, sender_id: str, target_id: str,
             user_ids: dict, arm: str) -> dict:
    """What the sender and the gateways actually hold, after a failed crossing.

    Section 2.3 routes cross-sphere traffic through the gateway manifests:
    "the union of all gateway manifests is the global directory of which user
    is delegated to which gateway". So the question a failure raises is
    whether that directory exists and whether the sender can reach it.
    """
    target_uid = user_ids[target_id]
    out: dict = {"sender": sender_id, "target": target_id, "target_user": target_uid}
    print(f"\n-- diagnosis: why {sender_id} could not reach {target_id} --")

    sender = Node(sender_id)
    known = set(sender.known_user_ids())
    out["sender_knows_target_user"] = target_uid in known
    out["sender_known_users"] = len(known)
    print(f"  sender knows {len(known)} user(s); target "
          f"{'IS' if target_uid in known else 'is NOT'} among them")

    if arm == "v2":
        for community in roles["communities"]:
            gateway = community["gateway"]
            try:
                manifests = Node(gateway).router_v2_manifests()
            except Exception as exc:
                print(f"  gateway {gateway}: manifests unavailable ({exc})")
                out.setdefault("gateways", {})[gateway] = {"error": str(exc)}
                continue
            count = len(manifests) if isinstance(manifests, list) else len(
                manifests.get("manifests", []) or [])
            out.setdefault("gateways", {})[gateway] = {"manifest_count": count}
            print(f"  gateway {gateway}: holds {count} foreign manifest(s)")
        print("  if the gateways hold each other's manifests but a member"
              " cannot send,\n  the directory exists and the send path does"
              " not consult it.")
        print("  if they hold none, the manifests never crossed and the"
              " directory was never built.")
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--arm", choices=("v1", "v2"), required=True,
                    help="label only; which arm is running is set at bringup")
    ap.add_argument("--timeout", type=float, default=120)
    ap.add_argument("--json", dest="json_out", default=None)
    ap.add_argument("--skip-control", action="store_true")
    ap.add_argument("--settle", type=float, default=90,
                    help="max seconds to wait for user discovery to stop "
                         "changing before reading containment (default 90)")
    args = ap.parse_args()

    with open(args.roles) as f:
        roles = json.load(f)

    print(f"=== {roles['name']} — arm {args.arm} ===")
    print(f"{roles['community_count']} communities x {roles['members_per_community']} "
          f"members, {roles['totals']['nodes']} nodes")

    print("\nresolving each node's own user id")
    user_ids = collect_user_ids(roles)
    print(f"  {len(user_ids)} accounts")

    report = {
        "arm": args.arm,
        "topology": roles["name"],
        "communities": roles["community_count"],
        "members_per_community": roles["members_per_community"],
        "timeout_s": args.timeout,
    }
    report["containment"] = containment(roles, user_ids, settle=args.settle)

    sender, cross, local, label = pick_pair(roles)
    print(f"\npath under test: {label}")

    if local is None:
        print("  (one member per community: both endpoints are gateways and"
              "\n   there is no second local node, so no control pair)")
    elif not args.skip_control:
        report["local"] = exchange(sender, local, user_ids, args.timeout,
                                   "local control")
    report["cross"] = exchange(sender, cross, user_ids, args.timeout, "cross-community")
    if report["cross"].get("deliver_s") is None:
        report["diagnosis"] = diagnose(roles, sender, cross, user_ids, args.arm)

    print("\n-- verdict --")
    cont = report["containment"]
    bridged, leaking = cont["bridged"], cont["leaking"]
    contained = not bridged and not leaking
    crossed = report["cross"].get("deliver_s") is not None
    state = "held" if contained else ("LEAKING" if leaking else "BRIDGED")
    print(f"  containment   {state} ({cont['max_foreign_known']}/"
          f"{cont['expected_foreign_if_bridged']} foreign users, "
          f"{cont['foreign_fraction'] * 100:.0f}%)")
    print(f"  reachability  {'ok' if crossed else 'FAILED at ' + report['cross'].get('failed_at', '?')}")

    if contained and crossed:
        print("  -> isolated and still reachable: section 2.3 as specified")
    elif leaking and crossed:
        print("  -> reachable, with a membrane that leaks a few entries")
    elif bridged and crossed:
        print("  -> reachable by bridging: no membrane (expected for v1)")
    elif not crossed:
        print("  -> UNREACHABLE: a message addressed to another community"
              f" never arrived (gave up at {args.timeout:.0f}s).")
        print("     Containment without reachability is not section 2.3"
              " working; it is cross-community messaging not working.")
    report["verdict"] = {"contained": contained, "leaking": leaking,
                         "bridged": bridged, "reachable": crossed}

    if args.json_out:
        parent = os.path.dirname(args.json_out)
        if parent:
            os.makedirs(parent, exist_ok=True)
        with open(args.json_out, "w") as f:
            json.dump(report, f, indent=1)
        print(f"\nwrote {args.json_out}")

    if not crossed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
