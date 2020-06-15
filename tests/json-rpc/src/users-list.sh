#!/usr/bin/env bash

# returns a list of all users
#
# usage:
# ./users_list.sh

echo "list all users:"

curl -iv \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"users\", 
          \"method\": \"list\"
        }" \
    "http://127.0.0.1:9900/rpc"
