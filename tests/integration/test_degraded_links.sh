#!/usr/bin/env bash
# test_degraded_links.sh
#
# Usage: bash test_degraded_links.sh <node_id> [node_id ...]
# Applies 30% packet loss on the middle node's uplink, sends 10 feed messages
# from the first node, asserts the far node receives at least 50%.
# Progress → stderr, JSON result → stdout.

NODES=("$@")
SENDER="${NODES[0]}"
FAR="${NODES[-1]}"
MID_IDX=$(( ${#NODES[@]} / 2 ))
LOSS_NODE="${NODES[$MID_IDX]}"
LOSS_PERCENT=30
SEND_COUNT=10
PROP_WAIT=90
MIN_RATE=50   # minimum delivery percent expected on far node

sock() { echo "/tmp/qaul-$1/qauld.sock"; }

get_pid() {
    pgrep -f "qauld --name=test-$1"
}

apply_loss() {
    local pid
    pid=$(get_pid "$1")
    sudo nsenter --net="/proc/${pid}/ns/net" \
        tc qdisc add dev uplink root netem loss "${LOSS_PERCENT}%"
}

remove_loss() {
    local pid
    pid=$(get_pid "$1")
    sudo nsenter --net="/proc/${pid}/ns/net" \
        tc qdisc del dev uplink root 2>/dev/null || true
}

echo "  applying ${LOSS_PERCENT}% packet loss on node $LOSS_NODE" >&2
apply_loss "$LOSS_NODE"

# send messages
STAMP=$(date +%s)
CONTENTS=()
for i in $(seq -w 1 $SEND_COUNT); do
    CONTENT="loss-${STAMP}-${i}"
    qauld-ctl -s "$(sock "$SENDER")" feed send --message "$CONTENT" >/dev/null
    CONTENTS+=("$CONTENT")
done
echo "  sent $SEND_COUNT messages from $SENDER, waiting ${PROP_WAIT}s..." >&2
sleep $PROP_WAIT

echo "  removing packet loss..." >&2
remove_loss "$LOSS_NODE"

# measure delivery on each node
RATES="{"
SEP=""
for NODE in "${NODES[@]}"; do
    RECEIVED=0
    for CONTENT in "${CONTENTS[@]}"; do
        COUNT=$(qauld-ctl -s "$(sock "$NODE")" --json feed list \
            | jq --arg c "$CONTENT" 'map(select(.content == $c)) | length' 2>/dev/null \
            || echo 0)
        if (( ${COUNT:-0} > 0 )); then
            (( RECEIVED += 1 )) || true
        fi
    done
    RATE_PCT=$(( RECEIVED * 100 / SEND_COUNT ))
    echo "  node $NODE: $RECEIVED/$SEND_COUNT (${RATE_PCT}%)" >&2
    RATES="${RATES}${SEP}\"${NODE}\":{\"received\":$RECEIVED,\"sent\":$SEND_COUNT,\"rate_pct\":$RATE_PCT}"
    SEP=","
done
RATES="${RATES}}"

# check far node meets minimum rate
FAR_RECEIVED=$(echo "$RATES" | jq --arg n "$FAR" '.[$n].received')
FAR_RATE=$(echo "$RATES" | jq --arg n "$FAR" '.[$n].rate_pct')

if (( FAR_RATE >= MIN_RATE )); then
    echo "  PASS: $FAR received ${FAR_RATE}% (min ${MIN_RATE}%)" >&2
    echo "{\"passed\": true, \"loss_percent\": $LOSS_PERCENT, \"degraded_node\": \"$LOSS_NODE\", \"delivery_rates\": $RATES, \"notes\": \"${LOSS_PERCENT}% loss on $LOSS_NODE: $FAR received ${FAR_RATE}%\"}"
    exit 0
else
    echo "  FAIL: $FAR received only ${FAR_RATE}% (min ${MIN_RATE}%)" >&2
    echo "{\"passed\": false, \"loss_percent\": $LOSS_PERCENT, \"degraded_node\": \"$LOSS_NODE\", \"delivery_rates\": $RATES, \"notes\": \"${LOSS_PERCENT}% loss on $LOSS_NODE: $FAR only ${FAR_RATE}% (min ${MIN_RATE}%)\"}"
    exit 1
fi
