use crate::{data::Encrypted, Id};
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{self, PublicKey, SecretKey};
use std::collections::BTreeMap;

/// Both public and private parts of the key tree
#[derive(Serialize, Deserialize)]
pub(crate) struct KeyTreePair {
    pub_: PubKeyTree,
    sec: SecKeyTree,
}

/// A tree of public keys
#[derive(Serialize, Deserialize)]
pub(crate) struct PubKeyTree {
    root: PublicKey,
    subs: BTreeMap<Id, PublicKey>,
}

impl PubKeyTree {
    /// Seal some data for the root key
    pub(crate) fn seal(&self, data: &[u8], sk: &SecretKey) -> Encrypted {
        let nonce = box_::gen_nonce();
        let data = box_::seal(data, &nonce, &self.root, sk);
        Encrypted { nonce, data }
    }

    /// Seal some data for a specific zone
    pub(crate) fn seal_zone(&self, zone: Id, data: &[u8], sk: &SecretKey) -> Encrypted {
        let pub_ = &self.subs.get(&zone).unwrap();
        let nonce = box_::gen_nonce();
        let data = box_::seal(data, &nonce, &pub_, sk);
        Encrypted { nonce, data }
    }
}

/// A tree of secret keys
#[derive(Serialize, Deserialize)]
pub(crate) struct SecKeyTree {
    root: SecretKey,
    subs: BTreeMap<Id, SecretKey>,
}

impl KeyTreePair {
    /// Create a new tree of keys
    pub(crate) fn new() -> Self {
        let (pub_, sec) = box_::gen_keypair();
        Self {
            pub_: PubKeyTree {
                root: pub_,
                subs: Default::default(),
            },
            sec: SecKeyTree {
                root: sec,
                subs: Default::default(),
            },
        }
    }
}

#[test]
fn encrypt() {
    use sodiumoxide::crypto::box_;

    // normally theirpk is sent by the other party
    let (theirpk, theirsk) = box_::gen_keypair();
    let nonce = box_::gen_nonce();
    let plaintext = b"some data";
    let ciphertext = box_::seal(plaintext, &nonce, &theirpk, &oursk);
    let their_plaintext = box_::open(&ciphertext, &nonce, &ourpk, &theirsk).unwrap();
    assert!(plaintext == &their_plaintext[..]);
}
