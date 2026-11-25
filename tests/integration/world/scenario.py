#!/usr/bin/env python3
"""Failure scenarios: apply a fault, watch the network react, measure it.

Run as root on a formed world (run.sh -f NAME does it after the check, so a
hybrid run still has its egress). PLAN-real-internet.md section 10 has the
design and the spec's predictions; each scenario prints them beside what it
measured.

  s1-down      S1 becomes unreachable: every lab link to S1 is cut, or in a
               hybrid run every far gateway's egress. Held --hold s, then
               restored and watched for --after s.
  t1-outage    T1 loses its Internet for 20 s, recovers, then loses it for
               90 s and recovers: a short and a long outage.
  bridge-cut   the radio link between twin villages X and Y is cut, then
               restored.
  walkaway     10% of the members with a gateway walk out of range of
               everyone at once, then come back.

Every --interval seconds it samples:
  gateway      which lab nodes hold the gateway role (router v2 status)
  inet         each gateway's Internet neighbours (router neighbours)
  subs         which gateway each watched member is subscribed to
  entries      which watched members each gateway lists in its own manifest
  origins      how many users each observer gateway holds per origin
and a probe sends a message on each probe pair every --probe-period seconds,
from the baseline to the end. A message counts as delivered if its
received_at - sent_at is under --prompt seconds, late if it arrived later,
and lost if it never arrived.

Faults are applied with faults.py and always cleared on exit.

Usage:
    ./scenario.py --roles topologies/world-1.roles.json [--hybrid --far S1=...]
                  s1-down [--hold 240 --after 240] [--json ../results/scenario.json]
"""

import argparse
import json
import math
import random
import sys
import time
from concurrent.futures import ThreadPoolExecutor

import faults
from check import delegation
from worldlib import (Node, egress_nodes, identities, infer_far_peers, lab_nodes, load_roles,
                      name, parse_far, reverse)

POOL = ThreadPoolExecutor(max_workers=48)


def gather(nodes: dict, method: str, *args) -> dict:
    """Like batch_query, but a node that fails a call is None, not an error:
    during a fault the sampler must keep going."""
    futs = {nid: POOL.submit(getattr(node, method), *args) for nid, node in nodes.items()}
    out = {}
    for nid, f in futs.items():
        try:
            out[nid] = f.result(timeout=30)
        except Exception:
            out[nid] = None
    return out


# ---------------------------------------------------------------- the world as seen


class World:
    def __init__(self, roles, hybrid, far):
        self.roles, self.hybrid, self.far = roles, hybrid, far
        self.lab = sorted(lab_nodes(roles, hybrid))
        self.nodes = {n: Node(n) for n in self.lab}
        self.ids = identities(roles, self.lab)
        self.by_peer = reverse(self.ids, "peer")
        self.by_v2 = reverse(self.ids, "v2")
        self.gateways = [g for g in roles["expected"]["gateway_role"] if g in self.nodes]
        self.deleg = delegation(roles, self.nodes, self.ids)
        self.far_peer = {}
        if hybrid:
            nb = gather({g: self.nodes[g] for g in self.gateways}, "router_neighbours")
            self.far_peer = infer_far_peers(roles, {k: v for k, v in nb.items() if v},
                                            self.by_peer)
        for t, p in self.far_peer.items():
            self.by_peer[p] = t
        # each member's routing user id, from its own subscription
        self.user_of: dict[str, str] = {}
        members = [n for n in self.lab if roles["nodes"][n]["role"] == "member"]
        dl = gather({m: self.nodes[m] for m in members}, "router_v2_delegations")
        for m, d in dl.items():
            for s in (d or {}).get("subscriptions") or []:
                self.user_of[s["user_id"]] = m

    def n(self, nid):
        return name(self.roles, nid)

    def id(self, nm):
        return self.roles["by_name"][nm]

    def members(self, *comms):
        return [n for n in self.lab if self.roles["nodes"][n]["role"] == "member"
                and self.roles["nodes"][n]["community"] in comms]

    def baseline_gateway(self, m):
        g = self.deleg["members"].get(self.n(m), {}).get("gateway")
        return self.id(g) if g else None

    def nearest(self, m):
        metrics = self.deleg["members"].get(self.n(m), {}).get("metric_to_gateways") or {}
        if not metrics:
            return None
        best = min(metrics.values())
        tied = [g for g, v in metrics.items() if v == best]
        return self.id(tied[0]) if len(tied) == 1 else None


# ---------------------------------------------------------------- probe messages


class Probe:
    """One group per pair; a message every period; deliveries read from the
    recipient's own sent_at / received_at."""

    def __init__(self, world, pairs, period, prompt):
        self.w, self.pairs, self.period, self.prompt = world, pairs, period, prompt
        self.groups: dict[int, str] = {}
        self.sent: dict[int, list[dict]] = {i: [] for i in range(len(pairs))}
        self.next_at = 0.0

    def setup(self, timeout=120):
        stamp = time.strftime("%H%M%S")

        def one(i):
            s, r, label = self.pairs[i]
            sn, rn = self.w.nodes[s], self.w.nodes[r]
            gid = sn.create_group(f"fault-{stamp}-{i:02d}")
            sn.invite_to_group(gid, self.w.ids[r]["user"])
            end = time.time() + timeout
            while time.time() < end:
                if any(x.get("group_id") == gid for x in (rn.invitations() or [])):
                    rn.accept_invitation(gid)
                    break
                time.sleep(1)
            else:
                return i, None
            while time.time() < end:
                info = sn.group_info(gid)
                if any(m.get("user_id") == self.w.ids[r]["user"] and m.get("state") == "Activated"
                       for m in info.get("members", [])):
                    return i, gid
                time.sleep(1)
            return i, None

        for i, gid in POOL.map(one, range(len(self.pairs))):
            if gid:
                self.groups[i] = gid
            else:
                print(f"  probe {self.label(i)}: group setup failed before the fault; left out")

    def label(self, i):
        s, r, label = self.pairs[i]
        return f"{self.w.n(s)} -> {self.w.n(r)} ({label})"

    def send_due(self, now):
        if now < self.next_at:
            return
        self.next_at = now + self.period

        def one(i):
            gid = self.groups[i]
            seq = len(self.sent[i])
            content = f"{gid[:8]}-p{seq}"
            t = time.time()
            try:
                self.w.nodes[self.pairs[i][0]].send_chat_message(gid, content)
                return i, {"seq": seq, "content": content, "t_send": t}
            except Exception as e:
                return i, {"seq": seq, "content": content, "t_send": t, "send_error": str(e)}

        for i, rec in POOL.map(one, list(self.groups)):
            self.sent[i].append(rec)

    def poll(self):
        def one(i):
            try:
                conv = self.w.nodes[self.pairs[i][1]].conversation(self.groups[i])
            except Exception:
                return i, None
            got = {}
            for msg in conv.get("messages", []):
                for c in msg.get("content") or []:
                    got[c] = (msg.get("sent_at"), msg.get("received_at"))
            return i, got

        for i, got in POOL.map(one, list(self.groups)):
            if not got:
                continue
            for rec in self.sent[i]:
                if "received_at" not in rec and rec["content"] in got:
                    sa, ra = got[rec["content"]]
                    rec["sent_at"], rec["received_at"] = sa, ra


# ---------------------------------------------------------------- sampling


def sample(w: World, watch_members, observers) -> dict:
    gws = {g: w.nodes[g] for g in w.gateways}
    status = gather(gws, "router_v2_status")
    nbs = gather(gws, "router_neighbours")
    gdl = gather(gws, "router_v2_delegations")
    mdl = gather({m: w.nodes[m] for m in watch_members}, "router_v2_delegations")
    man = gather({o: w.nodes[o] for o in observers}, "router_v2_manifests")
    out = {"gateway": {}, "inet": {}, "subs": {}, "entries": {}, "entries_n": {}, "origins": {}}
    for g in w.gateways:
        # "?" marks a read that failed; it never counts as a change.
        st = status.get(g)
        out["gateway"][w.n(g)] = st.get("is_gateway") if st else "?"
        nb = nbs.get(g)
        out["inet"][w.n(g)] = (sorted(w.n(w.by_peer[x["node_id"]]) if x["node_id"] in w.by_peer
                                      else "?" + x["node_id"][-6:] for x in nb.get("internet") or [])
                               if nb else "?")
        d = gdl.get(g)
        if d is None:
            out["entries_n"][w.n(g)] = "?"
        else:
            ents = [e["user_id"] for e in d.get("entries") or []]
            out["entries_n"][w.n(g)] = len(ents)
            out["entries"][w.n(g)] = sorted(w.n(w.user_of[u]) for u in ents if u in w.user_of)
    for m in watch_members:
        d = mdl.get(m)
        if d is None:
            out["subs"][w.n(m)] = "?"
            continue
        subs = d.get("subscriptions") or []
        tgt = w.by_v2.get(subs[0]["target_node_id"]) if subs else None
        out["subs"][w.n(m)] = w.n(tgt) if tgt else None
    for o in observers:
        rows = man.get(o)
        if rows is None:
            out["origins"][w.n(o)] = "?"
            continue
        out["origins"][w.n(o)] = {(w.n(w.by_v2[r["node_id"]]) if r["node_id"] in w.by_v2
                                   else "?" + r["node_id"][-4:]): r.get("trusted_users") or 0
                                  for r in rows}
    return out


def snapshot(w: World, m: str, t: float, why: str) -> dict:
    """A member's own view of every gateway, read once: why it has (or has
    no) gateway. §10.3 makes a gateway eligible only if both hold:
      1. the member's node route to it has local_only = 1;
      2. the gateway's latest manifest says is_gateway.
    The raw table, manifests and delegations are kept too, for anything the
    summary doesn't show."""
    node = w.nodes[m]
    out = {"member": w.n(m), "t": t, "why": why}
    for key, method in (("table", "router_v2_table"), ("manifests", "router_v2_manifests"),
                        ("delegations", "router_v2_delegations"), ("status", "router_v2_status")):
        try:
            out[key] = getattr(node, method)()
        except Exception as e:  # a failed read is recorded, not fatal
            out[key] = None
            out[f"{key}_error"] = str(e)[:300]

    def nm(v2):
        return w.n(w.by_v2[v2]) if v2 in w.by_v2 else "?" + str(v2)[-6:]

    routes = {e.get("target_id"): e for e in out["table"] or [] if e.get("space") == "node"}
    mans = {r.get("node_id"): r for r in out["manifests"] or []}
    view = {}
    for g in w.gateways:
        v2 = w.ids[g].get("v2")
        e, mf = routes.get(v2), mans.get(v2)
        view[w.n(g)] = {
            "route": None if e is None else {
                "metric": e.get("metric"), "hops": e.get("hop_count"),
                "local_only": e.get("local_only"), "via": nm(e.get("next_hop_id")),
                "transport": e.get("transport"), "age_ms": e.get("age_ms")},
            "manifest_is_gateway": None if mf is None else mf.get("is_gateway"),
        }
    out["view"] = view
    d = out["delegations"] or {}
    out["outstanding"] = [nm(o.get("target_node_id")) for o in d.get("outstanding") or []]
    out["declined"] = [nm(x.get("node_id")) for x in d.get("declined") or []]
    out["subscribed"] = [nm(s.get("target_node_id")) for s in d.get("subscriptions") or []]
    return out


class StuckWatch:
    """Snapshots a watched member once it has had no gateway for `after`
    seconds during a fault phase, and, for contrast, one member that had
    the same gateway at baseline and has a gateway now. At most `limit`
    snapshots per run."""

    def __init__(self, w: World, watch_members, after, limit=12):
        self.w, self.after, self.limit = w, after, limit
        self.watch = list(watch_members)
        self.since: dict[str, float] = {}   # member -> when first seen with no gateway
        self.done: set = set()              # (member, since) already taken
        self.contrast_done: set = set()     # baseline gateways already given a contrast
        self.snaps: list[dict] = []

    def check(self, s: dict, in_fault: bool):
        t = s["t"]
        due = []
        for m in self.watch:
            v = s["subs"].get(self.w.n(m))
            if v == "?":
                continue  # a failed read neither starts nor ends a gap
            if v is not None or not in_fault or self.w.baseline_gateway(m) is None:
                self.since.pop(m, None)
                continue
            start = self.since.setdefault(m, t)
            if t - start >= self.after and (m, start) not in self.done:
                self.done.add((m, start))
                due.append((m, f"no gateway since +{start:.0f}s"))
                g0 = self.w.baseline_gateway(m)
                if g0 not in self.contrast_done:
                    other = next((x for x in self.watch if x != m
                                  and self.w.baseline_gateway(x) == g0
                                  and s["subs"].get(self.w.n(x)) not in (None, "?")), None)
                    if other:
                        self.contrast_done.add(g0)
                        due.append((other, f"contrast: was on {self.w.n(g0)} like "
                                           f"{self.w.n(m)}, now on {s['subs'][self.w.n(other)]}"))
        due = due[: max(0, self.limit - len(self.snaps))]
        if due:
            futs = [POOL.submit(snapshot, self.w, m, t, why) for m, why in due]
            for f in futs:
                self.snaps.append(f.result())
            print(f"  snapshot at +{t:.0f}s: " + "; ".join(f"{self.w.n(m)} ({why})"
                                                         for m, why in due), flush=True)


# ---------------------------------------------------------------- scenarios


def probe_pairs(w: World, rng, wanted):
    """wanted: list of (label, sender candidates, recipient candidates)."""
    pairs = []
    for label, senders, recips in wanted:
        if senders and recips:
            pairs.append((rng.choice(senders), rng.choice(recips), label))
    return pairs


def define(w: World, name_: str, args, rng) -> dict:
    roles = w.roles
    x_senders = [m for m in w.members("X") if w.nearest(m) == w.id("X1")]
    if name_ == "s1-down":
        s1 = w.id("S1")
        if w.hybrid:
            targets = egress_nodes(roles)
            cut = lambda: [faults.cut_egress(n) for n in targets]
            fix = lambda: [faults.restore_egress(n) for n in targets]
            how = f"egress of {', '.join(w.n(n) for n in targets)} dropped"
        else:
            nbrs = sorted({l["dialer"] if l["target"] == s1 else l["target"]
                           for l in roles["internet_links"] if s1 in (l["dialer"], l["target"])})
            cut = lambda: [faults.cut_link(s1, n) for n in nbrs]
            fix = lambda: [faults.restore_link(s1, n) for n in nbrs]
            how = f"lab links S1-{{{', '.join(w.n(n) for n in nbrs)}}} dropped"
        on = lambda g: [m for m in w.members("T", "V") if w.baseline_gateway(m) == w.id(g)]
        wanted = [("on T1, must move to T3", x_senders, on("T1")),
                  ("on T2, must move to T3", x_senders, on("T2")),
                  ("on V1, must move to V2", x_senders, on("V1")),
                  ("W: cut off with S1", x_senders, w.members("W")),
                  ("H: cut off with S1", x_senders, w.members("H")),
                  ("control, on T3", x_senders, on("T3")),
                  ("control, on V2", x_senders, on("V2"))]
        return {"how": how,
                "phases": [("baseline", args.baseline, None), ("fault", args.hold or 240, cut),
                           ("recovery", args.after or 240, fix)],
                "watch": w.members("T", "V", "W", "H"),
                "observers": [w.id(g) for g in ("T3", "V2", "X1", "S2", "r2") if w.id(g) in w.nodes],
                "pairs": probe_pairs(w, rng, wanted),
                "predict": [
                    "T1, T2, V1 lose the gateway role (§2.3); W1 and H1 keep it (their link)",
                    "Internet link to S1 retired in about 10-15 s (pings every 5 s, 2 misses)",
                    "T's members on T1/T2 move to T3, V's on V1 to V2 (§10.3)",
                    "until they move, they are unreachable (§10.6)",
                    "W's and H's users: how long other gateways keep listing them is measured",
                    "after the restore, links come back and the gateway role returns"]}
    if name_ == "t1-outage":
        t1, s1 = w.id("T1"), w.id("S1")
        if w.hybrid:
            cut, fix = (lambda: faults.cut_egress(t1)), (lambda: faults.restore_egress(t1))
            how = "T1's egress dropped"
        else:
            cut, fix = (lambda: faults.cut_link(t1, s1)), (lambda: faults.restore_link(t1, s1))
            how = "lab link T1-S1 dropped"
        on_t1 = [m for m in w.members("T") if w.baseline_gateway(m) == t1]
        on_t3 = [m for m in w.members("T") if w.baseline_gateway(m) == w.id("T3")]
        wanted = [("on T1", x_senders, on_t1), ("on T1", x_senders, on_t1),
                  ("on T1", x_senders, on_t1), ("control, on T3", x_senders, on_t3)]
        return {"how": how,
                "phases": [("baseline", args.baseline, None), ("fault-20s", 20, cut),
                           ("recovery-1", 150, fix), ("fault-90s", 90, cut),
                           ("recovery-2", args.after or 180, fix)],
                "watch": w.members("T"),
                "observers": [w.id(g) for g in ("T3", "V2", "X1", "S2") if w.id(g) in w.nodes],
                "pairs": probe_pairs(w, rng, wanted),
                "predict": [
                    "both outages outlast detection (10-15 s): T1 loses the gateway role both times",
                    "20 s ends before route expiry (35 s, §7.5); 90 s outlasts it, near §10.7's 95 s",
                    "measured: do T1's members move to T2/T3, and do they come back"]}
    if name_ == "bridge-cut":
        b = roles["radio_bridges"][0]
        a_, b_ = b["a"], b["b"]
        cut, fix = (lambda: faults.cut_link(a_, b_)), (lambda: faults.restore_link(a_, b_))
        t_senders = [m for m in w.members("T") if w.nearest(m) == w.id("T3")]
        wanted = [("Y, loses X1", t_senders, w.members("Y")),
                  ("Y, loses X1", t_senders, w.members("Y")),
                  ("Y, loses X1", t_senders, w.members("Y")),
                  ("control, X", t_senders, w.members("X"))]
        return {"how": f"radio bridge {w.n(a_)}-{w.n(b_)} dropped",
                "phases": [("baseline", args.baseline, None), ("fault", args.hold or 180, cut),
                           ("recovery", args.after or 240, fix)],
                "watch": w.members("X", "Y"),
                "observers": [w.id(g) for g in ("T3", "V2", "S2") if w.id(g) in w.nodes],
                "pairs": probe_pairs(w, rng, wanted),
                "predict": [
                    "Y's members have no eligible gateway left: local-only (§10.3)",
                    "X1 drops Y's users from its manifest within ~95 s (§10.7)",
                    "other gateways follow a few seconds per hop later",
                    "after the restore, Y re-delegates to X1 and reappears"]}
    if name_ == "walkaway":
        cands = [m for m in w.lab if w.roles["nodes"][m]["role"] == "member"
                 and w.baseline_gateway(m)]
        k = max(1, math.ceil(0.10 * len(cands)))
        away = sorted(rng.sample(cands, k))
        cut = lambda: [faults.isolate(m) for m in away]
        fix = lambda: [faults.rejoin(m) for m in away]
        wanted = [("walked away", x_senders, [m]) for m in away[:5]]
        wanted.append(("control", x_senders, [m for m in w.members("T", "V") if m not in away]))
        return {"how": f"{k} members isolated: {', '.join(w.n(m) for m in away)}",
                "phases": [("baseline", args.baseline, None), ("fault", args.hold or 150, cut),
                           ("recovery", args.after or 240, fix)],
                "watch": away,
                "observers": [w.id(g) for g in ("T3", "V2", "X1", "S2") if w.id(g) in w.nodes],
                "pairs": probe_pairs(w, rng, wanted),
                "predict": [
                    "their gateways drop them within ~95 s (§10.7)",
                    "on return they re-delegate (§10.3); rejoin time measured"]}
    raise SystemExit(f"unknown scenario {name_}")


# ---------------------------------------------------------------- analysis


def analyse(samples, events, base, probe: Probe, prompt, t0):
    """For each watched value and each phase after the baseline: how long
    after the phase began it first differed from the baseline, how long until
    it was back to the baseline, and what it was at the phase's end. Only the
    samples inside that phase are looked at; a "?" (failed read) is skipped."""
    out = {"gateway": {}, "inet": {}, "subs": {}, "entries": {}, "origins": {}, "probes": []}
    windows = []
    for k, e in enumerate(events):
        if e["phase"] == "baseline":
            continue
        end = events[k + 1]["t"] if k + 1 < len(events) else float("inf")
        windows.append((e["phase"], e["t"], [s for s in samples if e["t"] <= s["t"] < end]))

    def track(section, key, base_value, current):
        rows = []
        off = False  # differs from the baseline, carried from one phase to the next
        for phase, t, win in windows:
            off_at_start = off
            seen = [(s["t"], current(s)) for s in win if current(s) != "?"]
            changed = next((round(ts - t, 1) for ts, v in seen if v != base_value), None)
            if off_at_start and changed == round(seen[0][0] - t, 1) if seen else False:
                changed = None  # not a new change: it was already off when the phase began
            back = None
            for ts, v in seen:
                if v != base_value:
                    off = True
                elif off:
                    back = round(ts - t, 1)  # first return to the baseline after a change
                    off = False
            if seen:
                off = seen[-1][1] != base_value
            rows.append({"phase": phase, "changed_after_s": changed, "back_after_s": back,
                         "off_at_start": off_at_start,
                         "value_at_phase_end": seen[-1][1] if seen else "?"})
        if any(r["changed_after_s"] is not None for r in rows):
            out[section][key] = {"baseline": base_value, "phases": rows}

    for g, v in base["gateway"].items():
        track("gateway", g, v, lambda s, g=g: s["gateway"].get(g))
    for g, v in base["inet"].items():
        track("inet", g, v, lambda s, g=g: s["inet"].get(g))
    for m, v in base["subs"].items():
        track("subs", m, v, lambda s, m=m: s["subs"].get(m))
    for g, v in base["entries_n"].items():
        track("entries", g, v, lambda s, g=g: s["entries_n"].get(g))
    for o, origins in base["origins"].items():
        if origins == "?":
            continue
        for origin, v in origins.items():
            track("origins", f"{o} sees {origin}", v,
                  lambda s, o=o, origin=origin: (s["origins"].get(o) or {}).get(origin, 0)
                  if s["origins"].get(o) != "?" else "?")

    for i, recs in probe.sent.items():
        msgs = []
        for r in recs:
            ts = round(r["t_send"] - t0, 1)
            if "received_at" in r and r.get("sent_at") is not None:
                delay = (r["received_at"] - r["sent_at"]) / 1000
                state = "delivered" if delay <= prompt else "late"
                msgs.append({"t": ts, "state": state, "delay_s": round(delay, 3)})
            else:
                msgs.append({"t": ts, "state": "send-error" if "send_error" in r else "lost"})
        summary = []
        for phase, t, _ in windows:
            k = [e["t"] for e in events].index(t)
            end = events[k + 1]["t"] if k + 1 < len(events) else float("inf")
            inside = [m for m in msgs if t <= m["t"] < end]
            # the trouble window: the first message not delivered promptly, and
            # the first one after it that was (times are when they were sent)
            bad = next((m["t"] for m in inside if m["state"] != "delivered"), None)
            ok_again = (next((m["t"] for m in inside if m["t"] > bad and m["state"] == "delivered"),
                             None) if bad is not None else None)
            summary.append({"phase": phase, "sent": len(inside),
                            "delivered": sum(m["state"] == "delivered" for m in inside),
                            "late": sum(m["state"] == "late" for m in inside),
                            "lost": sum(m["state"] in ("lost", "send-error") for m in inside),
                            "trouble_from_s": None if bad is None else round(bad - t, 1),
                            "prompt_again_s": None if ok_again is None else round(ok_again - t, 1)})
        out["probes"].append({"pair": probe.label(i), "messages": msgs, "phases": summary})
    return out


def report_snapshots(snaps, roles):
    if not snaps:
        return
    print("\n-- members left without a gateway: their own view of each gateway (§10.3 needs "
          "a local_only route AND a manifest saying is_gateway) --")
    for sn in snaps:
        print(f"  {sn['member']} at +{sn['t']:.0f}s ({sn['why']}); subscribed {sn['subscribed'] or '-'}, "
              f"outstanding {sn['outstanding'] or '-'}, declined {sn['declined'] or '-'}")
        for k in ("table", "manifests", "delegations"):
            if sn.get(f"{k}_error"):
                print(f"    {k} read failed: {sn[k + '_error']}")
        community = roles["nodes"][roles["by_name"][sn["member"]]]["community"]
        for g, v in sn["view"].items():
            r = v["route"]
            same = roles["nodes"][roles["by_name"][g]]["community"] == community
            if r is None and not same:
                continue  # another community's gateway it has no route to: expected
            eligible = bool(r and r["local_only"] and v["manifest_is_gateway"])
            route = ("no node route" if r is None else
                     f"metric {r['metric']}, {r['hops']} hops, local_only={r['local_only']}, "
                     f"via {r['via']} ({r['transport']}), age {r['age_ms']} ms")
            print(f"    {g:<4} {'ELIGIBLE' if eligible else 'not eligible':<12} {route}; "
                  f"manifest is_gateway={v['manifest_is_gateway']}")


def report(name_, spec, events, result):
    print(f"\n==================== {name_}: {spec['how']} ====================")
    print("phases: " + ", ".join(f"{e['phase']} at +{e['t']:.0f}s" for e in events))
    print("\nwhat the spec and the code predict:")
    for p in spec["predict"]:
        print(f"  - {p}")

    def cell(r, fmt):
        parts = []
        if r["changed_after_s"] is not None:
            parts.append(f"changed +{r['changed_after_s']}s")
        if r["back_after_s"] is not None:
            parts.append(f"back +{r['back_after_s']}s")
        if not parts:
            parts.append("still changed" if r["off_at_start"] else "unchanged")
        return f"{r['phase']}: {', '.join(parts)}, ends {fmt(r['value_at_phase_end'])}"

    def show(title, section, fmt=lambda v: v):
        rows = result[section]
        print(f"\n-- {title}: {len(rows)} changed --")
        for k, v in sorted(rows.items())[:40]:
            print(f"  {k:<16} was {fmt(v['baseline'])!s:<12} | "
                  + " | ".join(cell(r, fmt) for r in v["phases"]))

    def spread(xs):
        xs = sorted(x for x in xs if x is not None)
        if not xs:
            return "-"
        return f"+{xs[0]}s / +{xs[len(xs) // 2]}s / +{xs[-1]}s"

    show("gateway role", "gateway")
    show("Internet neighbours", "inet", lambda v: ",".join(v) if isinstance(v, list) else v)

    # members, grouped by where they went in each phase
    subs = result["subs"]
    print(f"\n-- member subscriptions: {len(subs)} members changed "
          f"(times: fastest / median / slowest) --")
    phases = [r["phase"] for r in next(iter(subs.values()))["phases"]] if subs else []
    for k, phase in enumerate(phases):
        groups = {}
        for m, v in subs.items():
            r = v["phases"][k]
            if r["changed_after_s"] is None and r["back_after_s"] is None:
                continue
            key = (v["baseline"], r["value_at_phase_end"])
            groups.setdefault(key, []).append(r["changed_after_s"] if r["changed_after_s"] is not None
                                              else r["back_after_s"])
        for (was, now), times in sorted(groups.items(), key=lambda kv: str(kv[0])):
            verb = "back to" if now == was else "moved to" if now else "lost their gateway, now"
            print(f"  {phase:<11} {len(times):>3} on {was} {verb} {now}: {spread(times)}")

    show("entries in each gateway's own manifest", "entries")
    show("users per origin, at observer gateways", "origins")

    print("\n-- probe messages per phase: ok (within --prompt) / late / lost; trouble = from the "
          "first message not delivered promptly to the next one that was (by send time) --")
    for p in result["probes"]:
        print(f"  {p['pair']}")
        for ph in p["phases"]:
            if ph["trouble_from_s"] is None:
                trouble = "no trouble"
            elif ph["prompt_again_s"] is None:
                trouble = f"trouble from +{ph['trouble_from_s']}s to the end of the phase"
            else:
                trouble = f"trouble from +{ph['trouble_from_s']}s, prompt again from +{ph['prompt_again_s']}s"
            print(f"    {ph['phase']:<11} sent {ph['sent']:>2}: ok {ph['delivered']:>2}, "
                  f"late {ph['late']:>2}, lost {ph['lost']:>2}; {trouble}")

    print("\n-- probe messages (t = when sent, seconds from the start) --")
    word = {"delivered": "ok", "late": "LATE", "lost": "LOST", "send-error": "ERR"}
    for p in result["probes"]:
        marks = []
        for m in p["messages"]:
            mark = f"{m['t']:.0f}:{word[m['state']]}"
            if m["state"] == "late":
                mark += f"({m['delay_s']:.1f}s)"
            marks.append(mark)
        print(f"  {p['pair']}\n    {' '.join(marks)}")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--roles", required=True)
    ap.add_argument("--hybrid", action="store_true")
    ap.add_argument("--far", action="append", metavar="NAME=MULTIADDR")
    ap.add_argument("scenario", choices=("s1-down", "t1-outage", "bridge-cut", "walkaway"))
    ap.add_argument("--baseline", type=float, default=30)
    ap.add_argument("--hold", type=float, help="fault duration (default per scenario)")
    ap.add_argument("--after", type=float, help="recovery watch (default per scenario)")
    ap.add_argument("--interval", type=float, default=5)
    ap.add_argument("--probe-period", type=float, default=10)
    ap.add_argument("--prompt", type=float, default=5,
                    help="a message slower than this (s) counts as late, not delivered")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--stuck-after", type=float, default=30,
                    help="snapshot a watched member's own view once it has had no gateway "
                         "this long (s) after the baseline")
    ap.add_argument("--json")
    args = ap.parse_args()

    roles = load_roles(args.roles)
    far = parse_far(roles, args.far) if args.hybrid else {}
    rng = random.Random(args.seed)
    w = World(roles, args.hybrid, far)
    spec = define(w, args.scenario, args, rng)
    print(f"=== {args.scenario}: {spec['how']} ===")
    print(f"watching {len(spec['watch'])} members, {len(w.gateways)} gateways, observers "
          f"{', '.join(w.n(o) for o in spec['observers'])}")

    probe = Probe(w, spec["pairs"], args.probe_period, args.prompt)
    print(f"setting up {len(spec['pairs'])} probe pairs")
    probe.setup()
    for i in probe.groups:
        print(f"  probe {probe.label(i)}")

    samples, events = [], []
    stuck = StuckWatch(w, spec["watch"], args.stuck_after)
    t0 = time.time()
    try:
        for phase, dur, action in spec["phases"]:
            if action:
                action()
            t_phase = time.time()
            events.append({"phase": phase, "t": round(t_phase - t0, 1), "t_abs": t_phase})
            print(f"[+{t_phase - t0:6.1f}s] {phase} ({dur:.0f}s)", flush=True)
            end = t_phase + dur
            while time.time() < end:
                tick = time.time()
                probe.send_due(tick)
                s = sample(w, spec["watch"], spec["observers"])
                s["t"] = round(tick - t0, 1)
                samples.append(s)
                stuck.check(s, in_fault=phase != "baseline")
                probe.poll()
                time.sleep(max(0.0, args.interval - (time.time() - tick)))
        # a last look for late deliveries
        time.sleep(args.interval)
        probe.poll()
    finally:
        removed = faults.clear()
        if removed:
            print(f"cleared faults: {', '.join(removed)}")

    first_fault = events[1]["t"] if len(events) > 1 else float("inf")
    before = [s for s in samples if s["t"] < first_fault]
    base = before[-1] if before else samples[0]  # the last sample before the fault
    result = analyse(samples, events, base, probe, args.prompt, t0)
    report(args.scenario, spec, events, result)
    report_snapshots(stuck.snaps, roles)
    if args.json:
        with open(args.json, "w") as f:
            json.dump({"scenario": args.scenario, "world": roles["name"], "hybrid": args.hybrid,
                       "how": spec["how"], "predict": spec["predict"], "events": events,
                       "baseline": base, "samples": samples, "result": result,
                       "snapshots": stuck.snaps}, f, indent=1)
        print(f"\nwrote {args.json}")


if __name__ == "__main__":
    main()
