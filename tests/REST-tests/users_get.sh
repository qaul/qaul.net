#!/bin/sh

# Get a the information of the currently authorized user
# 
# usage
# ./users_get.sh
#

TRIM=$(echo $QAUL_ID | tr -d ' ')

http GET 127.0.0.1:9900/rest/users/$TRIM \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
