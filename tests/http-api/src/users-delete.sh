#!/usr/bin/env bash

# delete the currently authorized user
#
# usage:
# ./users-delete.sh <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http DELETE 127.0.0.1:9900/rest/users/$TRIMMED_ID \
  purge:=true \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
