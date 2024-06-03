#!/bin/bash
set -e

#-----------------------------------------------------------------NoticeStart-
# CI Scripts
#
# Copyright (c) 2024 Open Community Project Association https://ocpa.ch
# This software is published under the AGPLv3 license.
#-----------------------------------------------------------------NoticeEnd---
#
# DESCRIPTION
#   This post-clone script is meant to be used to setup the XCode Cloud
#   environment. It does:
#     - install flutter
#     - install rust
#     - install protoc (rust dependency)
#
#   It's based on the Flutter documentation for setting up XCode Cloud:
#   https://docs.flutter.dev/deployment/cd#custom-build-script
#

FLUTTER_VERSION="3.19.6"

# Install Flutter using git.
git clone https://github.com/flutter/flutter.git --depth 1 -b stable $HOME/flutter
export PATH="$PATH:$HOME/flutter/bin"

# Make sure the right version is being used
cd $HOME/flutter
git fetch --tags
git checkout tags/$FLUTTER_VERSION

# The default execution directory of this script is the ci_scripts directory.
cd $CI_PRIMARY_REPOSITORY_PATH/qaul_ui # change working directory to the root of cloned repo.

# Install Flutter artifacts for MacOS
flutter precache --macos

# Install Flutter dependencies.
flutter pub get

# Install CocoaPods using Homebrew.
HOMEBREW_NO_AUTO_UPDATE=1 # disable homebrew's automatic updates.
brew install cocoapods

# Install CocoaPods dependencies.
cd macos && pod install # run `pod install` in the `macos` directory.

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install protobuf
brew install protobuf

# Pre-build for macos
flutter build macos --config-only

exit 0