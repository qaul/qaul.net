#!/bin/bash

# delete a chat-room by id
# 
# usage:
# ./chat-rooms-delete.sh <CHATROOM_ID> <USER_ID> <USER_TOKEN>

TRIMMED_ID=$(echo $1 | tr -d ' ')

http DELETE 127.0.0.1:9900/rest/chat/rooms/$TRIMMED_ID \
    "Authorization:{\"id\":\"$2\",\"token\":\"$3\"}"
