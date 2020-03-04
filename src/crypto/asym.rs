//! Asymmetric cryto utilities

use crate::{
    crypto::{CipherText, Encrypter},
    error::{Error, Result},
    wire::Encoder,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

impl<T> Encrypter<T> for KeyPair
where
    T: Encoder<T> + Serialize + DeserializeOwned,
{
    fn seal(&self, data: &T) -> Result<CipherText> {
        let non = box_::gen_nonce();
        let enc = data.encode()?;
        let data = box_::seal(&enc, &non, &self.pub_, &self.sec);
        let nonce = non.0.into_iter().cloned().collect();
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
