#!/bin/sh

# creates a new user on node B with password '123456'
# 
# usage:
# ./users_create.sh

http POST 127.0.0.1:9901/rest/users \
    pw=123456
