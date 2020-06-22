#!/usr/bin/env bash

set -ex

# Create users
source src/user-bootstrap.sh

# Create a chat room for user A
source src/chat-room-create.sh "$B_ID" "$A_ID" $A_TOKEN

# List all chat rooms for user A
source src/chat-room-list.sh "$A_ID" $A_TOKEN

# Modify a chat room
source src/chat-room-modify.sh "$ROOM_ID" "$A_ID" $A_TOKEN

# Get a chat room
source src/chat-room-get.sh "$ROOM_ID" "$A_ID" $A_TOKEN

## TODO: delete chat room does not exist, 
##       maybe 'leave group' for real group chat rooms
##
# delete chat room
#source src/chat-room-delete.sh "$ROOM_ID" "$A_ID" $A_TOKEN

echo "Done"
