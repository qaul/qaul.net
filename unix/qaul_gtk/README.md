Linux Instructions
==================

qaul.net GTK client has been successfully tested on:
* Ubuntu 12.04: Precise Pangolin, 32-Bit & 64-Bit
* Ubuntu 14.04: Precise Pangolin, 32-Bit & 64-Bit
* Debian 7: Wheezy, 32-Bit & 64-Bit
* Linux Mint 14 & 16 Cinnamon: 32-Bit & 64-Bit

It should run on all recent Debian & Ubuntu based distributions.
Please feel free to add your tested distribution.


Installation and Compile Instructions
--------------------------------------

Compile olsrd

    # install required package
    sudo apt-get install bison flex
    # compile olsrd
    cd olsrd-0.6.6.2
    make
    # copy the executable to the qaul installation directory
    sudo mkdir -p /usr/share/qaul
    sudo cp olsrd /usr/share/qaul/
    cd ../

Compile olsrd_qaul plugin

    cd olsrd-0.6.6.2/lib/olsrd_qaul
    make
    # copy the shared library to the qaul installation directory
    sudo cp olsrd_qaul.so.0.1 /usr/share/qaul/
    # link it from the library directory
    sudo ln -fs /usr/share/qaul/olsrd_qaul.so.0.1 /usr/lib/olsrd_qaul.so.0.1
    cd ../../../

Compile qaulhelper

	cd linux/qaulhelper
	make
	# copy the executable to the qaul installation directory
	sudo cp qaulhelper /usr/share/qaul/
	# set SUID rights
	sudo chmod 6755 /usr/share/qaul/qaulhelper
	cd ../../

Compile pjsip library for VoIP

	# install required libraries
	sudo apt-get install g++ libasound2-dev
	# compile pjsip
    cd pjproject-2.3
    ./configure --disable-ffmpeg --disable-ssl --disable-video
    make dep
    make
    cd ../

Compile qaul.net

    # install gtk develper libraries
    sudo apt-get install libgtk-3-dev libwebkitgtk-3.0-dev libdbus-1-dev
    # change into directory and compile
    cd linux/qaul_gtk
    make
	# copy the executable to the qaul installation directory
	sudo cp qaul /usr/share/qaul/
    # link it from the program directory
    sudo ln -fs /usr/share/qaul/qaul /usr/bin/qaul
    cd ../../

Copy all the needed files 

    sudo cp linux/usr_share_qaul/* /usr/share/qaul/
    sudo cp -R www /usr/share/qaul/

Now you can run qaul.net from the command line

    qaul


Installer
---------

To build a qaul.net debian installer you may download and install the 
application "Debreate":
http://debreate.sourceforge.net/

Copy the binaries and files to the /usr/share/qaul directory

    /usr/share/qaul
        app_icon.png
        olsrd
        olsrd_linux.conf
        olsrd_qaul.so.0.1
        portfwd
        portfwd.conf
        qaul
        qaulhelper
        tail
        www

Strip the binaries from 

    cd /usr/share/qaul
    sudo strip -s olsrd olsrd_qaul.so.0.1 portfwd qaul qaulhelper

Create the debian installer:
* Start the Debreate application
* Open the Debreate configuration file linux/qaul_Debreate-Installer.dbp
  in Debrate
  file > open > linux/qaul_Debreate-Installer.dbp
* Navigate to the "Control" page and select the correct processor 
  "Architecture".
* Navigation to the "Build" page and click the green build button to
  build the installer.


Additional Software
-------------------
portfwd
Build "portfwd" to be able to forward the DHCP packages to qaul.

* Download latest version from sourceforge: 
  http://sourceforge.net/projects/portfwd/ 
  http://portfwd.sourceforge.net/

    # install required libraries
    sudo apt-get install automake1.4
    # compile software
    cd portfwd-0.29
    ./configure
    make
    # copy binary to qaul location
    sudo cp src/portfwd /usr/share/qaul/
    cd ../
