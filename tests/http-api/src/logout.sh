#!/bin/sh

# logout from session
# 
# usage
# ./logout.sh <USER_ID> <USER_TOKEN>
#

http GET 127.0.0.1:9900/rest/logout \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
