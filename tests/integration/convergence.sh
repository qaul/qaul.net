#!/usr/bin/env bash

set -uo pipefail

N=${1:-0}
DIRS=(/tmp/qaul-*/)
[[ ${#DIRS[@]} -eq 0 || ! -d ${DIRS[0]} ]] && { echo "no /tmp/qaul-*/ dirs found"; exit 1; }
[[ $N -eq 0 ]] && N=${#DIRS[@]}

TIMEOUT=${TIMEOUT:-180}
START=$(date +%s)

declare -A t_neighbour t_routing t_users

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "measuring ${#DIRS[@]} node(s), expecting $N users each; timeout ${TIMEOUT}s"

# One node's three counters. Run in parallel below: done sequentially, a
# 25-node sweep made 50 qauld-ctl calls per iteration and took longer than the
# events it was timing, so milestones landed late or were missed outright.
probe() {
    local d=$1 out=$2 status neighbours entries users
    status=$(sudo qauld-ctl -d "$d" router v2 status 2>/dev/null)
    neighbours=$(awk '/^  neighbours/{print $2}' <<<"$status")
    entries=$(awk '/^  routing entries/{print $3}' <<<"$status")
    users=$(sudo qauld-ctl -d "$d" users online 2>/dev/null | grep -c "^[0-9][0-9]* |")
    printf '%s %s %s\n' "${neighbours:-0}" "${entries:-0}" "${users:-0}" > "$out"
}

while :; do
    t=$(( $(date +%s) - START ))

    for d in "${DIRS[@]}"; do
        id=$(basename "$d")
        # Poll until every milestone is in. Stopping as soon as users were
        # known meant a routing figure that landed later was never recorded,
        # which is what printed "-" for a node that had in fact converged.
        [[ -n ${t_neighbour[$id]:-} && -n ${t_routing[$id]:-} && -n ${t_users[$id]:-} ]] && continue
        probe "$d" "$TMP/$id" &
    done
    wait

    pending=0
    for d in "${DIRS[@]}"; do
        id=$(basename "$d")
        if [[ -r $TMP/$id ]]; then
            read -r neighbours entries users < "$TMP/$id"
            rm -f "$TMP/$id"
            [[ -z ${t_neighbour[$id]:-} && ${neighbours:-0} -ge 1          ]] && t_neighbour[$id]=$t
            [[ -z ${t_routing[$id]:-}   && ${entries:-0}    -ge $((N-1))   ]] && t_routing[$id]=$t
            [[ -z ${t_users[$id]:-}     && ${users:-0}      -ge $N         ]] && t_users[$id]=$t
        fi
        [[ -z ${t_routing[$id]:-} || -z ${t_users[$id]:-} ]] && pending=$((pending+1))
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

# Routing and users are reported separately: `users online` is knowledge, not
# reachability, so a users figure alone once called a run converged when no
# node had installed a full routing table.
slowest_routing=0; missing_routing=0
slowest_users=0;   missing_users=0
for d in "${DIRS[@]}"; do
    id=$(basename "$d")
    r=${t_routing[$id]:-}; u=${t_users[$id]:-}
    if [[ -z $r ]]; then missing_routing=$((missing_routing+1));
    elif [[ $r -gt $slowest_routing ]]; then slowest_routing=$r; fi
    if [[ -z $u ]]; then missing_users=$((missing_users+1));
    elif [[ $u -gt $slowest_users ]]; then slowest_users=$u; fi
done

echo
if [[ $missing_routing -gt 0 ]]; then
    printf 'routing converged: INCOMPLETE, %s node(s) never reached %s entries\n' \
        "$missing_routing" "$((N-1))"
else
    printf 'routing converged: %ss\n' "$slowest_routing"
fi
if [[ $missing_users -gt 0 ]]; then
    printf 'full mesh discovery: INCOMPLETE, %s node(s) short of %s users\n' \
        "$missing_users" "$N"
else
    printf 'full mesh discovery: %ss\n' "$slowest_users"
fi
echo "note: a node now originates its own entry as soon as its identity is bound"
echo "      and as each neighbour appears, so routing figures are propagation."
echo "      Per spec 7.1 that is roughly one relay hop per second."
