# qaul.net Contributers Guide

This guide should always be readable in it's latest version online:
https://docs.qaul.net

This guide is created with the help of the free software 
[mdBook](https://rust-lang-nursery.github.io/mdBook/).
It is written by [Markdown](https://www.markdownguide.org/getting-started) files.
Feel free to edit it.


## Install mdBook

In order to build this guide you may want to install the free mdBook 
software.

mdBook is written in the programming language Rust. To be able to 
install mdBook, please install Rust first. To install Rust please 
follow these instructions:
https://rustup.rs/

```
# Install mdBook
cargo install mdbook
```


## Develop

While writing and contributing to this guide

```
# start mdBook development server
mdbook serve
```

You can now browse your latest changes locally: http://localhost:3000


## Build this Guide

To build the HTML version of this guide, run the following script:

```
# Build this mdBook
./build.sh
```

You can find the finished book in the directory `book`.