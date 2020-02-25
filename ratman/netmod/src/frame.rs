//! Networking frames

use crate::{SeqBuilder, SeqData, SeqId};
use identity::Identity;
use serde::{Deserialize, Serialize};

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

/// Describes an endpoint's send target
///
/// This is different from a Recipient in that it doesn't encode
/// information about a user on the global network.  It's values are
/// used by one-to-many Endpoint implementors to desambiguate their
/// own routing tables without having to replicate the Ratman internal
/// routing table.
///
/// If your endpoint doesn't implement a one-to-many link (i.e. if
/// it's always one-to-one), just let this value to `Single(0)` (`Target::default()`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Send message to all reachable endpoints
    Flood,
    /// Encodes a specific target ID
    Single(u16),
}

impl Default for Target {
    fn default() -> Self {
        Self::Single(0)
    }
}

/// A sequence of data, represented by a single network packet
///
/// Because a `Frame` is usually created in a sequence, the
/// constructors assume chainable operations, such as a `Vec<Frame>`
/// can be returned with all sequence ID information correctly setup.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    /// Sender information
    pub sender: Identity,
    /// Recipient information
    pub recipient: Recipient,
    /// Data sequence identifiers
    pub seq: SeqData,
    /// Raw data payload
    pub payload: Vec<u8>,
}

impl Frame {
    /// Produce a new dummy frame that sends nonsense data from nowhere to everyone.
    pub fn dummy() -> Self {
        SeqBuilder::new(
            Identity::from([0; 16]),
            Recipient::Flood,
            Identity::random(),
        )
        .add(vec![0xDE, 0xAD, 0xBE, 0xEF])
        .build()
        .remove(0)
    }

    /// Return the sequence Id of a frame
    pub fn seqid(&self) -> SeqId {
        self.seq.seqid
    }
}
