#!/usr/bin/env bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-message-create.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http -v POST 127.0.0.1:9900/http/chat_message \
    room="$1" \
    text="hello world!" \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
