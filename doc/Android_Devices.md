Android Device Compatibility of qaul.net release-1.0.0
======================================================

Requirements
------------

* qaul.net requires the android devices to be rooted! 

* qaul.net needs the IBSS/Ad-hoc wifi mode. This Wifi-Standard mode was 
  removed by google. 
  * On Android < 4.x IBSS mode can be activated via the wext (wireless 
    extensions). 
    Some vendors ship also their 4.x devices with wext.
  * On Android >= 4 IBSS mode can be activated on some devices via 
    wpa_supplicant.
* qaul.net needs ARM and Android > 2.3 Gingerbread
  (due to before unsupported pthread functions in pjsip). 


Device Compatibility List
-------------------------

quul.net has been tested on the following devices:

| Device            | 2.3 | 4.x | CM9 |
| ----------------- | --- | --- | --- |
| Samsung Galaxy S2 | Y   | N   | N   |
| HTC Sensation     | ?   | Y   | Y   |
| fairphone         | ?   | Y   | ?   |

Legend:

* Y = it works
* N = it doesn't work yet
* ? = it hasn't been tested yet

