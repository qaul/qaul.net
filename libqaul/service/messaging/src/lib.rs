//! `qaul.net` messaging service
//!
//! Provides a simple interface to deliver plain-text
//! messages, with inline attachments, or optionally
//! async-delivery links (requires `filesharing` service).
//! It's basically decentralised e-mail.
//! It's basically e-mail.

use files::File;
use identity::Identity;
use qaul::{Qaul, QaulResult, Recipient, UserAuth};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

const ASC_NAME: &'static str = "qaul-messaging";

/// A list of file-attachments
pub type Attachments = Vec<File>;

/// A plain-text message with optional attachments
#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    text: String,
    // attachments: Option<Attachments>,
}

/// Messaging service state
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
    pub fn init(qaul: Arc<Qaul>) -> Self {
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
    pub fn send(&self, user: UserAuth, recipient: Recipient, msg: TextMessage) -> QaulResult<()> {
        // self.qaul.message_send(user, recipient, ASC_NAME,
        unimplemented!()
    }

    /// Non-blockingly poll for new `TextMessage`s for a session
    pub fn poll(&self, user: UserAuth) -> QaulResult<Vec<TextMessage>> {
        unimplemented!()
    }

    /// Setup a `TextMessage` listener for a specific user session
    pub fn listen<F>(&self, user: UserAuth, listener: F) -> QaulResult<()>
    where
        F: Fn(TextMessage) -> QaulResult<()>,
    {
        unimplemented!()
    }
}
