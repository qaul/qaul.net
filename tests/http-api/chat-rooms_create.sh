#!/bin/bash

# creates a new chat room
# 
# usage:
# ./chat-rooms_create.sh <USER_ID>

http --offline POST 127.0.0.1:9900/rest/chat/rooms \
    users:="[\"$1\"]" \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
