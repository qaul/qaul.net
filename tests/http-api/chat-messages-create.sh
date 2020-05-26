#!/usr/bin/env bash

set -ex

# Create users
source src/users-bootstrap.sh

# Create a chat-room for user A
source src/chat-rooms-create.sh "$B_ID" "$A_ID" $A_TOKEN

# Send a message from user A
source src/chat-messages-create.sh "$ROOM_ID" "$A_ID" $A_TOKEN

## Sleep a bit
sleep 1;

# Receive the message from user B
source src/chat-messages-get.sh "$ROOM_ID" "$B_ID" $B_TOKEN
