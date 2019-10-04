//! Service API: contact book endpoints

use super::models::{QaulResult, UserAuth};
use crate::{Qaul, User, ContactBook, LocalContactData};

impl Qaul {
    /// Add a new contact to a user's known contacts
    pub fn contacts_add(&self, user: UserAuth, contact: User) -> QaulResult<()> {
        let (my_id, _) = user.trusted()?;
        let contact_id = contact.id.clone();

        // If the user isn't know to this libqaul instance, add it there first
        {
            let mut users = self.users.lock().expect("Users lock poisioned. Error");
            users.insert(contact_id.clone(), contact);
        }

        // Now, add it to the user's contact book.
        {
            let my_id = my_id.clone();
            let mut contacts = self.contacts.lock().expect("Contacts lock poisoned. Error");
            let mut contacts_book = contacts.entry(my_id).or_insert(ContactBook::new());
            contacts_book.insert(contact_id, LocalContactData::default());
        }

        Ok(())
    }

    /// Find a subset of contacts with some query
    pub fn contacts_find<S: Into<String>>(
        &self,
        user: UserAuth,
        query: S,
    ) -> QaulResult<Vec<User>> {
        let query = query.into();
        let (my_id, _) = user.trusted()?;

        let mut results = Vec::new();
        let users = self.users.lock().expect("Users lock poisoned. Error");
        let mut contacts = self
            .contacts
            .lock()
            .expect("Contacts lock poisioned. Error");
        let contacts_book = contacts.entry(my_id).or_insert(ContactBook::new());
        for (contact_id, _) in contacts_book {
            let contact_info = users
                .get(&contact_id)
                .expect("User in contact book not present in users map.");
            if contact_info.data.like_query(&query) {
                results.push(contact_info.clone());
            }
        }
        Ok(results)
    }

    /// Enumerate all contacts known by a user
    pub fn contacts_get_all(&self, user: UserAuth) -> QaulResult<Vec<User>> {
        let (my_id, _) = user.trusted()?;

        let users = self.users.lock().expect("Users lock poisoned. Error");
        let mut contacts = self
            .contacts
            .lock()
            .expect("Contacts lock poisioned. Error");
        let contacts_book = contacts.entry(my_id).or_insert(ContactBook::new());

        Ok(contacts_book
            .iter()
            .map(|(id, _)| {
                users
                    .get(&id)
                    .expect("User in contact book not present in users map.")
            })
            .cloned()
            .collect())
    }
}
