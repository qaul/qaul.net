#!/bin/sh

# validate a session token
# 
# usage
# ./auth-validate.sh <USER_ID> <USER_TOKEN>
#

http -v GET 127.0.0.1:9900/http/auth \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
