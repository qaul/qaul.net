#!/usr/bin/env bash

# Only call this script directly when you're using the docker
# setup in "dev" mode

set -e

# Required for libsodium & libopus x-compile
export PKG_CONFIG_ALLOW_CROSS=1

/qaul.net/clients/android/gradlew cargoBuild
chown $QAUL_USER:$QAUL_GROUP -R /qaul.net/target
