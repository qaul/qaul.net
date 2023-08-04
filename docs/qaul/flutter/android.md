# Build qaul App for Android

This Document describes how to build the qaul Android app.

## Prerequisits

### Android Studio

To build the app you need to install Android Studio.
Download and install it from it's official web page:

<https://developer.android.com/studio>

#### Android NDK, CMAKE & Command-line Tools

Install Android NDK. You can do that from within Android Studio.
This is a small guide how to do this:

1) In Android Studio open 'SDK Manager'
    * click on the box icon on the right side of the toolbar. A settings window will open.
2) In the settings window, select the tag 'SDK Tools'
3) Check the following check boxes
    * 'NDK (Side by side)'
    * 'Android SDK Command-line Tools (latest)'
    * 'CMake'
4) Apply the changes.

Set `$ANDROID_SDK_ROOT` variable and set the `$ANDROID_NDK_HOME` variable to the latest NDK.
Set it to the correct locations on your machine. e.g.:

```sh
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/23.1.7779620
```

### Install Rust for Cross Compilation for Android

Install rust on your computer, see for that [install rust](qaul/rust/rust-install.md) documentation.

Additionally you need to install the build targets for android in rust:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

Install cargo ndk in cargo

```sh
cargo install cargo-ndk
```

## Build and Run the Android app on your Phone

### Build and Run Android App via automated Script

There is an automated script to build the `libqaul.so` libraries for android,
build the app, install the app on your phone and start it.
Please make sure your phone is connected to your computer, before executing the script.

You can find the script in the `utilities/scripts` folder.

```sh
# move into the utilities/scripts folder of the repo
cd utilities/scripts

# run the script
./run-android.sh
```

### Run Android App via Android Studio

Make sure you have the `libqaul.so` libraries installed at `qaul_ui/android/libqaul/src/main/jniLibs`

a) you can build the android `libqaul.so` libraries as described below in the step by step instructions.
b) if you don't want to build the rust `libqaul.so` for android yourself, you can download them from the
  github repo and install it manually
    - download the file `jniLibs.zip` from the latest release <https://github.com/qaul/qaul.net/releases>
    - unzip `jniLibs.zip` in the folder `qaul_ui/android/libqaul/src/main/jniLibs`

Once you did that, open the `qaul_ui` folder from the repo in Android Studio and simply click the run
button in Android Studio in order to build and run qaul.

### Step By Step Instruction

#### Build Libqaul Libraries for Android

Build rust libqaul libraries from `rust/libqaul` folder, via the `build_libqaul_android.sh` script

```sh
# move to the rust/libqaul folder of the repo
cd rust/libqaul

# on linux run
./build_libqaul_android.sh
```

The build script builds the shared libqaul library for all supported android
targets and copies them to the appropriate location in the `qaul_ui/android` folder
`qaul_ui/android/libqaul/src/main/jniLibs`.

#### Build and Run the App

To build and run the debug version of the Android app on your phone,
attach your phone to your computer and make sure it is found by adb.

```sh
# move to the qaul_ui folder 
cd qaul_ui

# build and run the qaul app on your phone
flutter run
```

!> Note: for apple m1, add `protoc_platform=osx-x86_64` in `$HOME/.gradle/gradle.properties`

```sh
echo "protoc_platform=osx-x86_64" >> "$HOME/.gradle/gradle.properties"
```

## Build Android App Bundle for Release and for Store

To build an Android App Bundle (AAB) for uploading to the play store.

```sh
# after having built the rust libqaul libraries for android,
# change into the flutter android folder
cd qaul_ui/android

# start fastlane build and upload script
bundle exec fastlane upload_beta_playstore
```

## Debugging Tipps

To see the apps debug messages from the terminal, open logcat in a terminal windows
while the qaul app is running.

```sh
# start logcat in a separate terminal and filter for qaul
# log messages
adb logcat | grep qaul
```
