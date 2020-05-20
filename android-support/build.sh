#!/usr/bin/env bash
JNI_LIBS=../clients/android/app/src/main/jniLibs

if [ ! -d $JNI_LIBS ]; then
    echo "JNI_LIBS directory $JNI_LIBS does not exist, exiting."
    exit 1
fi

NDK_ROOT=~/.NDK
echo $NDK_ROOT
echo "$NDK_ROOT/whatev"

export AR="$NDK_ROOT/arm64/bin/aarch64-linux-android-ar"
export CC="$NDK_ROOT/arm64/bin/aarch64-linux-android-clang"
cargo build --target aarch64-linux-android --release

export AR="$NDK_ROOT/arm/bin/arm-linux-androideabi-ar"
export CC="$NDK_ROOT/arm/bin/arm-linux-androideabi-clang"
cargo build --target armv7-linux-androideabi --release

export AR="$NDK_ROOT/x86/bin/i686-linux-android-ar"
export CC="$NDK_ROOT/x86/bin/i686-linux-android-clang"
cargo build --target i686-linux-android --release

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

cp target/aarch64-linux-android/release/librobot.so $JNI_LIBS/arm64-v8a/librobot.so
cp target/armv7-linux-androideabi/release/librobot.so $JNI_LIBS/armeabi-v7a/librobot.so
cp target/i686-linux-android/release/librobot.so $JNI_LIBS/x86/librobot.so