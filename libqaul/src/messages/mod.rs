//! Service API messaging primitives

// Public exports
pub use crate::api::messages::{Message, MessageQuery, MsgId, MsgRef, Recipient, SigTrust, ID_LEN};

mod store;
pub(crate) use self::store::{MsgState, MsgStore};
#[cfg(feature = "generate-message")]
pub(crate) mod message_generation;

use crate::error::{Error, Result};
use ratman_netmod::Recipient as RatRecipient;
use ratman::{
    Identity, Message as RatMessage, MsgId as RatMsgId, Router,
};
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

impl From<RatMsgId> for MsgId {
    fn from(id: RatMsgId) -> Self {
        Self(id.0)
    }
}

impl From<MsgId> for RatMsgId {
    fn from(id: MsgId) -> Self {
        Self(id.0)
    }
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
        let id = proto.env.id.clone().into();
        let sender = proto.env.sender;
        let signature = proto.signature;
        let payload = conjoiner::serialise(&proto.env).unwrap();

        proto
            .recipients
            .into_iter()
            .map(|recipient| RatMessage {
                id,
                sender,
                recipient,
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
    pub(crate) fn sign(env: &Envelope) -> Vec<u8> {
        let mut v = vec![1, 3, 1, 2];
        v.extend_from_slice(&env.id.0);
        v
    }

    /// Sends a `RatMessageProto`, calls a set of `send` commands
    pub(crate) async fn send(router: &Router, msg: RatMessageProto) -> Result<()> {
        let messages: Vec<RatMessage> = msg.into();
        for result in messages
            .into_iter()
            .map(|msg| { router.send(msg) }) {
                match result.await {
                    Ok(_) => (),
                    Err(e) => return Err(e)
                }
            }
            return Ok(())
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
            recipient: match recipient {
                RatRecipient::User(u) => Recipient::User(u),
                _ => unimplemented!(),
            },
            associator,
            sign: SigTrust::Unverified,
            payload,
        }
    }
}
