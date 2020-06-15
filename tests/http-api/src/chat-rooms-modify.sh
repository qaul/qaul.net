#!/usr/bin/env bash

# modify chat-room: set chat-room name
#
# usage:
# ./chat-rooms-modify.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http 127.0.0.1:9900/rest/chat-rooms/$TRIMMED_ID \
  "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
