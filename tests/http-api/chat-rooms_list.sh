#!/bin/bash

# returns a list of all chat-rooms
#
# usage:
# ./chat-rooms_list.sh

http 127.0.0.1:9900/rest/chat-rooms \
  "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
