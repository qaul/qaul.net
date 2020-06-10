use chrono::{DateTime, Utc};
use identity::Identity;
use netmod::Recipient;
use serde::{Deserialize, Serialize};

/// A unique, randomly generated message ID
pub type MsgId = Identity;

/// Represents the time of sending and receiving this frame
///
/// Because there is no guarantee that the host clock is accurate or
/// being maliciously manipulated, the sending time should not be
/// trusted.  A timestamp that should be used by applications is
/// available via the `local()` function.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimePair {
    sent: DateTime<Utc>,
    recv: Option<DateTime<Utc>>,
}

impl TimePair {
    /// A utility function to create a new sending timestamp
    pub fn sending() -> Self {
        Self {
            sent: Utc::now(),
            recv: None,
        }
    }

    /// Update the received time in a timestamp locally received
    pub fn receive(&mut self) {
        self.recv = Some(Utc::now());
    }

    /// A test function to strip the recv-time
    pub(crate) fn into_sending(self) -> Self {
        Self { recv: None, ..self }
    }

    /// Get the most likely local time
    pub fn local(&self) -> DateTime<Utc> {
        self.recv.unwrap_or(self.sent)
    }
}

/// An atomic message with a variable sized payload
///
/// A message is only ever addressed to a single node, or everyone on
/// the network.  The signature is required to be present, if a
/// payload is.  The payload can be empty, which can be used to create
/// a ping, or using the 16 byte MsgId as payload.  In these cases,
/// the sigature can also be empty.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// A random message ID
    pub id: MsgId,
    /// Sender of a message
    pub sender: Identity,
    /// Final recipient of a message
    pub recipient: Recipient,
    /// The timestamp at which this message was received
    /// Some raw message payload
    pub payload: Vec<u8>,
    /// Time signature information
    pub timesig: TimePair,
    /// Signature data for userspace layers
    pub sign: Vec<u8>,
}

impl Message {
    /// This function exists to make unit tests easier.  Do not use it
    /// in your application under any circumstances.  Really, please
    /// don't.  You would have to rely on the sender timestamp to be
    /// accurate, and that's a _bad_ idea!  Using this function
    /// contributes to the killing of baby seals.
    #[doc(hidden)]
    pub fn remove_recv_time(self) -> Self {
        Self {
            timesig: self.timesig.into_sending(),
            ..self
        }
    }
}

/// A wrapper around payload and signature
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct Payload {
    pub(crate) payload: Vec<u8>,
    pub(crate) timesig: TimePair,
    pub(crate) sign: Vec<u8>,
}
