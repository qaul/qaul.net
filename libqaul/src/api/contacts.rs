//! Service API contact book
//!
//! A user can store information about other users and query this
//! information later. The contact book is zoned per-user. Ideally it
//! would be encrypted (not for now).

use super::models::{QaulResult, UserAuth};
use crate::{ContactData, ContactList, ContactQuery, ContactStore, Qaul, UserProfile};

impl Qaul {
    /// Add a new contact to a user's known contacts
    pub fn contacts_add(&self, user: UserAuth, contact: UserProfile) -> QaulResult<()> {
        let (ref my_id, ref token) = user.trusted()?;
        self.auth.verify_token(my_id, token)?;

        self.contacts.modify(&my_id, &contact.id, |mut c| {});
        Ok(())
    }

    /// Find a subset of contacts with some query
    pub fn contacts_query(
        &self,
        user: UserAuth,
        query: ContactQuery,
    ) -> QaulResult<Vec<UserProfile>> {
        let (ref my_id, ref token) = user.trusted()?;
        self.auth.verify_token(my_id, token)?;
        self.contacts
            .query(my_id, query)?
            .into_iter()
            .map(|ref id| self.users.get(id))
            .collect()
    }

    /// Enumerate all contacts known by a user
    pub fn contacts_get_all(&self, user: UserAuth) -> QaulResult<Vec<UserProfile>> {
        let (ref my_id, ref token) = user.trusted()?;
        self.auth.verify_token(my_id, token)?;

        self.contacts
            .get_all(my_id)?
            .into_iter()
            .map(|ref id| self.users.get(id))
            .collect()
    }
}
