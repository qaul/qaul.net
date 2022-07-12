# Build qaul.net for iOS

In order to build qaul.net for iOS you need an apple computer.

## Pre-requisites

You need to install the following software

### XCode, XCode Developer Tools & CocoaPods

#### XCode

* Install XCode from the AppStore.
* Once XCode is installed, open it and accept the license.

#### XCode Developer Tools

* After installing XCode, run the following command in a terminal to install the XCode Developer Tools

```sh
sudo xcode-select --install
```

#### CocoaPods XCode Extension

CocoaPods is needed by flutter to retrieve the iOS and macOS platform plugin code. Without CocoaPods, plugins will not work on iOS or macOS.

After installing XCode run the following code in your Terminal to install the CocoaPods:

```sh
sudo gem install cocoapod
```

### Rust

libqaul is written in the Rust programming language. Follow the steps outlined [here](qaul/rust/rust-install.md) to install it.

#### Install the iOS rust targets from Terminal:
> Note: On M1 machines, to run libqaul on an iPhone Simulator, we must also add the following target:
> 
> `aarch64-apple-ios-sim`
> 
> For more info, check [this doc](https://doc.rust-lang.org/nightly/rustc/platform-support/aarch64-apple-ios-sim.html).

```sh
# rust targets for ios
rustup target add aarch64-apple-ios x86_64-apple-ios

# [Mac M1 machines only]
rustup target add aarch64-apple-ios-sim

# cargo-lipo subcommand that builds a universal library
cargo install cargo-lipo
```

### Android Studio (Optional)

There are two reasons you might want to install Android Studio:

1) If you want to develop for Android
2) There is a nice GUI button to run and test the flutter app within Android Studio

[Install Android Studio](qaul/flutter/android.md)

After installation open it and install the following things:

* Android SDK & NDK (if you want to develop for android)
* Android studio flutter plugin

### Flutter

The Qaul GUI is built using Flutter. Instructions to have it installed can be found in [this document](flutter-install.md).

## Build & Run qaul.net iOS App

To build and run the qaul.net desktop app you have to perform the following steps:

1) Build libqaul shared library
2) Build & run the flutter iOS app

### 1. Build libqaul shared Library

Open a terminal and run the following commands

```sh
# move into the rust libqaul folder
cd rust/libqaul

# build libqaul universal target for ios
## for release build do:
## cargo lipo --release
cargo lipo

# [Mac M1 machines only]
# If you only wish to run libqaul on an iOS Simulator, run this command instead the one above:
cargo lipo --release --targets aarch64-apple-ios-sim
```

### 2. Build & Run qaul app on iOS Simulator or on your iOS Device

You can build and run the qaul flutter desktop app from the terminal:

```sh
# move into the flutter directory
cd qaul_ui

# build and run the Flutter app
flutter run
```

