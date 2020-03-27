# Build qaul.net on Linux

## Prerequisites

The core of qaul.net is written in the programming language `Rust`.
In order to build libqaul, you need to install `rust` on your Computer.
You'll find the installation instructions here: 
https://www.rust-lang.org/tools/install


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


## Run and Test qaul.net

The rust build created the following binaries

* `qaul-linux`
* `linux-cli`
* `linux-http-test`

To run the test `linux-http-test` enter the following into your terminal.

```bash
cargo run --bin linux-http-test
```
