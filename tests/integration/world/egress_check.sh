#!/usr/bin/env bash
# Plan step 3: prove the far gateways can reach test.qaul.net, and only they.
#
# No qauld runs and nothing touches the server: this applies a world's hybrid
# topology (the lab part, without S1 and r1), gives the lab ends of the far
# links their egress (egress.sh), and pings the server from inside each node.
# The server answers ping already; this sends nothing else to it.
#
#   sudo -v && sudo MESHNET_LAB=$HOME/meshnet-lab ./egress_check.sh -w world-1-half
#
# It first archives and scans the running lab, as run.sh does, then removes
# it. At the end it removes everything it made, egress included.
#
# Options:
#   -w NAME   world (default world-1-half)
#   -s IP     server address (default: test.qaul.net resolved now)
#   -c N      pings per node (default 10)
set -uo pipefail

WORLD="world-1-half"
SERVER_IP=""
COUNT=10
while getopts "w:s:c:" opt; do
  case "$opt" in
    w) WORLD="$OPTARG" ;;
    s) SERVER_IP="$OPTARG" ;;
    c) COUNT="$OPTARG" ;;
    *) echo "unknown option; see the header for usage"; exit 2 ;;
  esac
done

if [[ $EUID -ne 0 ]]; then
  echo "must run as root: sudo -v && sudo MESHNET_LAB=\$HOME/meshnet-lab $0 $*"
  exit 1
fi

SUITE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$SUITE")"
LAB="${LAB:-${MESHNET_LAB:-$HOME/meshnet-lab}}"
TOP="$SUITE/topologies"
ROLES="$TOP/$WORLD.roles.json"
HYBRID="$TOP/$WORLD-hybrid.json"
LINK_CMD="$SUITE/link_filter.sh {action} {ifname} {internet_only}"
[[ -d $LAB ]] || { echo "no meshnet-lab at $LAB"; exit 1; }
[[ -f $HYBRID ]] || { echo "no $HYBRID; run ./gen_world.py worlds/$WORLD.json"; exit 1; }

if [[ -z $SERVER_IP ]]; then
  SERVER_IP="$(getent ahostsv4 test.qaul.net | awk 'NR == 1 {print $1}')"
  [[ -n $SERVER_IP ]] || { echo "cannot resolve test.qaul.net; pass -s IP"; exit 1; }
fi

# shellcheck source=common.sh
. "$SUITE/common.sh"
step() { printf '\n========== %s ==========\n' "$1"; }

# Everything below also goes to a log in results/, owned by the sudo user.
LOG="$INT/results/egress-check-$WORLD-$(date -u +%Y%m%dT%H%M%SZ).txt"
mkdir -p "$INT/results"
: > "$LOG"
[[ -n ${SUDO_UID:-} ]] && chown "$SUDO_UID:${SUDO_GID:-$SUDO_UID}" "$LOG"
exec > >(tee -a "$LOG") 2>&1
echo "egress check: $WORLD, server $SERVER_IP, $(date -u +%FT%TZ), log $LOG"

mapfile -t EGRESS < <(python3 -c "
import sys; sys.path.insert(0, '$SUITE')
from worldlib import load_roles, egress_nodes
print('\n'.join(egress_nodes(load_roles('$ROLES'))))")
mapfile -t MEMBER < <(python3 -c "
import json
r = json.load(open('$ROLES'))
print(next(n for n, m in sorted(r['nodes'].items()) if m['role'] == 'member'))")
nm() { python3 -c "import json; print(json.load(open('$ROLES'))['nodes']['$1']['name'])"; }

step "archive the outgoing lab"
archive_and_scan || echo "(the outgoing lab merged spheres at some point; see above)"

step "teardown"
teardown_lab
# A marker left from the last run would be stale for this lab, which runs no
# qauld and writes no logs; leaving it is harmless, archive_lab.sh checks age.

cleanup() {
  step "cleanup"
  "$SUITE/egress.sh" down
  (cd "$LAB" && ./network.py clear >/dev/null 2>&1)
  echo "lab removed"
}
trap cleanup EXIT

step "hybrid lab ($WORLD without its far nodes), no daemons"
(cd "$LAB" && ./network.py --link-command "$LINK_CMD" apply "$HYBRID") || exit 1

step "egress for ${#EGRESS[@]} node(s) to $SERVER_IP"
"$SUITE/egress.sh" up "$SERVER_IP" "${EGRESS[@]}" || exit 1

FAIL=0
summary() {  # ping output -> "loss avg"
  awk -F'[ /,%]+' '/packet loss/ {for (i = 1; i <= NF; i++) if ($i == "packet") loss = $(i - 1)}
                   /^rtt/ {avg = $8} END {printf "%s%% loss, avg %s ms", loss, (avg == "" ? "-" : avg)}'
}

step "ping $SERVER_IP: the host, then each egress node ($COUNT each)"
printf '  %-6s %-5s %s\n' "host" "" "$(ping -c "$COUNT" -i 0.2 -W 2 "$SERVER_IP" 2>&1 | summary)"
for id in "${EGRESS[@]}"; do
  out="$(ip netns exec "ns-$id" ping -c "$COUNT" -i 0.2 -W 2 "$SERVER_IP" 2>&1)"
  rc=$?
  printf '  %-6s %-5s %s\n' "$(nm "$id")" "$id" "$(summary <<<"$out")"
  [[ $rc -eq 0 ]] || FAIL=1
done

step "must NOT reach it"
id="${MEMBER[0]}"
if ip netns exec "ns-$id" ping -c 2 -W 2 "$SERVER_IP" >/dev/null 2>&1; then
  echo "  member $(nm "$id") reached the server: WRONG"; FAIL=1
else
  echo "  member $(nm "$id") ($id): no way out [ok]"
fi
echo "  routes inside $(nm "${EGRESS[0]}") (${EGRESS[0]}):"
ip -n "ns-${EGRESS[0]}" route | sed 's/^/    /'

step "filter counters"
"$SUITE/egress.sh" status | grep -E 'counter' | sed 's/^[[:space:]]*/  /'

printf '\n========== egress check %s ==========\n' "$([[ $FAIL -eq 0 ]] && echo passed || echo FAILED)"
exit $FAIL
