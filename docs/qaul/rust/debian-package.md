# Build a Debian Installer of the qauld Deamon

Build a Debian installer on Linux of `qauld`, the qaul server daemon

## Install Requirements

Install `cargo-deb` package

```sh
cargo install cargo-deb
```

## Build the Debian Installer

To build the debian install do the following in the terminal:

```sh
# move into the qauld rust directory
cd rust/clients/qauld

# run the debian installer creator to build it for your platform
cargo deb

# to build an install for the raspberry pi, run:
cargo deb --target=armv7-unknown-linux-gnueabihf
```

You'll find the debian installer in the `rust/target/debian/` directory.
The Raspberry Pi installer is in the `rust/target/armv7-unknown-linux-gnueabihf/debian/` directory.

## Further Customization

To build the installer we are using the `cargo-deb` package. They have good documentation, for further customization:

<https://crates.io/crates/cargo-deb>
