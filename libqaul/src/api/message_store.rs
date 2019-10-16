//! Service API: message store

use super::UserAuth;
use crate::{api::Message, Qaul, QaulResult};

impl Qaul {
    /// A temporary endpoint designed to get all messages from a user
    ///
    /// After passing authentication, it always returns the same list
    /// of messages
    #[deprecated]
    pub fn store_all(&self, user: UserAuth) -> QaulResult<Vec<Message>> {
        let (ref id, ref token) = user.trusted()?;
        self.auth.verify_token(id, token)?;
        
        Ok(vec![])
    }
}
