# Build qaul.net on Linux

## Prerequisites

The core of qaul.net is written in the programming language [Rust].
In order to build libqaul, you need to install [Rust] on your Computer.

There are some other build tools that need to be installed.
Here an hopefully complete list of all prerequisites:

* Rust: [Rust installation instructions]
* make
* GCC
* cmake


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
