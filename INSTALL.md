# Download, Build and Install
## Prerequisites

In order to build this project, you need to install the following prerequisites:

**Rust programming language** with it's compiler cargo.
Easiest install is via rustup: https://rustup.rs/

Install compiler tool chain for C code

```sh
# Debian, Ubuntu, Mint
sudo apt install build-essential
```

## Get Repository

```sh
# clone repository
git clone https://git.open-communication.net/qaul/experiments/qaul-libp2p.git

# move into project folder
cd qaul-libp2p
```

## Build

Run the following command from this folder.

```sh
# build the entire project
cargo build
```

## Usage

Please have a look at the clients `clients/cli` & `clients/rpc-cli` on how to use them.
