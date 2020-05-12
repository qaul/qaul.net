#!/usr/bin/env bash

set -e

source ./bootstrap-users.sh

echo "Creating room for $A_ID"

# creates a new chat room
# 
# usage:
# ./chat-rooms_create.sh

RETURN=$(curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"create\",
          \"data\": {
            \"users\": [\"$B_ID\"]
          },
          \"auth\": {
            \"id\":\"$A_ID\",
            \"token\":\"$A_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/rpc" 2>/dev/null | tail -n 1)

export ROOM_ID=$(echo $RETURN | jq '.data.room_id[0]' | sed -e 's/"//g')
