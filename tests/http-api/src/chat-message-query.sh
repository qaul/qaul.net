#!/usr/bin/env bash

# queries all chat message of a specific chat room
# 
# usage:
# ./chat-message-query.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http -v GET 127.0.0.1:9901/http/chat_message?chat_room=$1 \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
