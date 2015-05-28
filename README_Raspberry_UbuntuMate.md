Build qaul.net-ng on Raspberry Pi running Ubuntu Mate
======================================================

Prerequesites
-------------

Install needed Software to download and build qaul.net from Source.

	sudo apt-get install git cmake build-essential pkg-config \
	libgtk-3-dev  libwebkitgtk-3.0-dev libdbus-1-dev autotools-dev \
	libasound2-dev bison flex automake


Download and Build
------------------

	# Download source code from github
	git clone https://github.com/WachterJud/qaul.net-ng.git
	
	# create build directory
	cd qaul.net-ng
	mkdir build
	cd build
	
	# generate make files
	cmake .. -DPORT=GTK
	
	# make and install qaul.net
	make
	sudo make install


#### Troubleshooting - Patching Needed

The endianness needs to be specified on this platform. The following lines 
need to be put in to the file
third_party/pjsip/src/pjsip/pjlib/include/pj/config_site.h 
(in the local build folder).

	#define PJ_IS_LITTLE_ENDIAN 1 
	#define PJ_IS_BIG_ENDIAN 0


The olsrd plugin 'olsrd_httpinfo.so.0.1' failed linking. Don't compile it.

	rm -R third_party/olsr/src/olsr/lib/httpinfo
	cp -R third_party/olsr/src/olsr/lib/txtinfo third_party/olsr/src/olsr/lib/httpinfo


Run qaul.net GUI client
-----------------------

Run qaul.net GKT client from the command line

	# run qaul.net GUI client form the command line	
	/opt/qaul/bin/qaul-gtk


Link the binary to your execution path

	sudo ln -s /opt/qaul/bin/qaul-gtk /usr/local/bin/qaul-gtk


Create the .deb Installer package
---------------------------------

To create the .deb installer, execute the following command in your build 
directory.

	# create the installer package
	make package

