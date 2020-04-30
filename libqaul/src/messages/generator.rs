#![allow(unused)]

use crate::{
    helpers::TagSet,
    messages::{Message, MsgId, SigTrust},
    qaul::Identity,
};
use rand::distributions::{Distribution, Standard};
use ratman::ID_LEN;

/// A builder that can generate random messages with constraints
///
/// In some testing situations most fields of a message can be random,
/// while others should be a discrete value that can be asserted on.
/// This message builder generates random messages with given
/// constraints.
#[derive(Debug, Default, PartialEq)]
pub struct MsgBuilder {
    /// The message ID
    id: Option<MsgId>,
    /// The sender identity
    sender: Option<Identity>,
    /// The embedded service associator
    associator: Option<String>,
    /// The associated search tags
    tags: Option<TagSet>,
    /// A raw byte `Message` payload
    payload: Option<Vec<u8>>,
}

impl MsgBuilder {
    /// Create an empty MsgBuilder
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

    pub fn with_tags(mut self, tags: impl Into<TagSet>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// Create an iterator over an infinite sequence of `Message`s, with randomized
    /// or default fields for those not set on the `MessageGenBuilder`.
    pub fn messages(self) -> impl Iterator<Item = Message> {
        MsgGenerator { mgb: self }
    }

    pub(crate) fn generate(&self) -> Message {
        let mut rng = rand::thread_rng();
        let sender = self.sender.clone().unwrap_or_else(|| {
            Identity::truncate(&Standard.sample_iter(rng).take(ID_LEN).collect())
        });
        let associator = self.associator.clone().unwrap_or("".into());
        let id = self.id.clone().unwrap_or_else(|| MsgId::random());
        let payload = self
            .payload
            .clone()
            .unwrap_or_else(|| Standard.sample_iter(rng).take(1024).collect());
        let tags = self.tags.clone().unwrap_or_default();
        Message {
            id,
            associator,
            sender,
            tags,
            payload,
        }
    }
}

/// This structure, created by a `MessageGenBuilder`, generates an infinite stream of
/// `Message`s.
pub struct MsgGenerator {
    mgb: MsgBuilder,
}
impl Iterator for MsgGenerator {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        Some(self.mgb.generate())
    }
}

#[test]
fn iter_messages_are_different() {
    let mut iter = MsgBuilder::new().messages();
    assert!(iter.next() != iter.next());
}
