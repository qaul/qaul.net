#!/usr/bin/env bash

# delete the authentiticated user
#
# usage:
# ./users_delete.sh <USER_ID> <USER_TOKEN>

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"users\", 
          \"method\": \"delete\",
          \"data\": {
            \"purge\": true
          },
          \"auth\": {
            \"id\":\"$1\",
            \"token\":\"$2\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"
