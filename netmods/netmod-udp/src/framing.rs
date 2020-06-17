//! UDP overlay protocol and framing

use netmod::{Frame, Target};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Envelope {
    /// Announcing an endpoint via multicast
    Announce,
    /// Reply to an announce
    Reply,
    /// A raw data frame
    Data(Vec<u8>),
}

impl Envelope {
    pub(crate) fn frame(f: &Frame) -> Vec<u8> {
        let inner = bincode::serialize(f).unwrap();
        let env = Envelope::Data(inner);
        bincode::serialize(&env).unwrap()
    }

    pub(crate) fn get_frame(&self) -> Frame {
        match self {
            Self::Data(ref vec) => bincode::deserialize(vec).unwrap(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn announce() -> Vec<u8> {
        let env = Envelope::Announce;
        bincode::serialize(&env).unwrap()
    }

    pub(crate) fn reply() -> Vec<u8> {
        let env = Envelope::Reply;
        bincode::serialize(&env).unwrap()
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub(crate) fn from_bytes(vec: &Vec<u8>) -> Self {
        bincode::deserialize(&vec).unwrap()
    }
}

/// A frame wrapped with the ID that it was targeted with
///
/// The ID can be resolved via the AddrTable to find out where to send
/// a payload
#[derive(Debug, Clone)]
pub(crate) struct FrameExt(pub(crate) Frame, pub(crate) Target);
