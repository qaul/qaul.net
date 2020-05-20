#!/usr/bin/env bash

set -ex

source src/chat-rooms-create.sh 

# creates and sends a new chat message to a specific room
# 
# usage:
# ./chat-messages_create.sh

curl -i  \
    -H "Content-Type: application/json" \
    -d "{
        \"id\": \"1\",
        \"kind\": \"chat-messages\",
        \"method\": \"create\",
        \"data\": {
            \"text\": \"hello world!\",
            \"room\": \"$ROOM_ID\"
        },
        \"auth\": {
            \"id\":\"$A_ID\",
            \"token\":\"$A_TOKEN\"
        }
    }" \
    "http://127.0.0.1:9900/rpc"

## Sleep a bit
sleep 1;

## Hard mode: get the message from B!

echo "Was your message..." $(curl -i  \
    -H "Content-Type: application/json" \
    -d "{
        \"id\": \"1\",
        \"kind\": \"chat-messages\",
        \"method\": \"get\",
        \"data\": {
            \"room\": \"$ROOM_ID\"
        },
        \"auth\": {
            \"id\":\"$B_ID\",
            \"token\":\"$B_TOKEN\"
        }
    }" \
    "http://127.0.0.1:9901/rpc" 2>/dev/null | tail -n 1 | jq '.data.chat_message[0].content') "?"
