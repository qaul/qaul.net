#!/bin/bash

# creates a new user with password '123456'
# 
# usage:
# ./chat-rooms_create.sh

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"chat-rooms\", 
          \"method\": \"create\",
          \"data\": {
            
          },
          \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"
