#!/usr/bin/env bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages-create.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http POST 127.0.0.1:9900/rest/chat-messages \
    room="$1" \
    text="hello world!" \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
