#!/bin/bash

#-----------------------------------------------------------------NoticeStart-
# Utilities
#
# Copyright (c) 2021 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# build_libqaul_android
#
# Compiles the rust jniLibs in either 'release' or 'debug' mode.
# WARNING: you need to have the ANDROID_NDK_HOME environment variable set.
# Note: please run this script from the rust/libqaul path
#
# == Usage ==
# 1. To compile in debug mode:
#   sh build_libqaul_android.sh
#
# 2. To compile in release mode:
#   sh build_libqaul_android.sh release
# Pre-step: assert that the script is being run from rust/libqaul
PATTERN="^.*/rust/libqaul$"
if [[ ! $(pwd) =~ $PATTERN ]]; then
  echo "This script uses relative paths. Please run directly from it's directory."
  exit 1
fi

# Initialize variables
mode=${1:-debug}
release="false"
buildTypeCargo=
if [ "$mode" == "release" ]; then
  release="true"
  buildTypeCargo=--release
fi

echo "Compiling libqaul in release mode: $release"
echo ""

# 0. Check if the required environment variable exists
if [ "$ANDROID_NDK_HOME" = "" ] || [ "$(find "$ANDROID_NDK_HOME" -type d)" == "" ] && [ "$(find "$ANDROID_NDK_HOME" -type l)" == "" ]; then
  echo "could not find required 'ANDROID_NDK_HOME' environment variable" >&2
  exit 1
fi

# 1. Add required targets
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  x86_64-linux-android

# 2. make sure cargo-ndk is installed
cargo install cargo-ndk

# 3. clean jni location
jniLibs=../../qaul_ui/android/libqaul/src/main/jniLibs
rm -rf ${jniLibs}

# 4. build libqaul for all targets
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86_64 -o $jniLibs build ${buildTypeCargo}
