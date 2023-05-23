# Crosscompile qaul for Raspberry Pi 64 bit

To build qaul for Raspberry Pi's 64 bit ARM architecture on another
architecture, you can cross-compile it on your development machine 
and transfer the binaries to the raspberry pi.

## Prerequisites

Install prerequisites to cross-build qaul for raspberry pi.

### Debian, Ubuntu, Mint

Install the gcc linker and the rust target for arm

```sh
# Install gcc linker for arm
apt install -y aarch64-unknown-linux-gnu

# Install cross compiler via rustup
rustup target add aarch64-unknown-linux-gnu
```

### Arch, Manjaro

Install the gcc linker and the rust target for arm.
Unfortunately the Arch AUR package has currently compilation problems, 
WI therefore suggest to install the already compiled toolchain.

```sh
# Arch
pacman -S aarch64-linux-gnu-gcc

# Manjaro
pamac install aarch64-linux-gnu-gcc
```

Install rust target for arm

```sh
# Install cross compiler via rustup
rustup target add aarch64-unknown-linux-gnu
```

## Configuration of the Linker

In order for cargo to find the linker for the target,
the linker is referenced and configured in the file `rust/.cargo/config`.

`rust/.cargo/config`

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

## Run Build

Start the build from terminal from within this project folder.

```sh
# build the debug binaries with cargo
cargo build --target=aarch64-unknown-linux-gnu

# build the release binaries with cargo
cargo build --release --target=aarch64-unknown-linux-gnu
```

The binaries can be found in the `target/aarch64-unknown-linux-gnu` folder.
