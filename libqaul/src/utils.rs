//! General utility module

use rand::prelude::*;
use std::sync::RwLock;

/// Generate some secure random data into an allocated slice
pub(crate) fn random(len: usize) -> Vec<u8> {
    (0..)
        .map(|_| rand::thread_rng().next_u64())
        .take(len)
        .map(|x| x.to_be_bytes())
        .fold(Vec::new(), |mut acc, arr| {
            acc.extend(arr.iter().cloned());
            acc
        })
}

pub(crate) trait IterUtils<E>: Iterator + Sized
where
    E: Copy,
{
    fn fold_errs(self, e: E) -> Result<(), E>;
}

impl<I, E> IterUtils<E> for I
where
    I: Iterator<Item = Result<(), E>>,
    E: Copy,
{
    fn fold_errs(self, e: E) -> Result<(), E> {
        self.fold(Ok(()), |acc, x| match (acc, x) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(_)) => Err(e),
            (x, _) => x,
        })
    }
}

/// A functional remove/add API for datastructures
pub(crate) trait VecUtils<T: PartialEq> {
    /// Remove from vector, by element
    fn strip(self, t: &T) -> Self;
    /// Add to vector, returning `Self`
    fn add(self, t: T) -> Self;
}

impl<T: PartialEq> VecUtils<T> for Vec<T> {
    #[inline(always)]
    fn strip(self, t: &T) -> Self {
        self.into_iter().filter(|i| i != t).collect()
    }

    #[inline(always)]
    fn add(mut self, t: T) -> Self {
        self.push(t);
        self
    }
}

/// A utility RunLock wrapper to reduce boilerplate
pub(crate) struct RunLock {
    inner: RwLock<bool>,
}

impl RunLock {
    /// Create a RunLock with an initial value
    pub(crate) fn new(inner: bool) -> Self {
        Self {
            inner: RwLock::new(inner),
        }
    }

    /// Check the current value of RunLock
    pub(crate) fn check(&self) -> bool {
        *self.inner.read().expect("RunLock was poisoned!")
    }

    /// Set the new value of RunLock
    pub(crate) fn set(&self, val: bool) {
        let mut _val = self.inner.write().expect("RunLock was poisoned!");
        *_val = val;
    }

    pub(crate) fn and(&self, other: &RunLock) -> bool {
        self.check() && other.check()
    }
}
