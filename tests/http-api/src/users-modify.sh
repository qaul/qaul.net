#!/usr/bin/env bash

# Set `dispaly_name` and `real_name`
# 
# usage
# ./users-modify.sh <USER_ID> <USER_TOKEN>
#

TRIMMED_ID=$(echo $1 | tr -d ' ')

http PATCH 127.0.0.1:9900/http/users/$TRIMMED_ID \
    display_name=testuser \
    real_name="My Real Name" \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
