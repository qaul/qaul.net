#!/usr/bin/env bash

# get chat-room information
#
# usage:
# ./chat-rooms-get.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http 127.0.0.1:9900/http/chat-rooms/$TRIMMED_ID \
  "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
