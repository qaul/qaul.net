# Build qaul.net on MacOS

This guide will give you a step by step instruction to build qaul.net on MacOS.

## Prerequisites

**XCode**

In order to get all the developer tools to compile qaul.net you need to install the XCode developer environment.
It brings most of the tools with it.

To install `XCode`:

* Download and install `XCode` from app store
* Open the XCode app once and accept the license
* Install the 'XCode Command Line Tools' via terminal

```sh
xcode-select --install
```

**Cmake**

To install `Cmake` you can download and install the binary from the web page:
https://cmake.org/download/


**Rust**

The core of qaul.net is written in the programming language [Rust].
In order to build libqaul, you need to install [Rust] on your Computer.

[Rust installation instructions]



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
