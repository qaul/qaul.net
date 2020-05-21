#!/usr/bin/env bash

# Don't call this script directly, use the build.sh script instead

set -e

/qaul.net/clients/android/gradlew cargoBuild
chown $1:$2 -R /qaul.net
