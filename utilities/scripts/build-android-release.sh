#!/usr/bin/env bash

# build android release version

# build libqaul
cd ../../rust/libqaul || exit 1

#cargo clean
./build_libqaul_android.sh release

# android aar
cd ../../android || exit 1

# clean gradle
./gradlew clean

# build andorid aar
./gradlew assemble

# copy aar files to flutter
if [ "$(uname)" = 'Darwin' ]; then
  install -b -d ../qaul_ui/android/app/libs libqaul/build/outputs/aar
  install -b -d ../qaul_ui/android/app/libs blemodule/build/outputs/aar
elif [ "$(expr substr "$(uname -s)" 1 5)" = 'Linux' ]; then
  install -D libqaul/build/outputs/aar/*.aar ../qaul_ui/android/app/libs
  install -D blemodule/build/outputs/aar/*.aar ../qaul_ui/android/app/libs
fi

# flutter
cd ../qaul_ui/android || exit 1

## clean flutter
#flutter clean

## build android AAB release file
#bundle exec fastlane upload_beta_playstore
