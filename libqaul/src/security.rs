//! An internal security subsystem
//!
//! This code is responsible for creating identities (keys, and
//! `Identity`), verifying signatures, and encrypting messages

use crate::{
    error::{Error, Result},
    Identity,
};
use bincode;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{self, Nonce, PublicKey, SecretKey};

/// A near-stateless security handler
pub(crate) struct Sec {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Keypair {
    secret: SecretKey,
    public: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct CryptoText {
    nonce: Vec<u8>,
    data: Vec<u8>,
}

impl Keypair {
    fn swap_pub(&mut self, id: Identity) {
        self.public = PublicKey::from_slice(id.as_ref()).unwrap();
    }

    fn seal(&self, data: &Vec<u8>) -> Vec<u8> {
        let nonce = box_::gen_nonce();

        let data = box_::seal(data, &nonce, &self.public, &self.secret);
        let nonce = nonce.0.iter().cloned().collect();
        bincode::serialize(&CryptoText { nonce, data }).unwrap()
    }

    fn open(&self, data: &Vec<u8>) -> Result<Vec<u8>> {
        let CryptoText { nonce, data } = bincode::deserialize(data).unwrap();
        let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();
        box_::open(data.as_ref(), &nonce, &self.public, &self.secret)
            .map_err(|_| Error::InvalidPayload)
    }
}

/// A keypair, and Identity
pub(crate) struct KeyId {
    pub keypair: Keypair,
    pub id: Identity,
}

/// An encrypted piece of data
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct CipherText {
    /// Number only used once
    nonce: Vec<u8>,
    /// Data buffer
    data: Vec<u8>,
}

impl Sec {
    /// Create a new security handler
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Generate an Id and keypair for a new user
    pub(crate) async fn generate(&self) -> KeyId {
        let (public, secret) = box_::gen_keypair();
        let id = Identity::from_bytes(public.as_ref());
        KeyId {
            keypair: Keypair { public, secret },
            id,
        }
    }

    /// Decrypt a payload from a friend
    pub(crate) fn decrypt(mut pair: Keypair, friend: Identity, enc: &Vec<u8>) -> Result<Vec<u8>> {
        pair.swap_pub(friend);
        pair.open(enc)
    }

    /// Encrypt a payload to a friend
    pub(crate) fn encrypt(mut pair: Keypair, friend: Identity, data: &Vec<u8>) -> Vec<u8> {
        pair.swap_pub(friend);
        pair.seal(data)
    }
}

#[async_std::test]
async fn single_ratchet() {
    let sec = Sec::new();
    let a = sec.generate().await;
    let b = sec.generate().await;

    let plaintext = b"ACAB";
    let encrypted = Sec::encrypt(a.keypair, b.id, &plaintext.to_vec());
    let acab = Sec::decrypt(b.keypair, a.id, &encrypted).unwrap();
    assert!(acab == plaintext);
}
