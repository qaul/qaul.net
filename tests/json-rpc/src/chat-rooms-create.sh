#!/usr/bin/env bash

# creates a new chat room
# 
# usage:
# ./chat-rooms_create.sh <Friend> <USER_ID> <USER_TOKEN>

RETURN=$(curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"create\",
          \"data\": {
            \"users\": [\"$1\"]
          },
          \"auth\": {
            \"id\":\"$2\",
            \"token\":\"$3\"
          }
        }" \
    "http://127.0.0.1:9900/rpc" 2>/dev/null | tail -n 1)

export ROOM_ID=$(echo $RETURN | jq '.data.room_id[0]' | sed -e 's/"//g')

echo "Created room: $ROOM_ID"
