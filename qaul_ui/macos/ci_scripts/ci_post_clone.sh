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
#   environment. It does mainly two things:
#     - download and install flutter
#     - download the latest libqaul *.dylib
#
#   It's based on the Flutter documentation for setting up XCode Cloud:
#   https://docs.flutter.dev/deployment/cd#custom-build-script
#

# The default execution directory of this script is the ci_scripts directory.
cd $CI_PRIMARY_REPOSITORY_PATH/qaul_ui # change working directory to the root of cloned repo.

# Install Flutter using git.
git clone https://github.com/flutter/flutter.git --depth 1 -b stable $HOME/flutter
export PATH="$PATH:$HOME/flutter/bin"

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

# Pre-build for macos
flutter build macos --config-only

exit 0