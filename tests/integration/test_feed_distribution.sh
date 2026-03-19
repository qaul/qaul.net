#!/usr/bin/env bash
# test_feed_distribution.sh
#
# Usage: bash test_feed_distribution.sh <node_id> [node_id ...]
# Sends one feed message from each node, asserts all nodes receive all messages.
# Progress → stderr, JSON result → stdout.

NODES=("$@")
PROP_WAIT=120
POLL=5
STAMP=$(date +%s)

sock() { echo "/tmp/qaul-$1/qauld.sock"; }

# send one unique message from each node
declare -A SENT
for NODE in "${NODES[@]}"; do
    CONTENT="feed-dist-${NODE}-${STAMP}"
    qauld-ctl -s "$(sock "$NODE")" feed send --message "$CONTENT" >/dev/null
    SENT[$NODE]="$CONTENT"
    echo "  sent from $NODE: $CONTENT" >&2
done

# poll until all nodes have all messages
START=$(date +%s)
while (( $(date +%s) - START < PROP_WAIT )); do
    ALL_DONE=true
    for NODE in "${NODES[@]}"; do
        for SENDER_NODE in "${NODES[@]}"; do
            CONTENT="${SENT[$SENDER_NODE]}"
            COUNT=$(qauld-ctl -s "$(sock "$NODE")" --json feed list \
                | jq --arg c "$CONTENT" 'map(select(.content == $c)) | length' 2>/dev/null \
                || echo 0)
            if (( ${COUNT:-0} == 0 )); then
                ALL_DONE=false
                break 2
            fi
        done
    done

    if $ALL_DONE; then
        ELAPSED=$(( $(date +%s) - START ))
        echo "  PASS: all ${#NODES[@]} nodes have all ${#NODES[@]} messages after ${ELAPSED}s" >&2
        echo "{\"passed\": true, \"node_count\": ${#NODES[@]}, \"message_count\": ${#NODES[@]}, \"propagation_time_s\": $ELAPSED, \"notes\": \"all ${#NODES[@]} nodes received all ${#NODES[@]} messages in ${ELAPSED}s\"}"
        exit 0
    fi
    sleep $POLL
done

# collect what's missing for the failure report
MISSING="["
SEP=""
for NODE in "${NODES[@]}"; do
    for SENDER_NODE in "${NODES[@]}"; do
        CONTENT="${SENT[$SENDER_NODE]}"
        COUNT=$(qauld-ctl -s "$(sock "$NODE")" --json feed list \
            | jq --arg c "$CONTENT" 'map(select(.content == $c)) | length' 2>/dev/null \
            || echo 0)
        if (( ${COUNT:-0} == 0 )); then
            MISSING="${MISSING}${SEP}{\"node\":\"$NODE\",\"missing\":\"$CONTENT\"}"
            SEP=","
        fi
    done
done
MISSING="${MISSING}]"

echo "  FAIL: propagation incomplete after ${PROP_WAIT}s" >&2
echo "{\"passed\": false, \"notes\": \"propagation incomplete after ${PROP_WAIT}s\", \"missing\": $MISSING}"
exit 1
