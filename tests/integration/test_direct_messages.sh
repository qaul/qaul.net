#!/usr/bin/env bash
# test_direct_messages.sh
#
# Usage: bash test_direct_messages.sh <node_id> [node_id ...]
# Nodes must be sorted (caller's responsibility).
# Progress → stderr, JSON result → stdout.

NODES=("$@")
SENDER="${NODES[0]}"
RECIPIENT="${NODES[-1]}"
MID_IDX=$(( ${#NODES[@]} / 2 ))
SNOOPER="${NODES[$MID_IDX]}"
DELIVERY_WAIT=60
POLL=5

sock() { echo "/tmp/qaul-$1/qauld.sock"; }

echo "  sender=$SENDER  recipient=$RECIPIENT  snooper=$SNOOPER" >&2

# get group_id for recipient from sender's users list
RECIPIENT_NAME="test-$RECIPIENT"
GROUP_ID=$(qauld-ctl -s "$(sock "$SENDER")" --json users list \
    | jq -r --arg name "$RECIPIENT_NAME" '.[] | select(.name==$name) | .group_id')

if [[ -z "$GROUP_ID" || "$GROUP_ID" == "null" ]]; then
    echo "{\"passed\": false, \"notes\": \"$RECIPIENT_NAME not found in $SENDER users list\"}"
    exit 1
fi
echo "  group_id=$GROUP_ID" >&2

# send message
CONTENT="dm-$(date +%s)-$$"
qauld-ctl -s "$(sock "$SENDER")" chat send --group-id "$GROUP_ID" --message "$CONTENT" >/dev/null
echo "  sent: $CONTENT" >&2

# poll recipient until message appears
DELIVERED_AT=""
START=$(date +%s)
while (( $(date +%s) - START < DELIVERY_WAIT )); do
    COUNT=$(qauld-ctl -s "$(sock "$RECIPIENT")" --json chat conversation \
        --group-id "$GROUP_ID" --index 0 2>/dev/null \
        | jq --arg c "$CONTENT" '[.messages[].content[] | select(. == $c)] | length' 2>/dev/null \
        || echo 0)
    if (( ${COUNT:-0} > 0 )); then
        DELIVERED_AT=$(( $(date +%s) - START ))
        echo "  delivered in ${DELIVERED_AT}s" >&2
        break
    fi
    sleep $POLL
done

if [[ -z "$DELIVERED_AT" ]]; then
    echo "{\"passed\": false, \"notes\": \"not delivered to $RECIPIENT within ${DELIVERY_WAIT}s\"}"
    exit 1
fi

# check snooper has nothing
SNOOPER_COUNT=$(qauld-ctl -s "$(sock "$SNOOPER")" --json chat conversation \
    --group-id "$GROUP_ID" --index 0 2>/dev/null \
    | jq --arg c "$CONTENT" '[.messages[].content[] | select(. == $c)] | length' 2>/dev/null \
    || echo 0)

if (( ${SNOOPER_COUNT:-0} > 0 )); then
    echo "{\"passed\": false, \"notes\": \"snooper $SNOOPER received the direct message\"}"
    exit 1
fi

echo "  PASS: delivered in ${DELIVERED_AT}s, not on snooper $SNOOPER" >&2
echo "{\"passed\": true, \"sender\": \"$SENDER\", \"recipient\": \"$RECIPIENT\", \"snooper\": \"$SNOOPER\", \"delivery_time_s\": $DELIVERED_AT, \"notes\": \"delivered in ${DELIVERED_AT}s, not seen on $SNOOPER\"}"
