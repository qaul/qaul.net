//! Asymmetric cryto utilities

use crate::{
    crypto::{CipherText, Encrypter},
    error::{Error, Result},
    wire::Encoder,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sodiumoxide::crypto::box_::{self, Nonce, PublicKey, SecretKey};

pub(crate) type SharedKey = KeyPair;

/// Both public and private keys for a user
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl<T> Encrypter<T> for KeyPair
where
    T: Encoder<T> + Serialize + DeserializeOwned,
{
    fn seal(&self, data: &T) -> Result<CipherText> {
        let non = box_::gen_nonce();
        let enc = data.encode()?;
        let data = box_::seal(&enc, &non, &self.pub_, &self.sec);
        let nonce = non.0.iter().cloned().collect();
        Ok(CipherText { nonce, data })
    }

    fn open(&self, data: &CipherText) -> Result<T> {
        let CipherText {
            ref nonce,
            ref data,
        } = data;
        let nonce = Nonce::from_slice(nonce.as_slice()).ok_or(Error::InternalError {
            msg: "Failed to read nonce!".into(),
        })?;
        let clear = box_::open(data.as_slice(), &nonce, &self.pub_, &self.sec).map_err(|_| {
            Error::InternalError {
                msg: "Failed to decrypt data".into(),
            }
        })?;

        Ok(T::decode(&clear)?)
    }
}

#[test]
fn sign_and_encrypt() {
    use ed25519_dalek::Keypair as DKP;
    use rand::rngs::OsRng;
    let mut rng = OsRng {};

    let DKP { secret, public } = DKP::generate(&mut rng);

    let nacl_pair = KeyPair {
        sec: SecretKey::from_slice(secret.as_bytes()).unwrap(),
        pub_: PublicKey::from_slice(public.as_bytes()).unwrap(),
    };
    let dalek_pair = DKP { secret, public };

    let message = "this can be signed and encrypted!";

    // Try to sign data
    let sign = dalek_pair.sign(message.as_bytes());

    // Encrypt the message
    let ctext = nacl_pair
        .seal(&message.as_bytes().iter().cloned().collect::<Vec<u8>>())
        .unwrap();

    // Verify signature
    dalek_pair.verify(message.as_bytes(), &sign).unwrap();

    // Decrypt secret
    nacl_pair.open(&ctext).unwrap()
}
