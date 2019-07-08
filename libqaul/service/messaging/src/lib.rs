//! `qaul.net` messaging service
//!
//! Provides a simple interface to deliver plain-text
//! messages, with inline attachments, or optionally
//! async-delivery links (requires `filesharing` service).
//! It's basically decentralised e-mail.
//! It's basically e-mail.

use qaul::{Qaul, QaulResult, UserAuth};
use identity::Identity;
use files::File;

/// A list of file-attachments
pub type Attachments = Vec<File>;

/// A plain-text message with optional attachments
pub struct Message {
    text: String,
    attachments: Option<Attachments>,
}

/// Messaging service state
pub struct Messaging<'q> {
    async_files: bool,
    qaul: &'q Qaul,
}

impl<'q> Messaging<'q> {
    /// Initialise the messaging service
    ///
    /// In order to initialise, a valid and running
    /// `Qaul` reference needs to be provided.
    /// This is then used to register this service,
    /// but also check for the existence of a `filesharing` service.
    /// Depending on this check, the `async_files` capability
    /// can be set.
    pub fn init(qaul: &'q Qaul) -> Self {
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
    pub fn send_message(
        &self,
        user: UserAuth,
        recipients: Vec<Identity>,
        msg: Message,
    ) -> QaulResult<()> {
        unimplemented!()
    }

    /// Get all messages that were received since the last poll
    pub fn poll_messages(&self, user: UserAuth) -> QaulResult<Vec<Message>> {
        unimplemented!()
    }
}
