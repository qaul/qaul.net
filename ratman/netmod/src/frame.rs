//! Networking frames

use crate::payload::Payload;
use identity::Identity;

/// A frame represents a single packet sent over a netmod
#[derive(Debug)]
pub struct Frame {
    /// Indicate sequence window for this frame
    ///
    /// A `Message` can be split into multiple `Frame`s that are
    /// indicated into a sequence by these upper and current bounds.
    /// The first number is current, the second is upper bound
    pub sequence: (u16, u16),
    /// Origin ID (who sent the frame)
    pub sender: Identity,
    /// Destination ID (who will receive the frame)
    pub recipient: Option<Identity>,
    /// Origin-verification payload signature
    pub signature: [u8; 18],
    /// The actual data being transmitted, with validation metadata
    pub payload: Payload,
}
