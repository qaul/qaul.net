#!/bin/sh

# creates a new user on node B with password '123456'
# 
# usage:
# ./users-create.sh

http POST 127.0.0.1:9901/http/users \
    pw=123456
