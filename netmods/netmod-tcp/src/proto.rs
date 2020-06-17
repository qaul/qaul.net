//! TCP internal protocol used to share connection state

use netmod::Frame;
use serde::{Deserialize, Serialize};

/// An internally used packet format
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Packet {
    /// A general handshake message
    ///
    /// If this message get's ignored the connection will time out
    /// automatically and the peer should be marked as "unreachable"
    /// in the peer list.
    Hello { port: u16 },
    /// A periodic update to or from a peer
    KeepAlive,
    /// An actual data packet
    Frame(Frame),
}
