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
    * Install NDK `22.1.7171670` (NDK `23.0.7599858` IS NOT WORKING)
  * 'Android SDK Command-line Tools (latest)'
  * 'CMake'
4) Apply the changes.


Set `$ANDROID_SDK_ROOT` variable and set the `$ANDROID_NDK_HOME` variable to the latest NDK.
Set it to the correct locations on your machine. e.g.:

```sh
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/22.1.7171670
```

TROUBLESHOOTING:

Don't use the latest NDK `23.0.7599858` as there are some platform linking tools missing
needed to compile libqaul's dependencies.
Install NDK `22.1.7171670`, it is working.


#### Android Studio Gradle Tasks

In order to see the gradle tasks, which are needed to 
build the android AAR library, activate them in the settings window:

1) Open the settings window from the menu: File > Settings 
2) In the settings window go to 'Experimental'
3) Uncheck 'Don not build Gradle task list during the Gradle build'


### Rust Cross Compilation for Android

Install rust on your computer, see for that [install rust](rust-install.md) documentation.

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


## Build Android Libqaul AAR Library

1) Open the `android` folder in android studio.
2) In Android Studio do the following steps
  * open the gradle tab
  * in the gradle tab open the following path libqaul > Tasks > build
  * run the `assemble` script in the build folder
3) now you can find the library files `libqaul-target.aar` and `libqaul-debug.aar`
   in the folder `build/libqaul/outputs`.


## Build the Flutter Android App

Open the `flutter` folder in android-studio, build it for Android or run 
it in the virtual android device.
