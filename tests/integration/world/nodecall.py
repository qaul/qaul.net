#!/usr/bin/env python3
"""Answer the laptop's calls on server-lab nodes (ring_traffic.py's far end).

Runs on test.qaul.net as root, started over one SSH session that stays open:

    sudo -n python3 -u nodecall.py serve ROLES

and reads one JSON request per line from stdin:

    {"id": 7, "op": "time"}                                   -> the server's clock
    {"id": 8, "op": "ids", "nodes": ["0095", ...]}            -> {id: {peer, user, v2}}
    {"id": 9, "op": "call", "node": "0095", "method": "invitations", "args": []}
    {"id": 10, "op": "check_done", "tag": "ring-1-server-v2"}  -> has run_far.sh's
                                                    check written its result yet

and answers each with one line, {"id": ..., "ok": ..., "result" | "error": ...}.
"time" is answered at once, on the reading thread, so a clock reading costs
one round trip and nothing else. Node calls run on a pool, so a slow node
doesn't hold up the others. Only Node methods ring_traffic.py uses are
allowed.
"""

import json
import os
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

from worldlib import Node, identities, load_roles

ALLOWED = {"create_group", "invite_to_group", "invitations", "accept_invitation",
           "group_info", "send_chat_message", "conversation", "router_v2_delegations",
           "local_user_id", "node_id"}
OUT = threading.Lock()
RESULTS = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "results")


def reply(rid, ok, value):
    line = json.dumps({"id": rid, "ok": ok, ("result" if ok else "error"): value})
    with OUT:
        sys.stdout.write(line + "\n")
        sys.stdout.flush()


def handle(roles, req):
    rid = req.get("id")
    try:
        if req["op"] == "ids":
            reply(rid, True, identities(roles, req["nodes"]))
        elif req["op"] == "check_done":
            # Has run_far.sh's check written its result since this run's dial?
            # Messages must wait for it: a group invite makes the two users
            # known to each other, which the containment check would count.
            res = os.path.join(RESULTS, f"world-check-{req['tag']}.json")
            t0 = os.path.join(RESULTS, f"server-t0-{req['tag']}.txt")
            done = (os.path.exists(res) and os.path.exists(t0)
                    and os.path.getmtime(res) >= os.path.getmtime(t0))
            reply(rid, True, done)
        elif req["op"] == "call":
            if req["method"] not in ALLOWED:
                raise ValueError(f"method {req['method']} not allowed")
            reply(rid, True, getattr(Node(req["node"]), req["method"])(*req.get("args", [])))
        else:
            raise ValueError(f"unknown op {req['op']}")
    except Exception as e:  # the laptop decides what a failure means
        reply(rid, False, f"{type(e).__name__}: {e}")


def main() -> None:
    if len(sys.argv) != 3 or sys.argv[1] != "serve":
        raise SystemExit("usage: nodecall.py serve ROLES")
    roles = load_roles(sys.argv[2])
    pool = ThreadPoolExecutor(max_workers=24)
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        req = json.loads(line)
        if req.get("op") == "time":
            reply(req.get("id"), True, time.time())
        else:
            pool.submit(handle, roles, req)
    pool.shutdown(wait=True)


if __name__ == "__main__":
    main()
