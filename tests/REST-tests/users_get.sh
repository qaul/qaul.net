#!/bin/sh

# Get a the information of the currently authorized user
# 
# usage
# ./users_get.sh
#

http GET 127.0.0.1:9900/rest/users/$QAUL_ID \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
