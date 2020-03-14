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

pub(crate) struct Cache<K, V>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<KeyPair> + Serialize + DeserializeOwned,
{
    /// Cache from K -> V with an asymmetric encryption key
    cache: LockNotify<EncryptedMap<K, Notify<V>, KeyPair>>,
    /// Indicade whether the cache is hot
    ///
    /// A hot cache does write-through caching to disk, meaning that
    /// changes are mirrored to disk immediately to avoid data loss
    /// when crashing.  If this is `false` the cache can be used to
    /// improve in-memory lookups, but will not be persistent across
    /// reboots.  This can still be useful for frequently used data.
    hot: AtomicBool,
    /// The path the cache is written to
    path: Option<PathBuf>,
}

impl<K, V> Cache<K, V>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<KeyPair> + Serialize + DeserializeOwned,
{
    /// Create a new in-memory cache
    pub(crate) fn new<P>(path: P) -> Self
    where
        P: Into<Option<PathBuf>>,
    {
        Self {
            cache: Notify::new(Lock::new(EncryptedMap::new())),
            hot: false.into(),
            path: path.into(),
        }
    }

    /// Check whether this cache is hot
    pub(crate) fn is_hot(&self) -> bool {
        self.hot.load(Ordering::Relaxed)
    }

    /// Set the cache to hot, enabling write-through caching
    ///
    /// Reversing this option is not possible.  You can only scrub the
    /// cache from disk, and build a new one.
    pub(crate) fn hot(self: Arc<Self>) {
        self.hot.swap(true, Ordering::Relaxed);

        // TODO: start sync worker
        task::spawn(async move {});
    }
}
