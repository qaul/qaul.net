#!/bin/bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages_create.sh <ROOM_ID>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{
        \"id\": \"1\",
        \"kind\": \"chat-messages\",
        \"method\": \"create\",
        \"data\": {
            \"text\": \"hello world!\",
            \"room\": \"$1\"
        },
        \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
        }
    }" \
    "http://127.0.0.1:9900/api"

