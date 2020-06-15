#!/usr/bin/env bash

# returns a list of all chat-rooms for authenticated user
#
# usage:
# ./chat-rooms-list.sh <USER_ID> <USER_TOKEN>

echo "list chat-rooms for user: $1"

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"list\",
          \"auth\": {
            \"id\":\"$1\",
            \"token\":\"$2\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"

