# Manual build

Building `qaul-hubd` with [nix](../nix.md) is recommended!  If you
need to build via system dependencies, follow this guide, depending on
your platform.


## Install dependencies

**Rust**

You will need to have a Rust compiler, and cargo toolchain installed
on your system (minimum version `v1.42`).  [Install instructions][rust]

[rust]: https://www.rust-lang.org/tools/install


**Homebrew**

On MacOS you will need to install a package manager first to install
all the developer tools.  Alternatively you can also get them by
downloading XCode from the App Store.

[Homebrew](https://brew.sh/) is a popular package manager for MacOS!


**Build Tools**

You need to install a general development environment with the
following tools available.

* Debian/ Ubuntu/ Mint: `sudo apt install make gcc cmake git`
* Fedora/ CentOS: `sudo dnf install make gcc cmake git`
* Arch Linux: `sudo pacman -Sy make gcc cmake git`
* MacOS: `brew install make gcc cmake git`


## Build from source

Clone the main code repo:

```console
$ git clone https://git.open-communication.net/qaul/qaul.net
$ cd qaul.net/
```

Then simply run `cargo build --bin qaul-hubd --release` to build the
hubd binary.

The output artefact will be written to `./target/release/qaul-hubd`.
