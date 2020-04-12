#!/bin/sh

# creates a new user with password '123456'
# 
# usage:
# ./users_create.sh

http POST 127.0.0.1:9900/rest/users \
    pw=123456
