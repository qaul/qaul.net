//! Service user managment

use super::error::{Error, QResult};

// FIXME: Replace with HashID or whatever?
pub type UserID = String;

// FIXME: Replace with actual auth token
pub type AuthID = String;

/// A sevice user's external/public representation
pub struct User {
    /// An easy to read user handle
    pub name: Option<String>,
    /// An even more human readable name
    pub clear_name: String,
    /// Some generic user metadata
    pub bio: String, // Should this be a Vec<(Key, Val)> kinda thing?
    /// An avatar or profile picture
    pub avatar: Vec<u8>, // Find better abstraction
}

/// Create a new user on this machine
///
/// Fundamentally, a user doesn't require any metadata.
/// If a user has optionally decided to provide data
/// about themselves, use `update` to add it.
pub fn create() -> QResult<UserID> {
    unimplemented!()
}

/// Provides an interface to update a user in-place
pub fn update<F>(id: UserID, cb: F) -> QResult<()>
where
    F: Fn(&mut User),
{
    unimplemented!()
}

/// Get a read-only reference to user data
pub fn get(id: UserID) -> QResult<&'static User> {
    unimplemented!()
}

/// Delete an authenticated user and all their data
///
/// After passing authentication it is possible
/// for a user to delete themselves from a node,
/// including all of their stored data.
pub fn delete(id: UserID) -> QResult<()> {
    unimplemented!()
}

/// Authenticate as a user, returning access token
pub fn authenticate(id: UserID, pw: String) -> QResult<AuthID> {
    unimplemented!()
}

/// Hand in an access token, ending a current session
pub fn unauth(id: UserID, auth: AuthID) -> QResult<()> {
    unimplemented!()
}

