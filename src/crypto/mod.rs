//! Provides more convenient crypto wrappers
#![allow(unused)]

pub(crate) mod aes;
pub(crate) mod asym;

mod map;
pub(crate) use map::EncryptedMap;

use crate::{
    error::{Error, Result},
    Id,
};
use async_std::sync::Arc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// An encrypted piece of data
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
    fn key(&self) -> Option<Arc<K>> {
        None
    }
}

// Ids are special and should just impl this
impl<K> DetachedKey<K> for Id {}

/// A generic wrapper around the unlock state of data
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Encrypted<T, K>
where
    T: Serialize + DeserializeOwned + DetachedKey<K>,
    K: Encrypter<T>,
{
    /// An in-use data variant
    #[serde(skip_serializing)]
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    Open(T),
    /// An encrypted value
    Closed(CipherText),

    /// Purely here to make rustc happy about the generic bounds
    #[doc(hidden)]
    #[serde(skip)]
    Never(Option<PhantomData<K>>),
}

impl<T, K> Encrypted<T, K>
where
    T: Serialize + DeserializeOwned + DetachedKey<K>,
    K: Encrypter<T>,
{
    pub(crate) fn new(init: T) -> Self {
        Self::Open(init)
    }

    /// Check if the value is encrypted
    pub(crate) fn encrypted(&self) -> bool {
        match self {
            Self::Closed(_) => true,
            _ => false,
        }
    }

    /// Attempt to deref the entry
    pub(crate) fn deref<'s>(&'s self) -> Result<&'s T> {
        match self {
            Self::Open(ref t) => Ok(t),
            _ => Err(Error::LockedState {
                msg: "Encrypted::Closed(_) can't be derefed".into(),
            }),
        }
    }

    /// Attempt to deref the entry
    pub(crate) fn deref_mut<'s>(&'s mut self) -> Result<&'s mut T> {
        match self {
            Self::Open(ref mut t) => Ok(t),
            _ => Err(Error::LockedState {
                msg: "Encrypted::Closed(_) can't be derefed".into(),
            }),
        }
    }

    /// Call to the inner unlocked `key()` if the entry is open
    pub(crate) fn key(&self) -> Option<Arc<K>> {
        match self {
            Self::Open(t) => t.key(),
            _ => None,
        }
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
    pub(crate) fn close(&mut self, key: Arc<K>) -> Result<()> {
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

    /// Consume the `Encrypted<T>` type into the inner value
    ///
    /// Pancis if the value is encrypted
    #[cfg(test)]
    pub(crate) fn consume(self) -> T {
        match self {
            Self::Open(data) => data,
            _ => panic!("Couldn't consume encrypted value!"),
        }
    }
}

#[test]
fn aes_encrypt_decrypt() {
    use aes::{Constructor, Key};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Data {
        num: i32,
    };

    impl DetachedKey<Key> for Data {
        fn key(&self) -> Option<Arc<Key>> {
            None
        }
    }

    let key = Arc::new(Key::from_pw("fuck", "cops"));
    let data = Data { num: 1312 };

    // Encrypted data wrapper
    let mut enc = Encrypted::new(data.clone());

    // Close the entry
    enc.close(Arc::clone(&key)).unwrap();
    assert!(enc.encrypted());

    // Re-open the entry
    enc.open(&*key).unwrap();
    assert_eq!(enc.encrypted(), false);

    let data2 = enc.consume();

    assert_eq!(data, data2);
}

#[test]
fn asym_encrypt_decrypt() {
    use asym::KeyPair;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Data {
        num: i32,
    };

    impl DetachedKey<KeyPair> for Data {
        fn key(&self) -> Option<Arc<KeyPair>> {
            None
        }
    }

    let key = Arc::new(KeyPair::new());
    let data = Data { num: 1312 };

    // Encrypted data wrapper
    let mut enc = Encrypted::new(data.clone());

    // Close the entry
    enc.close(Arc::clone(&key)).unwrap();
    assert!(enc.encrypted());

    // Re-open the entry
    enc.open(&*key).unwrap();
    assert_eq!(enc.encrypted(), false);

    let data2 = enc.consume();

    assert_eq!(data, data2);
}
