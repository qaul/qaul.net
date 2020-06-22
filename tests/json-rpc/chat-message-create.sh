#!/usr/bin/env bash

set -ex

# Create users
source src/user-bootstrap.sh

# Create a chat room for user A
source src/chat-room-create.sh "$B_ID" "$A_ID" $A_TOKEN

# Send a message from user A
source src/chat-message-create.sh "$ROOM_ID" "$A_ID" $A_TOKEN

## Sleep a bit
sleep 1;

# Receive the message from user B
source src/chat-message-query.sh "$ROOM_ID" "$B_ID" $B_TOKEN
