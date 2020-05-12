#!/usr/bin/env bash

set -x

## Create user on node A

NODE_A=$(curl -iv \
    -H "Content-Type: application/json" \
    -d '{
        "id": "1",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }' \
    "http://127.0.0.1:9900/rpc" 2> /dev/null | tail -n 1)

export A_ID=$(echo $NODE_A | jq '.data.auth.id' | sed -e 's/"//g')
export A_TOKEN=$(echo $NODE_A | jq '.data.auth.token' | sed -e 's/"//g')

## Creat a user on node B

NODE_B=$(curl -iv \
    -H "Content-Type: application/json" \
    -d '{
        "id": "1",
        "kind": "users",
        "method": "create",
        "data": {
            "pw": "123456"
        }
    }' \
    "http://127.0.0.1:9901/rpc" 2> /dev/null | tail -n 1)

export B_ID=$(echo $NODE_B | jq '.data.auth.id' | sed -e 's/"//g')
export B_TOKEN=$(echo $NODE_B | jq '.data.auth.token' | sed -e 's/"//g')

## Wait just a little bit
sleep 1
