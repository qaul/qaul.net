//! User storage

mod contacts;
mod profile;
mod store;

pub(crate) use contacts::{ContactData, ContactList, ContactStore};
pub use profile::{UserProfile, UserUpdate};
pub(crate) use store::{User, UserStore};
