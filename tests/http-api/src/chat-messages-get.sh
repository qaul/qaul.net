#!/bin/bash

# retrieves a new chat message for a specific room
# 
# usage:
# ./chat-messages-get.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http GET 127.0.0.1:9901/rest/chat-messages/$TRIMMED_ID \
    room="$1" \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
