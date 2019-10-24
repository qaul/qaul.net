//! Networking frames

use crate::SeqId;
use identity::Identity;
use serde::{Serialize, Deserialize};

/// Encoded recipient data
///
/// A `Frame` can either be addressed to a single user on the network,
/// or to the network as a whole. The latter is called `Flood` and
/// should primarily be used for small payload sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recipient {
    /// Addressed to a single user ID on the network
    User(Identity),
    /// Spreading a `Frame` to the whole network
    Flood,
}

/// A sequence of data, represented by a single network packet
///
/// Because a `Frame` is usually created in a sequence, the
/// constructors assume chainable operations, such as a `Vec<Frame>`
/// can be returned with all sequence ID information correctly setup.
#[derive(Clone)]
pub struct Frame {
    /// Sender information
    pub sender: Identity,
    /// Recipient information
    pub recipient: Recipient,
    /// Data sequence identifiers
    pub seqid: SeqId,
    /// Raw data payload
    pub payload: Vec<u8>,
}
