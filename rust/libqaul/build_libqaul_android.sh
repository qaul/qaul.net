#!/usr/bin/env bash

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
# WARNING: you heed to have the ANDROID_NDK_HOME environment variable set.
# Note: please run this script from the rust/libqaul path
#
# == Usage ==
# 1. To compile in debug mode:
#   sh build_libqaul_android.sh
#
# 2. To compile in release mode:
#   sh build_libqaul_android.sh release

# Initialize variables
mode=${1:-debug}
release="false"
buildType="debug"
buildTypeCargo=
if [ "$mode" == "release" ]; then
  release="true"
  buildType="release"
  buildTypeCargo=--release
fi

libName=liblibqaul.so

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
  i686-linux-android \
  x86_64-linux-android

# 2. Define linker variables
PREBUILT_BINARIES="$(find "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt" -maxdepth 1 -mindepth 1 -type d )/bin"
echo "Using prebuilt binaries found in '$PREBUILT_BINARIES'"

AARCH64_LINKER="$PREBUILT_BINARIES/aarch64-linux-android26-clang"
ARMV7_LINKER="$PREBUILT_BINARIES/armv7a-linux-androideabi26-clang"
I686_LINKER="$PREBUILT_BINARIES/i686-linux-android26-clang"
X86_64_LINKER="$PREBUILT_BINARIES/x86_64-linux-android26-clang"

# 3. Set Path to all NDK binaries for some of the additional packages such as 'ring'
export PATH="$PREBUILT_BINARIES:$PATH"

# 4. Build
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$AARCH64_LINKER cargo build --target=aarch64-linux-android ${buildTypeCargo}
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ARMV7_LINKER cargo build --target=armv7-linux-androideabi ${buildTypeCargo}
CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$I686_LINKER cargo build --target=i686-linux-android ${buildTypeCargo}
AR=llvm-ar CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$X86_64_LINKER cargo build --target=x86_64-linux-android ${buildTypeCargo}

# Copy to flutter shared library location for android
jniLibs=../../android/libqaul/src/main/jniLibs
rm -rf ${jniLibs}
mkdir ${jniLibs}
mkdir -p ${jniLibs}/arm64-v8a
mkdir -p ${jniLibs}/armeabi-v7a
mkdir -p ${jniLibs}/x86
mkdir -p ${jniLibs}/x86_64
cp ../target/aarch64-linux-android/${buildType}/${libName} ${jniLibs}/arm64-v8a/${libName}
cp ../target/armv7-linux-androideabi/${buildType}/${libName} ${jniLibs}/armeabi-v7a/${libName}
cp ../target/i686-linux-android/${buildType}/${libName} ${jniLibs}/x86/${libName}
cp ../target/x86_64-linux-android/${buildType}/${libName} ${jniLibs}/x86_64/${libName}
