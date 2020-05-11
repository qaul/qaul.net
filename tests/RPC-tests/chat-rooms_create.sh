#!/bin/bash

# creates a new chat room
# 
# usage:
# ./chat-rooms_create.sh <USER_ID>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"create\",
          \"data\": {
            \"users\": [\"$1\"]
          },
          \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"
