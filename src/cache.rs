//! Alexandria internal caching system
//!
//! The caches are divided into a hot cache, which is in active
//! rotation and <user-id>-<zone>-<record> indexed, and a cold cache
//! which can be used to pre-validate a set of changes, which is
//! <delta-id> indexed.  Because each transaction is assigned a new
//! delta id,

use crate::{
    crypto::{asym::KeyPair, DetachedKey, EncryptedMap},
    notify::{Lock, LockNotify, Notify},
    Id,
};
use async_std::{sync::Arc, task};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    cmp::{Ord, Ordering as Order /* slurred yelling */, PartialOrd},
    hash::Hash,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

/// A key that expresses an (id, zone) tuple
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub(crate) struct CombKey {
    pub(crate) id: Id,
    pub(crate) zone: String,
}

impl Ord for CombKey {
    fn cmp(&self, other: &Self) -> Order {
        self.id.cmp(&other.id).then(self.zone.cmp(&other.zone))
    }
}

impl PartialOrd for CombKey {
    fn partial_cmp(&self, other: &Self) -> Option<Order> {
        self.id.partial_cmp(&other.id).and_then(|id| {
            self.zone
                .partial_cmp(&other.zone)
                .and_then(|zone| Some(id.then(zone)))
        })
    }
}

impl ToString for CombKey {
    fn to_string(&self) -> String {
        format!("{}/{}", self.id, self.zone)
    }
}

/// An Arc reference to a cache
pub(crate) type CacheRef<K, V> = Arc<Cache<K, V>>;

pub(crate) struct Cache<K, V>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<KeyPair> + Serialize + DeserializeOwned,
{
    /// Cache from K -> V with an asymmetric encryption key
    cache: LockNotify<EncryptedMap<K, Notify<V>, KeyPair>>,
    /// The path the cache is written to
    path: Option<PathBuf>,
}

impl<K, V> Cache<K, V>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<KeyPair> + Serialize + DeserializeOwned,
{
    /// Create a new in-memory cache
    pub(crate) fn new<P>(path: P) -> CacheRef<K, V>
    where
        P: Into<Option<PathBuf>>,
    {
        Arc::new(Self {
            cache: Notify::new(Lock::new(EncryptedMap::new())),
            path: path.into(),
        })
    }

    /// Set the cache to hot, enabling write-through caching
    ///
    /// Reversing this option is not possible.  A hot cache does
    /// write-through caching to disk, meaning that changes are
    /// mirrored to disk immediately to avoid data loss when crashing.
    /// By default the cache can be used to improve in-memory lookups,
    /// but will not be persistent across reboots.
    pub(crate) fn hot(self: Arc<Self>) {
        task::spawn(async move {});
    }
}
