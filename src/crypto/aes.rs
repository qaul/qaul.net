//! Symmetric cipher utilities
//!
//! These functions are only used to bootstrap the unlocking process
//! for the database user table.  For all other cryptographic
//! operations, see the `asym` module instead.

use crate::crypto::Encrypted;
use keybob::{Key as KeyBuilder, KeyType};
use sodiumoxide::crypto::secretbox::{gen_nonce, open, seal, Key, Nonce};

/// Create an AES symmetric key from a user password and salt
pub(crate) fn key_from_pw(pw: &str, salt: &str) -> Key {
    let kb = KeyBuilder::from_pw(KeyType::Aes128, pw, salt);
    Key::from_slice(kb.as_slice()).unwrap()
}

pub(crate) trait Crypto {
    fn encrypt(data: &[u8], key: &Key) -> Self;
    fn decrypt(&self, key: &Key) -> Option<Vec<u8>>;
}

impl Crypto for Encrypted {
    fn encrypt(data: &[u8], key: &Key) -> Self {
        let nonce = gen_nonce();
        let data = seal(data, &nonce, key);

        Self {
            nonce: nonce.0.into_iter().cloned().collect(),
            data,
        }
    }

    fn decrypt(&self, key: &Key) -> Option<Vec<u8>> {
        let Encrypted { nonce, data } = self;
        let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();
        open(data.as_slice(), &nonce, key).ok()
    }
}
