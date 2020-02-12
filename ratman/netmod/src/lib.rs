//! # A network module abstraction for Ratman
//!
//! Netmod provides an async interface to interact with endpoints
//! basic data frame definitions, and frame sequencing.
//!
//! The interface itself makes no assumption about underlying address
//! spacing or resend behaviour.  Using netmod as a library allows you
//! to write Ratman compatible network adapters.
//!
//! ## Frames, Sequences and Signatures
//!
//! A `Frame` is a single packet that is sent over a network
//! connection.  It corresponds (for example) to a UDP packet in other
//! protocols.  It contains sender, recipient information, and a
//! sequence indicator (seqid), which is constructed over data slices,
//! and can be reassembled on the other side of a circuit.
//!
//! When constructing a `Frame` sequence, the payload is split into
//! appropriately sized chunks, then hashed, and those signature
//! hashes are entered into the sequence ID `next` sequentially.  The
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
//! The payload signature is used to validate transport layer
//! integrity (resends are up to a user of this interface to
//! implement, as well as associating sequential frames into a data
//! set.
#![allow(warnings)]

#[macro_use]
extern crate serde;

mod endpoint;
mod frame;
mod result;
mod seq;

pub use endpoint::Endpoint;
pub use frame::{Frame, Recipient, Target};
pub use result::{Error, Result};
pub use seq::{SeqId, SeqData, SeqBuilder};
