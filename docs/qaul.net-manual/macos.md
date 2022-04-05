# Build qaul.net on MacOS

In order to build qaul.net on your Apple MacOS machine, follow the steps below.

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
sudo gem install cocoapods
```

### Rust
libqaul is written in the Rust programming language. Follow the steps outlined [here](rust-install.md) to install it.

### Android Studio (Optional)

There are two reasons you might want to install Android Studio:

1) If you want to develop for Android
2) There is a nice GUI button to run and test the flutter app within Android Studio

[Install Android Studio](android.md)

After installation open it and install the following things:

* Android SDK & NDK (if you want to develop for android)
* Android studio flutter plugin

### Flutter

The Qaul GUI is built using Flutter. Instructions to have it installed can be found in [this document](flutter-install.md).

## Build & Run qaul.net MacOS Desktop App

To build and run the qaul.net desktop app you have to perform the following steps:

1) Build libqaul shared library
2) Build & Run the flutter MacOS desktop app

### 1. Build libqaul shared Library and CLI binaries

Open a terminal and run the following commands

```sh
# move into the rust folder
cd rust

# build libqaul and the CLI binaries
cargo build
```

### 2. Build & Run qaul flutter Desktop App

You can build and run the qaul flutter desktop app from the terminal:

```sh
# move into the flutter directory
cd qaul_ui

# build and run the MacOS desktop app in debug mode
flutter run -d macos
```

## Build Distributable MacOS Application

You can build the qaul flutter desktop app from the terminal:

```sh
# move into the flutter directory
cd qaul_ui

# build and run the MacOS desktop app
flutter build macos
```
