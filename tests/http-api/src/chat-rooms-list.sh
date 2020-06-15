#!/usr/bin/env bash

# returns a list of all chat-rooms
#
# usage:
# ./chat-rooms-list.sh <USER_ID> <USER_TOKEN>

http 127.0.0.1:9900/http/chat-rooms \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
