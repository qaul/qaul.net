#!/usr/bin/env bash
# v1 against v2 on one community topology, timed and measured identically.
#
# For each repeat and each arm, on a fresh lab:
#   1. run.sh with TIMELINE set: the wire counters are sampled just before
#      bring-up, and every node's user list is recorded once a second from
#      then on (timeline.py), with the bring-up and the gateway dial marked
#   2. the same settle for both arms, then the overhead window
#   3. the crossing (containment, invite, accept, deliver, names)
#   4. A's gateway loses its hub link, then gets it back
#   5. Z is cut off its community, then comes back
#   6. the timeline and both overhead windows (bring-up, steady) are reduced,
#      and the run's archive is refreshed with all of it
#
# Every arm gets the same durations, starting at the same offsets after
# bring-up: unequal sample timing is what invalidated the 2026-10-27
# comparison.
#
#   sudo -v && sudo -E ./compare.sh                     # communities-3x10
#   REPEATS=1 WINDOW=300 sudo -E ./compare.sh -t communities-3x10
#
# Environment (defaults in brackets): REPEATS [2], ARMS ["v2 v1"],
# SETTLE [180], WINDOW [600], CROSS_TIMEOUT [120], CUT_GATEWAY [120],
# CUT_MEMBER [180], HEAL [120], RUST_LOG (passed to both arms as given).
set -uo pipefail

TOPO="communities-3x10"
while getopts "t:" opt; do
  case "$opt" in
    t) TOPO="$OPTARG" ;;
    *) echo "unknown option; see the header for usage"; exit 2 ;;
  esac
done

REPEATS="${REPEATS:-2}"
ARMS="${ARMS:-v2 v1}"
SETTLE="${SETTLE:-180}"
WINDOW="${WINDOW:-600}"
CROSS_TIMEOUT="${CROSS_TIMEOUT:-120}"
CUT_GATEWAY="${CUT_GATEWAY:-120}"
CUT_MEMBER="${CUT_MEMBER:-180}"
HEAL="${HEAL:-120}"

if [[ $EUID -ne 0 ]]; then
  echo "must run as root (each arm starts by tearing down the running lab)."
  echo "  sudo -v && sudo -E $0 $*"
  exit 1
fi

SUITE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INT="$(dirname "$SUITE")"
TOP="$SUITE/topologies"
ROLES="$TOP/$TOPO.roles.json"
RESULTS="$INT/results"
LAB="${LAB:-${MESHNET_LAB:-$HOME/meshnet-lab}}"
export MESHNET_LAB="$LAB"
mkdir -p "$RESULTS"

[[ -f "$ROLES" ]] || { echo "no roles file at $ROLES"; exit 1; }

step() { printf '\n========== %s ==========\n' "$1"; return; }
apply() { (cd "$LAB" && ./network.py apply "$1" >/dev/null) || exit 1; }

# The two cut topologies, derived from the full one: A's gateway without its
# hub link, and Z without any link.
CUT_GW="$RESULTS/$TOPO-cut-gateway.json"
CUT_Z="$RESULTS/$TOPO-cut-member.json"
python3 - "$TOP/$TOPO.json" "$ROLES" "$CUT_GW" "$CUT_Z" <<'EOF' || exit 1
import json, sys
topo, roles = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
hub = roles["hub"]
gateway = roles["communities"][0]["gateway"]
z = roles["communities"][-1]["members"][-1]

def without(drop):
    cut = dict(topo)
    cut["links"] = [l for l in topo["links"] if not drop(l)]
    return cut

json.dump(without(lambda l: {l["source"], l["target"]} == {hub, gateway}), open(sys.argv[3], "w"))
json.dump(without(lambda l: z in (l["source"], l["target"])), open(sys.argv[4], "w"))
print(f"cuts: {hub}-{gateway} for the gateway reconnect, every link of {z} for the member leave")
EOF

for rep in $(seq 1 "$REPEATS"); do
  for arm in $ARMS; do
    printf '\n################ repeat %s of %s: %s ################\n' "$rep" "$REPEATS" "$arm"
    TL="$RESULTS/timeline-$TOPO-$arm"
    rm -f "$TL.jsonl" "$TL.pid" "$TL"-*.json "$TL"-*.jsonl "$TL-record.log"
    mark() { "$SUITE/timeline.py" mark --events "$TL-events.jsonl" "$1"; }

    {
      echo "TOPO=$TOPO"; echo "ARM=$arm"; echo "REPEAT=$rep of $REPEATS"
      echo "SETTLE=$SETTLE"; echo "WINDOW=$WINDOW"; echo "CROSS_TIMEOUT=$CROSS_TIMEOUT"
      echo "CUT_GATEWAY=$CUT_GATEWAY"; echo "CUT_MEMBER=$CUT_MEMBER"; echo "HEAL=$HEAL"
      echo "RUST_LOG=${RUST_LOG:-}"
    } >"$RESULTS/compare-$TOPO-$arm.txt"

    # run.sh settles v2 only, so both arms get -s 0 and the settle below
    if [[ $arm == v2 ]]; then
      TIMELINE="$TL" QAUL_ROUTING_V2=1 "$SUITE/run.sh" -t "$TOPO" -s 0 || exit 1
    else
      TIMELINE="$TL" env -u QAUL_ROUTING_V2 "$SUITE/run.sh" -t "$TOPO" -1 -s 0 || exit 1
    fi

    step "settle ${SETTLE}s"
    sleep "$SETTLE"
    "$SUITE/overhead.py" sample --force --roles "$ROLES" -o "$TL-wire-1.json" >/dev/null

    step "overhead window ${WINDOW}s"
    sleep "$WINDOW"
    "$SUITE/overhead.py" sample --force --roles "$ROLES" -o "$TL-wire-2.json" >/dev/null

    step "crossing"
    mark cross
    "$SUITE/crossing.py" --roles "$ROLES" --arm "$arm" --timeout "$CROSS_TIMEOUT" \
      --json "$RESULTS/crossing-$TOPO-$arm.json"

    step "A's gateway loses its hub link for ${CUT_GATEWAY}s"
    mark cut_gateway
    apply "$CUT_GW"
    sleep "$CUT_GATEWAY"
    mark heal_gateway
    apply "$TOP/$TOPO.json"
    sleep "$HEAL"

    step "Z is cut off for ${CUT_MEMBER}s"
    mark cut_member
    apply "$CUT_Z"
    sleep "$CUT_MEMBER"
    mark heal_member
    apply "$TOP/$TOPO.json"
    sleep "$HEAL"

    step "reduce"
    pid="$(cat "$TL.pid" 2>/dev/null)"
    if [[ -n $pid ]]; then
      kill "$pid" 2>/dev/null
      while kill -0 "$pid" 2>/dev/null; do sleep 0.5; done
    fi
    "$SUITE/timeline.py" reduce --roles "$ROLES" --recording "$TL.jsonl" \
      --events "$TL-events.jsonl" --arm "$arm" --json "$RESULTS/times-$TOPO-$arm.json"
    "$SUITE/overhead.py" reduce --roles "$ROLES" --pre "$TL-wire-0.json" --post "$TL-wire-1.json" \
      --json "$RESULTS/wire-bringup-$TOPO-$arm.json" >/dev/null
    "$SUITE/overhead.py" reduce --roles "$ROLES" --pre "$TL-wire-1.json" --post "$TL-wire-2.json" \
      --json "$RESULTS/wire-steady-$TOPO-$arm.json"

    # archive_lab.sh keeps results written since bring-up; these two predate it
    touch "$TL-wire-0.json" "$RESULTS/compare-$TOPO-$arm.txt"
    "$INT/lab/archive_lab.sh"
  done
done

printf '\n========== compare done: %s, %s repeat(s) of %s ==========\n' "$TOPO" "$REPEATS" "$ARMS"
