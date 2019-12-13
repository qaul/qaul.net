use crate::error::{Error, Result};
use crate::messages::{SigTrust, Recipient, Message, MsgUtils, RatMessageProto};
use crate::qaul::{Identity, Qaul};
use crate::users::UserAuth;
use crate::utils::VecUtils;

/// A builder struct that can be used to generate any and all fields of a `Message` for
/// either in or out functionality.
#[derive(Debug, Default)]
pub struct MessageGenBuilder {
    /// The sender identity
    sender: Option<Identity>,
    /// Recipient information
    recipient: Option<Recipient>,
    /// The embedded service associator
    associator: Option<String>,
    /// A raw byte `Message` payload
    payload: Option<Vec<u8>>,
    /// Attached `Message` signature, for all fields in an `In` message
    sign: Option<SigTrust>,
}

impl MessageGenBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_sender(mut self, sender: Identity) -> Self {
    }

    pub fn with_recipient(mut self, recipient: Recipient) -> Self {
    }

    pub fn with_associator<S: Into<String>>(mut self, associator: S) -> Self {
    }

    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
    }

    pub fn with_signature(mut self, signature: SigTrust) -> Self {
    }

    pub fn make_receivable(mut self) -> Message {
    }

    pub fn make_sendable(mut self) -> Message {
    }
}

