#!/usr/bin/env bash

set -uo pipefail

N=${1:-0}
DIRS=(/tmp/qaul-*/)
[[ ${#DIRS[@]} -eq 0 || ! -d ${DIRS[0]} ]] && { echo "no /tmp/qaul-*/ dirs found"; exit 1; }
[[ $N -eq 0 ]] && N=${#DIRS[@]}

TIMEOUT=${TIMEOUT:-180}
START=$(date +%s)

declare -A t_neighbour t_routing t_users

echo "measuring ${#DIRS[@]} node(s), expecting $N users each; timeout ${TIMEOUT}s"

while :; do
    t=$(( $(date +%s) - START ))
    pending=0

    for d in "${DIRS[@]}"; do
        id=$(basename "$d")
        [[ -n ${t_users[$id]:-} ]] && continue

        status=$(sudo qauld-ctl -d "$d" router v2 status 2>/dev/null)
        neighbours=$(awk '/^  neighbours/{print $2}' <<<"$status")
        entries=$(awk '/^  routing entries/{print $3}' <<<"$status")
        users=$(sudo qauld-ctl -d "$d" users online 2>/dev/null | grep -c "^[0-9] |")

        [[ -z ${t_neighbour[$id]:-} && ${neighbours:-0} -ge 1 ]] && t_neighbour[$id]=$t
        [[ -z ${t_routing[$id]:-}   && ${entries:-0}    -ge $((N-1)) ]] && t_routing[$id]=$t
        [[ -z ${t_users[$id]:-}     && ${users:-0}      -ge $N ]] && t_users[$id]=$t

        [[ -z ${t_users[$id]:-} ]] && pending=$((pending+1))
    done

    [[ $pending -eq 0 ]] && break
    if [[ $t -ge $TIMEOUT ]]; then
        echo "timed out at ${t}s with $pending node(s) short"
        break
    fi
    sleep 1
done

# "-" when a node never reached the milestone, otherwise "13s"
fmt() { [[ -z ${1:-} ]] && printf -- '-' || printf '%ss' "$1"; }

echo
echo "all figures are seconds since the daemons started"
printf '%-14s %12s %12s %12s\n' node neighbour-at routing-at users-at
printf '%-14s %12s %12s %12s\n' -------------- ------------ ------------ ------------
for d in "${DIRS[@]}"; do
    id=$(basename "$d")
    printf '%-14s %12s %12s %12s\n' "$id" \
        "$(fmt "${t_neighbour[$id]:-}")" "$(fmt "${t_routing[$id]:-}")" "$(fmt "${t_users[$id]:-}")"
done

slowest=0
for d in "${DIRS[@]}"; do
    id=$(basename "$d")
    v=${t_users[$id]:-}
    [[ -n $v && $v -gt $slowest ]] && slowest=$v
done
printf '\nfull mesh discovery: %ss\n' "$slowest"
echo "note: the first origin tick is at t=10s (router_v2/init.rs spawn_origin_tick),"
echo "      so ~10s of any routing figure is startup delay, not propagation."
