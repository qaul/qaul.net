#!/usr/bin/env bash
set -o errexit -o noclobber -o nounset -o pipefail

#-----------------------------------------------------------------NoticeStart-
# Utilities
#
# Copyright (c) 2021 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# build_libqaul_macos
#
# Compiles the rust library in either 'release' or 'debug' mode.
# == Usage ==
# 1. To compile in debug mode:
#   sh build_libqaul_macos.sh
#
# 2. To compile in release mode:
#   sh build_libqaul_macos.sh release

# Initialize variables
mode=${1:-debug}
release="false"
if [ "$mode" == "release" ]; then
  release="true"
fi

echo "Compiling libqaul in release mode: $release"
echo ""

# 1. Check if the lipo command is available in PATH
if [ "$(which lipo)" = "" ]; then
  echo "could not find required 'lipo' binary" >&2
  exit 1
fi

# 2. Compile binaries
echo "Adding required rust targets..."
rustup target add x86_64-apple-darwin aarch64-apple-darwin

echo "Compiling MacOS x86_64 library..."
if [ $release == true ]; then
  cargo build --target=x86_64-apple-darwin "--release"
else
  cargo build --target=x86_64-apple-darwin
fi

echo ""
echo "Compiling MacOS aarch64 library..."
if [ $release == true ]; then
  cargo build --target=aarch64-apple-darwin --release
else
  cargo build --target=aarch64-apple-darwin
fi

# 3. Create fat file
LIBRARY_PATH="release/liblibqaul.dylib"
if [ $release == false ]; then
  LIBRARY_PATH="debug/liblibqaul.dylib"
fi

echo ""
echo "Creating fat library from generated dylib files..."
lipo -create \
  "../target/aarch64-apple-darwin/$LIBRARY_PATH" \
  "../target/x86_64-apple-darwin/$LIBRARY_PATH" \
  -output "../target/$LIBRARY_PATH"

echo ""
echo "fat library generated;"
lipo -info "../target/$LIBRARY_PATH"
