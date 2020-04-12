#!/bin/bash

# creates a new chat room
# 
# usage:
# ./chat-rooms_create.sh

http POST 127.0.0.1:9900/rest/chat/rooms \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
