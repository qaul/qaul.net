#!/bin/bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages-create.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http POST 127.0.0.1:9900/rest/chat-messages/$TRIMMED_ID \
    room="$1" \
    text="hello world!" \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
