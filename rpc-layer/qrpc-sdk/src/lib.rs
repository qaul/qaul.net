//! # qaul rpc sdk
//!
//! This library provides the basic capabilities of interacting with a
//! qrpc-broker, and other qaul services.  These docs outline API
//! usage and concrete types.  For an overview of concepts, consult
//! the [contributors manual][manual]
//!
//! [manual]: https://docs.qaul.net/contributors/technical/rpc-layer
//!
//! ## Using this sdk
//!
//! In order to interact with a running qrpc-broker instance your
//! service needs to register itself and it's capabilities.  This
//! mechanism is handled by this sdk.
//!
//! If you want other third-party services to be able to depend on you
//! service you may want to split it into two parts: one part contains
//! the service logic, the other type and RPC interface information.
//! This way a third-party service can include your services' type
//! library (usually called `<service name>-rpc`), and gain access to
//! all public RPC functions.
