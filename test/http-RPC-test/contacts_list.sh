#!/bin/bash

# returns a list of all contacts
#
# usage:
# ./contacts_list.sh

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ \"id\": \"1\", 
          \"kind\": \"contacts\", 
          \"method\": \"list\",
          \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
          }
        }" \
    "http://127.0.0.1:9900/api"

