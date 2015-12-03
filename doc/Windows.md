Build qaul.net on Windows
=========================

This tutorial describes all the steps to build qaul.net-ng client on Windows. 
qaul.net works on Windows 7 / 8 / 10. Windows XP is not supported.

This Tutorial has been tested on the following versions of Windows:

* Windows 7


Prerequisites
-------------

The following tools need to be installed

* Git (version control)
  * Download git for Windows: https://www.git-scm.com/download/win
* MSYS2
  * Download MSYS2 Installer: http://msys2.github.io/
  * Install MSYS2 to C:/msys2
  * Open MinGW Shell 'C:/msys2/ming32_shell'
  * Install needed programs from the mingw32_shell

	# update your installation
	pacman -Sy
	pacman -Su
	## close and reopen mingw32_shell after update
	
	# install needed programs
	pacman -S mingw-w64-i686-gcc mingw-w64-i686-cmake make patch bison 

* Microsoft Visual Studio	
	

Get the source
--------------

Download the source from [github](https://github.com/WachterJud/qaul.net-ng) 

	git clone https://github.com/WachterJud/qaul.net-ng.git

	
Build
-----

Build qaul libraries, executables and Visual Studio project file using MinGW

    # create build directory
	mkdir build
    cd build
	
	# build OLSRD, pjsip library, qaullib
    cmake .. -G "MSYS Makefiles" -DGUI=NATIVE
    make
	
	# build Visual Studio project files
	cd src/client/win
	mkdir build
	cd build
	cmake .. -G "Visual Studio 12 2013"
