//! Service API: peer-to-peer messages

use super::models::{Message, QaulResult, Recipient, UserAuth};
use crate::Qaul;
use identity::Identity;

impl Qaul {
    /// Send a message into the network
    ///
    /// Because the term `Message` is overloaded slightly in
    /// `libqaul`, here is a small breakdown of what a message means
    /// in this context.
    ///
    /// The Service API provides an interface to communicate with
    /// other users on a qaul network. These messages are relatively
    /// low-level, meaning that their payload (for example) is simply
    /// a `Vec`, and it's left to a service to do anything meaningful
    /// with it.
    ///
    /// However when users write text-messages to each other in
    /// qaul.net, these are being sent via the `messaging` service,
    /// which implements it's own `Message`, on top of `libqaul`. In
    /// that case a message is plain text and can have binary
    /// attachments.
    ///
    /// Underlying `libqaul`, the routing layer (`RATMAN`) uses the
    /// term Message to refer to the same concept as a Service API
    /// message, with some more raw data inlined, such as signatures
    /// and checksums. Fundamentally they share the same idea of what
    /// a payload or recipient is however, and payloads that are
    /// unsecured in a Service API message will have been encrypted by
    /// the time that `RATMAN` handles them.
    pub fn message_send(
        &self,
        user: UserAuth,
        recipient: Recipient,
        associator: String,
        payload: Vec<u8>,
    ) -> QaulResult<()> {
        let (ref my_id, _) = user.trusted()?;
        self.router.send(ratman::Message::build_signed(
            my_id.clone(),
            match recipient {
                Recipient::User(u) => ratman::netmod::Recipient::User(u),
                _ => unimplemented!(),
            },
            associator,
            payload,
        ));

        Ok(())
    }

    pub fn message_poll(&self, user: UserAuth) -> QaulResult<Vec<Message>> {
        unimplemented!()
    }

    pub fn message_listen<S, F>(&self, user: UserAuth, associator: S, listener: F) -> QaulResult<()>
    where
        S: Into<String>,
        F: Fn(Message) -> QaulResult<()>,
    {
        unimplemented!()
    }
}
