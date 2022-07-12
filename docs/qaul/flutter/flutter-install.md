# qaul uses Flutter for it's GUI

## Prerequisits

In order to build the flutter toolchain, install the following packages on your system:

* clang


### Install Flutter

Please follow this guide to install flutter on your platform:
<https://flutter.dev/docs/get-started/install>


Check your flutter installation in the terminal

```sh
# check installation
flutter doctor
## The result should all be green check marks

# if on linux:
# activate linux as a target, 
# if it's not showing `linux toolchain` in flutter doctor.
flutter config --enable-linux-desktop
```


### Install Flutter Plugin for Android Studio

In order to develop and test the qaul.net with it's flutter GUI we recommend to install Android Studio as an IDE.
You can download and install it from it's official web page:

<https://developer.android.com/studio>


In order to test and build flutter applications, please install the flutter plugin in the Android Studio.

1) Open the settings window from the menu bar: File > Settings
2) Select the section 'Plugins' in the side bar.
3) Search for 'Flutter' in the Plugins Marketplace.
4) Install the 'Flutter' plugin.
5) Restart Android Studio.



### Build Libqaul on your System

Check the following resources:

* [Install rust](qaul/rust/rust-install.md)
* [Build rust code](qaul/rust/rust-build.md)


## Build & Test qaul.net App on your System

Open the folder 'flutter' of the qaul repository in Android Studio.

When opening the project for the first time, you need to set the path to your flutter SDK directory in Android Studio.
To do that run the following steps:

1) Open the settings window from the menu bar: File > Settings
2) Select the section 'Languages & Frameworks' and in there the sub section 'Flutter' in the side bar.
3) Write the path to your flutter installation directory into the field 'Flutter SDK path'
4) Click the 'OK' button.

Now, you should be able to select a testing environment and run qaul, by clicking the green play icon.
