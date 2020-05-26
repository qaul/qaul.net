#!/bin/sh

# a user with <USER_ID> and password '123456'
# 
# usage:
# ./login.sh <USER_ID>

http POST 127.0.0.1:9900/rest/login \
    id="$1" \
    pw=123456
