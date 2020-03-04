//! Asymmetric cryto utilities

use crate::{crypto::Encrypted, Id};
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{self, Nonce, PublicKey, SecretKey};

/// Both public and private keys for a user
#[derive(Serialize, Deserialize)]
pub(crate) struct KeyPair {
    pub_: PublicKey,
    sec: SecretKey,
}

impl KeyPair {
    /// Create a new tree of keys
    pub(crate) fn new() -> Self {
        let (pub_, sec) = box_::gen_keypair();
        Self { pub_, sec }
    }
}

// impl Crypto for Encrypted {
//     fn encrypt(data: &[u8], keypair: &KeyPair) -> Self {
//         let nonce = box_::gen_nonce();
//         let data = box_::seal(data, &nonce, &keypair.pub_, &keypair.sec);
//         Self {
//             nonce: nonce.0.into_iter().cloned().collect(),
//             data,
//         }
//     }

//     fn decrypt(&self, keypair: &KeyPair) -> Option<Vec<u8>> {
//         let Encrypted { nonce, data } = self;
//         let nonce = Nonce::from_slice(nonce.as_slice()).unwrap();
//         box_::open(data.as_slice(), &nonce, &keypair.pub_, &keypair.sec).ok()
//     }
// }
