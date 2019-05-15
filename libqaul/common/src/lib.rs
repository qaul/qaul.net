//! A small library with data types used across qaul.net

/// A cryptographically backed ID for a user on a network
pub struct UserID {
    pub seed: Vec<u8>,
    pub inner: [u8; 32],
}

impl Default for UserID {
    fn default() -> Self {
        unimplemented!()
    }
}