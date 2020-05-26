#!/bin/bash

# This script will:
# Create a user, 
# modify this user, 
# list all users, 
# delte the user.
#
# usage:
# ./users-crud.sh

set -ex

# Create users
source src/users-bootstrap.sh

# Modify user A
source src/users-modify.sh "$A_ID" $A_TOKEN

# List all users on node A
source src/users-list.sh

# Delete user A
source src/users-delete.sh "$A_ID" $A_TOKEN

echo "Done"
