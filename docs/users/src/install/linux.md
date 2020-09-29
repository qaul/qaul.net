# Build qaul.net on Linux

## Prerequisites

**Rust**

The core of qaul.net is written in the programming language [Rust].
In order to build libqaul, you need to install [Rust] on your Computer.

Use this web site to install [Rust]: [Rust installation instructions]


**Build Tools**

There are some other build tools that need to be installed.
Here an hopefully complete list of all prerequisites:

* make
* GCC
* cmake

To install it on Debian / Ubuntu / Linux Mint run:

```sh
# to download and build qaul.net install the following:
sudo apt install make gcc cmake git 
```


## Get and Compile the Sources

Clone the [qaul.net git repository](https://git.open-communication.net/qaul/qaul.net.git) from the terminal. 

```bash
git clone https://git.open-communication.net/qaul/qaul.net.git
```

Now you can move into the repository and build the application.

```bash
# move into the qaul.net project folder
cd qaul.net

# build the application via the rust build tool `cargo`
cargo build
```

The rust build created the following binaries:

* `qaul-linux`
* `linux-cli`
* `linux-http-test`


## Run and Test qaul.net

See the [test chapter](../test/index.md).



[Rust]: https://www.rust-lang.org/tools/install
[Rust installation instructions]: https://www.rust-lang.org/tools/install
