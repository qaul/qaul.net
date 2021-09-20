# Build qaul App for Android

This Document describes how to build the qaul Android app.


## Prerequisits

### Android Studio

To build the app you need to install Android Studio.
Download and install it from it's official web page:

https://developer.android.com/studio


### Rust Cross Compilation for Android

To build libqaul for android targets we are using mozilla's rust-android-gradle plugin for android studio.
The instructions below are following the official plugin installation instructions:
https://github.com/mozilla/rust-android-gradle


install build targets for android in rust

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

Install cargo ndk in cargo

```sh
cargo install cargo-ndk
```

Set $ANDROID_NDK_HOME variable to the latest NDK

```sh
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/22.1.7171670
```

Build rust libqaul from QAUL_REPOSITORY/libqaul folder, via the build_libqaul_android.sh script

```sh
# on linux run
./build_libqaul_android.sh

## the build scripts for other operating systems can be setup accordingly
```

The build script builds the shared libqaul library for different android 
targets and copies them to the appropriate location in the android folder 
`android/app/src/main/jniLibs`.


## Build Android Libqaul AAR Library

1) Open the `android` folder in android studio.
2) In Android Studio do the following steps
  * open the gradle tab
  * in the gradle tab open the following path libqaul > Tasks > build
  * run `assemble` script in the build folder
3) now you can find the library files libqaul-target.aar and libqaul-debug.aar
   in the folder `build/libqaul/outputs`.


## Build App

Now you can build and start the android app from within Android Studio.
