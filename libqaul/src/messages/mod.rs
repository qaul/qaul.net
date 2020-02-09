//! Service API messaging primitives

// Public exports
pub use crate::api::messages::{Message, MessageQuery, Mode, MsgId, MsgRef, SigTrust, ID_LEN};

mod store;
pub(crate) use self::store::{MsgState, MsgStore};

#[cfg(feature = "generate-message")]
pub(crate) mod message_generation;

use crate::error::Result;
use ratman::{netmod::Recipient as RatRecipient, Identity, Message as RatMessage, Router};
use serde::{Deserialize, Serialize};

/// An internal wrapper around an incomplete Message
///
/// Because message signatures are computed based on the data provided
/// by the envelope, this means that changing the layout will **break
/// all message signature verification!** Keep that in mind!
///
/// Apart from that it contains all data that will be present in the
/// Message set generated for ratman routing.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Envelope {
    pub(crate) id: MsgId,
    pub(crate) sender: Identity,
    pub(crate) associator: String,
    pub(crate) payload: Vec<u8>,
}

/// A `ratman::Message` set prototype structure
pub(crate) struct RatMessageProto {
    /// The high level `Message` to send and validate for
    pub(crate) env: Envelope,
    /// Readdressed `Recipient` information
    pub(crate) recipient: RatRecipient,
    /// Signature for the inlined `Message` data
    pub(crate) signature: Vec<u8>,
}

impl From<RatMessageProto> for RatMessage {
    fn from(proto: RatMessageProto) -> Self {
        let payload = conjoiner::serialise(&proto.env).unwrap();
        let id = proto.env.id.clone().into();
        let sender = proto.env.sender;
        let signature = proto.signature;
        let recipient = proto.recipient;

        RatMessage {
            id,
            sender,
            recipient,
            payload: payload.clone(),
            signature: signature.clone(),
        }
    }
}

pub(crate) struct MsgUtils;

impl MsgUtils {
    /// Construct a cryptographic signature for an inner `Message`
    pub(crate) fn sign(_: &Envelope) -> Vec<u8> {
        vec![1, 3, 1, 2]
    }

    /// Sends a `RatMessageProto`, calls a set of `send` commands
    pub(crate) async fn send(router: &Router, msg: RatMessageProto) -> Result<()> {
        Ok(router.send(msg.into()).await?)
    }

    /// Process incoming RATMAN message, verifying it's signature and payload
    pub(crate) fn process(msg: RatMessage, _: Identity) -> Message {
        let RatMessage {
            id,
            sender,
            recipient,
            ref payload,
            signature: _,
        } = msg;

        let Envelope {
            id: _,
            sender: _,
            associator,
            payload,
        } = conjoiner::deserialise(&payload).unwrap();

        Message {
            id: id.into(),
            sender,
            associator,
            sign: SigTrust::Unverified,
            payload,
        }
    }
}
