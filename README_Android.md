Android Instructions
====================

Compatibility
-------------

qaul.net has been tested on the following devices:

| Device            | Android version | Runs |
|:----------------- |:--------------- |:----:|
| Samsung Galaxy S2 | 2.3.x           | Y    |
| Samsung Galaxy S2 | 4.x.x           | N    |
| Samsung Galaxy S2 | 4.0.1 (CM9)     | N    |
| HTC Sensation     | 4.x.x           | Y    |
| HTC Sensation     | 4.0.1 (CM9)     | Y    |
| fairphone         | 4.x.x           | Y    |
| Oneplus One       | CM12.1 (5.1.1)  | ?    |

Legend:

  - Y = it works
  - N = it doesn't work yet
  - ? = it wasn't tested yet


Problems & Solutions
--------------------

  - qaul.net needs the IBSS/Ad-hoc wifi mode. This Wifi-Standard mode was removed by Google. 
    - On Android < 4.x IBSS mode can be activated via the wext (wireless extensions). Some vendors ship also their 4.x devices with wext.
    - On Android >= 4 IBSS mode can be activated on some devices via wpa_supplicant.
  - qaul.net needs ARM and Android > 2.3 Gingerbread (due to before unsupported pthread functions in pjsip). 
  - The Android device needs to be rooted.


Prerequisites
-------------

### Android SDK/NDK

For building the Android version of Qaul.net you will need both the Source Develpment Kit and the Native Development Kit.

  - Get the NDK here: https://developer.android.com/tools/sdk/ndk/index.html
  - Get the SDK here: https://developer.android.com/sdk/index.html

Add SDK location to your `PATH` if the installer had not done it automatically.

### Needed extra libraries

  1. Create directory for extra libs
  2. `cd` into it

  ```
  adb pull /system/lib/libcutils.so
  adb pull /system/lib/libwpa_client.so
  adb pull /system/bin/ifconfig
  adb pull /system/bin/iptables
  ```

Get the source
--------------

Download the source from https://github.com/WachterJud/qaul.net-ng/releases or get the current master from github:

```git clone https://github.com/WachterJud/qaul.net.git```


Build
-----

```
mkdir builddir
cd builddir

cmake /path/to/source -DPORT=Android -DNDK_ROOT=/path/to/ndk -DEXTRALIB_PATH=/path/to/extra/lib
make
```

You will find the newly built apk in `android/bin`.


Eclipse
-------

- Import Existing Project
- Do **NOT** copy files.
