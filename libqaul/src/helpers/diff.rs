//! API diffs

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Represents a generic change made to some structure
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ItemDiff<T> {
    /// Don't change this value
    Ignore,
    /// Set a field to a value
    Set(T),
    /// Unset a field to "None"
    Unset,
}

impl<T> Default for ItemDiff<T> {
    fn default() -> Self {
        ItemDiff::Ignore
    }
}

pub trait ItemDiffExt<T> {
    /// Apply the contents of a change to some field value
    fn apply(self, prev: T) -> T;
}

impl<T> ItemDiffExt<Option<T>> for ItemDiff<T> {
    fn apply(self, prev: Option<T>) -> Option<T> {
        match self {
            ItemDiff::Ignore => prev,
            ItemDiff::Set(t) => Some(t),
            ItemDiff::Unset => None,
        }
    }
}

impl<T> ItemDiffExt<&mut Option<T>> for ItemDiff<T> {
    fn apply(self, prev: &mut Option<T>) -> &mut Option<T> {
        match self {
            ItemDiff::Ignore => {}
            ItemDiff::Set(t) => {
                *prev = Some(t);
            }
            ItemDiff::Unset => {
                *prev = None;
            }
        }
        prev
    }
}

impl<T: Default> ItemDiffExt<T> for ItemDiff<T> {
    fn apply(self, prev: T) -> T {
        match self {
            ItemDiff::Ignore => prev,
            ItemDiff::Set(t) => t,
            ItemDiff::Unset => T::default(),
        }
    }
}

/// Represents a generic change made to an unordered set of items
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SetDiff<T> {
    /// Add an item to the set
    Add(T),
    /// Remove an item from the set
    Remove(T),
    /// Make no change to the set
    Ignore,
}

impl<T> Default for SetDiff<T> {
    fn default() -> Self {
        SetDiff::Ignore
    }
}

pub trait SetDiffExt<T> {
    /// Apply a set of changes to some object
    ///
    /// **This may not respect ordering**
    fn apply<I: IntoIterator<Item = SetDiff<T>>>(&mut self, iter: I);
}

impl<T: Ord> SetDiffExt<T> for BTreeSet<T> {
    fn apply<I: IntoIterator<Item = SetDiff<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(|item| match item {
            SetDiff::Add(t) => {
                self.insert(t);
            }
            SetDiff::Remove(t) => {
                self.remove(&t);
            }
            _ => {}
        });
    }
}

/// Represents a generic change made to a map
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MapDiff<K, V> {
    /// Set the key `key` to value `value` in the map
    Add { key: K, value: V },
    /// Remove the given key from the map
    Remove(K),
    /// Make no change to the map
    Ignore,
}

impl<K, V> Default for MapDiff<K, V> {
    fn default() -> Self {
        MapDiff::Ignore
    }
}

pub trait MapDiffExt<K, V> {
    /// Apply a set of changes to some object
    ///
    /// **This may not respect ordering**
    fn apply<I: IntoIterator<Item = MapDiff<K, V>>>(&mut self, iter: I);
}

impl<K: Ord, V> MapDiffExt<K, V> for BTreeMap<K, V> {
    fn apply<I: IntoIterator<Item = MapDiff<K, V>>>(&mut self, iter: I) {
        iter.into_iter().for_each(|item| match item {
            MapDiff::Add { key, value } => {
                self.insert(key, value);
            }
            MapDiff::Remove(key) => {
                self.remove(&key);
            }
            _ => {}
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn json_serde() {
        let variants = [
            (ItemDiff::Ignore, "\"ignore\""),
            (ItemDiff::Set(true), "{\"set\":true}"),
            (ItemDiff::Unset, "\"unset\""),
        ];

        for (v, s) in variants.iter() {
            let string = serde_json::to_string(v).unwrap();
            assert_eq!(&string, s);
            let value: ItemDiff<bool> = serde_json::from_str(&string).unwrap();
            assert_eq!(value, *v);
        }
    }

    #[test]
    fn bincode_serde() {
        let variants = [
            (ItemDiff::Ignore, vec![0, 0, 0, 0]),
            (ItemDiff::Set(true), vec![1, 0, 0, 0, 1]),
            (ItemDiff::Unset, vec![2, 0, 0, 0]),
        ];

        for (v, s) in variants.iter() {
            let data = bincode::serialize(v).unwrap();
            assert_eq!(&data, s);
            let value: ItemDiff<bool> = bincode::deserialize(&data).unwrap();
            assert_eq!(value, *v);
        }
    }
}
