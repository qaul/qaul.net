#!/bin/bash

# delete a chatroom by id
# 
# usage:
# ./chat-rooms_delete.sh <CHATROOM_ID>

http DELETE 127.0.0.1:9900/rest/chat/rooms/$1 \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
