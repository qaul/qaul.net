//! API wrapper structures

pub mod contacts;
pub mod messages;

/// Represents a generic change made to some structure
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
