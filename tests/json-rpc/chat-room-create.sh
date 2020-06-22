#!/usr/bin/env bash

set -ex

# Create users
source src/user-bootstrap.sh

# Create a chat room for user A
source src/chat-room-create.sh "$B_ID" "$A_ID" $A_TOKEN
