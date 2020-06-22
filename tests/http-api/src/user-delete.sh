#!/usr/bin/env bash

# delete the currently authorized user
#
# usage:
# ./user-delete.sh <USER_ID> <USER_TOKEN>

http -v DELETE 127.0.0.1:9900/http/user/$1 \
  purge:=true \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
