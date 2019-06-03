//! A small library with data types used across qaul.net
mod message;
mod payload;

pub type HashBytes = [u8; 32];

/// A cryptographically backed ID for a user on a network
pub struct UserID {
    pub seed: Vec<u8>,
    pub inner: HashBytes,
}

impl Default for UserID {
    fn default() -> Self {
        unimplemented!()
    }
}