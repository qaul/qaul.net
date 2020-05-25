#!/usr/bin/env bash

# receives a new chat message from a specific room
# 
# usage:
# ./chat-messages_get.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

RETURN=$(curl -i  \
    -H "Content-Type: application/json" \
    -d "{
        \"id\": \"1\",
        \"kind\": \"chat-messages\",
        \"method\": \"get\",
        \"data\": {
            \"room\": \"$1\"
        },
        \"auth\": {
            \"id\":\"$2\",
            \"token\":\"$3\"
        }
    }" \
    "http://127.0.0.1:9901/rpc" 2>/dev/null | tail -n 1)
    
export MSG=$(echo $RETURN | jq '.data.chat_message[1].content')

echo "Message received: $MSG"
