#!/bin/bash

# delete a chatroom by id
# 
# usage:
# ./chat-rooms_delete.sh <CHATROOM_ID>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"delete\",
          \"data\": {
            \"id\": \"$1\"
          },
          \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"

