#!/usr/bin/env bash

# creates a new chat room with user <FRIEND_ID>
# 
# usage:
# ./chat-rooms_create.sh <FRIEND_ID> <USER_ID> <USER_TOKEN>

RETURN=$(http POST 127.0.0.1:9900/rest/chat-rooms \
    users:="[\"$1\"]" \
    name="Test Name" \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}" 2>/dev/null | tail -n 1)

export ROOM_ID=$(echo $RETURN | jq '.chat_room.id' | sed -e 's/"//g')

echo "Created room: $ROOM_ID"
