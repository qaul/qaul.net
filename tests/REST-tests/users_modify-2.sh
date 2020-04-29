#!/bin/bash

# Set `dispaly_name` 'testuser' and unset `real_name`
# 
# usage
# ./users_modify.sh
#

http PATCH 127.0.0.1:9900/rest/users/$QAUL_ID \
    display_name:='{"set":"testuser"}' \
    real_name=unset \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
