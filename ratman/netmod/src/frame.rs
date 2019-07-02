//! Networking frames

use identity::Identity;
use crate::payload::Payload;

/// A frame represents a single packet sent over a netmod
pub struct Frame {
    /// Some messages are split across multiple frames
    /// that have sequence numbers so they can be
    /// re-assembled into complete messages at the other end
    pub sequence: u16,
    /// Origin ID (who sent the frame)
    pub sender: Identity,
    /// Destination ID (who will receive the frame)
    pub recipient: Option<Identity>,
    /// Origin-verification payload signature
    pub signature: [u8; 18],
    /// The actual data being transmitted, with validation metadata
    pub payload: Payload
}
