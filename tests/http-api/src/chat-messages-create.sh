#!/bin/bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages_create.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http POST 127.0.0.1:9900/rest/chat-messages/$TRIMMED_ID \
    id=\"$1\" \
    name:='{"set":"Test-Room Name"}' \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
