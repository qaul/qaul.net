#!/bin/bash

# Set `dispaly_name` and `real_name`
# 
# usage
# ./users-modify.sh <USER_ID> <USER_TOKEN>
#

TRIMMED_ID=$(echo $1 | tr -d ' ')

http PATCH 127.0.0.1:9900/rest/users/$TRIMMED_ID \
    display_name:='{"set":"testuser"}' \
    real_name:='{"set":"My Real Name"}' \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
