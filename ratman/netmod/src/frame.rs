//! Networking frames

use identity::Identity;

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
    /// Payload length
    pub length: u32,
    /// Various length payload
    pub payload: Vec<u8>,
}
