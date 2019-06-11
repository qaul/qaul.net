//! A small library with data types used across qaul.net
extern crate blake2;
mod message;
mod payload;

pub type HashBytes = [u8; 32];  // Because of the lack of type-level generics,
                                // array functions we need are not implemented
                                // above 32 elements.

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