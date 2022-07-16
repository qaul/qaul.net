#!/usr/bin/env bash

# build libqaul on MacOS for iOS
# and copy the binaries automatically to the ios location.

# Build universel binary for ios
cargo lipo --release

# copy library and header file to ios
#cp ../include/libqaul.h ../../flutter/ios/
#cp target/universal/debug/liblibqaul.a ../../flutter/ios
