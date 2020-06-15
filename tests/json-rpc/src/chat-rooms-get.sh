#!/usr/bin/env bash

# get a chatroom by id
# 
# usage:
# ./chat-rooms_get.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"get\",
          \"data\": {
            \"id\": \"$1\"
          },
          \"auth\": {
            \"id\":\"$2\",
            \"token\":\"$3\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"

