#!/bin/bash

# Set `dispaly_name` 'testuser' and unset `real_name`
# 
# usage
# ./users_modify.sh
#

curl -i  \
    -H "Content-Type: application/json" \
    -d "{ 
        \"id\": \"1\", 
        \"kind\": \"users\", 
        \"method\": \"modify\",
        \"data\": {
            \"display_name\": {
				\"set\": \"testuser\"
			},
            \"real_name\": \"unset\"
        },
        \"auth\": {
            \"id\":\"$QAUL_ID\",
            \"token\":\"$QAUL_TOKEN\"
        }
    }" \
    "http://127.0.0.1:9900/api"
