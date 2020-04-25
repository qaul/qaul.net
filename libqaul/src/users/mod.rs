//! User storage

mod profile;
mod store;

pub(crate) use store::{UserStore, TAG_LOCAL, TAG_PROFILE};
pub use {
    crate::api::users::{Token, UserAuth},
    profile::{UserProfile, UserUpdate},
};
