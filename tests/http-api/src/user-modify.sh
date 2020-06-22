#!/usr/bin/env bash

# Set `dispaly_name` and `real_name`
# 
# usage
# ./users-modify.sh <USER_ID> <USER_TOKEN>
#

http -v PATCH 127.0.0.1:9900/http/user/$1 \
    display_name:='{"set":"testuser"}' \
    real_name:='{"set":"My Real Name"}' \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
