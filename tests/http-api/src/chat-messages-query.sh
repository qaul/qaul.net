#!/usr/bin/env bash

# queries all chat message of a specific chat room
# 
# usage:
# ./chat-messages-query.sh <ROOM_ID> <USER_ID> <USER_TOKEN>

http GET 127.0.0.1:9901/rest/chat-messages?chat-room=$1 \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
