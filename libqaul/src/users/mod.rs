//! User storage

mod contacts;
mod profile;
mod store;

pub(crate) use {
    contacts::{ContactList, ContactStore},
    store::{User, UserStore},
};
pub use {
    contacts::{ContactData, ContactQuery},
    profile::{UserProfile, UserUpdate},
};
