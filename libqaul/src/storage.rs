//! General database storage abstraction

use std::collections::HashMap;

/// Provides a persistent key-value store
pub(crate) struct Store<K, V> {
    inner: HashMap<K, V>,
}
