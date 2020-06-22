#!/usr/bin/env bash

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-message-create.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{
        \"id\": \"1\",
        \"kind\": \"chat_message\",
        \"method\": \"create\",
        \"data\": {
            \"text\": \"hello world!\",
            \"room\": \"$1\"
        },
        \"auth\": {
            \"id\":\"$2\",
            \"token\":\"$3\"
        }
    }" \
    "http://127.0.0.1:9900/rpc"
