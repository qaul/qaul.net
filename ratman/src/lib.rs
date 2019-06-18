//! RATMAN decentralised routing protocol
//!
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

use identity::Identity;

/// A communication message sent over RATMAN
///
/// In the simplest case, this maps directly to a `netmod` `frame`,
/// but it doesn't have to.
/// In some circumstances a message might be split into multiple parts
/// to be more digestable for different networking backplanes.
///
/// Also to consider is context: a `Message` is part of a service,
/// while a `frame` is only a routing layer transport concept.
/// The contents of the `Message` might be encrypted or otherwise
/// unknown (i.e. with unknown plugin services), whereas a `frame`
/// always has a known context (i.e. sending Data `X` to node `Y`).
pub struct Message {
    origin: Identity,
    target: Identity,
    /// Payload length
    pub length: u32,
    /// Various length payload
    pub payload: Vec<u8>,
}
