#![allow(unused)]

use crate::messages::{Message, MsgId, SigTrust};
use crate::qaul::Identity;
use rand::distributions::{Distribution, Standard};

/// A builder struct that can be used to generate any and all fields of a `Message`.
#[derive(Debug, Default, PartialEq)]
pub struct MessageGenBuilder {
    /// The message ID
    id: Option<MsgId>,
    /// The sender identity
    sender: Option<Identity>,
    /// The embedded service associator
    associator: Option<String>,
    /// A raw byte `Message` payload
    payload: Option<Vec<u8>>,
    /// Attached `Message` signature, for all fields in a message
    /// If not set, this defaults to Unverified.
    sign: Option<SigTrust>,
}

impl MessageGenBuilder {
    /// Create an empty MessageGenBuilder that will create a totally randomized message.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the ID of the resulting message.
    pub fn with_id(mut self, id: MsgId) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the sender of the resulting message.
    pub fn with_sender(mut self, sender: Identity) -> Self {
        self.sender = Some(sender);
        self
    }

    /// Set the service associator of the resulting message.
    pub fn with_associator<S: Into<String>>(mut self, associator: S) -> Self {
        self.associator = Some(associator.into());
        self
    }

    /// Set the payload of the resulting message.
    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Set the signature (`SigAuth`) of the resulting message.
    pub fn with_signature(mut self, signature: SigTrust) -> Self {
        self.sign = Some(signature);
        self
    }

    /// Create an iterator over an infinite sequence of `Message`s, with randomized
    /// or default fields for those not set on the `MessageGenBuilder`.
    pub fn messages(self) -> impl Iterator<Item = Message> {
        MessageGenerator { mgb: self }
    }

    pub(crate) fn generate_message(&self) -> Message {
        let mut rng = rand::thread_rng();
        let sender = self
            .sender
            .clone()
            .unwrap_or_else(|| Identity::truncate(&Standard.sample_iter(rng).take(16).collect()));
        let associator = self.associator.clone().unwrap_or("".into());
        let id = self.id.clone().unwrap_or_else(|| MsgId::random());
        let payload = self
            .payload
            .clone()
            .unwrap_or_else(|| Standard.sample_iter(rng).take(1024).collect());
        Message {
            id,
            associator,
            sender,
            sign: self.sign.clone().unwrap_or(SigTrust::Unverified),
            payload,
        }
    }
}

/// This structure, created by a `MessageGenBuilder`, generates an infinite stream of
/// `Message`s.
pub struct MessageGenerator {
    mgb: MessageGenBuilder,
}
impl Iterator for MessageGenerator {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        Some(self.mgb.generate_message())
    }
}

#[test]
fn iter_messages_are_different() {
    let mut iter = MessageGenBuilder::new().messages();
    assert!(iter.next() != iter.next());
}
