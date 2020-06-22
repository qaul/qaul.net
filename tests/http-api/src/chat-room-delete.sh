#!/usr/bin/env bash

# delete a chat-room by id
# 
# usage:
# ./chat-room-delete.sh <CHATROOM_ID> <USER_ID> <USER_TOKEN>

http -v DELETE 127.0.0.1:9900/http/chat_room/$1 \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
