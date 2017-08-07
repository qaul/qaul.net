Android Instructions
====================

To check whether your Android Device is known to work with qaul.net check 
[README_Android_Devices.md]


Prerequisites
-------------

### Java Development Kit (JDK)

Install latest Oracle Java Development Kit (JDK)

* e.g. Download and install the JDK from
  http://www.oracle.com/technetwork/java/javase/downloads/index.html


### Programs and Libraries

The following programs and libraries need to be installed:

* ant
* lib32stdc++6
* lib32z1

	# install ant on Debian / Ubuntu Linux
	sudo apt-get install ant lib32stdc++6 lib32z1


### Android Studio

Android NDK and Android SDK will be installed with the Android Studio.
To Install Android Studio do the following:

* Download & Install Android Studio https://developer.android.com/studio/index.html
* Open Android Studio and open the SDK Manager from within Android Studio
  via the menu 'Tools > Android > SDK Manager'. Do the following within
  the SDK Manager.
  * Open the location 'Appearance & Behaviour > System Settings > Android SDK > SDK Platforms' 
    and install the SDK:
    * Android 4.2 (Jelly Bean)
    * Android 2.2 (Froyo)
  * Open the location 'Appearance & Behaviour > System Settings > Android SDK > SDK Tools'
    and install:
    * NDK

Add the Android platform-tools to your system PATH to use adb from CLI.

	# under Linux execute the following
	# (if you installed everything to default locations)
	export PATH="$PATH:$HOME/Android/Sdk/platform-tools


To build socat one needs an NDK which is not newer than NDK 13b. NDK 13b 
can be downloaded from this source: 
https://developer.android.com/ndk/downloads/older_releases.html

When building with this NDK, the folder 'sysroot' needs to be set manually
as a symbolic link to the correct platform target

	# open a terminal and cd into your NDK direktory.
	# create a symbolic link as sysroot to your platform
	ln -s platforms/android-9/arch-arm sysroot



Get the source
--------------

Download the source from [https://github.com/qaul/qaul.net](github)

	git clone --recursive https://github.com/qaul/qaul.net.git



Build
-----

### Build from CLI.

	# change to the root folder of the qaul.net source

	# The compiler needs some libraries in the build directory
	# Connect your Android device to your computer and pull the libraries
	# from your device to the directory android_extra_lib:
	adb pull /system/lib/libcutils.so android_extra_lib/
	adb pull /system/lib/libwpa_client.so android_extra_lib/
	adb pull /system/bin/ifconfig android_extra_lib/
	adb pull /system/bin/iptables android_extra_lib/

	# build the qaul.net Android app
	## check ANDROID_EABI version in NDK's 'toolchains' folder. The number suffix
	## of the folder name arm-linux-androideabi-XXX is the ANDROID_EABI version.
	## toolchains/arm-linx-androideabi-4.9 => -DANDROID_EABI="4.9"
	##
	## The SDK and the NDK were installed via AndroidStudio. The default
	## path under Linux is:
	##   SDK_ROOT: $HOME/Android/Sdk
	##   NDK_ROOT: $HOME/Android/Sdk/ndk-bundle
	export ANDROID_HOME=/absolute/path/to/sdk
	cmake . -DPORT=ANDROID -DSDK_ROOT=/absolute/path/to/sdk -DNDK_ROOT=/absolute/path/to/ndk -DEXTRALIB_PATH=/absolute/path/to/android_extra_lib -DANDROID_EABI="4.9"
	make


You will find the newly built apk in `android/app/build/outputs/apk`.

    # install your app from the command line
    # make sure your phone is connected and debugging mode is activated
    adb install -r android/app/build/outputs/apk/qaul-debug.apk

    # uninstall app from your phone
    adb uninstall net.qaul.qaul


### Build with Android Studio

Open the android folder of the qaul.net source and let Android Studio run
the gradle scripts. Everything should be in place and you should be
able to build.

Before the qaul.net app can be built via Android Studio, one needs to
build it at least once from CLI.



Testing
-------

Start qaul.net app from CLI

	# login to your android device
	adb shell
	# become super user
	su
	# start bash
	bash
	# install location of the app is: /data/data/net.qaul.qaul
	cd /data/data/net.qaul.qaul
	
	# start the qaul app
	am start -n net.qaul.qaul/net.qaul.qaul.QaulActivity

