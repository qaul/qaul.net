#!/usr/bin/env bash
# Bring up a world end to end, wire its Internet side and inspect it.
#
# The world counterpart of communities/run.sh, in one invocation so a sudo
# timestamp cannot expire mid-sequence. Every node's log is archived to
# tests/integration/runs/ by lab/archive_lab.sh: the outgoing lab's before the
# teardown destroys it, and this run's when it ends, however it ends.
#
# Run as root with the arm chosen in the environment:
#
#   export QAUL_ROUTING_V2=1
#   export RUST_LOG='libqaul::router_v2=debug,libqaul=info'
#   sudo -v && sudo -E ~/qaul.net/tests/integration/world/run.sh -w world-1-half
#
# Options:
#   -w NAME   world name, from topologies/ (default world-1-half). Generate it
#             first: ./gen_world.py worlds/NAME.json
#   -1        v1 arm: skips the v2-only checks. Also unset QAUL_ROUTING_V2.
#   -s SECS   how long check.py keeps re-reading the directory after the dial
#             (default 300; SETTLE= also works)
#   -H        hybrid run: S1 and r1 run on test.qaul.net, started by hand
#             beforehand (README, "The hybrid run"). The lab runs
#             <name>-hybrid.json, the far gateways reach S1 through egress.sh,
#             and every check is made from the lab side. This script never
#             touches the server.
#   -S ADDR   hybrid: S1's multiaddress (default
#             /ip4/<test.qaul.net>/udp/9229/quic-v1)
#   -m        after the check, send messages between chosen pairs and time
#             them (traffic.py, scenario 2); results/traffic-<tag>.json
#   -f NAME   then run a failure scenario (scenario.py): s1-down, t1-outage,
#             bridge-cut or walkaway; results/scenario-<name>-<tag>.json.
#             Faults are applied from the laptop and always cleared after.
#
# Every Internet link gets its mDNS dropped by link_filter.sh, passed to
# meshnet-lab as --link-command. Radio links are left alone. The whole network
# goes up in one apply, before any daemon starts, because meshnet-lab bridges a
# new link before it isolates and filters it (see gen_world.py).
#
# Each archive, the outgoing lab's and this run's, is then read by
# sphere_scan.py for LAN traffic between spheres at any moment of the lab's
# life. A crossing in this run fails it; one in the outgoing lab is reported,
# since it may have come after that run's own check.
set -uo pipefail

WORLD="world-1-half"
V1=""
SETTLE="${SETTLE:-300}"
HYBRID=""
FAR_ADDR=""
MESSAGES=""
SCENARIO=""

while getopts "w:1s:HS:mf:" opt; do
  case "$opt" in
    w) WORLD="$OPTARG" ;;
    1) V1="--v1" ;;
    s) SETTLE="$OPTARG" ;;
    H) HYBRID=1 ;;
    S) FAR_ADDR="$OPTARG" ;;
    m) MESSAGES=1 ;;
    f) SCENARIO="$OPTARG" ;;
    *) echo "unknown option; see the header for usage"; exit 2 ;;
  esac
done

if [[ $EUID -ne 0 ]]; then
  echo "must run as root (the first step tears down the running lab)."
  echo "  sudo -v && sudo -E $0 $*"
  exit 1
fi

SUITE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$SUITE")"
LAB="${LAB:-${MESHNET_LAB:-$HOME/meshnet-lab}}"
TOP="$SUITE/topologies"
ROLES="$TOP/$WORLD.roles.json"
RESULTS="$INT/results"
LINK_CMD="$SUITE/link_filter.sh {action} {ifname} {internet_only}"

[[ -d "$LAB" ]]   || { echo "no meshnet-lab at $LAB (set LAB= or MESHNET_LAB=)"; exit 1; }
[[ -f "$ROLES" ]] || { echo "no roles file at $ROLES; run ./gen_world.py worlds/$WORLD.json"; exit 1; }
export MESHNET_LAB="$LAB"
mkdir -p "$RESULTS"

if [[ -n $V1 ]] && [[ -n ${QAUL_ROUTING_V2:-} ]]; then
  echo "QAUL_ROUTING_V2 is set but -1 (v1 arm) was requested; unset it first."
  exit 1
fi
if [[ -z $V1 ]] && [[ -z ${QAUL_ROUTING_V2:-} ]]; then
  echo "QAUL_ROUTING_V2 is not set, so this would measure v1 while labelled v2."
  echo "Set it, or pass -1 to run the v1 arm deliberately."
  exit 1
fi

ARM="v2"; [[ -n $V1 ]] && ARM="v1"
TAG="$WORLD-$ARM"
TOPO_FILE="$TOP/$WORLD.json"
HYB=()
if [[ -n $HYBRID ]]; then
  [[ -z $V1 ]] || { echo "-H runs v2 only: S1 and r1 on the server run v2."; exit 1; }
  TOPO_FILE="$TOP/$WORLD-hybrid.json"
  [[ -f $TOPO_FILE ]] || { echo "no $TOPO_FILE; run ./gen_world.py worlds/$WORLD.json"; exit 1; }
  if [[ -z $FAR_ADDR ]]; then
    ip4="$(getent ahostsv4 test.qaul.net | awk 'NR == 1 {print $1}')"
    [[ -n $ip4 ]] || { echo "cannot resolve test.qaul.net; pass -S /ip4/.../udp/9229/quic-v1"; exit 1; }
    FAR_ADDR="/ip4/$ip4/udp/9229/quic-v1"
  fi
  SERVER_IP="$(sed -n 's#^/ip4/\([0-9.]*\)/.*#\1#p' <<<"$FAR_ADDR")"
  [[ -n $SERVER_IP ]] || { echo "-S must start with /ip4/<address>/"; exit 1; }
  HYB=(--hybrid --far "S1=$FAR_ADDR")
  TAG="$WORLD-hybrid-$ARM"
  mapfile -t EGRESS < <(cd "$SUITE" && python3 -c "
from worldlib import load_roles, egress_nodes
print('\n'.join(egress_nodes(load_roles('$ROLES'))))")
fi

step() { printf '\n========== %s ==========\n' "$1"; }

# shellcheck source=common.sh
. "$SUITE/common.sh"

step "archive the outgoing lab"
archive_and_scan || echo "(the outgoing lab merged spheres at some point; see above)"

step "teardown"
teardown_lab
cd "$LAB" || exit 1

GIT_REV="$(git -c safe.directory='*' -C "$INT" rev-parse --short HEAD 2>/dev/null)"
git -c safe.directory='*' -C "$INT" diff --quiet HEAD 2>/dev/null || GIT_REV="$GIT_REV-dirty"
cat > /tmp/qaul_lab_run.env <<EOF
TAG=$TAG
TOPO=$WORLD
ARM=$ARM
ROLES=$ROLES
SETTLE=$SETTLE
QAUL_ROUTING_V2=${QAUL_ROUTING_V2:-}
RUST_LOG=${RUST_LOG:-}
GIT_REV=$GIT_REV
HYBRID=${HYBRID:-0}
FAR=${FAR_ADDR}
EOF

trap 'archive_and_scan' EXIT

step "network: every link, mDNS dropped on Internet links, before any daemon"
./network.py --link-command "$LINK_CMD" apply "$TOPO_FILE" || exit 1

names() {  # "id rest..." lines -> "name id rest..."
  python3 -c "
import json, sys
r = json.load(open('$ROLES'))
for line in sys.stdin:
    f = line.split()
    if f and f[0] in r['nodes']:
        print(r['nodes'][f[0]]['name'], *f)
    else:
        print(line, end='')"
}
ping_server() {  # $1 = label; appends to the run's ping file and shows it
  { echo "## $1 $(date -u +%FT%TZ): host -> $SERVER_IP"
    ping -c 20 -i 0.2 -W 2 "$SERVER_IP" 2>&1 | tail -2
  } >> "$RESULTS/hybrid-ping-$TAG.txt"
  tail -3 "$RESULTS/hybrid-ping-$TAG.txt" | sed 's/^/  /'
}

if [[ -n $HYBRID ]]; then
  step "hybrid: S1 must already be running on test.qaul.net at $FAR_ADDR"
  echo "  (started by hand, with r1 connected to it; see README, 'The hybrid run')"
  : > "$RESULTS/hybrid-ping-$TAG.txt"
  ping_server "start"

  step "egress: ${#EGRESS[@]} far gateway(s) get a way out, only to $SERVER_IP"
  "$SUITE/egress.sh" up "$SERVER_IP" "${EGRESS[@]}" || exit 1
fi

step "bringup ($ARM)"
cd "$SUITE" || exit 1
./wire.py bringup --roles "$ROLES" $V1 "${HYB[@]}" || exit 1

step "phase 1: servers' LAN off, assert they joined no Local sphere"
./wire.py phase1 --roles "$ROLES" $V1 "${HYB[@]}" || exit 1

step "phase 2: dial the Internet links"
# The dial time goes into check.py's JSON (t0), never into the marker:
# lab/archive_lab.sh trusts a marker only if it was written just before the
# logs began, so touching it now would leave the run's archive untagged.
T0="$(date +%s.%N)"
./wire.py phase2 --roles "$ROLES" $V1 "${HYB[@]}" || exit 1

step "check (directory re-read for up to ${SETTLE}s after the dial)"
./check.py --roles "$ROLES" $V1 "${HYB[@]}" --poll "$SETTLE" --t0 "$T0" \
  --json "$RESULTS/world-check-$TAG.json"
CHECK_RC=$?

TRAFFIC_RC=""
if [[ -n $MESSAGES ]]; then
  if [[ -n $V1 ]]; then
    echo "(-m skipped on the v1 arm: pairs are chosen from v2 delegations)"
  else
    step "messages: timed delivery between chosen pairs (scenario 2)"
    ./traffic.py --roles "$ROLES" "${HYB[@]}" --json "$RESULTS/traffic-$TAG.json"
    TRAFFIC_RC=$?
  fi
fi

SCENARIO_RC=""
if [[ -n $SCENARIO ]]; then
  if [[ -n $V1 ]]; then
    echo "(-f skipped on the v1 arm: the scenarios read v2 state)"
  else
    step "failure scenario: $SCENARIO"
    ./scenario.py --roles "$ROLES" "${HYB[@]}" "$SCENARIO" \
      --json "$RESULTS/scenario-$SCENARIO-$TAG.json"
    SCENARIO_RC=$?
    ./faults.py --roles "$ROLES" clear  # belt and braces: scenario.py clears too
  fi
fi

if [[ -n $HYBRID ]]; then
  step "hybrid: bytes each far gateway sent and received over the real Internet"
  {
    echo "# name id up_bytes down_bytes, egress up until +$(python3 -c "import time; print(round(time.time() - $T0))") s after the dial"
    "$SUITE/egress.sh" counters | names
  } | tee "$RESULTS/hybrid-egress-$TAG.txt" | sed 's/^/  /'
  ping_server "end"
fi

step "archive, and scan every log for sphere crossings"
trap - EXIT
archive_and_scan
SCAN_RC=$?

if [[ -n $HYBRID ]]; then
  step "hybrid: egress down (S1 stops next, and the host goes back to normal)"
  "$SUITE/egress.sh" down
  A="${LAST_ARCHIVE:-<the archive above>}"
  # Printed for the user to run by hand: this script never touches the server.
  printf '%s\n' \
    "" \
    "========== now on the server, by hand ==========" \
    "In the shell where you started S1 and r1 (\$RUN is set there):" \
    "" \
    "  for n in S1 r1; do" \
    "    ~/bin/qauld-ctl --socket \$RUN/\$n/qauld.sock --json router v2 status    > \$RUN/\$n/v2-status.json" \
    "    ~/bin/qauld-ctl --socket \$RUN/\$n/qauld.sock --json router neighbours   > \$RUN/\$n/neighbours.json" \
    "    ~/bin/qauld-ctl --socket \$RUN/\$n/qauld.sock --json router v2 manifests > \$RUN/\$n/v2-manifests.json" \
    "  done" \
    "  sudo nft list table inet qaul_hybrid > \$RUN/nft.txt" \
    "  kill \$(cat \$RUN/S1/qauld.pid) \$(cat \$RUN/r1/qauld.pid)" \
    "  echo \$RUN" \
    "" \
    "Then here, with STAMP = the last part of that path (e.g. 20261113T155308Z):" \
    "" \
    "  mkdir -p $A/far/S1 $A/far/r1" \
    "  scp qaul@test.qaul.net:hybrid/STAMP/S1/qauld.err  $A/far/S1/" \
    "  scp 'qaul@test.qaul.net:hybrid/STAMP/S1/*.json'   $A/far/S1/" \
    "  scp qaul@test.qaul.net:hybrid/STAMP/r1/qauld.err  $A/far/r1/" \
    "  scp 'qaul@test.qaul.net:hybrid/STAMP/r1/*.json'   $A/far/r1/" \
    "  scp 'qaul@test.qaul.net:hybrid/STAMP/nft*.txt'    $A/far/" \
    "" \
    "qauld.err, the JSON files and nft.txt only: never config.yaml, which holds the keys."
fi

verdict() { [[ $1 -eq 0 ]] && echo passed || echo FAILED; }
printf '\n========== done: %s (check %s, sphere scan %s%s%s) ==========\n' "$TAG" \
  "$(verdict $CHECK_RC)" "$(verdict $SCAN_RC)" \
  "$([[ -n $TRAFFIC_RC ]] && echo ", messages $(verdict "$TRAFFIC_RC")")" \
  "$([[ -n $SCENARIO_RC ]] && echo ", scenario $SCENARIO $( [[ $SCENARIO_RC -eq 0 ]] && echo ran || echo FAILED)")"
echo "The lab keeps running. The next run archives it again and scans its whole life."
[[ $CHECK_RC -eq 0 && $SCAN_RC -eq 0 && ${TRAFFIC_RC:-0} -eq 0 && ${SCENARIO_RC:-0} -eq 0 ]]
