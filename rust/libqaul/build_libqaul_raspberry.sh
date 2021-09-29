#!/usr/bin/env bash

# Build libqaul on linux for raspberry pi ARM architecture
#
# For this script to work, one needs to have installed the
# target armv7-unknown-linux-gnueabihf for cargo and also the gcc linker.
# You may install it using the following commands:
#
# On Debian, Ubuntu, Mint
#
# ```
# rustup target add armv7-unknown-linux-gnueabihf
# apt install -y gcc-arm-linux-gnueabihf
# ```
#
# Alternatively one can use the docker build image in `/utilities/build-raspberry-docker`
# which comes preinstalled with the crosscompile toolchain.
#

cargo build --release --target=armv7-unknown-linux-gnueabihf
