#!/usr/bin/env bash
# The server half of a split world (ring-1), run on test.qaul.net as root.
#
# The laptop runs its half with run.sh -H. This script runs the other half in
# a second meshnet-lab on the server. S1 and r1 run on the server's host,
# started by hand as before; the server-lab nodes that dial them reach them
# through hostlink.sh.
#
#   run_far.sh up WORLD
#       Archive the previous server lab if it was one of these runs, then
#       remove it. Apply topologies/WORLD-server.json, with mDNS dropped on
#       its Internet links. Start every daemon, wait for their accounts, and
#       give each node that dials S1 or r1 its host link.
#       Bring-up kills every qauld on the machine, so start S1 and r1 after it.
#   run_far.sh dial WORLD S1_SOCKET [SETTLE]
#       Wait until S1 has an Internet neighbour, i.e. the laptop's half has
#       dialled. Then dial this half's links and run check.py on it, timed
#       from that dial (results/world-check-WORLD-server-v2.json).
#   run_far.sh finish
#       Archive this lab's logs and scan them for sphere crossings. Then
#       remove the host links and the lab.
#
# Environment: QAUL_ROUTING_V2 and RUST_LOG as on the laptop; MESHNET_LAB
# (default ~qaul/meshnet-lab).
set -uo pipefail

SUITE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$SUITE")"
LAB="${LAB:-${MESHNET_LAB:-/home/qaul/meshnet-lab}}"
export MESHNET_LAB="$LAB"
export PATH="/usr/local/bin:$PATH"
RESULTS="$INT/results"
MARKER=/tmp/qaul_lab_run.env
HOST_ADDR=198.19.0.1
LINK_CMD="$SUITE/link_filter.sh {action} {ifname} {internet_only}"

[[ $EUID -eq 0 ]] || { echo "run as root"; exit 1; }
mkdir -p "$RESULTS"
step() { printf '\n========== %s ==========\n' "$1"; }
# shellcheck source=common.sh
. "$SUITE/common.sh"

roles_of() {
  ROLES="$SUITE/topologies/$1.roles.json"
  [[ -f $ROLES ]] || { echo "no $ROLES; copy the generated topologies over"; exit 1; }
}

# --far NAME=/ip4/198.19.0.1/udp/PORT/quic-v1 for every host node this side dials.
far_args() {
  python3 - "$ROLES" "$HOST_ADDR" <<'EOF'
import json, sys
r = json.load(open(sys.argv[1]))
lab = {n for n, m in r["nodes"].items() if m["location"] == "far" and not m.get("host")}
seen = set()
for l in r["internet_links"]:
    t = l["target"]
    if l["dialer"] in lab and r["nodes"][t].get("host") and t not in seen:
        seen.add(t)
        print(f"--far {r['nodes'][t]['name']}=/ip4/{sys.argv[2]}/udp/{r['nodes'][t]['port']}/quic-v1")
EOF
}

# Server-lab nodes that dial a host node: the ones that need a host link.
hostlink_ids() {
  python3 - "$ROLES" <<'EOF'
import json, sys
r = json.load(open(sys.argv[1]))
lab = {n for n, m in r["nodes"].items() if m["location"] == "far" and not m.get("host")}
print(" ".join(sorted({l["dialer"] for l in r["internet_links"]
                       if l["dialer"] in lab and r["nodes"][l["target"]].get("host")})))
EOF
}

cmd="${1:-}"; [[ $# -gt 0 ]] && shift
case "$cmd" in
up)
  [[ $# -eq 1 ]] || { echo "usage: run_far.sh up WORLD"; exit 2; }
  WORLD="$1"; roles_of "$WORLD"
  TOPO="$SUITE/topologies/$WORLD-server.json"
  [[ -f $TOPO ]] || { echo "no $TOPO: $WORLD has no far communities"; exit 1; }
  [[ -n ${QAUL_ROUTING_V2:-} ]] || { echo "QAUL_ROUTING_V2 is not set"; exit 1; }

  step "archive the outgoing server lab"
  if grep -q "^TAG=.*-server-" "$MARKER" 2>/dev/null; then
    archive_and_scan || echo "(the outgoing lab merged spheres at some point; see above)"
  else
    echo "  the outgoing lab was not one of these runs; not archived"
  fi

  step "teardown (every qauld on this machine, S1 and r1 included)"
  "$SUITE/hostlink.sh" down
  teardown_lab
  cd "$LAB" || exit 1

  cat > "$MARKER" <<EOF
TAG=$WORLD-server-v2
TOPO=$WORLD-server
ARM=v2
ROLES=$ROLES
QAUL_ROUTING_V2=${QAUL_ROUTING_V2:-}
RUST_LOG=${RUST_LOG:-}
HYBRID=1
SIDE=server
EOF

  step "network: the server lab, mDNS dropped on its Internet links"
  ./network.py --link-command "$LINK_CMD" apply "$TOPO" || exit 1

  step "bringup"
  cd "$SUITE" || exit 1
  mapfile -t FAR < <(far_args | tr ' ' '\n')
  ./wire.py bringup --roles "$ROLES" --hybrid --side server "${FAR[@]}" || exit 1

  step "host links for the nodes that dial S1 or r1"
  # shellcheck disable=SC2046
  "$SUITE/hostlink.sh" up $(hostlink_ids) || exit 1
  echo
  echo "up. Now start S1 and r1 on the host (S1 --port 9229, r1 --port 9230,"
  echo "LAN off on both), then: run_far.sh dial $WORLD <S1's qauld.sock>"
  ;;

dial)
  [[ $# -ge 2 ]] || { echo "usage: run_far.sh dial WORLD S1_SOCKET [SETTLE]"; exit 2; }
  WORLD="$1"; S1_SOCK="$2"; SETTLE="${3:-420}"; roles_of "$WORLD"
  [[ -S $S1_SOCK ]] || { echo "no socket $S1_SOCK: is S1 running?"; exit 1; }
  mapfile -t FAR < <(far_args | tr ' ' '\n')
  TAG="$WORLD-server-v2"

  step "waiting for the laptop's half to reach S1"
  deadline=$(( $(date +%s) + 3600 ))
  while :; do
    n="$(qauld-ctl --socket "$S1_SOCK" --json router neighbours 2>/dev/null |
         python3 -c 'import json,sys; print(len(json.load(sys.stdin).get("internet") or []))' 2>/dev/null)"
    [[ ${n:-0} -ge 1 ]] && break
    (( $(date +%s) >= deadline )) && { echo "nothing reached S1 within an hour"; exit 1; }
    sleep 0.5
  done
  echo "  S1 has $n Internet neighbour(s) at $(date -u +%FT%T.%3NZ)"

  step "phase 2: dial the server half"
  T0="$(date +%s.%N)"
  echo "T0=$T0" > "$RESULTS/server-t0-$TAG.txt"
  cd "$SUITE" || exit 1
  ./wire.py phase2 --roles "$ROLES" --hybrid --side server "${FAR[@]}" --settle 15
  WIRE_RC=$?

  step "check (directory re-read for up to ${SETTLE}s after the dial)"
  ./check.py --roles "$ROLES" --hybrid --side server "${FAR[@]}" --poll "$SETTLE" \
    --t0 "$T0" --json "$RESULTS/world-check-$TAG.json"
  CHECK_RC=$?
  echo
  echo "========== server half: dial $([[ $WIRE_RC -eq 0 ]] && echo ok || echo INCOMPLETE)," \
       "check $([[ $CHECK_RC -eq 0 ]] && echo passed || echo FAILED) =========="
  echo "The lab keeps running for the laptop's messages; run_far.sh finish ends it."
  ;;

finish)
  step "archive, and scan every log for sphere crossings"
  archive_and_scan
  SCAN_RC=$?
  echo "ARCHIVE=${LAST_ARCHIVE:-none}"
  step "remove the host links and the lab"
  "$SUITE/hostlink.sh" down
  teardown_lab
  echo "sphere scan $([[ $SCAN_RC -eq 0 ]] && echo passed || echo FAILED)"
  ;;

*)
  echo "usage: run_far.sh up WORLD | dial WORLD S1_SOCKET [SETTLE] | finish"
  exit 2
  ;;
esac
