# Build qaul.net's Rust Code

This guide will build the following libraries and executables:

* `libqaul` - the shared library that runs the heart of qaul.net
* [qaul-cli](../../rust/clients/cli/README.md) - a CLI testing application for qaul.net
* [qaul-rpc-cli](../../rust/clients/rpc-cli/README.md) - a CLI testing application for qaul.net

## Build

Run the following command from this folder:

```sh
# build the entire rust code of the project
cd rust
cargo build
```

## Run CLI Clients

Please have a look at the clients `rust/clients/cli` & `rust/clients/rpc-cli` on how to use them.

* [qaul-cli](../../rust/clients/cli/README.md) - a CLI testing application for qaul.net
* [qaul-rpc-cli](../../rust/clients/rpc-cli/README.md) - a CLI testing application for qaul.net
