//! libqaul websocket RPC
//!
//! The native interface for libqaul in async Rust.  But a few other
//! RPC interfaces are exposed via the libqaul-rpc collection.  One of
//! them is the websocket interface, which is primarily used by the
//! qaul.net webui.
//!
//! The structures are encoded in JSON, as described by the
//! libqaul-rpc structures.  Every request has an envelope, which
//! contains in ID and some data.  the data can either be a request or
//! a response, with appropriate data or error values inside.
//!
//! Because web devs are a bunch of pussies this crate also wraps the
//! envelope in a way that web devs will like, such as making the data
//! generic (a string) and pulling out the method names; things that
//! other rpc layers would hit you for but hey, it's 2020.

mod env;
pub(crate) use env::{JsonAuth, JsonEnvelope, JsonMap};

mod parser;
