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
};
use serde::{de::DeserializeOwned, Serialize};

use std::{
    hash::Hash,
    sync::atomic::{AtomicBool, Ordering},
};

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
}

impl<K, V> Cache<K, V>
where
    K: Serialize + DeserializeOwned + Ord + PartialOrd + Hash + ToString,
    V: DetachedKey<KeyPair> + Serialize + DeserializeOwned,
{
    // /// Create a new in-memory cache
    // pub(crate) fn new() -> Self {
    //     Self {
    //         cache: RwLock::new(EncryptedMap::new()),
    //         hot: false.into(),
    //     }
    // }

    /// Set the cache to hot, enabling write-through caching
    ///
    /// Reversing this option is not possible.  You can only scrub the
    /// cache from disk, and build a new one.
    pub(crate) fn hot(&mut self) {
        self.hot.swap(true, Ordering::Relaxed);
        // TODO: start sync worker
    }
}
