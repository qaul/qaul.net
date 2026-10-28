#!/usr/bin/env python3
"""Classify the manifest origins each node tracks, by sphere and role.

`router v2 manifests` returns one row per origin whose manifest this node
tracks, carrying `learn_sphere`, `is_gateway` and `delegated_users`
(router.rs:403-414). Counting the rows says nothing; the question is which
sphere they were learned over.

Section 2.3 makes the gateway manifests the cross-sphere directory: "the
union of all gateway manifests is the global directory of which user is
delegated to which gateway". So a gateway that cannot deliver to another
community is one of two faults, and these fields separate them:

  origins learned over Internet, is_gateway, with delegated_users > 0
      the directory exists; the send path is not consulting it

  origins only ever learned over Local
      nothing crossed into the directory; it was never built

Read-only, so it is safe against a running lab.

Usage:
    ./manifests.py --roles topologies/communities-3x10.roles.json
    ./manifests.py --roles ... --raw 0001
"""

import argparse
import json
import os
import sys
from collections import Counter

# lib/ is one level up, shared with the rest of tests/integration.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.node import Node


def origins_of(payload) -> list[dict]:
    if isinstance(payload, list):
        return payload
    for key in ("origins", "manifests"):
        value = payload.get(key)
        if isinstance(value, list):
            return value
    return []


def describe(node_id: str, label: str) -> dict:
    try:
        payload = Node(node_id).router_v2_manifests()
    except Exception as exc:
        print(f"  {node_id} ({label}): unavailable - {exc}")
        return {"node": node_id, "role": label, "error": str(exc)}

    origins = origins_of(payload)
    spheres = Counter(str(o.get("learn_sphere")) for o in origins)
    gateways = [o for o in origins if o.get("is_gateway")]
    delegating = [o for o in origins if (o.get("delegated_users") or 0) > 0]
    signed = [o for o in origins if o.get("has_signature")]
    users = sum(o.get("delegated_users") or 0 for o in origins)
    trusted = sum(o.get("trusted_users") or 0 for o in origins)

    print(f"  {node_id} ({label}): {len(origins)} origin(s)")
    print(f"      by sphere      {dict(spheres)}")
    print(f"      is_gateway     {len(gateways)}")
    print(f"      delegating     {len(delegating)} origin(s), "
          f"{users} delegated / {trusted} trusted user(s)")
    print(f"      signed         {len(signed)}")
    cross = [o for o in origins if "internet" in str(o.get("learn_sphere")).lower()]
    if cross:
        print(f"      learned over Internet: {len(cross)} -> "
              + ", ".join(f"gw={bool(o.get('is_gateway'))}/"
                          f"users={o.get('delegated_users')}" for o in cross[:5]))
    return {
        "node": node_id, "role": label, "origins": len(origins),
        "by_sphere": dict(spheres), "gateway_origins": len(gateways),
        "delegating_origins": len(delegating), "delegated_users": users,
        "trusted_users": trusted, "signed": len(signed),
        "internet_learned": len(cross),
    }


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--raw", default=None, help="dump one node's rows verbatim")
    ap.add_argument("--json", dest="json_out", default=None)
    args = ap.parse_args()

    with open(args.roles) as f:
        roles = json.load(f)

    if args.raw:
        print(json.dumps(Node(args.raw).router_v2_manifests(), indent=1)[:4000])
        return

    rows = []
    print(f"=== {roles['name']} manifest origins ===")
    print("\nhub:")
    rows.append(describe(roles["hub"], "hub"))

    print("\ngateways:")
    for community in roles["communities"]:
        rows.append(describe(community["gateway"], f"gateway c{community['index']}"))

    print("\none non-gateway member per community (the senders that must"
          "\nreach another community):")
    for community in roles["communities"]:
        members = [m for m in community["members"] if m != community["gateway"]]
        if members:
            rows.append(describe(members[-1], f"member c{community['index']}"))

    # Name the missing gateway instead of counting rows. `router v2 status`
    # carries each node's own 8-byte node_id, so the hub's node space can be
    # matched against the gateways by identity. Manifest counts cannot do this
    # job: they move while the directory is still assembling, so a count taken
    # early reads as a defect.
    gateways = [c["gateway"] for c in roles["communities"]]
    identity, convergence = {}, {}
    for gateway in gateways:
        try:
            status = Node(gateway).router_v2_status()
            identity[gateway] = status.get("node_id")
            convergence[gateway] = {
                "manifest_version": status.get("manifest_version"),
                "manifest_entries": status.get("manifest_entries"),
                "pending_node_intros": status.get("pending_node_intros"),
                "pending_user_intros": status.get("pending_user_intros"),
                "node_entries": status.get("node_entries"),
            }
        except Exception as exc:
            identity[gateway] = None
            convergence[gateway] = {"error": str(exc)}

    print("\n-- gateway convergence (is the directory still assembling?) --")
    print(f"  {'node':>6} {'manifest':>10} {'entries':>8} {'node_ent':>9} "
          f"{'pend_node':>10} {'pend_user':>10}")
    for gateway in gateways:
        c = convergence[gateway]
        if "error" in c:
            print(f"  {gateway:>6}  unavailable: {c['error']}")
            continue
        print(f"  {gateway:>6} {'v' + str(c['manifest_version']):>10} "
              f"{c['manifest_entries']:>8} {c['node_entries']:>9} "
              f"{c['pending_node_intros']:>10} {c['pending_user_intros']:>10}")
    expected_entries = roles["members_per_community"]
    unconverged = [g for g in gateways
                   if (convergence[g].get("manifest_entries") or 0) < expected_entries]
    if unconverged:
        print(f"  {len(unconverged)} gateway(s) have fewer than {expected_entries}"
              f" manifest entries: {unconverged}")
        print("  the directory is still assembling, so anything below is a"
              " snapshot of a\n  moving system, not a steady state.")

    hub_nodes = {"expected": len(gateways)}
    try:
        table = Node(roles["hub"]).router_v2_table()
        node_rows = [e for e in table
                     if str(e.get("space")).lower() == "node"]
        # The field is target_id (router.rs:318). Reading a key that is not
        # there yields None for every row, which collapses to a single
        # "None" and reports every gateway missing - a confident wrong
        # answer, so an absent key is an error rather than a verdict.
        if node_rows and any(e.get("target_id") is None for e in node_rows):
            raise RuntimeError(
                "node-space rows have no target_id; the table schema changed. "
                f"keys seen: {sorted(node_rows[0].keys())}")
        targets = {e["target_id"] for e in node_rows}
        print("\n-- hub node space (the defect itself) --")
        print(f"  {len(targets)} node entry(ies), expected {len(gateways)}")
        missing, present = [], []
        for gateway in gateways:
            node_id = identity.get(gateway)
            if node_id is None:
                print(f"  {gateway}: identity unknown, cannot match")
                continue
            if node_id in targets:
                present.append(gateway)
                print(f"  {gateway} ({node_id}): present [ok]")
            else:
                missing.append(gateway)
                print(f"  {gateway} ({node_id}): MISSING from the hub's node space")
        hub_nodes.update({"found": len(targets), "present": present,
                          "missing": missing, "targets": sorted(targets)})
        if missing:
            print(f"\n  => invisible to every other gateway: "
                  f"{', '.join(missing)}")
            print("     The hub cannot relay a node it has no entry for, so no"
                  " other gateway\n     can route a reply to it, and its §11.5"
                  " key fetches cannot complete.")
        else:
            print("\n  => every gateway is present; the node-space defect did"
                  " not reproduce")
    except Exception as exc:
        print(f"\n-- hub node space -- unavailable: {exc}")
        hub_nodes["error"] = str(exc)

    internet_aware = [r for r in rows if r.get("internet_learned")]
    print("\n-- reading --")
    if not internet_aware:
        print("  No node tracks a manifest origin learned over the Internet"
              " sphere.\n  The cross-sphere directory was never built, so no"
              " node can know\n  which gateway a foreign user sits behind.")
    else:
        names = ", ".join(r["node"] for r in internet_aware)
        print(f"  Internet-learned origins present on: {names}")
        members = [r for r in internet_aware if r["role"].startswith("member")]
        if not members:
            print("  ...but on no ordinary member. If a member cannot address a"
                  " foreign\n  user, the directory exists only on the gateways"
                  " and the send path\n  does not forward to them.")

    summary = {"hub_node_space": hub_nodes, "gateway_convergence": convergence,
               "gateway_identity": identity, "origins": rows}
    if args.json_out:
        parent = os.path.dirname(args.json_out)
        if parent:
            os.makedirs(parent, exist_ok=True)
        with open(args.json_out, "w") as f:
            json.dump(summary, f, indent=1)
        print(f"\nwrote {args.json_out}")


if __name__ == "__main__":
    main()
