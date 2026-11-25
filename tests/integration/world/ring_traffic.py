#!/usr/bin/env python3
"""Messages around a ring world (ring-1), timed across two machines.

Run on the laptop as root after check.py; run.sh -m does it for a world with
communities on the server. Half the ring runs in a lab on test.qaul.net, so
some senders and recipients are there. They are reached through one SSH
session to nodecall.py, kept open for the whole run.

Pairs are chosen by how far apart their communities' gateways are on the
ring, from 0 (same community) to 6 (opposite sides), and by where the two
ends run: laptop or server. Per distance, at most one pair of each kind
(laptop->laptop, laptop->server, server->laptop, server->server), so every
distance has a pair that crosses the real Internet where the ring allows
it. Each pair gets a group, as in traffic.py, and the sender sends
--messages messages.

Before any group is made, it waits until run_far.sh's check on the server has
finished: a group invite makes the two users known to each other, which the
server's containment test would count as a leak (it did on 2026-11-24).

Delivery time is the recipient's received_at minus the sender's sent_at, as
in traffic.py. When the two run on different machines those are two clocks,
so the server's offset from the laptop is measured over the SSH session
before and after the messages (one round trip per reading, the fastest of
30 kept) and taken off. Its uncertainty, half that round trip, is reported.

Usage:
    ./ring_traffic.py --roles topologies/ring-1.roles.json --hybrid \\
        --far S1=/ip4/<server>/udp/9229/quic-v1 --far r1=/ip4/<server>/udp/9230/quic-v1 \\
        [--remote qaul@test.qaul.net] [--remote-dir ring/tests/integration/world] \\
        [--messages 5] [--interval 2] [--timeout 180] [--seed 1] [--json ...]
"""

import argparse
import itertools
import json
import os
import random
import statistics
import subprocess
import threading
import time
from concurrent.futures import Future, ThreadPoolExecutor

from traffic import far_hops, internet_path, run_pair
from worldlib import Node, identities, lab_nodes, load_roles, name, parse_far

KINDS = [("laptop", "laptop"), ("laptop", "server"), ("server", "laptop"), ("server", "server")]


class Remote:
    """One SSH session to nodecall.py on the server. Thread-safe."""

    def __init__(self, target: str, remote_dir: str, roles_rel: str):
        cmd = ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=15",
               "-o", "ServerAliveInterval=15", target,
               f"cd {remote_dir} && sudo -n python3 -u nodecall.py serve {roles_rel}"]
        user = os.environ.get("SUDO_USER")
        if os.geteuid() == 0 and user:  # run.sh runs as root; the key is the user's
            cmd = ["sudo", "-u", user, "-H"] + cmd
        self.p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                  text=True, bufsize=1)
        self.lock = threading.Lock()
        self.pending: dict[int, Future] = {}
        self.ids = itertools.count(1)
        threading.Thread(target=self._read, daemon=True).start()

    def _read(self):
        for line in self.p.stdout:
            try:
                msg = json.loads(line)
            except ValueError:
                continue
            fut = self.pending.pop(msg.get("id"), None)
            if fut:
                fut.set_result(msg)
        for fut in list(self.pending.values()):
            fut.set_exception(RuntimeError("the SSH session to the server ended"))

    def request(self, op: str, timeout: float = 60, **kw):
        rid = next(self.ids)
        fut: Future = Future()
        self.pending[rid] = fut
        with self.lock:
            self.p.stdin.write(json.dumps({"id": rid, "op": op, **kw}) + "\n")
            self.p.stdin.flush()
        msg = fut.result(timeout=timeout)
        if not msg["ok"]:
            raise RuntimeError(msg["error"])
        return msg["result"]

    def offset(self, rounds: int = 30) -> dict:
        """Server clock minus laptop clock (ms), from the fastest round trip."""
        best = None
        for _ in range(rounds):
            t1 = time.time()
            srv = self.request("time")
            t2 = time.time()
            if best is None or t2 - t1 < best[0]:
                best = (t2 - t1, srv - (t1 + t2) / 2)
            time.sleep(0.05)
        return {"offset_ms": round(best[1] * 1000, 1), "uncertainty_ms": round(best[0] * 500, 1),
                "rtt_ms": round(best[0] * 1000, 1)}

    def close(self):
        try:
            self.p.stdin.close()
            self.p.wait(timeout=10)
        except Exception:
            self.p.kill()


class RemoteNode:
    """A server-lab node, with Node's methods, reached through Remote."""

    def __init__(self, remote: Remote, nid: str):
        self.remote, self.id = remote, nid

    def __getattr__(self, method):
        if method.startswith("_"):
            raise AttributeError(method)
        return lambda *args: self.remote.request("call", node=self.id, method=method,
                                                 args=list(args))


def choose_pairs(roles: dict, rng: random.Random) -> list[dict]:
    comms = [c for c in roles["communities"] if c["gateways"]]
    gw = {c["name"]: c["gateways"][0] for c in comms}
    members = {c["name"]: [m for m in c["members"] if roles["nodes"][m]["role"] == "member"]
               for c in comms}

    def where(c):
        return "server" if roles["nodes"][gw[c]]["location"] == "far" else "laptop"

    by_dist: dict[int, dict[tuple, list]] = {}
    for a in gw:
        for b in gw:
            path = [gw[a]] if a == b else internet_path(roles, gw[a], gw[b])
            if path is None:
                continue
            d = len(path) - 1
            by_dist.setdefault(d, {}).setdefault((where(a), where(b)), []).append((a, b, path))

    ring = len(roles["expected"]["gateway_role"])  # stations on the ring
    pairs = []
    for d in sorted(by_dist):
        for kind in KINDS:
            options = sorted(by_dist[d].get(kind, []))
            if not options:
                continue
            a, b, path = rng.choice(options)
            s = rng.choice(members[a])
            r = rng.choice([m for m in members[b] if m != s])
            pairs.append({
                "sender": s, "recipient": r,
                "sender_name": name(roles, s), "recipient_name": name(roles, r),
                "class": f"{d} hop{'s' if d != 1 else ''}", "distance": d,
                "kind": f"{kind[0]}->{kind[1]}",
                "path": [name(roles, x) for x in path],
                "crossings": far_hops(roles, path),
                "either_way": 2 * d == ring,
            })
    return pairs


def stats(xs):
    xs = sorted(x for x in xs if x is not None)
    if not xs:
        return None
    return {"n": len(xs), "min": xs[0], "median": round(statistics.median(xs), 1),
            "p90": xs[min(len(xs) - 1, int(round(0.9 * (len(xs) - 1))))], "max": xs[-1]}


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--hybrid", action="store_true", required=True)
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR")
    ap.add_argument("--remote", default="qaul@test.qaul.net")
    ap.add_argument("--remote-dir", default="ring/tests/integration/world",
                    help="the harness on the server, relative to the login's home")
    ap.add_argument("--messages", type=int, default=5)
    ap.add_argument("--interval", type=float, default=2.0)
    ap.add_argument("--timeout", type=float, default=180)
    ap.add_argument("--workers", type=int, default=10)
    ap.add_argument("--wait-check", type=float, default=900,
                    help="seconds to wait for the server's check before sending: an invite "
                         "would otherwise show up in its containment test (default 900)")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--json")
    args = ap.parse_args()
    args.island_timeout = args.timeout  # run_pair reads it; no island on a ring

    roles = load_roles(args.roles)
    parse_far(roles, args.far, "laptop")
    laptop = set(lab_nodes(roles, True, "laptop"))
    server = set(lab_nodes(roles, True, "server"))
    rng = random.Random(args.seed)
    pairs = choose_pairs(roles, rng)

    remote = Remote(args.remote, args.remote_dir,
                    "../world/topologies/" + os.path.basename(args.roles))
    try:
        tag = f"{roles['name']}-server-v2"
        deadline = time.time() + args.wait_check
        while not remote.request("check_done", tag=tag):
            if time.time() >= deadline:
                raise SystemExit(f"the server's check ({tag}) did not finish within "
                                 f"{args.wait_check:.0f}s; no messages sent")
            print("waiting for the server's check to finish before any group is made")
            time.sleep(10)
        before = remote.offset()
        print(f"server clock - laptop clock: {before['offset_ms']:+} ms "
              f"(+/- {before['uncertainty_ms']} ms, round trip {before['rtt_ms']} ms)")
        recips_l = sorted({p["recipient"] for p in pairs if p["recipient"] in laptop})
        recips_s = sorted({p["recipient"] for p in pairs if p["recipient"] in server})
        users = {nid: v["user"] for nid, v in identities(roles, recips_l).items()}
        users.update({nid: v["user"] for nid, v in
                      remote.request("ids", timeout=120, nodes=recips_s).items()})
        for p in pairs:
            p["recipient_user"] = users[p["recipient"]]

        print(f"\n=== ring messages: {len(pairs)} pairs, {args.messages} messages each, "
              f"{args.interval}s apart ===")
        for i, p in enumerate(pairs):
            print(f"  {i:2d} {p['class']:<7} {p['kind']:<15} {p['sender_name']:>5} -> "
                  f"{p['recipient_name']:<5} {' - '.join(p['path'])}"
                  f"{'  (either way round)' if p['either_way'] else ''}")

        def make_node(nid):
            return RemoteNode(remote, nid) if nid in server else Node(nid)

        stamp = time.strftime("%H%M%S")
        t_start = time.time()
        with ThreadPoolExecutor(max_workers=max(1, min(args.workers, len(pairs)))) as pool:
            results = list(pool.map(lambda ip: run_pair(ip[0], ip[1], args, stamp, make_node),
                                    enumerate(pairs)))
        after = remote.offset()
    finally:
        remote.close()

    off = (before["offset_ms"] + after["offset_ms"]) / 2
    for r in results:
        sign = {"laptop->server": -1, "server->laptop": 1}.get(r["kind"], 0)
        r["clock_correction_ms"] = sign * off
        if r.get("latency_ms"):
            r["latency_raw_ms"] = list(r["latency_ms"])
            r["latency_ms"] = [None if x is None else round(x + sign * off, 1)
                               for x in r["latency_ms"]]
        if "invite_ms" in r and r["recipient"] in server:
            r["invite_ms"] = round(r["invite_ms"] - off, 1)  # recipient's clock - laptop's

    def group(key):
        out = {}
        for r in results:
            g = out.setdefault(r[key], {"pairs": 0, "sent": 0, "delivered": 0,
                                        "first": [], "later": []})
            g["pairs"] += 1
            g["sent"] += r.get("sent", 0)
            g["delivered"] += r.get("delivered", 0)
            lat = r.get("latency_ms") or []
            if lat:
                g["first"].append(lat[0])
                g["later"] += lat[1:]
        return {k: {"pairs": v["pairs"], "sent": v["sent"], "delivered": v["delivered"],
                    "first_ms": stats(v["first"]), "later_ms": stats(v["later"])}
                for k, v in out.items()}

    by_distance, by_kind = group("distance"), group("kind")

    print("\n-- per pair (ms, clock-corrected where the ends are on different machines) --")
    for r in results:
        lat = ", ".join("-" if x is None else f"{x:g}" for x in (r.get("latency_ms") or []))
        print(f"  {r['class']:<7} {r['kind']:<15} {r['sender_name']:>5} -> {r['recipient_name']:<5} "
              f"delivered {r.get('delivered', 0)}/{r.get('sent', args.messages)}  [{lat}]"
              f"{'' if 'failed_at' not in r else '  FAILED at ' + r['failed_at']}")
    print("\n-- by ring distance: delivery time in ms --")
    print(f"  {'hops':>4} {'pairs':>5} {'delivered':>10}  {'first msg median':>16}  "
          f"{'later msgs median / p90 / max':>30}")
    for d in sorted(by_distance):
        v = by_distance[d]
        f, l = v["first_ms"], v["later_ms"]
        first = str(f["median"]) if f else "-"
        later = f"{l['median']} / {l['p90']} / {l['max']}" if l else "-"
        print(f"  {d:>4} {v['pairs']:>5} {v['delivered']:>4}/{v['sent']:<5}  "
              f"{first:>16}  {later:>30}")
    print("\n-- by where the ends run --")
    for k in [f"{a}->{b}" for a, b in KINDS]:
        if k in by_kind:
            v = by_kind[k]
            l = v["later_ms"]
            print(f"  {k:<15} {v['delivered']:>4}/{v['sent']:<5} later msgs median "
                  f"{l['median'] if l else '-'} ms")
    print(f"\nclock offset {before['offset_ms']:+} ms before, {after['offset_ms']:+} ms after "
          f"(+/- {max(before['uncertainty_ms'], after['uncertainty_ms'])} ms)")
    ok = all(r.get("delivered", 0) == r.get("sent", -1) for r in results)
    print("every message arrived" if ok else "SOME MESSAGES DID NOT ARRIVE")

    if args.json:
        with open(args.json, "w") as f:
            json.dump({"world": roles["name"], "hybrid": True, "t_start": t_start,
                       "clock": {"before": before, "after": after, "used_ms": off},
                       "settings": {k: getattr(args, k) for k in
                                    ("messages", "interval", "timeout", "seed")},
                       "pairs": results, "by_distance": by_distance, "by_kind": by_kind,
                       "ok": ok}, f, indent=1)
        print(f"wrote {args.json}")
    if not ok:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
