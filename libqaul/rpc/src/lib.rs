//! ## libqaul rpc utilities
//!
//! This library exposes structures and serialisation utilities to
//! interact with the `libqaul` API remotely, via a simple RPC
//! protocol.  By default the `proto` feature flag is enabled that
//! also creates a serialisation/deserialisation protocol via cap'n
//! proto.
//!
//! The transport layer is left out of this library, you can layer
//! this RPC protocol over `libqaul-ws` for web sockets, `libqaul-ipc`
//! for a client/server socket API, or `libqaul-http` for a json http
//! server.

mod api;
pub use api::{Change, contacts};


