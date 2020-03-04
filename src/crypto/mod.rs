//! Provides more convenient crypto wrappers
#![allow(unused)]

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

/// A type that can provide an out-of-band key
///
/// Sometimes a type that is stored inside the `Encrypted` can bring
/// it's own key, to avoid having to have a second control-structure
/// for the keys.
pub(crate) trait DetachedKey<K> {
    fn key(&self) -> Option<&K>;
}

/// A generic wrapper around the unlock state of data
#[derive(Serialize, Deserialize)]
pub(crate) enum Encrypted<T, K>
where
    T: Serialize + DeserializeOwned + DetachedKey<K>,
    K: Encrypter<T>,
{
    /// An in-use data variant
    #[serde(skip)]
    Open(T),
    /// An encrypted value
    Closed(CipherText),
    /// Purely here to make rustc happy about the generic bounds
    #[doc(hidden)]
    __Never(std::marker::PhantomData<K>),
}

impl<T, K> Encrypted<T, K>
where
    T: Serialize + DeserializeOwned + DetachedKey<K>,
    K: Encrypter<T>,
{
    pub(crate) fn new(init: T) -> Self {
        Self::Open(init)
    }

    /// Perform the open operation in place with a key
    pub(crate) fn open(&mut self, key: &K) -> Result<()> {
        match self {
            Self::Open(_) => Err(Error::InternalError {
                msg: "tried to open ::Open(_) variant".into(),
            }),
            Self::Closed(enc) => {
                *self = Self::Open(key.open(enc)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Perform the close operation in place with a key
    pub(crate) fn close(&mut self, key: &K) -> Result<()> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "tried to close ::Closed(_) variant".into(),
            }),
            Self::Open(data) => {
                let key = data.key().unwrap_or(key);
                *self = Self::Closed(key.seal(data)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Performs the close operation, purely with an detached key
    ///
    /// This function can panic and shouldn't be used unless _really_
    /// neccessary.
    pub(crate) fn close_detached(&mut self) -> Result<()> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "tried to close ::Closed(_) variant".into(),
            }),
            Self::Open(data) => {
                let key = data.key().unwrap();
                *self = Self::Closed(key.seal(data)?);
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    /// Get the value, if it was decrypted before
    pub(crate) fn read(&self) -> Result<&T> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "Tried reading ::Closed variant".into(),
            }),
            Self::Open(ref data) => Ok(data),
            _ => unreachable!(),
        }
    }

    /// Get a mutable reference to modify the value, if it was decrypted before
    pub(crate) fn modify(&mut self) -> Result<&mut T> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "Tried reading ::Closed variant".into(),
            }),
            Self::Open(ref mut data) => Ok(data),
            _ => unreachable!(),
        }
    }

    /// Replace the value, if it was decrypted before
    pub(crate) fn replace(&mut self, new: T) -> Result<T> {
        match self {
            Self::Closed(_) => Err(Error::InternalError {
                msg: "Tried reading ::Closed variant".into(),
            }),
            Self::Open(ref mut prev) => Ok(std::mem::replace(prev, new)),
            _ => unreachable!(),
        }
    }
}
