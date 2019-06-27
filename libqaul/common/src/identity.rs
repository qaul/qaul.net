//! Shared identity module for `libqaul`


use generic_array::typenum::U64;
use generic_array::GenericArray;

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
