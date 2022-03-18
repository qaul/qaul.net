# Build qaul.net for Raspberry Pi

qaul.net cannot be built on raspberry pi directly, as the
protobuf compiler does not run on ARM.
To build qaul.net for Raspberry Pi's ARM architecture, you
can cross-compile it on your development machine and transfer the
binaries to the raspberry pi.

## Prerequisites

Install prerequisites to cross-build qaul for raspberry pi.

### Debian, Ubuntu, Mint

Install the gcc linker and the rust target for arm

```sh
# Install gcc linker for arm
apt install -y gcc-arm-linux-gnueabihf

# Install cross compiler via rustup
rustup target add armv7-unknown-linux-gnueabihf
```

### Arch, Manjaro

Install the gcc linker and the rust target for arm.
Unfortunately the Arch AUR package has currently compilation problems, 
WI therefore suggest to install the already compiled toolchain.

```sh
# Arch
pacman -S arm-none-linux-gnueabihf-toolchain-bin

# Manjaro
pamac build arm-none-linux-gnueabihf-toolchain-bin
```

Install rust target for arm

```sh
# Install cross compiler via rustup
rustup target add armv7-unknown-linux-gnueabihf
```

In order to make the package `ring` link,  you need to symbolic link `arm-linux-gnueabihf-gcc`
to `arm-linux-gnueabihf-gcc`.

```sh
sudo ln -s /usr/bin/arm-none-linux-gnueabihf-gcc /usr/bin/arm-linux-gnueabihf-gcc
```

## Configuration of the Linker

In order for cargo to find the linker for the target,
the linker is referenced and configured in the file `rust/.cargo/config`.

`rust/.cargo/config`

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

## Run Build

Start the build from terminal from within this project folder.

```sh
# build the debug binaries with cargo
cargo build --target=armv7-unknown-linux-gnueabihf

# build the release binaries with cargo
cargo build --release --target=armv7-unknown-linux-gnueabihf
```

The binaries can be found in the `target/armv7-unknown-linux-gnueabihf` folder.
