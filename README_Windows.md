
# Windows Instructions

## Prerequisites

### Git
https://www.git-scm.com/download/win
 
### Mingw

For building the Windows version of Qaul.net you will need a recent MinGW
install

get it from http://sourceforge.net/projects/mingw-w64/ or http://mingw-w64.yaxm.org


install i686-w64-mingw32-gcc cmake bison yacc


## get the source

Download the source from <insert source here> or
get the sources from github: git clone https://github.com/WachterJud/qaul.net.git

## build

    mkdir builddir
    cd builddir

    cmake ../path/to/source -G "MSYS Makefiles" -DPORT=Windows
    make

