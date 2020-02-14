//! # libqaul RPC structures
//!
//! This library exposes structures and serialisation utilities to
//! interact with the `libqaul` API remotely, via a simple RPC
//! protocol.  By default the `proto` feature flag is enabled that
//! also creates a serialisation/deserialisation protocol via [cap'n
//! proto](https://capnproto.org).
//!
//! The RPC protocol defined in this library makes no assumption about
//! the layering/ framing used to communicate with the libqaul core.
//! You can layer it over `libqaul-ws` for web sockets, `libqaul-ipc`
//! for a client/server socket API, or `libqaul-http` for a json http
//! api.
//!
//! In order to use this RPC crate correctly you will also have to
//! depend on the `libqaul` crate for structure, error, and return
//! type definitions.
#![allow(unused)]

mod api;
pub use api::{contacts, files, messages, users};

//mod proto;
