# Build qaul on Windows

## Prerequisits
### Install Visual Studio

In order to develop for the windows desktop platform you need to have 'Microsoft Visual Studio 2022' installed.

<https://visualstudio.microsoft.com/downloads/>

In Microsoft Visual Studio 2022, you need to have 'Desktop development with C++' workload installed.

### Install Git for Windows

Git is a free version control system and needed for source code management.
Please download and install the git from the following page:

<https://git-scm.com/download/win>

### Install Rust

libqaul is written in the programming language rust.
In order to install rust, open the page [rustup.rs](https://rustup.rs) and follow the installation instructions.

<https://rustup.rs>

### Install Android Studio

If you wan't to build and develop for Android OS devices, you need to install 'Android Studio'

[Install Android Studio](qaul/flutter/android.md)


### Install Flutter

[Install Flutter](qaul/flutter/flutter-install.md)

Check your flutter Installation 

To check your flutter installation, if it can build Windows desktop applications run the following commands in the Windows Powershell:

```sh
# run flutter doctor to check your installation
flutter doctor

## The result needs to show the following line, for you to 
## successfully build the flutter desktop application for windows:
## 
## [√] Visual Studio - develop for Windows
## 
## If this line does not appear, please make sure, you installed 
## 'Microsoft Visual Studio 2019' and enabled the 
## 'Desktop development with C++' workload in it.
## If yes, run the following flutter command to enable windows desktop
## development:

# change flutter configuration to enable windows desktop development
flutter config --enable-windows-desktop
```

## Build qaul

### Build Rust libqaul library and CLI binaries

To build libqaul and the CLI binaries, open the Windows Power Shell and enter the following commands:

```sh
# move into the rust directory of this repository
cd rust

# build libqaul & CLI binaries
cargo build

# copy libqaul.dll to flutter runner
## this step is required in order to run flutter app
## create the location folder if it does not yet exist
cp .\target\debug\libqaul.dll ..\qaul_ui\build\windows\x64\runner\Debug\
```

### Build and Run Windows Desktop App

You can now build and run the flutter desktop app from the terminal:

```sh
# move into the flutter directory
cd qaul_ui

# build and run the Windows desktop app
flutter run -d windows
```
