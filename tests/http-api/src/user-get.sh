#!/bin/sh

# Get a the information of the currently authorized user
# 
# usage
# ./user-get.sh <USER_ID> <USER_TOKEN>
#

http -v GET 127.0.0.1:9900/http/user/$1 \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
