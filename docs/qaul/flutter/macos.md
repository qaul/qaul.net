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
libqaul is written in the Rust programming language. Follow the steps outlined [here](qaul/rust/rust-install.md) to install it.

#### Install Cross-Compile Targets

As MacOS runs on multiple processor architectures, we need to install the rust compile targets for the other processor architectures.

##### On Intel Machines

When you are compiling on a machine with an intel machine you need to install the target for the M1 processor architecture.

```sh
rustup target add aarch64-apple-darwin
```

##### On M1 Machines

When you are compiling on a machine with a M1 processor you need to install the target for the intel x86_64 architecture.

```sh
rustup target add x86_64-apple-darwin
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

# copy libqaul to flutter
cp target/debug/liblibqaul.dylib ../qaul_ui/macos/
```

### 2. Build & Run qaul flutter Desktop App

You can build and run the qaul flutter desktop app from the terminal:

```sh
# move into the flutter directory
cd qaul_ui

# build and run the MacOS desktop app in debug mode
flutter run -d macos
```

## Build Distributable (*.dmg) MacOS Application

_**Step 1: Archiving the App**_:

To manually create a release archive, you can do the following:

1. Open the project in XCode: `open qaul_ui/macos/Runner.xcworkspace`;
2. Press `Product >> Archive`;
3. Once done, the **Organizer** window should open by itself (if not, open it via `Window >> Organizer`):
    1. Select the `Archives` sub-menu over the left panel;
    2. Select the archive you've just created in the middle panel and;
    3. Select the `Distribute App` option in the right panel to upload the archive to be notarized.

_**Step 2: Notarizing**_:

The steps below outline what to do in each screen of the "Distribute App" dialog:

1. **Select a method of distribution**
   > select the "Developer ID" option and press *Next*;
2. **Select a destination**
   > select "Upload" and press *Next*;
3. **Select certificate and Developer ID profiles**
   > select your **Developer ID Application** certificate; No profile is required. Press *Next*;

Once this is done, wait until the app is properly notarized - you should receive an email confirming the process is complete.

After that, you can export the newly-notarized app from the Organizer window itself (you may need to `View >> Reload Organizer`).
> Export it onto `utilities/installers/macos`.

_**Step 3: Generating the dmg**_:

```sh
# Build dmg file
cd ../utilities/installers/macos/bin
./dmgbuild
```
