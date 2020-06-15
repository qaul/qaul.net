#!/usr/bin/env bash

set -ex

# Create users
source src/users-bootstrap.sh

# Create a chat-room for user A
source src/chat-rooms-create.sh "$B_ID" "$A_ID" $A_TOKEN
