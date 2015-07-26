Build qaul.net on Mac OSX
=========================

This tutorial describes all the steps to build qaul.net-ng on Mac OSX. qaul.net works on
Mac OSX > 10.5. This Tutorial has been tested on the following versions of Mac OSX:

* OSX 10.8 (Snow Leopard)


Prerequisites
-------------

The following tools need to be installed

* XCode (download from app store)
* cmake (install via macports)


Get the source
--------------

Download the source from [github](https://github.com/WachterJud/qaul.net-ng) 

	git clone https://github.com/WachterJud/qaul.net-ng.git


Build
-----

    # create build directory in qaul.net-ng source folder
    mkdir build
    cd build
	
	# build qaul.net
    cmake ../ -DPORT=OSX -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="/Library/qaul.net"
    make
    sudo make install


You can start the qaul app by double clicking the app in your Applications folder.
To start qaul from the terminal execute the following:

	# start qaul from the terminal to see the log messages
	/Applications/qaul.app/Contents/MacOS/qaul
