//! Service API messaging primitives

// Public exports
pub use crate::api::messages::{Message, Mode, MsgId, MsgQuery, MsgRef, SigTrust, ID_LEN};

mod store;
pub(crate) use self::store::{MsgState, MsgStore};

#[cfg(feature = "generate-message")]
pub(crate) mod message_generation;

use crate::{api::helpers::Tag, error::Result, security::Sec, users::UserStore};
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
    pub(crate) tags: Vec<Tag>,
}

/// A `ratman::Message` set prototype structure
pub(crate) struct RatMessageProto {
    /// The high level `Message` to send and validate for
    pub(crate) env: Envelope,
    /// Readdressed `Recipient` information
    pub(crate) recipient: RatRecipient,
}

impl RatMessageProto {
    pub(crate) fn build(&self, store: &UserStore) -> RatMessage {
        let id = self.env.sender.clone().into();
        let keypair = store.get_key(id).unwrap();

        let payload = conjoiner::serialise(&self.env).unwrap();
        let sign: Vec<_> = keypair
            .sign(payload.as_slice())
            .to_bytes()
            .into_iter()
            .cloned()
            .collect();

        let sender = self.env.sender;
        let recipient = self.recipient;

        RatMessage {
            id,
            sender,
            recipient,
            payload,
            sign,
        }
    }
}

pub(crate) struct MsgUtils;

impl MsgUtils {
    /// Sends a `RatMessageProto`, calls a set of `send` commands
    pub(crate) async fn send(
        store: &UserStore,
        router: &Router,
        msg: RatMessageProto,
    ) -> Result<()> {
        Ok(router.send(msg.build(store)).await?)
    }

    /// Process incoming RATMAN message, verifying it's signature and payload
    pub(crate) fn process(id: Identity, msg: RatMessage) -> Message {
        let RatMessage {
            id,
            sender,
            recipient: _,
            ref payload,
            ref sign,
        } = msg;

        let Envelope {
            id: _,
            sender: _,
            associator,
            payload,
            tags,
        } = conjoiner::deserialise(&payload).unwrap();

        // Verify signature
        let sign = Sec::verify(id, payload.as_slice(), sign.as_slice());

        Message {
            id: id.into(),
            sender,
            associator,
            tags: Default::default(),
            sign,
            payload,
        }
    }
}
