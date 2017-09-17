Build qaul.net on Debian Linux
==============================

This tutorial describes all the steps to build qaul.net on a Debian 
Linux distro. It has been tested on the following distributions:

* Debian
* Ubuntu
* Linux Mint


Prerequesites
-------------

Install needed software to download and build qaul.net from Source.

	sudo apt-get install git cmake build-essential pkg-config \
	libgtk-3-dev  libwebkitgtk-3.0-dev libdbus-1-dev autotools-dev \
	libasound2-dev bison flex automake



Download and Build
------------------

	# Download source code from github
	git clone --recursive https://github.com/qaul/qaul.net.git
	
	# create build directory
	cd qaul.net
	mkdir build
	cd build
	
	# generate make files
	cmake ..
	
	# make and install qaul.net
	make
	sudo make install


#### Troubleshooting

##### CMAKE Failure

cmake needs to be of a version 3.1 or higher to be able to run 
sucessfully. 

###### Debian Wheezy and Debian Jessie
On debian wheezy and debian jessie one needs to install cmake from 
wheezy-backports.

Add the following line to the file /etc/apt/sources.list to be able to 
use the wheezy-backports repository 

	deb http://ftp.de.debian.org/debian/ wheezy-backports main non-free


Install the cmake version from backports

    # update the package list
    sudo apt-get update
    # install cmake from wheezy-backports
    sudo apt-get -t wheezy-backports install cmake

###### Ubuntu Trusty

Install cmake from ppa

	# add ppa
	sudo add-apt-repository ppa:george-edison55/cmake-3.x
	# install newer CMAKE
	sudo apt-get update
	sudo apt-get install cmake


##### Debian Wheezy

###### Make error

Make fails to build portfwd due to missing execution permissions for 
bootstrap. Add the permissions to bootstrap:

	chmod +x third_party/portfwd/src/portfwd/bootstrap


Run qaul.net GUI client
-----------------------

Run qaul.net GKT client from the command line

	# run qaul.net GUI client form the command line	
	/opt/qaul/bin/qaul-gtk


Run qaul.net GUI client
-----------------------

Run qaul.net GKT client from the command line

	# run qaul.net GUI client form the command line	
	/opt/qaul/bin/qaul-gtk


Create the .deb Installer package
---------------------------------

To create the .deb installer, execute the following command in your build 
directory.

	make package
	
	# if you have run `sudo make install` before running `make package`
	# you may need to delete some files created with root permissions
	# first. You may find these files executing the follwing command:
	## find . -user root
