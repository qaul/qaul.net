#!/usr/bin/env bash

set -e

## Create user on node A
echo "create user on node A"
NODE_A=$(${BASH_SOURCE%/*}/users-create.sh 2> /dev/null | tail -n 1)

export A_ID=$(echo $NODE_A | jq '.auth.id' | sed -e 's/"//g')
export A_TOKEN=$(echo $NODE_A | jq '.auth.token' | sed -e 's/"//g')

## Creat a user on node B
echo "create user on node B"
NODE_B=$(${BASH_SOURCE%/*}/users-create-b.sh 2> /dev/null | tail -n 1)

export B_ID=$(echo $NODE_B | jq '.auth.id' | sed -e 's/"//g')
export B_TOKEN=$(echo $NODE_B | jq '.auth.token' | sed -e 's/"//g')

## Wait just a little bit
sleep 1

## For debugging the logs
echo "ID A: $A_ID"
echo "ID B: $B_ID"
