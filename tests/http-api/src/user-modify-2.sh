#!/usr/bin/env bash

# Set `dispaly_name` and unset `real_name`
# 
# usage
# ./user-modify-2.sh <USER_ID> <USER_TOKEN>
#

http -v PATCH 127.0.0.1:9900/http/user/$1 \
    display_name:='{"set":"testuser"}' \
    real_name:='{"set":"Test User Name"}' \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
