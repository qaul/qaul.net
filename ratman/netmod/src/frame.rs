//! Networking frames

use identity::Identity;

/// A frame represents a single packet sent over a netmod
pub struct Frame {
    /// Origin ID (who sent the frame)
    pub origin: Identity,
    /// Destination ID (who will receive the frame)
    pub destination: Identity,
    /// Origin-verification payload signature
    pub signature: [u8; 18],
    /// Payload length
    pub length: u32,
    /// Various length payload
    pub payload: Vec<u8>,
}