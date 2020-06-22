#!/usr/bin/env bash

# returns a list of all chat rooms
#
# usage:
# ./chat-room-list.sh <USER_ID> <USER_TOKEN>

http -v 127.0.0.1:9900/http/chat_room \
  "Authorization:{\"id\":\"$1\",\"token\":\"$2\"}"
