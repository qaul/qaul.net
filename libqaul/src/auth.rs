//! Internal user authentication modules
//!
//! Hooks up to the secret data store and validates user passphrases,
//! tokens and secrets.

/// Handles all authentication state
pub(crate) struct Authenticator {}

/// A wrapper around a salted password hash
pub(crate) struct PwHash {
    hash: Vec<u8>,
    salt: Vec<u8>,
}

impl PwHash {
    pub(crate) fn matches_with<S>(pw: S) -> bool
    where
        S: Into<String>,
    {
        false
    }

    pub(crate) fn new<S>(pw: S) -> Self
    where
        S: Into<String>,
    {
        
        
    }
}

impl Authenticator {}
