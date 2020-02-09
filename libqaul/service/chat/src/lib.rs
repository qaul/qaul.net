//! `qaul.net` chat service
#![allow(unused)]

// use libqaul::{
//     error::{Error, Result},
//     messages::{Message, MsgQuery, Mode, MsgId, MsgRef, SigTrust},
//     users::UserAuth,
//     Identity, Qaul,
// };

mod msg;
pub use msg::ChatMessage;

pub mod room;
use room::{Room, RoomId};

use async_std::sync::Arc;
use libqaul::{error::Result, Qaul};

const ASC_NAME: &'static str = "net.qaul.chat";

/// Messaging service state
#[derive(Clone)]
pub struct Chat {
    qaul: Arc<Qaul>,
}

impl Chat {
    /// Create a new chat service instance
    pub fn new(qaul: Arc<Qaul>) -> Result<Self> {
        qaul.services().register(ASC_NAME)?;
        Ok(Self { qaul })
    }

    /// Get the next available chat message
    pub async fn next(&self) -> ChatMessage {
        unimplemented!()
    }

    /// Send a new message
    pub async fn send(&self) -> Result<()> {
        unimplemented!()
    }
}

/// Small API wrapper for room management
pub struct Rooms<'c> {
    chat: &'c Chat
}

impl<'c> Rooms<'c> {
    /// Create a new room
    pub async fn create(&self, _room: Room) -> Result<RoomId> {
        Ok(RoomId::random())
    }

    /// Make modifications to an existing room
    pub async fn modify<F>(&self, _id: RoomId, _f: F) -> Result<()>
    where
        F: Fn(&mut Room) -> Result<()>,
    {
        unimplemented!()
    }

    /// Delete a room locally
    pub async fn delete(&self, _id: RoomId) -> Result<()> {
        unimplemented!()
    }
}

// impl Messaging {
//     /// Initialise the messaging service
//     ///
//     /// In order to initialise, a valid and running
//     /// `Qaul` reference needs to be provided.
//     /// This is then used to register this service,
//     /// but also check for the existence of a `filesharing` service.
//     /// Depending on this check, the `async_files` capability
//     /// can be set.
//     pub fn new(qaul: Arc<Qaul>) -> Self {

//         Self {
//             async_files: false,
//             qaul,
//         }
//     }

//     /// Check if the `async_files` capability is set on this service
//     pub fn async_files(&self) -> bool {
//         self.async_files
//     }

//     /// Send a plain-text message with optional arbitrary attachments
//     ///
//     /// Under the hood, this function constructs a service API
//     /// `Message`, signs it and optionally encrypts it, if it's
//     /// `recipient` isn't `Recipient::Flood`, then queues it in the
//     /// routing layer.
//     pub async fn send(
//         &self,
//         user: UserAuth,
//         mode: Mode,
//         payload: TextPayload,
//     ) -> Result<MsgId> {
//         self.qaul
//             .messages()
//             .send(user, mode, ASC_NAME, vec![], conjoiner::serialise(&payload)?)
//             .await
//     }

//     /// Non-blockingly poll for new `TextMessage`s for a session
//     pub fn poll(&self, user: UserAuth) -> Result<TextMessage> {
//         self.qaul
//             .messages()
//             .poll(user, ASC_NAME)
//             .map(|msg| TextMessage::try_from(msg))?
//     }

//     /// Query existing messages from this service with a query
//     pub fn query(&self, user: UserAuth, query: MsgQuery) -> Result<Vec<TextMessage>> {
//         self.qaul
//             .messages()
//             .query(user, ASC_NAME, query)?
//             .into_iter()
//             .map(|msg| TextMessage::try_from(msg))
//             .collect()
//     }

//     /// Setup a `TextMessage` listener for a specific user session
//     pub fn listen<F: 'static + Send + Sync>(&self, user: UserAuth, listener: F) -> Result<()>
//     where
//         F: Fn(TextMessage) -> Result<()>,
//     {
//         self.qaul.messages().listen(user, ASC_NAME, move |msg| {
//             match TextMessage::try_from(msg) {
//                 Ok(text) => listener(text),
//                 Err(e) => return Err(e),
//             }
//         })
//     }
// }
