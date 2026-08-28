# Build the Debian Installers of the qauld Deamon

Build Debian installers on Linux of `qauld`, the qaul daemon, and of
`qauld-ctl`, the client that controls it. Each is packaged separately, so the
daemon can be installed on a headless machine without the client and vice versa.

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

# do the same for the qauld-ctl control client
cd ../qauld-ctl
cargo deb
cargo deb --target=armv7-unknown-linux-gnueabihf
```

You'll find the debian installers in the `rust/target/debian/` directory.
The Raspberry Pi installers are in the `rust/target/armv7-unknown-linux-gnueabihf/debian/` directory.

## Further Customization

To build the installer we are using the `cargo-deb` package. They have good documentation, for further customization:

<https://crates.io/crates/cargo-deb>
