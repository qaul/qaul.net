#!/bin/bash

# create a new user

curl -i  \
    -H "Content-Type: application/json" \
    -d '{
        "id": "1",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }' \
    "http://127.0.0.1:9900/api"

