#!/usr/bin/env bash

# build libqaul on linux for android
# you heed to have the ANDROID_NDK_HOME environment variable set.

# set build variables
## default android library location
libName=liblibqaul.so
buildType=debug
buildTypeCargo=
#buildType=release
#buildTypeCargo=--release

# Linker variables
AARCH64_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang
ARMV7_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang
I686_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang
X86_64_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android26-clang

# Set Path to all NDK binaries for some of the additional packages such as 'ring'
export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"

# Build
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$AARCH64_LINKER cargo build --target=aarch64-linux-android ${buildTypeCargo}
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ARMV7_LINKER cargo build --target=armv7-linux-androideabi ${buildTypeCargo}
CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$I686_LINKER cargo build --target=i686-linux-android ${buildTypeCargo}
CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$X86_64_LINKER cargo build --target=x86_64-linux-android ${buildTypeCargo}

# Copy to flutter shared library location for android
jniLibs=../android/libqaul/src/main/jniLibs
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

## Copy to android AAR library location
jniLibs=../flutter/android/libqaul/src/main/jniLibs
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
