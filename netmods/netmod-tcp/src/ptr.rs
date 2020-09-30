//! Atomic pointer abstraction
//!
//! This interface is far too useful for qaul.net as a whole to stay
//! in this file, so we should pull it out into it's own crate at some
//! point.  But for now...

use std::cmp::PartialEq;
use std::sync::{
    atomic::{AtomicPtr, Ordering},
    Arc,
};

/// An alias for a referenced pointer
pub(crate) type Ref<T> = Box<T>;

/// A safe atomic pointer wrapper
#[derive(Clone, Debug)]
pub(crate) struct AtomPtr<T> {
    inner: Arc<AtomicPtr<T>>,
}

// Implement Default for all T that implement default
impl<T: Default> Default for AtomPtr<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> PartialEq for AtomPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        let a = Box::into_raw(self.get_ref());
        let b = Box::into_raw(other.get_ref());

        a == b
    }
}

impl<T> AtomPtr<T> {
    /// Create a new atomic pointer for a type
    pub(crate) fn new(t: T) -> Self {
        let ptr = Box::into_raw(Box::new(t));
        let inner = Arc::new(AtomicPtr::from(ptr));
        Self { inner }
    }

    /// Get an immutable reference to the current value
    pub(crate) fn get_ref(&self) -> Ref<T> {
        let ptr = self.inner.load(Ordering::Relaxed);
        unsafe { Box::from_raw(ptr) }
    }

    /// Swap the data entry with a new value, returning the old
    pub(crate) fn swap(&self, new: T) -> Ref<T> {
        let ptr = Box::into_raw(Box::new(new));
        let prev = self.inner.swap(ptr, Ordering::Relaxed);
        unsafe { Box::from_raw(prev) }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    name: String,
}

#[test]
fn cloned() {
    let ts = TestStruct {
        name: "Hello".into(),
    };

    let ptr1 = AtomPtr::new(ts);
    let ptr2 = ptr1.clone();

    assert_eq!(ptr1, ptr2);
}

#[test]
fn swap() {
    let ts1 = TestStruct {
        name: "Hello".into(),
    };

    let ts2 = TestStruct {
        name: "Hello".into(),
    };

    let ptr = AtomPtr::new(ts1.clone());
    let still_ts1 = ptr.swap(ts2);

    assert_eq!(ts1, *still_ts1);
}
