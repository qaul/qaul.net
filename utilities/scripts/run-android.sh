#!/usr/bin/env bash

# build and run the debug version of android

# delete old libqaul libraries
#rm ../../android/libqaul/src/main/jniLibs/arm64-v8a/liblibqaul.so
#rm ../../android/libqaul/src/main/jniLibs/armeabi-v7a/liblibqaul.so
#rm ../../android/libqaul/src/main/jniLibs/x86/liblibqaul.so
#rm ../../android/libqaul/src/main/jniLibs/x86_64/liblibqaul.so

# build libqaul
cd ../../rust/libqaul
#cargo clean
#./build_libqaul_android.sh

# android aar
cd ../../android

# clean gradle
./gradlew clean

# build andorid aar
./gradlew assemble

# copy aar files to flutter
mkdir -p ../qaul_ui/android/app/libs
install -D libqaul/build/outputs/aar/*.aar ../qaul_ui/android/app/libs/
install -D blemodule/build/outputs/aar/*.aar ../qaul_ui/android/app/libs/

# flutter
cd ../qaul_ui

## clean flutter
#flutter clean

## run flutter
flutter run
