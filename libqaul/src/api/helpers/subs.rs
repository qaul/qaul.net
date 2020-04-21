use crate::Identity;
use async_std::{
    future::Future,
    pin::Pin,
    stream::Stream,
    sync::Receiver,
    task::{Context, Poll},
};

/// A unique, randomly generated subscriber ID
pub type SubId = Identity;

/// A generic subscription which can stream data from libqaul
///
/// Each subscription has a unique ID that can later on be used to
/// cancel the stream.  This type also allows for stream manipulation,
/// for example throttling throughput, or only taking a subset.
pub struct Subscription<T> {
    /// The subscription ID
    pub id: SubId,
    /// The subscription reveicer that get's polled
    pub(crate) rx: Receiver<T>,
}

impl<T> Subscription<T> {}

impl<T> Stream for Subscription<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unsafe { Pin::new_unchecked(&mut self.rx.recv()) }.poll(ctx)
    }
}
