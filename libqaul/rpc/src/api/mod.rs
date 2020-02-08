//! API wrapper structures

use serde::{Serialize, Deserialize};

pub mod contacts;
pub mod users;
pub mod messages;
pub mod files;

/// Represents a generic change made to some structure
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Change<T> {
    /// Don't change this value
    Ignore,
    /// Set a field to a value
    Set(T),
    /// Unset a field to "None"
    Unset,
}

impl<T> Default for Change<T> {
    fn default() -> Self {
        Change::Ignore
    }
}

pub trait ChangeExt<T> {
    /// Apply the contents of a change to some field value
    fn apply(self, prev: T) -> T;    
}

impl<T> ChangeExt<Option<T>> for Change<T> {
    fn apply(self, prev: Option<T>) -> Option<T> {
        match self {
            Change::Ignore => prev,
            Change::Set(t) => Some(t),
            Change::Unset => None,
        }
    }
}

impl<T: Default> ChangeExt<T> for Change<T> {
    fn apply(self, prev: T) -> T {
        match self {
            Change::Ignore => prev,
            Change::Set(t) => t,
            Change::Unset => T::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use conjoiner_engine;

    #[test]
    fn json_serde() {
        let variants = [
            (Change::Ignore, "\"ignore\""),
            (Change::Set(true), "{\"set\":true}"),
            (Change::Unset, "\"unset\""),
        ];

        for (v, s) in variants.iter() {
            let string = serde_json::to_string(v).unwrap();
            assert_eq!(&string, s);
            let value : Change<bool> = serde_json::from_str(&string).unwrap();
            assert_eq!(value, *v);
        }
    }

    #[test]
    fn conjoiner_serde() {
        let variants = [
            (Change::Ignore, vec![0_u8]),
            (Change::Set(true), vec![1, 1]),
            (Change::Unset, vec![2]),
        ];

        for (v, s) in variants.iter() {
            let data = conjoiner_engine::serialise(v).unwrap();
            assert_eq!(&data, s);
            let value : Change<bool> = conjoiner_engine::deserialise(&data).unwrap();
            assert_eq!(value, *v);
        }
    }
}
