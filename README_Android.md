
# Android Instructions

It should work on all ARM android devices starting from Android 2.3 Gingerbread
(due to before unsupported pthread functions in pjsip). 
The Android device needs to be rooted.

qaul.net was tested on the following devices:

| Device | 2.3 | 4.0 | CM9 |
| --- | --- | --- | --- |
| Samsung Galaxy S2 | Y | N | N |
| HTC Sensation | ? | Y | Y |

Legend:
* Y = it works
* N = it doesn't work yet
* ? = it wasn't tested yet

## Prerequisites

### Android SDK/NDK

For building the Android version of Qaul.net you will need both the Source
Develpment Kit *and* the Native Development Kit

get NDK https://developer.android.com/tools/sdk/ndk/index.html

get SDK https://developer.android.com/sdk/index.html
add sdk to path if not done by installer

### Needed extra libraries

create directory for extra libs

cd into it

	adb pull /system/lib/libcutils.so
	adb pull /system/lib/libwpa_client.so
	adb pull /system/bin/ifconfig
	adb pull /system/bin/iptables

## get the source

Download the source from <insert source here> or
get the sources from github: git clone https://github.com/WachterJud/qaul.net.git

## build

    mkdir builddir
    cd builddir

    cmake ../path/to/source -DPORT=Android -DNDK_ROOT=/path/to/ndk -DEXTRALIB_PATH=/path/to/extra/lib
    make

find the apk in android/bin

## Eclipse

- Import Existing Project
- Do NOT copy files.
