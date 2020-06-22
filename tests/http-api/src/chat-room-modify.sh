#!/usr/bin/env bash

# modify a chat room: set a chat room name
#
# usage:
# ./chat-room-modify.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http -v PATCH 127.0.0.1:9900/http/chat_room/$1 \
  id=$1 \
  set:='{"name":"My Room Name"}' \
  "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
