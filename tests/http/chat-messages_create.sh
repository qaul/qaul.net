#!/bin/bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages_create.sh <ROOM_ID>

http POST 127.0.0.1:9900/rest/chat/messages \
    text=\"hello world\" \
    room=\"$1\" \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
