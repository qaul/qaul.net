#!/bin/bash

set -ex

# Create users
source src/users-bootstrap.sh

# Create a chat-room for user A
source src/chat-rooms-create.sh "$B_ID" "$A_ID" $A_TOKEN

# List all chat-rooms for user A
source src/chat-rooms-list.sh "$A_ID" $A_TOKEN

# Modify a chat-room
source src/chat-rooms-modify.sh "$ROOM_ID" "$A_ID" $A_TOKEN

# Get a chat-room
source src/chat-rooms-get.sh "$ROOM_ID" "$A_ID" $A_TOKEN

## TODO: delete chat-room does not exist, 
##       maybe 'leave group' for real group chat-rooms
##
# delete chat-room
#source src/chat-rooms-delete.sh "$ROOM_ID" "$A_ID" $A_TOKEN

echo "Done"
