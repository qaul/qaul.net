#!/bin/sh

# logout from session
# 
# usage
# ./logout.sh <USER_ID> <USER_TOKEN>
#

http POST 127.0.0.1:9900/http/logout \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
