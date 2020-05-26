#!/bin/sh

# Get a the information of the currently authorized user
# 
# usage
# ./users-get.sh <USER_ID> <USER_TOKEN>
#

TRIMMED_ID=$(echo $1 | tr -d ' ')

http GET 127.0.0.1:9900/rest/users/$TRIMMED_ID \
    "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
