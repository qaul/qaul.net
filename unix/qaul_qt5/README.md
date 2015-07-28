qaul.net QT client [Deprecated]
===============================

qaul.net QT client had been tested on:
* Ubuntu 11.10: Oneiric Ocelot 32-Bit
* Ubuntu 12.04: Precise Pangolin 32-Bit


Installation and Compile Instructions
--------------------------------------

Compile olsrd

    # install required package
    sudo apt-get install bison flex
    # compile olsrd
    cd olsrd_0.6.2
    make
    cp olsrd ../linux/qaul-build-desktop/
    cd ../

Compile olsrd_qaul plugin

    cd olsrd_0.6.2/lib/olsrd_qaul
    make
    cp olsrd_qaul.so.0.1 ../../../linux/qaul-build-desktop/
    # you need to install the shared library
    sudo make install
    cd ../../../

Compile pjsip library for VoIP

	# install required libraries
	sudo apt-get install g++
	# compile pjsip
    cd pjproject-2.2.1
    ./configure
    make dep
    make
    cd ../

Compile static qaullib

    cd libqaul
    make OS=linux
    cd ../

qaul.net is implemented in C++ in QT. 

* To be able to develop qaul.net on Linux, install the latest QT Creator (QT's IDE).
* Open QT project and run the software.


Troubleshooting
---------------

On Ubuntu 12.04 64-Bit pjsip failed compiling because it could't find the 
following program and library: cc1plus, libstdc++
They had to be linked manually to the standard directories:

    # link executable cc1plus
    sudo ln -s /usr/lib/gcc/x86_64-linux-gnu/4.6/cc1plus /usr/bin/cc1plus
    # link library stdc++
    sudo ln -s /usr/lib/gcc/x86_64-linux-gnu/4.6/libstdc++.a /usr/lib/libstdc++.a
