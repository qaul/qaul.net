//! Internal user authentication modules
//!
//! Hooks up to the secret data store and validates user passphrases,
//! tokens and secrets.

use blake2::{Blake2b, Digest as _};
use rand::prelude::*;
use std::fmt::{self, Debug, Formatter};

/// A wrapper around a salted password hash
pub(crate) struct PwHash {
    hash: Vec<u8>,
    salt: Vec<u8>,
}

impl Debug for PwHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<HASH><SALT>")
    }
}

impl PwHash {
    /// Match a user password with a pre-existing salt
    pub(crate) fn matches_with<S>(&self, pw: S) -> bool
    where
        S: Into<String>,
    {
        let new = Blake2b::new()
            .chain(pw.into())
            .chain(&self.salt)
            .result()
            .to_vec();

        self.hash == new
    }

    /// Create a new password hash for a user
    ///
    /// This function will generate a salt, which must be used in all
    /// future compare operations. For this, please use
    /// `matches_with`.
    pub(crate) fn new<S>(pw: S) -> Self
    where
        S: Into<String>,
    {
        let salt: Vec<u8> = (0..)
            .map(|_| rand::thread_rng().next_u64())
            .take(8)
            .map(|x| x.to_be_bytes())
            .fold(Vec::new(), |mut acc, arr| {
                acc.extend(arr.iter().cloned());
                acc
            });
        let hash = Blake2b::new()
            .chain(pw.into())
            .chain(&salt)
            .result()
            .to_vec();

        Self { hash, salt }
    }
}

#[cfg(test)]
mod test {
    use super::PwHash;

    #[test]
    fn basic() {
        let pwhash = PwHash::new("horsecarbatterystaple");
        assert!(pwhash.matches_with("horsecarbatterystaple"));
    }
}
