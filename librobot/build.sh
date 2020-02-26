#!/usr/bin/env bash
JNI_LIBS=../clients/android/app/src/main/jniLibs

if [ ! -d $JNI_LIBS ]; then
    echo "JNI_LIBS directory $JNI_LIBS does not exist, exiting."
    exit 1
fi

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

cp target/aarch64-linux-android/release/librust.so $JNI_LIBS/arm64-v8a/librust.so
cp target/armv7-linux-androideabi/release/librust.so $JNI_LIBS/armeabi-v7a/librust.so
cp target/i686-linux-android/release/librust.so $JNI_LIBS/x86/librust.so