#!/usr/bin/env bash

# get chat room information
#
# usage:
# ./chat-room-get.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http -v 127.0.0.1:9900/http/chat_room/$1 \
  "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
