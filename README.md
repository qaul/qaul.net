# qaul.net routing core

This library exposes a few different things

- Rust `trait`s that allow easy extendability to the routing core, i.e. implementing different backend shims
- A C-like API that allows external code to call this library without needing a Rust compiler
- The actual routing core which implements a `BATMAN`-like routing protocol