//! Provides more convenient crypto wrappers

pub(crate) mod aes;
pub(crate) mod asym;

use crate::error::{Error, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// An encrypted piece of data
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CipherText {
    /// Number only used once
    nonce: Vec<u8>,
    /// Data buffer
    data: Vec<u8>,
}

/// A trait that encrypts data on an associated key
pub(crate) trait Encrypter<T>
where
    T: Serialize + DeserializeOwned,
{
    fn seal(&self, data: &T) -> Result<CipherText>;
    fn open(&self, data: &CipherText) -> Result<T>;
}

/// A generic wrapper around the unlock state of data
pub(crate) enum Encrypted<T>
where
    T: Serialize + DeserializeOwned,
{
    Open(T),
    Closed(CipherText),
}

impl<T> Encrypted<T>
where
    T: Serialize + DeserializeOwned,
{
    pub(crate) fn new(init: T) -> Self {
        Self::Open(init)
    }

    /// Perform the open operation in place with a key
    pub(crate) fn open<K>(&mut self, key: K) -> Result<()>
    where
        K: Encrypter<T>,
    {
        match self {
            Self::Open(_) => Err(Error::InternalError {
                msg: "tried to open ::Open(_) variant".into(),
            }),
            Self::Closed(enc) => {
                *self = Self::Open(key.open(enc)?);
                Ok(())
            }
        }
    }

    /// Perform the close operation in place with a key
    pub(crate) fn close<K>(&mut self, key: K) -> Result<()>
    where
        K: Encrypter<T>,
    {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "tried to close ::Closed(_) variant".into(),
            }),
            Self::Open(data) => {
                *self = Self::Closed(key.seal(data)?);
                Ok(())
            }
        }
    }
}
