#!/usr/bin/env bash

# returns a list of all contacts
#
# usage:
# ./contacts-list.sh <USER_ID> <USER_TOKEN>

http 127.0.0.1:9900/rest/contacts \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
