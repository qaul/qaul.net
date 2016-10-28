Android Instructions
====================

To check whether your Android Device is known to work with qaul.net check 
[README_Android_Devices.md]


Prerequisites
-------------

### Android SDK/NDK

For building the Android version of qaul.net you will need both the Source
Development Kit *and* the Native Development Kit

* Get the NDK https://developer.android.com/tools/sdk/ndk/index.html
* Get the SDK https://developer.android.com/sdk/index.html
  * Invoke the binary tools/android from the downloaded SDK. This starts the
	'Android SDK Manager'. Via the 'Android SDK Manager' window install the
	required packages:
	* Tools > Android SDK Platform-tools
	* Tools > Android SDK Build-tools
	* Android 2.2 (API 8)

Add `sdk` to `path` if the installer had not done it automatically.


### Java Development Kit (JDK)

Install latest Oracle Java Development Kit (JDK)

* Get the JDK http://www.oracle.com/technetwork/java/javase/downloads/index.html


### Programs and Libraries

The following programs and libraries need to be installed:

* ant
* lib32stdc++6
* lib32z1

	# install ant on Debian / Ubuntu Linux
	sudo apt-get install ant lib32stdc++6 lib32z1


Get the source
--------------

Download the source from [https://github.com/WachterJud/qaul.net](github)

	git clone https://github.com/WachterJud/qaul.net.git



Build
-----

	# create a build directory in your qaul.net source directory
	mkdir build
	cd build


	# The compiler needs some libraries in the build directory
	# Connect your Android device to your computer and pull the libraries
	# from your device to your build directory:
	adb pull /system/lib/libcutils.so
	adb pull /system/lib/libwpa_client.so
	adb pull /system/bin/ifconfig
	adb pull /system/bin/iptables


	# build the qaul.net Android app
	## check ANDROID_EABI version in NDK's 'toolchains' folder. The number suffix
	## of the folder name arm-linux-androideabi-XXX is the ANDROID_EABI version.
	## toolchains/arm-linx-androideabi-4.6 => -DANDROID_EABI="4.6"
	cmake ../ -DPORT=ANDROID -DNDK_ROOT=/absolute/path/to/ndk -DEXTRALIB_PATH=/absolute/path/to/libraries -DANDROID_EABI="4.6"
	make


You will find the newly built apk in `android/app/src/main/bin`.

    # install your app from the command line
    # make sure your phone is connected and debugging mode is activated
    adb install android/bin/QaulActivity-debug.apk
    
    # uninstall app from your phone
    adb uninstall net.qaul.qaul


Eclipse
-------

- Import Existing Project
- Do **NOT** copy files.


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

