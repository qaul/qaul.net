//! `netmod` is a network module abstraction for `RATMAN`
//!
//! It provides a small interface to interact with endpoints (send/
//! receive frames) and basic data frame definitions.
//!
//! The interface itself makes no assumption about underlying address
//! spacing or resend behaviour.  Using `netmod` as a library allows
//! you to write RATMAN-compatible network adapters.
//!
//! ## Frames, Sequences and Signatures
//!
//! A `Frame` is a single packet that is sent over a network
//! connection. It corresponds to a UDP packet in other protocols. It
//! contains `sender` and `recipient` information, as well as a
//! sequence indicator (`seqid`), which is used to re-associate a
//! `Frame` series back into a `Message` on higher layers.
//!
//! The signature is constructed over the payload, which is a
//! `Vec<u8>`, containing nested information from higher layers.
//!
//! When constructing a `Frame` sequence, the payload is split into
//! appropriately sized chunks, then hashed, and those signature
//! hashes are entered into the sequence ID `next` sequentially. The
//! following diagram explains the concept further.
//!
//! ```norun
//! |--------------|        |--------------|        |--------------|
//! |  Frame #1    |        |  Frame #2    |        |  Frame #2    |
//! | next: f4aa   | ------ | next: bb61   | ------ | next: NONE   |
//! | sig: a1a1    |        | sig: f4aa    |        | sig: bb61    |
//! |--------------|        |--------------|        |--------------|
//! ```
//!
//! The payload signature is therefore used to validate transport
//! layer integrity, as well as associating sequential frames into a
//! data set.
//!
//! **Important**: the payload signature is non-cryptographic and only
//! provides transport layer error detection, not cryptographic
//! tampering protection. Ideally the inner payload is a
//! cryptographically signed message which can be verified on higher
//! layers.
#![allow(warnings)]

#[macro_use] extern crate serde;

mod endpoint;
mod frame;
mod seq;
mod result;

pub use endpoint::Endpoint;
pub use frame::{Frame, Recipient, Target};
pub use seq::{SeqId, Sequence};

pub use result::{Error, Result};
