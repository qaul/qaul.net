# qaul.net Contributors' Manual

This guide should always be readable in it's latest version online:
https://docs.qaul.net

This guide is created with the help of the free software [mdBook].  It
is written in [Markdown] files.  Feel free to edit it.

[mdBook]: https://rust-lang-nursery.github.io/mdBook/
[Markdown]: https://www.markdownguide.org/getting-started

## Install mdBook

In order to build this guide you may want to install the free mdBook
software.

There are various ways to install mdbook: try getting it from your OS
vendor (Linux distribution, etc) first.  If that is not possible, or
your platform does not have a new enough version, you can also easily
build it from source.  For this you will need to have Rust installed
(as for the rest of qaul.net).

After installing Rust you can install mdbook as follows:

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
