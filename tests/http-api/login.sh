#!/bin/sh

# a user with $QAUL_ID and password '123456'
# 
# usage:
# ./login.sh

http POST 127.0.0.1:9900/rest/login \
    id="$QAUL_ID" \
    pw=123456
