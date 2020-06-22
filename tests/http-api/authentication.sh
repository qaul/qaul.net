#!/usr/bin/env bash

# This script makes a logout and a login as user
#
# usage:
# ./authentication.sh

set -ex

# Create users
source src/user-bootstrap.sh

# Validate the authentication token
source src/auth-validate.sh "$A_ID" $A_TOKEN

# Logout
source src/auth-logout.sh "$A_ID" $A_TOKEN

# Login
source src/auth-login.sh "$A_ID"

echo "Done"
