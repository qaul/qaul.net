
# Windows Instructions

This page contains all the instructions on building qaul.net on the **Windows** platform.

## Prerequisites

Below are all the required tools and files you will need in order to build qauls.net.

### Git

https://www.git-scm.com/download/win
 
### MinGW

For building the Windows version of Qaul.net you will need a recent MinGW installed.

1. Get the latest version of **MingGW** from http://sourceforge.net/projects/mingw-w64/ or http://mingw-w64.yaxm.org.

2. Install the following,  `i686-w64-mingw32-gcc`, `cmake`, `bison` and  `yacc`.

### Get the source

Download the source from <insert source here> or
get the sources from github: git clone https://github.com/WachterJud/qaul.net.git

## Build

    mkdir builddir
    cd builddir

    cmake ../path/to/source -G "MSYS Makefiles" -DPORT=Windows
    make
