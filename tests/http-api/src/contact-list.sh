#!/usr/bin/env bash

# returns a list of all contacts
#
# usage:
# ./contact-list.sh <USER_ID> <USER_TOKEN>

http -v 127.0.0.1:9900/http/contact \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
