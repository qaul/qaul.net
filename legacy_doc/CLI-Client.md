Build qaul.net CLI client
=========================

This tutorial describes all the steps and options to build qaul.net CLI client.
This client is meant for server instances and for testing purposes. 
CLI client runs a headless server instance.
There is no CLI interface to interact with the program (yet).

CLI client currently builds and works on the following platforms:
* Linux
* Raspberry Pi

CLI-client does not configure Wifi, networking or routing. The network interfaces
have to be configured manually before starting the CLI client. The routing 
protocol olsrd has to be started manually to.


Prerequisites
-------------

Please install the prerequisites for your build platform in accordance with the 
tutorials of the build platform.

* [Linux](Linux_Debian.md)
* [Raspberry Pi](Raspberry_UbuntuMate.md)


Get the source
--------------

Download the source from [github](https://github.com/qaul/qaul.net)

	git clone --recursive https://github.com/qaul/qaul.net.git


Build
-----

    # create build directory in qaul.net source folder
    mkdir build
    cd build
	
	# cmake configuration options:
	# 
	# build CLI client (mandatory)
	#   -DGUI=CLI
	# compile program without Voice over IP feature (optional)
	#   -DVOIP=NO
    cmake ../ -DGUI=CLI -DVOIP=NO
    make
    
    # install qaul.net CLI build on your local system
    sudo make install


Start qaul-cli
--------------

The following steps are needed to successfully run the CLI client:

* Configure your wifi manually (make sure the IP in your DB):
  SSID: qaul.net, 
  Channel: 11, 
  IPv4: Manual IP configuration, 
  IP: 10.233.89.32, 
  Netmask: 255.0.0.0, 
  Gateway: 0.0.0.0 

* Start olsrd manually

    # linux
    sudo ./olsrd -i wlan0 -f ./olsrd.conf -d 1
    # osx
    sudo ./olsrd -i en1 -f ./olsrd.conf -d 1
    # windows 
    # run shell as administrator

* Run CLI client from user's qaul home folder

    qaul-cli

* Open the GUI in a web browser 
  http://localhost:8081/qaul.html

