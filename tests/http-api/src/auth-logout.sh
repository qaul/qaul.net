#!/bin/sh

# logout from session
# 
# usage
# ./auth-logout.sh <USER_ID> <USER_TOKEN>
#

http -v DELETE 127.0.0.1:9900/http/auth \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
