#!/usr/bin/env bash

# returns a list of all contacts of node A
#
# usage:
# ./contact-list.sh

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"contact\", 
          \"method\": \"list\",
          \"auth\": {
            \"id\":\"$A_ID\",
            \"token\":\"$A_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/rpc"

