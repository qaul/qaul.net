use crate::crypto::DetachedKey;
use async_std::{
    sync::{Arc, RwLock},
    task,
};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use std::task::Waker;

/// Utility wrapper around a serialisable lock
pub(crate) struct Lock<T>(RwLock<T>)
where
    T: DeserializeOwned + Serialize;

impl<T> Lock<T>
where
    T: DeserializeOwned + Serialize,
{
    pub(crate) fn new(inner: T) -> Self {
        Self(RwLock::new(inner))
    }
}

impl<T> Serialize for Lock<T>
where
    T: DeserializeOwned + Serialize,
{
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        task::block_on(async {
            let l = self.0.read().await;
            l.serialize(ser)
        })
    }
}

impl<'de, T> Deserialize<'de> for Lock<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(de).map(|t| Self(RwLock::new(t)))
    }
}

/// A notifiable, serialisable lock type
///
/// After deserialisation the waker will have been removed and needs
/// to be re-initialised.  Otherwise the serde calls will be forwarded
/// to the implementation provided by `T`.
pub(crate) type LockNotify<T> = Notify<Lock<T>>;

/// Wake tasks on mutable accesses to the wrapped value
///
/// This can be used to transparently notify an asyncronous task that
/// it should, for example, check for more work in a queue or try
/// again to acquire a lock.
///
/// Most importantly, this type is serialisation transparent, meaning
/// it implements `Serialize`, `Deserialize` which is forwarded to the
/// implementations provided by `T`.
#[derive(Default, Debug, Clone)]
pub struct Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    inner: T,
    waker: Option<Waker>,
}

impl<T> Deref for Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.waker.as_ref().map(|w| w.wake_by_ref());
        &mut self.inner
    }
}

impl<T> Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    /// Create an empty Notify handler
    pub(crate) fn new(inner: T) -> Self {
        Self { inner, waker: None }
    }

    /// Call wake on the waker, if it's a waker, yehaa!
    pub(crate) fn wake(ptr: &mut Notify<T>) {
        if let Some(ref w) = ptr.waker {
            w.clone().wake();
        }
    }

    /// Register a `Waker` if the wrapped value is updated
    ///
    /// This function will return the previous waker, if one existed.
    /// If `None` is returned, there was no previous waker, so be
    /// careful not to simply unwrap this value.  You may want to use
    /// `unwrap_none()`.
    pub(crate) fn register(ptr: &mut Notify<T>, waker: &Waker) -> Option<Waker> {
        ptr.waker.replace(waker.clone())
    }

    /// Removes and returns the registered `Waker`
    pub(crate) fn clear(ptr: &mut Notify<T>) -> Option<Waker> {
        ptr.waker.take()
    }

    /// Consumes the `Notify`
    pub(crate) fn into_inner(ptr: Notify<T>) -> T {
        ptr.inner
    }

    /// Notifies any registered `Waker` immediately.
    pub fn notify(ptr: &Notify<T>) {
        ptr.waker.as_ref().map(|w| w.wake_by_ref());
    }
}

// If T implements DetachedKey, just proxy the trait
impl<K, T> DetachedKey<K> for Notify<T>
where
    T: DeserializeOwned + Serialize + DetachedKey<K>,
{
    fn key(&self) -> Option<Arc<K>> {
        self.inner.key()
    }
}

impl<T> Serialize for Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(ser)
    }
}

impl<'de, T> Deserialize<'de> for Notify<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(de).map(|inner| Self { inner, waker: None })
    }
}
