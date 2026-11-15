#!/usr/bin/env python3
"""Faults applied from the laptop, as root: cut and restore links, egress, nodes.

Nothing here touches the server. Every fault adds a rule and is undone by
deleting exactly that rule, so `clear` always gets the lab back.

  link     drop everything on a meshnet-lab link, both ends, in both
           directions: a tc matchall drop at preference 1 on ve-A-B and
           ve-B-A in the "switch" namespace. link_filter.sh's mDNS rules
           use the automatic preferences (49152 downwards), so they are
           never touched. Works for radio links and lab Internet links.
  node     drop everything on a node's port into its bridge (dl-ID): the
           person walks out of range of everyone at once. qauld keeps
           running and keeps its identity, so coming back is a return, not
           a new node.
  egress   (hybrid) drop everything on a far gateway's egress link to the
           server, in the host's qaul_egress table: its mobile data drops.
           The rules go at the top of the forward chain, ahead of the rule
           that lets the server's replies back in.

Packets vanish silently in all three, as when a radio link fades or mobile
data drops: nothing tells qaul, which has to notice through its own pings.

Usage, as root, with names or lab ids from the roles file:
  faults.py --roles R cut-link X.04 Y.02      faults.py --roles R restore-link X.04 Y.02
  faults.py --roles R isolate T.07 V.03       faults.py --roles R rejoin T.07 V.03
  faults.py --roles R cut-egress T1 [...]     faults.py --roles R restore-egress T1 [...]
  faults.py --roles R clear                   faults.py --roles R status
"""

import argparse
import json
import re
import subprocess
import sys

PREF = "1"
TABLE = "qaul_egress"
CHAIN = "forward_q"
TAG = "qaul-fault"


def run(cmd: list[str], check: bool = True) -> str:
    res = subprocess.run(cmd, capture_output=True, text=True)
    if check and res.returncode != 0:
        raise RuntimeError(f"{' '.join(cmd)}: {res.stderr.strip()}")
    return res.stdout


def sw(*cmd: str, check: bool = True) -> str:
    return run(["ip", "netns", "exec", "switch", *cmd], check=check)


def switch_devices() -> list[str]:
    out = sw("ip", "-o", "link", "show")
    return [m.group(1) for m in re.finditer(r"^\d+:\s+([^:@\s]+)", out, re.M)]


def _drop(dev: str) -> None:
    sw("tc", "qdisc", "add", "dev", dev, "clsact", check=False)  # may exist already
    for hook in ("ingress", "egress"):
        sw("tc", "filter", "del", "dev", dev, hook, "pref", PREF, check=False)
        sw("tc", "filter", "add", "dev", dev, hook, "pref", PREF, "matchall", "action", "drop")


def _undrop(dev: str) -> None:
    for hook in ("ingress", "egress"):
        sw("tc", "filter", "del", "dev", dev, hook, "pref", PREF, check=False)


def _has_drop(dev: str) -> bool:
    return any(f"pref {PREF} matchall" in sw("tc", "filter", "show", "dev", dev, hook, check=False)
               for hook in ("ingress", "egress"))


def link_devices(a: str, b: str) -> list[str]:
    devs = [d for d in (f"ve-{a}-{b}", f"ve-{b}-{a}") if d in set(switch_devices())]
    if len(devs) != 2:
        raise RuntimeError(f"no lab link between {a} and {b} (found {devs})")
    return devs


def cut_link(a: str, b: str) -> None:
    for d in link_devices(a, b):
        _drop(d)


def restore_link(a: str, b: str) -> None:
    for d in link_devices(a, b):
        _undrop(d)


def isolate(n: str) -> None:
    _drop(f"dl-{n}")


def rejoin(n: str) -> None:
    _undrop(f"dl-{n}")


def _egress_rules() -> list[tuple[str, str]]:
    """(handle, comment) of every fault rule in the egress chain."""
    out = run(["nft", "-a", "list", "chain", "ip", TABLE, CHAIN], check=False)
    return [(m.group(2), m.group(1)) for m in
            re.finditer(rf'comment "({TAG} [^"]+)" # handle (\d+)', out)]


def cut_egress(n: str) -> None:
    dev = f"qe-{n}"
    for direction in ("iifname", "oifname"):
        run(["nft", "insert", "rule", "ip", TABLE, CHAIN, direction, dev, "counter", "drop",
             "comment", f'"{TAG} {dev} {direction}"'])


def restore_egress(n: str) -> None:
    dev = f"qe-{n}"
    for handle, comment in _egress_rules():
        if comment.startswith(f"{TAG} {dev} "):
            run(["nft", "delete", "rule", "ip", TABLE, CHAIN, "handle", handle])


def clear() -> list[str]:
    """Undo every fault this module can have made. Returns what it removed."""
    removed = []
    for dev in switch_devices():
        if (dev.startswith("ve-") or dev.startswith("dl-")) and _has_drop(dev):
            _undrop(dev)
            removed.append(dev)
    for handle, comment in _egress_rules():
        run(["nft", "delete", "rule", "ip", TABLE, CHAIN, "handle", handle], check=False)
        removed.append(comment)
    return removed


def status() -> list[str]:
    active = [dev for dev in switch_devices()
              if (dev.startswith("ve-") or dev.startswith("dl-")) and _has_drop(dev)]
    return active + [c for _, c in _egress_rules()]


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("action", choices=("cut-link", "restore-link", "isolate", "rejoin",
                                       "cut-egress", "restore-egress", "clear", "status"))
    ap.add_argument("nodes", nargs="*", help="names (T1, X.04) or lab ids (0007)")
    args = ap.parse_args()
    with open(args.roles) as f:
        roles = json.load(f)

    def nid(x: str) -> str:
        if x in roles["nodes"]:
            return x
        if x in roles["by_name"]:
            return roles["by_name"][x]
        sys.exit(f"unknown node {x}")

    ids = [nid(x) for x in args.nodes]
    if args.action in ("cut-link", "restore-link"):
        if len(ids) != 2:
            sys.exit("a link is two nodes")
        (cut_link if args.action == "cut-link" else restore_link)(*ids)
    elif args.action in ("isolate", "rejoin"):
        for n in ids:
            (isolate if args.action == "isolate" else rejoin)(n)
    elif args.action in ("cut-egress", "restore-egress"):
        for n in ids:
            (cut_egress if args.action == "cut-egress" else restore_egress)(n)
    elif args.action == "clear":
        print("removed:", ", ".join(clear()) or "nothing")
        return
    print("active faults:", ", ".join(status()) or "none")


if __name__ == "__main__":
    main()
