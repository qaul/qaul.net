Build qaul.net on Windows
=========================

This tutorial describes all the steps to build qaul.net client on Windows.
qaul.net works on Windows 7 / 8 / 10. Windows XP is not supported.

This tutorial has been tested on the following versions of Windows:

* Windows 7


Prerequisites
-------------

The following tools need to be installed:

* Git (version control)
  * Download git for Windows: https://www.git-scm.com/download/win
* MSYS2
  * Download MSYS2 Installer: http://msys2.github.io/
  * Install MSYS2 to C:/msys2
  * Open MinGW Shell 'C:/msys2/ming32_shell'
  * Install needed programs from the mingw32_shell

```
# update your installation
pacman -Sy
pacman -Su
## close and reopen mingw32_shell after update

# install needed programs
pacman -S mingw-w64-i686-gcc mingw-w64-i686-cmake make patch bison 
```

* Microsoft Visual Studio
* MSBuild (for builds via command line interface)
* NSIS - Nullsoft Scriptable Install System
  * Download NSIS installer: http://nsis.sourceforge.net


Get the source
--------------

Download the source from [github](https://github.com/qaul/qaul.net)

	git clone --recursive https://github.com/qaul/qaul.net.git

	
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
	# depending on your Visual Studio version use one of the following
	# commands
	# for Visual Studio 2013 run:
	## cmake .. -G "Visual Studio 12 2013"
	# for Visual Studio 2010 run:
	## cmake .. -G "Visual Studio 10 2010"
	# or just let cmake guess your Visual Studio version and run:
	cmake ..


Open the Microsoft Visual Studio Solution qaul.sln in Visual Studio.

* To be able to run qaul.net from within Visual Studio, set the qaul project 
  as your start up project in Visual Studio:
  Right-click on the qaul project icon and select "Set as StartUp Project".


Build qaul.net executable from command line. 

* Download and install the Microsoft Build Tools that contain MSBuild

```
##########################################
# Build qaul.net with MSBuild
##########################################
# CLI build commands for the MinGW32 Shell
# Build debug version
MSBuild.exe qaul.vcxproj -p:Configuration=Debug
# Build release version
MSBuild.exe qaul.vcxproj -p:Configuration=Release

##########################################
# Build qaul.net with cmake (via MSBuild)
##########################################
cmake --build . --target qaul --config Release
```


Create Windows Installer
------------------------

To create the installer invoke the following command.

	cpack -C Release
