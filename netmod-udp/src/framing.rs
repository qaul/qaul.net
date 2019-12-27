//! UDP overlay protocol and framing

use serde::{Serialize, Deserialize};
use netmod::Frame;

/// A framing device to encapsulate the UDP overlay protocol
///
/// Multiple UDP endpoints need to be able to discover each other,
/// which is done with a simple protocol where announcements are
/// periodically sent via multicast to advertise an IP as a valid
/// endpoint.
///
/// These do not have to track what IDs are reachable via them, only
/// what internal ID they are represented by.  All other routing is
/// then done via Ratman and the netmod API which considers target
/// state.
#[derive(Serialize, Deserialize)]
pub(crate) enum Envelope {
    /// Announcing an endpoint via multicast
    Announce,
    /// A raw data frame
    Data(Vec<u8>),
}

/// A frame wrapped with the ID that it was targeted with
#[derive(Debug, Clone)]
pub(crate) struct FrameExt(Frame, u16);
