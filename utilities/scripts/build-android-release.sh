#!/usr/bin/env bash

# build android release version

# build libqaul
cd ../../rust/libqaul || exit 1

#cargo clean
./build_libqaul_android.sh release

# flutter
cd ../../qaul_ui/android || exit 1

## build android AAB release file
bundle exec fastlane upload_beta_playstore
