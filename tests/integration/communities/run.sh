#!/usr/bin/env bash
# Bring up a community/hub topology end to end and inspect the result.
#
# One invocation so a sudo timestamp cannot expire mid-sequence, which is what
# stretched an overhead window from 132 s to 1030 s on 2026-10-25.
#
# Every node's log is archived to tests/integration/runs/ by
# lab/archive_lab.sh: the outgoing lab's before the teardown destroys it, and
# this run's when it ends, however it ends. The lab keeps running afterwards,
# and the next run's pre-teardown archive refreshes the same directory with the
# logs it went on to write.
#
# Run as root with the arm already chosen in the environment:
#
#   export QAUL_ROUTING_V2=1
#   export RUST_LOG='libqaul::router_v2=debug,libqaul=info'
#   sudo -v && sudo -E ~/qaul.net/tests/integration/communities/run.sh -r
#
# Options:
#   -t NAME   topology base name (default communities-3x10)
#   -r        dial the gateways in reverse order (phase 2)
#   -1        v1 arm: skips the v2-only assertions. Also unset QAUL_ROUTING_V2.
#   -c        also run crossing.py after the inspection
#   -T SECS   crossing timeout (default 60)
#   -s SECS   settle before inspecting (default 180; SETTLE= also works)
set -uo pipefail

TOPO="communities-3x10"
REVERSE=""
V1=""
CROSS=""
CROSS_TIMEOUT=60
SETTLE="${SETTLE:-180}"

while getopts "t:r1cT:s:" opt; do
  case "$opt" in
    t) TOPO="$OPTARG" ;;
    r) REVERSE="--reverse" ;;
    1) V1="--v1" ;;
    c) CROSS="yes" ;;
    T) CROSS_TIMEOUT="$OPTARG" ;;
    s) SETTLE="$OPTARG" ;;
    *) echo "unknown option; see the header for usage"; exit 2 ;;
  esac
done

# Before anything else: the first real step is a teardown, so a run without
# the privileges to complete it would half-dismantle a live lab.
if [[ $EUID -ne 0 ]]; then
  echo "must run as root (the first step tears down the running lab)."
  echo "  sudo -v && sudo -E $0 $*"
  exit 1
fi

# The suite's own scripts and topologies live here; results and the lab tools
# are shared with the rest of tests/integration, one level up.
SUITE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$SUITE")"
LAB="${LAB:-${MESHNET_LAB:-$HOME/meshnet-lab}}"
TOP="$SUITE/topologies"
ROLES="$TOP/$TOPO.roles.json"
RESULTS="$INT/results"

[[ -d "$LAB" ]]     || { echo "no meshnet-lab at $LAB (set LAB= or MESHNET_LAB=)"; exit 1; }
[[ -f "$ROLES" ]]   || { echo "no roles file at $ROLES"; exit 1; }
export MESHNET_LAB="$LAB"
mkdir -p "$RESULTS"

if [[ -n $V1 ]] && [[ -n ${QAUL_ROUTING_V2:-} ]]; then
  echo "QAUL_ROUTING_V2 is set but -1 (v1 arm) was requested."
  echo "software.py is started with sudo -E, so qauld would run v2 anyway."
  echo "Unset it first:  set -e QAUL_ROUTING_V2   (fish)"
  exit 1
fi
if [[ -z $V1 ]] && [[ -z ${QAUL_ROUTING_V2:-} ]]; then
  echo "QAUL_ROUTING_V2 is not set, so this would measure v1 while labelled v2."
  echo "Set it, or pass -1 to run the v1 arm deliberately."
  exit 1
fi

ARM="v2"; [[ -n $V1 ]] && ARM="v1"
TAG="$TOPO-$ARM"; [[ -n $REVERSE ]] && TAG="$TAG-reversed"

step() { printf '\n========== %s ==========\n' "$1"; return; }

# The teardown destroys every node's log, so keep the outgoing lab's first.
# It is filed under that run's own marker, still in place at this point.
step "archive the outgoing lab"
"$INT/lab/archive_lab.sh"

step "teardown"
cd "$LAB" || exit 1
./network.py clear >/dev/null 2>&1
pkill -SIGKILL -x qauld 2>/dev/null
rm -f /tmp/qaul-*.pid 2>/dev/null
sleep 1

# lab/archive_lab.sh reads this to name and describe the run's archive.
GIT_REV="$(git -c safe.directory='*' -C "$INT" rev-parse --short HEAD 2>/dev/null)"
git -c safe.directory='*' -C "$INT" diff --quiet HEAD 2>/dev/null || GIT_REV="$GIT_REV-dirty"
cat > /tmp/qaul_lab_run.env <<EOF
TAG=$TAG
TOPO=$TOPO
ARM=$ARM
ROLES=$ROLES
REVERSE=${REVERSE:+yes}
SETTLE=$SETTLE
QAUL_ROUTING_V2=${QAUL_ROUTING_V2:-}
RUST_LOG=${RUST_LOG:-}
GIT_REV=$GIT_REV
EOF

# A run that stops early still leaves logs worth keeping.
trap '"$INT/lab/archive_lab.sh"' EXIT

step "stage 1: communities only, hub has no links"
./network.py apply "$TOP/$TOPO-stage1.json" || exit 1

step "bringup ($ARM)"
cd "$SUITE" || exit 1
./bringup.py --roles "$ROLES" $V1 || exit 1

step "phase 1: hub LAN off, assert its table is empty"
./phases.py --roles "$ROLES" --phase 1 $V1 || exit 1

step "stage 2: add the hub links"
cd "$LAB" || exit 1
./network.py apply "$TOP/$TOPO.json" || exit 1

step "phase 2: peer the gateways${REVERSE:+ (reversed)}"
cd "$SUITE" || exit 1
./phases.py --roles "$ROLES" --phase 2 $V1 $REVERSE || exit 1

if [[ -z $V1 ]]; then
  # Phase 2 waits only for the dials. The manifest directory keeps assembling
  # for minutes afterwards, so inspecting straight away samples a moving
  # system - which is what made the forward-order and reversed-order runs of
  # 2026-10-27 incomparable (one read at ~29 min, the other at ~20 s).
  step "settling ${SETTLE}s before inspection"
  sleep "$SETTLE"

  step "manifest origins and the hub's node space"
  ./manifests.py --roles "$ROLES" --json "$RESULTS/manifests-$TAG.json"
fi

if [[ -n $CROSS ]]; then
  step "crossing"
  ./crossing.py --roles "$ROLES" --arm "$ARM" --timeout "$CROSS_TIMEOUT" \
    --json "$RESULTS/crossing-$TAG.json"
fi

step "archive"
trap - EXIT
"$INT/lab/archive_lab.sh"

printf '\n========== done: %s ==========\n' "$TAG"
