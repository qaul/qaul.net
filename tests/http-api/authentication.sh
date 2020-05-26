#!/bin/bash

# This script makes a logout and a login as user
#
# usage:
# ./authentication.sh

set -ex

# Create users
source src/users-bootstrap.sh

# Logout
source src/logout.sh "$A_ID" $A_TOKEN

# Login
source src/login.sh "$A_ID"

echo "Done"
