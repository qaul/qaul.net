use crate::{error::Result, users::UserAuth, Identity, Qaul};
use serde::{Serialize, Deserialize};

/// A user-local set of contact metadata in their contact book
///
/// All fields in this structure are entirely optional and can not be
/// relied on. They are additional points of data, that a user can
/// specify about another user, that are not available or shared with
/// the network. This is meant to allow users to curate a list of
/// trusted contacts, or build friend circles.
#[derive(Default, Debug, Clone)]
pub struct ContactEntry {
    /// The name by which the associated contact is known by the owning user.
    pub nick: Option<String>,
    /// Set a user trust level
    pub trust: i8,
    /// The user has met this person
    pub met: bool,
    /// A free text location
    pub location: Option<String>,
    /// A general plain text notes section
    pub notes: Option<String>,
}

/// Query structure to find contacts by
///
/// A query is always applied to a field that is present in
/// `ContactEntry`, and will filter contacts by what set of
/// prerequisites they fulfill.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContactQuery {
    /// A fuzzy nickname search
    Nick(String),
    /// A fuzzy trust level search
    Trust { val: i8, fuz: i8 },
    /// Filter by physical meeting
    Met(bool),
    /// A fuzzy location string search
    Location(String),
    /// A fuzzy notes string search
    Notes(String),
}

/// API scope type to access contact book functions
///
/// The contact book is a user local store of metadata, that can be
/// assigned for each `Identity`, that a user is aware of on the
/// network. A contact entry is backed by an entry in the user store,
/// available via the `users()` endpoint scope.
///
/// A `ContactEntry` should be considered additional information a
/// user can keep on someone they interact with on the network,
/// independent of the `UserProfile`, which is fetched from the remote
/// user themselves.
///
/// When assembling a complete view of a user, it's important to
/// consider both their primary profile, as well as the contact
/// metadata stored via this API. Because of this, queries only return
/// the `ContactEntry` structures, not the profile itself.
///
/// Furthermore, it is possible to query users via metadata set in a
/// user's local contact book, such as their nick, trust, location and
/// more.
pub struct Contacts<'chain> {
    pub(crate) q: &'chain Qaul,
}

impl<'qaul> Contacts<'qaul> {
    /// Drop this scope and return back to global `Qaul` scope
    pub fn drop(&'qaul self) -> &'qaul Qaul {
        self.q
    }

    /// Modify a user's contact entry in a user-local contact book
    ///
    /// The `modify` lambda allows a user to add personal metadata for
    /// a contact, such as a nickname, or trust levels. Each contact
    /// list is user local and it's not possible to access other
    /// user's contact metadata.
    ///
    /// If no contact entry existed before, a fresh one will be
    /// created before calling the passed-in lambda.
    pub fn modify<F>(&self, user: UserAuth, contact: &Identity, modify: F) -> Result<()>
    where
        F: FnOnce(&mut ContactEntry),
    {
        let (ref id, _) = self.q.auth.trusted(user)?;
        self.q.contacts.modify(id, contact, modify);
        Ok(())
    }

    /// Get a single `ContactEntry` from a user's contact book
    ///
    /// Considering that a `ContactEntry` might contain large amounts
    /// of data, this is the only way to return a reference to the
    /// full object. When trying to query all data from all contact
    /// entries, it's advised to get a list of Identities via
    /// `Contacts::get_all` first, and then map this collection over
    /// `Contacts::get` afterwards.
    pub fn get(&self, user: UserAuth, contact: &Identity) -> Result<ContactEntry> {
        let (ref id, _) = self.q.auth.trusted(user)?;
        self.q.contacts.get(id, contact)
    }

    /// Query for a subset of users that have a `ContactEntry`
    ///
    /// To get a list of all `ContactEntry` objects, map the result of
    /// this function over `Contacts::get`.
    ///
    /// ```rust
    /// # use libqaul::{Qaul, error::Result, contacts::ContactQuery};
    /// # let qaul = Qaul::dummy();
    /// # let user = qaul.users().create("abc").unwrap();
    /// let contacts = qaul.contacts();
    /// # (|| -> Result<()> {
    /// contacts
    ///     .query(user.clone(), ContactQuery::Nick("buddy".to_string()))?
    ///     .into_iter()
    ///     .map(|i| contacts.get(user.clone(), &i));
    /// # Ok(())
    /// # })().unwrap();
    /// ````
    pub fn query(&self, user: UserAuth, query: ContactQuery) -> Result<Vec<Identity>> {
        let (ref id, _) = self.q.auth.trusted(user)?;
        self.q.contacts.query(id, query)
    }

    /// Get all users that have a `ContactEntry` for this user
    pub fn all(&self, user: UserAuth) -> Result<Vec<Identity>> {
        let (ref _id, _) = self.q.auth.trusted(user)?;
        Ok(vec![])
    }
}

// impl Qaul {

//     /// Find a subset of contacts with some query
//     pub fn contacts_query(
//         &self,
//         user: UserAuth,
//         query: ContactQuery,
//     ) -> QaulResult<Vec<UserProfile>> {
//         let (ref my_id, ref token) = user.trusted()?;
//         self.auth.verify_token(my_id, token)?;
//         self.contacts
//             .query(my_id, query)?
//             .into_iter()
//             .map(|ref id| self.users.get(id))
//             .collect()
//     }

//     /// Enumerate all contacts known by a user
//     pub fn contacts_get_all(&self, user: UserAuth) -> QaulResult<Vec<UserProfile>> {
//         let (ref my_id, ref token) = user.trusted()?;
//         self.auth.verify_token(my_id, token)?;

//         self.contacts
//             .get_all(my_id)?
//             .into_iter()
//             .map(|ref id| self.users.get(id))
//             .collect()
//     }
// }
