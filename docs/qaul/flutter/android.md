# Build qaul App for Android

This Document describes how to build the qaul Android app.


## Prerequisits

### Android Studio

To build the app you need to install Android Studio.
Download and install it from it's official web page:

https://developer.android.com/studio


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

#### Android Studio Gradle Tasks

In order to see the gradle tasks, which are needed to 
build the android AAR library, activate them in the settings window:

1) Open the settings window from the menu: File > Settings 
2) In the settings window go to 'Experimental'
3) Uncheck 'Don not build Gradle task list during the Gradle build'


### Rust Cross Compilation for Android

Install rust on your computer, see for that [install rust](qaul/rust/rust-install.md) documentation.

Additionally you need to install the build targets for android in rust:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

Install cargo ndk in cargo

```sh
cargo install cargo-ndk
```

Build rust libqaul from `libqaul` folder, via the `build_libqaul_android.sh` script

```sh
# on linux run
./build_libqaul_android.sh

## the build scripts for other operating systems can be setup accordingly
```

The build script builds the shared libqaul library for different android 
targets and copies them to the appropriate location in the android folder 
`android/app/src/main/jniLibs`.

## Build the qaul Android App 

### Build and Run a debuggable Android Version for your Phone

To build and run the debug version of the Android app on your phone,
attach your phone to your computer and make sure it is found by adb.

```sh
# move to the flutter 
cd qaul_ui

# build and run the qaul app on your phone
flutter run
```

!> Note: for apple m1, add `protoc_platform=osx-x86_64` in `$HOME/.gradle/gradle.properties`
```bash
echo "protoc_platform=osx-x86_64" >> "$HOME/.gradle/gradle.properties"
```

To see the debug messages via logcat, open logcat in a second terminal, 
while the qaul app is running.

```sh
# start logcat in a separate terminal and filter for qaul
# log messages
adb logcat | grep qaul
```

### Build Android AAR for Release and for Store

To build an Android app ARR archive for uploading to the play store.

```sh
# change into the flutter android folder
cd qaul_ui/android

# start fastline build and upload script
bundle exec fastlane upload_beta_playstore
```
