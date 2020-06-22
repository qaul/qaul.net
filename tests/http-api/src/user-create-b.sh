#!/bin/sh

# creates a new user on node B with password '123456'
# 
# usage:
# ./user-create-b.sh

http -v POST 127.0.0.1:9901/http/user \
    pw=123456
