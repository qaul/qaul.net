//! A utility wrapper type for notifying async tasks about mutation of data they're interested in.
use std::ops::{Deref, DerefMut};
use std::task::Waker;

/// A wrapper which wakes tasks on mutable accesses to the wrapped value.
///
/// This can be used to transparently notify an asyncronous task that it
/// should, for example, check for more work in a queue or try again to
/// acquire a lock.
#[derive(Default, Debug, Clone)]
pub struct AccessNotifier<T> {
    inner: T,
    waker: Option<Waker>,
}

impl<T> Deref for AccessNotifier<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for AccessNotifier<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.waker.as_ref().map(|w| w.wake_by_ref());
        &mut self.inner
    }
}

impl<T> AccessNotifier<T> {
    /// Check whether or not this `AccessNotifier` has a registered `Waker`.
    ///
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::has_waker()`.
    pub fn has_waker(ptr: &AccessNotifier<T>) -> bool {
        ptr.waker.is_some()
    }

    /// Get a copy of the registered `Waker` for this `AccessNotifier`.
    ///  
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::waker()`.
    pub fn waker(ptr: &mut AccessNotifier<T>) -> Option<Waker> {
        ptr.waker.as_ref().map(|w| w.clone())
    }

    /// Call wake on the waker, if it's a waker, yehaa!
    pub fn wake_if_waker(ptr: &mut AccessNotifier<T>) {
        if let Some(ref w) = ptr.waker {
            w.clone().wake();
        }
    }

    /// Register a `Waker` to be woken upon mutable accesses to the wrapped value.
    ///  
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::register_waker()`.
    ///
    /// # Panics
    /// Panics if there is an already registered `Waker`.
    /// Use `AccessNotifier::has_waker` to check the state before using this.
    pub fn register_waker(ptr: &mut AccessNotifier<T>, waker: &Waker) {
        if AccessNotifier::has_waker(ptr) {
            panic!("Tried to register a Waker on an AccessWaker with an already registered Waker.");
        } else {
            ptr.waker = Some(waker.clone())
        }
    }

    /// Removes and returns the `Waker` registered to this `AccessNotifier`.
    ///  
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::clear_waker()`.
    pub fn clear_waker(ptr: &mut AccessNotifier<T>) -> Option<Waker> {
        ptr.waker.take()
    }

    /// Consumes the `AccessNotifier`, dropping any associated `Waker` and
    /// returning the inner value without notifying the `Waker`.
    ///  
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::into_inner()`.
    pub fn into_inner(ptr: AccessNotifier<T>) -> T {
        ptr.inner
    }

    /// Notifies any registered `Waker` immediately.
    ///
    /// This function is implemented as an associated function rather than a
    /// method to avoid conflicts with methods on the wrapped type.
    /// Call it as `AccessNotifier::notify()`.
    pub fn notify(ptr: &AccessNotifier<T>) {
        ptr.waker.as_ref().map(|w| w.wake_by_ref());
    }
}
