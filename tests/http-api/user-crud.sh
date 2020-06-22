#!/usr/bin/env bash

# This script will:
# Create a user, 
# modify this user, 
# list all users, 
# delte the user.
#
# usage:
# ./user-crud.sh

set -ex

# Create users
source src/user-bootstrap.sh

# Modify user A
source src/user-modify.sh "$A_ID" $A_TOKEN

# List all users on node A
source src/user-list.sh

# Delete user A
source src/user-delete.sh "$A_ID" $A_TOKEN

echo "Done"
