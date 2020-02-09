//! `qaul.net` messaging service
//!
//! Provides a simple interface to deliver plain-text
//! messages, with inline attachments, or optionally
//! async-delivery links (requires `filesharing` service).
//! It's basically decentralised e-mail.
//! It's basically e-mail.

use conjoiner;
use qaul::{
    error::{Error, Result},
    messages::{Message, MsgQuery, Mode, MsgId, MsgRef, SigTrust},
    users::UserAuth,
    Identity, Qaul,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, sync::Arc};

const ASC_NAME: &'static str = "qaul-messaging";

// A list of file-attachments
// pub type Attachments = Vec<File>;

/// A plain-text message with optional attachments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPayload {
    pub text: String,
}

/// An incoming text message, with optional file attachments
///
/// This abstraction is a simplification of the `libqaul` internal
/// `Message` abstraction, and is not stored internally in this
/// form. Generally, when querying from the service API, the return
/// value needs to be mapped into this type.
#[derive(Debug, Clone)]
pub struct TextMessage {
    pub id: MsgId,
    pub sender: Identity,
    pub sign: SigTrust,
    pub payload: TextPayload,
}

impl TryFrom<MsgRef> for TextMessage {
    type Error = Error;

    /// Map from a `Message::In` into a `TextMessage`
    fn try_from(msg: MsgRef) -> Result<Self> {
        let Message {
            ref id,
            ref sender,
            ref sign,
            ref payload,
            associator: _,
            tags: _,
        } = msg.as_ref();

        Ok(Self {
            id: id.clone(),
            sender: sender.clone(),
            sign: sign.clone(),
            payload: conjoiner::deserialise(&payload)?,
        })
    }
}

/// Messaging service state
#[derive(Clone)]
pub struct Messaging {
    async_files: bool,
    qaul: Arc<Qaul>,
}

impl Messaging {
    /// Initialise the messaging service
    ///
    /// In order to initialise, a valid and running
    /// `Qaul` reference needs to be provided.
    /// This is then used to register this service,
    /// but also check for the existence of a `filesharing` service.
    /// Depending on this check, the `async_files` capability
    /// can be set.
    pub fn new(qaul: Arc<Qaul>) -> Self {
        qaul.services().register(ASC_NAME).unwrap();

        Self {
            async_files: false,
            qaul,
        }
    }

    /// Check if the `async_files` capability is set on this service
    pub fn async_files(&self) -> bool {
        self.async_files
    }

    /// Send a plain-text message with optional arbitrary attachments
    ///
    /// Under the hood, this function constructs a service API
    /// `Message`, signs it and optionally encrypts it, if it's
    /// `recipient` isn't `Recipient::Flood`, then queues it in the
    /// routing layer.
    pub async fn send(
        &self,
        user: UserAuth,
        mode: Mode,
        payload: TextPayload,
    ) -> Result<MsgId> {
        self.qaul
            .messages()
            .send(user, mode, ASC_NAME, vec![], conjoiner::serialise(&payload)?)
            .await
    }

    /// Non-blockingly poll for new `TextMessage`s for a session
    pub fn poll(&self, user: UserAuth) -> Result<TextMessage> {
        self.qaul
            .messages()
            .poll(user, ASC_NAME)
            .map(|msg| TextMessage::try_from(msg))?
    }

    /// Query existing messages from this service with a query
    pub fn query(&self, user: UserAuth, query: MsgQuery) -> Result<Vec<TextMessage>> {
        self.qaul
            .messages()
            .query(user, ASC_NAME, query)?
            .into_iter()
            .map(|msg| TextMessage::try_from(msg))
            .collect()
    }

    /// Setup a `TextMessage` listener for a specific user session
    pub fn listen<F: 'static + Send + Sync>(&self, user: UserAuth, listener: F) -> Result<()>
    where
        F: Fn(TextMessage) -> Result<()>,
    {
        self.qaul.messages().listen(user, ASC_NAME, move |msg| {
            match TextMessage::try_from(msg) {
                Ok(text) => listener(text),
                Err(e) => return Err(e),
            }
        })
    }
}
