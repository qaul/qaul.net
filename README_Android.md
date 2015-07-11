Android Instructions
====================

To check wheter your Android Device is known to work with qaul.net check 
[README_Android_Devices.md]


Prerequisites
-------------

### Android SDK/NDK

For building the Android version of qaul.net you will need both the Source
Develpment Kit *and* the Native Development Kit

* Get the NDK https://developer.android.com/tools/sdk/ndk/index.html
* Get the SDK https://developer.android.com/sdk/index.html

Add `sdk` to `path` if the installer had not done it automatically.


Get the source
--------------

Download the source from [https://github.com/WachterJud/qaul.net-ng](github)

	git clone https://github.com/WachterJud/qaul.net-ng.git



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
	cmake ../ -DPORT=Android -DNDK_ROOT=/path/to/ndk -DEXTRALIB_PATH=./
	make


You will find the newly built apk in `android/bin`.


Eclipse
-------

- Import Existing Project
- Do **NOT** copy files.

