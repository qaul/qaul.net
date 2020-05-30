#!/usr/bin/env bash

# Only call this script directly when you're using the docker
# setup in "dev" mode

set -e

# This makes libopus happy
# TODO: figure out why this is a flag at all
export PKG_CONFIG_ALLOW_CROSS=1

/qaul.net/clients/android/gradlew cargoBuild
chown $1:$2 -R /qaul.net/target
