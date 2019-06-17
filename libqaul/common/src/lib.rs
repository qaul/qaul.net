//! A small library with data types used across qaul.net
extern crate blake2;
extern crate generic_array;
use generic_array::typenum::U64;
use generic_array::GenericArray;
mod message;
mod payload;

pub type HashBytes = GenericArray<u8, U64>;

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
