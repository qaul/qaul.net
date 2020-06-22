#!/bin/sh

# creates a new user with password '123456'
# 
# usage:
# ./user-create.sh

http -v POST 127.0.0.1:9900/http/user \
    pw=123456
