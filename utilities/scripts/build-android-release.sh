#!/usr/bin/env bash

# build android release version

# build libqaul
cd ../../rust/libqaul

#cargo clean
./build_libqaul_android.sh release

# android aar
cd ../../android

# clean gradle
./gradlew clean

# build andorid aar
./gradlew assemble

# copy aar files to flutter
install -D libqaul/build/outputs/aar/*.aar ../qaul_ui/android/app/libs
install -D blemodule/build/outputs/aar/*.aar ../qaul_ui/android/app/libs

# flutter
cd ../qaul_ui/android

## clean flutter
#flutter clean

## build android AAB release file
bundle exec fastlane upload_beta_playstore
