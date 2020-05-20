#!/bin/bash

# creates a new user with password '123456'
# 
# usage:
# ./users_create.sh

curl -iv  \
    -H "Content-Type: application/json" \
    -d '{
        "id": "1",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }' \
    "http://127.0.0.1:9900/rpc"

