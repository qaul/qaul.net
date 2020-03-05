//! An encrypted map datastructure

use crate::{
    crypto::{DetachedKey, Encrypted, Encrypter},
    error::{Error, Result},
};
use async_std::sync::Arc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
};

/// A mapper around encrypted data in a map
#[derive(Serialize, Deserialize)]
pub(crate) struct EncryptedMap<K, V, Q>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<Q> + Serialize + DeserializeOwned,
    Q: Encrypter<V>,
{
    #[serde(bound(deserialize = "V: DeserializeOwned"))]
    inner: BTreeMap<K, Encrypted<V, Q>>,
}

impl<K, V, Q> EncryptedMap<K, V, Q>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<Q> + Serialize + DeserializeOwned,
    Q: Encrypter<V>,
{
    /// Create a new encrypted map
    pub(crate) fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Open an entry in the map with a key
    pub(crate) fn open(&mut self, id: K, key: &Q) -> Result<()> {
        match self.inner.get_mut(&id) {
            Some(entry) => Ok(entry.open(key)?),
            None => Err(Error::UnlockFailed { id: id.to_string() }),
        }
    }

    /// Close an entry in the map
    pub(crate) fn close<P>(&mut self, id: K, key: P) -> Result<()>
    where
        P: Into<Option<Arc<Q>>>,
    {
        let key = key.into();
        match self.inner.get_mut(&id) {
            Some(entry) => {
                let key = entry
                    .key()
                    .or(key)
                    .expect("No key provided for `open` operation");
                Ok(entry.close(key)?)
            }
            None => Err(Error::UnlockFailed { id: id.to_string() }),
        }
    }

    /// Get a reference to the mapped value, if opened
    pub(crate) fn get(&self, id: K) -> Result<&V> {
        match self.inner.get(&id) {
            Some(Encrypted::Open(ref data)) => Ok(data),
            Some(Encrypted::Closed(_)) => Err(Error::InternalError {
                msg: "Tried reading ::Closed variant".into(),
            }),
            Some(Encrypted::Never(_)) => unreachable!(),
            None => Err(Error::InternalError {
                msg: "No data for key".into(),
            }),
        }
    }

    /// Get a mutable reference to the mapped value, if opened
    pub(crate) fn get_mut(&mut self, id: K) -> Result<&V> {
        match self.inner.get_mut(&id) {
            Some(Encrypted::Open(ref mut data)) => Ok(data),
            Some(Encrypted::Closed(_)) => Err(Error::InternalError {
                msg: "Tried reading ::Closed variant".into(),
            }),
            Some(Encrypted::Never(_)) => unreachable!(),
            None => Err(Error::InternalError {
                msg: "No data for key".into(),
            }),
        }
    }
}

impl<K, V, Q> Deref for EncryptedMap<K, V, Q>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<Q> + Serialize + DeserializeOwned,
    Q: Encrypter<V> + Debug,
{
    type Target = BTreeMap<K, Encrypted<V, Q>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V, Q> DerefMut for EncryptedMap<K, V, Q>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<Q> + Serialize + DeserializeOwned,
    Q: Encrypter<V> + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
