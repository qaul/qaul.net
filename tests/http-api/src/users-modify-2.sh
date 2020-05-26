#!/bin/bash

# Set `dispaly_name` and unset `real_name`
# 
# usage
# ./users-modify-2.sh <USER_ID> <USER_TOKEN>
#

TRIMMED_ID=$(echo $1 | tr -d ' ')

http PATCH 127.0.0.1:9900/rest/users/$TRIMMED_ID \
    display_name:='{"set":"testuser"}' \
    real_name=unset \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
