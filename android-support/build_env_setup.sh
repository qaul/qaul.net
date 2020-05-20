#!/usr/bin/env bash
mkdir ~/.NDK

$ANDROID_HOME/../android-sdk/ndk/21.0.6113669/build/tools/make-standalone-toolchain.sh --platform=30 --arch=arm64 --install-dir=~/.NDK/arm64;
$ANDROID_HOME/../android-sdk/ndk/21.0.6113669/build/tools/make-standalone-toolchain.sh --platform=30 --arch=arm --install-dir=~/.NDK/arm;
$ANDROID_HOME/../android-sdk/ndk/21.0.6113669/build/tools/make-standalone-toolchain.sh --platform=30 --arch=x86 --install-dir=~/.NDK/x86;
