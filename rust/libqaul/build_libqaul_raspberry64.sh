#!/usr/bin/env bash

# Build libqaul on linux for raspberry pi ARM 64 bit architecture
#
# For this script to work, one needs to have installed the
# target aarch64-unknown-linux-gnu for cargo and also the gcc linker.
# You may install it using the following commands:
#
# On Debian, Ubuntu, Mint
#
# ```
# rustup target add aarch64-unknown-linux-gnu
# apt install -y gcc-aarch64-linux-gnu
# ```
#

cargo build --release --target=aarch64-unknown-linux-gnu
