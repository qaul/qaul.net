# Unix Instructions

This page contains all the instructions on building Qual.net on Unix and Unix-like platform.

## Prerequisites

### Get the source

Download the source from <insert source here> or
get the sources from github: git clone https://github.com/WachterJud/qaul.net.git

## Build

    mkdir builddir
    cd builddir

    cmake ../path/to/source -DPORT=GTK
    make package

You will find the package in your `builddir`, you may want install it?

