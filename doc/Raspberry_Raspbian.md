Build qaul.net on Raspberry Pi running Raspbian
===============================================

Prerequesites
-------------

Install needed software to download and build qaul.net from source.

	sudo apt-get install git cmake build-essential pkg-config \
	libgtk-3-dev  libwebkitgtk-3.0-dev autotools-dev \
	libasound2-dev bison flex automake


Download and Build
------------------

	# Download source code from github
	# qaul.net uses git submodules, therefore qaul.net should be cloned recursively or
	# the submodules must be initialized and updated separately
	## git submodule init
	## git submodule update
	git clone --recursive https://github.com/qaul/qaul.net.git
	
	# create build directory
	cd qaul.net
	mkdir build
	cd build
	
	# generate make files
	cmake .. -DRaspberry=1
	
	# make and install qaul.net
	make
	sudo make install


Run qaul.net GUI client
-----------------------

Run qaul.net GKT client from the command line

	# run qaul.net GUI client form the command line	
	qaul-gtk


Create the .deb Installer package
---------------------------------

To create the `.deb` installer, execute the following command in your build 
directory.

	# create the installer package
	make package
