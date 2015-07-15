Build qaul.net on Mac OSX
=========================

This tutorial describes all the steps to build qaul.net-ng on Mac OSX>
It has been tested on the following versions of Mac OSX:

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
    cmake ../ -DPORT=OSX -G "Unix Makefiles"
    make package


You will find the package in your `build` directory

