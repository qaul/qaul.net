#!/usr/bin/env bash
# Churn behaviour on an already-converged mesh. Run as root:
#
#   sudo -E tests/integration/churn.sh cut  0000 0001 [hold_secs]
#   sudo -E tests/integration/churn.sh kill 0004      [hold_secs]
#
# Records every node once a second through all phases, so transitions are read
# off afterwards instead of being detected live and missed.
#
# `cut` is *silent* loss: netem drops 100% on both veth ends with the carrier
# still up, so nothing closes a socket. `kill` is the socket-close path. The
# two are different failure modes and historically had detection times three
# orders of magnitude apart, so they are tested separately.
set -uo pipefail

MODE=${1:-}
case "$MODE" in
  cut)  A=${2:?node a}; B=${3:?node b}; HOLD=${4:-60} ;;
  kill) K=${2:?node id};                HOLD=${3:-60} ;;
  *) echo "usage: churn.sh cut <a> <b> [hold] | kill <id> [hold]"; exit 1 ;;
esac

[ "$(id -u)" -eq 0 ] || { echo "must run as root (sudo -E)"; exit 1; }

OUT=${OUT:-/root/qaul-churn-$(date +%F-%H%M)}
[ -n "${SUDO_USER:-}" ] && OUT=${OUT//\/root\//$(getent passwd "$SUDO_USER" | cut -d: -f6)/}
mkdir -p "$OUT"
DIRS=(/tmp/qaul-*/)
[ -d "${DIRS[0]}" ] || { echo "no /tmp/qaul-*/ dirs; start the lab first"; exit 1; }
START=$(date +%s)

sample() {
  local d=$1 id=$2 phase=$3 t=$4 status nb entries users table metrics vias
  status=$(qauld-ctl -d "$d" router v2 status 2>/dev/null)
  nb=$(awk '/^  neighbours/{print $2}' <<<"$status")
  entries=$(awk '/^  routing entries/{print $3}' <<<"$status")
  users=$(qauld-ctl -d "$d" users online 2>/dev/null | grep -c "^[0-9][0-9]* |")
  table=$(qauld-ctl -d "$d" router v2 table 2>/dev/null)
  # columns: space | idx | target | seq | metric | hops | local | transport | via | age
  metrics=$(awk -F'|' 'NF>=10 && $5+0==$5 {gsub(/ /,"",$5); print $5}' <<<"$table" | sort -n | paste -sd, -)
  # A next hop the node no longer counts as a neighbour is a black hole: more
  # distinct vias than neighbours means at least one entry still points at a
  # peer that is gone. Needs no id mapping, which is why it is measured this way.
  vias=$(awk -F'|' 'NF>=10 && $5+0==$5 {gsub(/ /,"",$9); print $9}' <<<"$table" | sort -u | grep -c .)
  printf 't=%-4s phase=%-9s nb=%-3s entries=%-3s users=%-3s vias=%-3s metrics=%s\n' \
    "$t" "$phase" "${nb:-?}" "${entries:-?}" "${users:-?}" "${vias:-?}" "${metrics:-none}" \
    >> "$OUT/$id.log"
}

record() {
  local phase=$1 secs=$2 i t d
  for ((i=0; i<secs; i++)); do
    t=$(( $(date +%s) - START ))
    for d in "${DIRS[@]}"; do
      [ -d "$d" ] || continue
      sample "$d" "$(basename "$d")" "$phase" "$t" &
    done
    wait
    sleep 1
  done
}

echo "recording baseline (10s), output in $OUT"
record baseline 10

case "$MODE" in
  cut)
    for ifc in "ve-$A-$B" "ve-$B-$A"; do
      ip netns exec switch ip link show "$ifc" >/dev/null 2>&1 \
        || { echo "no interface $ifc in the switch namespace"; exit 1; }
    done
    echo "cutting $A <-> $B silently at t=$(( $(date +%s) - START ))s"
    for ifc in "ve-$A-$B" "ve-$B-$A"; do
      ip netns exec switch tc qdisc add dev "$ifc" root netem loss 100%
    done
    record cut "$HOLD"
    echo "restoring at t=$(( $(date +%s) - START ))s"
    for ifc in "ve-$A-$B" "ve-$B-$A"; do
      ip netns exec switch tc qdisc del dev "$ifc" root
    done
    record restored "$HOLD"
    for n in "$A" "$B"; do
      base=$(awk '/phase=baseline/{for(i=1;i<=NF;i++) if ($i ~ /^nb=/) {split($i,a,"="); v=a[2]}} END{print v}' "$OUT/qaul-$n.log")
      last=$(awk '/phase=restored/{for(i=1;i<=NF;i++) if ($i ~ /^nb=/) {split($i,a,"="); v=a[2]}} END{print v}' "$OUT/qaul-$n.log")
      if [ "$base" != "$last" ]; then
        echo "WARNING: $n did not regain its adjacency (baseline nb=$base, now nb=$last)."
        echo "         The mesh is NOT back to its original topology — restart the lab"
        echo "         before the next run or it will measure a different graph."
      fi
    done
    ;;
  kill)
    pid=$(cat "/tmp/qaul-$K.pid" 2>/dev/null)
    [ -n "${pid:-}" ] || { echo "no pid file for $K"; exit 1; }
    echo "killing $K (pid $pid) at t=$(( $(date +%s) - START ))s"
    kill -9 "$pid" && rm -f "/tmp/qaul-$K.pid"
    record killed "$HOLD"
    ;;
esac

echo
echo "========== transitions (only lines where something changed) =========="
for f in "$OUT"/*.log; do
  printf '\n--- %s ---\n' "$(basename "$f" .log)"
  awk '{ key=$0; sub(/^t=[^ ]+ +/,"",key); if (key != prev) { print; prev=key } }' "$f"
done
printf '\nfull per-second logs: %s\n' "$OUT"
