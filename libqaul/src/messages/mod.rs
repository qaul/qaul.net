//! Internal `Message` handling module

// Public exports
pub use crate::api::messages::{Message, MessageQuery, MsgId, Recipient, SigTrust};

use crate::error::{Error, Result};

use ratman::{netmod::Recipient as RatRecipient, Identity, Message as RatMessage, Router};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// A searchable index of messages encountered by this system
#[derive(Clone)]
pub(crate) struct MsgStore {
    inner: Arc<Mutex<BTreeMap<Identity, Vec<Message>>>>,
}

/// An internal wrapper around an unsent Message
pub(crate) struct Envelope {
    pub(crate) id: MsgId,
    pub(crate) sender: Identity,
    pub(crate) recipient: Recipient,
    pub(crate) associator: String,
    pub(crate) payload: Vec<u8>,
}

/// A `ratman::Message` set prototype structure
pub(crate) struct RatMessageProto {
    /// The high level `Message` to send and validate for
    pub(crate) env: Envelope,
    /// Readdressed `Recipient` information
    pub(crate) recipients: Vec<RatRecipient>,
    /// Signature for the inlined `Message` data
    pub(crate) signature: Vec<u8>,
}

impl From<RatMessageProto> for Vec<RatMessage> {
    fn from(proto: RatMessageProto) -> Self {
        let sender = proto.env.sender;
        let associator = proto.env.associator.clone();
        let payload = proto.env.payload;
        let signature = proto.signature;
        proto
            .recipients
            .into_iter()
            .map(|recipient| RatMessage {
                sender: sender.clone(),
                recipient,
                associator: associator.clone(),
                payload: payload.clone(),
                signature: signature.clone(),
            })
            .collect()
    }
}

pub(crate) struct MsgUtils;

impl MsgUtils {
    /// Readdress the `libqaul` `Recipient` to a routing `Recipient`
    pub(crate) fn readdress(recp: &Recipient) -> Vec<RatRecipient> {
        match recp {
            Recipient::Group(ref group) => group
                .into_iter()
                .cloned()
                .map(|id| RatRecipient::User(id))
                .collect(),
            Recipient::User(ref id) => vec![RatRecipient::User(*id)],
            Recipient::Flood => vec![RatRecipient::Flood],
        }
    }

    /// Construct a cryptographic signature for an inner `Message`
    pub(crate) fn sign(_env: &Envelope) -> Vec<u8> {
        vec![1, 3, 1, 2]
    }

    /// Sends a `RatMessageProto`, calls a set of `send` commands
    pub(crate) fn send(router: &Router, msg: RatMessageProto) -> Result<()> {
        let messages: Vec<RatMessage> = msg.into();
        messages
            .into_iter()
            .map(|msg| router.send(msg))
            .fold(Ok(()), |acc, res| match (acc, res) {
                (_, Err(_)) => Err(Error::NetworkFault),
                (Err(e), _) => Err(e),
                (_, _) => Ok(()),
            })
    }
}
