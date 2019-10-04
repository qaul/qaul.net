//! Service API: peer-to-peer messages

use super::models::{Message, QaulResult, UserAuth};
use crate::Qaul;
use identity::Identity;

impl Qaul {
    /// Send a message to another user
    pub fn message_send(
        &self,
        user: UserAuth,
        recipient: Identity,
        payload: Vec<u8>,
    ) -> QaulResult<()> {
        unimplemented!()
    }

    pub fn message_poll(&self, user: UserAuth) -> QaulResult<Vec<Message>> {
        unimplemented!()
    }

    /// Register a new service with this `qaul` instance
    ///
    /// Internally this function dispatches a query to a UI service
    /// (marked "primary" to allow the user to either verify or
    /// deny the registration request).
    pub fn service_register(&self, user: UserAuth, service_id: String) -> QaulResult<()> {
        Ok(())
    }
}
