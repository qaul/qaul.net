#!/usr/bin/env bash
set -eo pipefail

#-----------------------------------------------------------------NoticeStart-
# Utilities
#
# Copyright (c) 2022 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# run-android
#
# DESCRIPTION
#   Build and run the debug version of android. This script has 3 distinct phases:
#   Compile the Rust jniLibs, assemble the aar library and build the Flutter application.
#
# OPTIONS
#   --clean      Cleans any cached data in all steps - rust compilation, gradle assembly & flutter build.
#
# --- Note: ---
#   In case the `gradlew` commands fail with a `LockTimeoutException`, you can attempt to delete gradle's lockfiles with:
#   `find ~/.gradle -type f -name "*.lock" | while read -r f; do rm "$f"; done`

while true; do
  if [ "$1" = "--clean" ]; then
    CLEAN=true
    shift 1
  else
    break
  fi
done

# build libqaul
cd ../../rust/libqaul || exit 1

if [ "$CLEAN" == "true" ]; then
  cargo clean
fi
./build_libqaul_android.sh



# flutter
cd ../../qaul_ui || exit 1

## clean flutter
if [ "$CLEAN" == "true" ]; then
  flutter clean
fi

## run flutter
flutter run
