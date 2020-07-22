#!/usr/bin/env bash

# Contacts are all discovered and known users on
# all nodes execpt the localone.
# 
# This script will:
# list all contacts
#
# usage:
# ./contact-list.sh

set -ex

# Create users
source src/user-bootstrap.sh

# Modify user A
source src/user-get.sh "$A_ID" $A_TOKEN

# List all users
source src/user-list.sh

# List all contacts
source src/contact-list.sh "$A_ID" $A_TOKEN

echo "Done"
