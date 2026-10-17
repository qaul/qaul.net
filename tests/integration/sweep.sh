#!/usr/bin/env bash
# Convergence sweep across topologies.
#
# Run from anywhere, as root, with the routing env already exported:
#
#   export QAUL_ROUTING_V2=1
#   export RUST_LOG='libqaul::router_v2=debug,libqaul=info'
#   sudo -v && sudo -E ~/qaul.net/tests/integration/sweep.sh
#
# Each topology is torn down before the next, so a stale pid file cannot make
# a start a silent no-op. Tables and per-node logs land in $OUT.
set -u

LAB="${LAB:-$HOME/meshnet-lab}"
CONV="${CONV:-$HOME/qaul.net/tests/integration/convergence.sh}"
OUT="${OUT:-$HOME/qaul-sweep-$(date +%F-%H%M)}"

# name : topology.py args : node count
SPECS=(
  "line-8:line 8:8"
  "circle-8:circle 8:8"
  "grid4-3x3:grid4 3 3:9"
  "grid8-4x4:grid8 4 4:16"
  "grid4-4x4:grid4 4 4:16"
  "grid4-4x5:grid4 4 5:20"
  "grid4-5x5:grid4 5 5:25"
)

teardown() {
  ./software.py stop qaul >/dev/null 2>&1
  ./network.py clear    >/dev/null 2>&1
  pkill -SIGKILL -x qauld 2>/dev/null
  rm -f /tmp/qaul-*.pid
  rm -rf /tmp/qaul-0*
  sleep 1
}

[ -d "$LAB" ]  || { echo "no meshnet-lab at $LAB (set LAB=)"; exit 1; }
[ -x "$CONV" ] || { echo "no convergence.sh at $CONV (set CONV=)"; exit 1; }
mkdir -p "$OUT"
cd "$LAB" || exit 1

for spec in "${SPECS[@]}"; do
  name="${spec%%:*}"; rest="${spec#*:}"
  args="${rest%%:*}"; nodes="${rest##*:}"

  printf '\n========== %s (%s nodes) ==========\n' "$name" "$nodes"
  teardown

  if ! ./topology.py $args > "/tmp/$name.json"; then
    echo "SKIP $name: topology.py failed"; continue
  fi
  if ! ./network.py apply "/tmp/$name.json" >/dev/null; then
    echo "SKIP $name: network.py apply failed"; continue
  fi
  ./software.py start qaul >/dev/null || { echo "SKIP $name: start failed"; continue; }

  # t=0 is when convergence.sh begins, so it runs immediately after start
  "$CONV" "$nodes" | tee "$OUT/$name.txt"

  # A node's status at the end, which the table cannot show: the table only
  # records a milestone the poll loop happened to observe.
  for d in /tmp/qaul-*/; do
    printf '\n--- %s ---\n' "$(basename "$d")"
    qauld-ctl -d "$d" router v2 status 2>&1
  done > "$OUT/$name-status.txt"

  # mkdir first: cp with several sources needs an existing directory, and
  # without it every copy failed silently and the logs were lost.
  mkdir -p "$OUT/$name-logs"
  cp -r /tmp/qaul-*/ "$OUT/$name-logs/" 2>/dev/null
done

teardown

printf '\n========== summary ==========\n'
for spec in "${SPECS[@]}"; do
  name="${spec%%:*}"; rest="${spec#*:}"; nodes="${rest##*:}"
  f="$OUT/$name.txt"
  if [ -f "$f" ]; then
    mesh=$(sed -n 's/^full mesh discovery: //p' "$f")
    routing=$(sed -n 's/^routing converged: //p' "$f")
    spread=$(awk '/^qaul-/ && $3 != "-" {gsub(/s/,"",$3); if (min==""||$3<min) min=$3; if ($3>max) max=$3} END {if (min=="") print "n/a"; else print min"-"max"s"}' "$f")
    printf '%-12s %-7s routing %-12s users %-12s (per-node routing %s)\n' \
      "$name" "$nodes" "${routing:-?}" "${mesh:-?}" "$spread"
  else
    printf '%-12s %-7s %s\n' "$name" "$nodes" "no result"
  fi
done
printf '\ntables and logs: %s\n' "$OUT"
