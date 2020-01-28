//! Ratman highly decentralised, delay resistent key frame router

mod core;
mod data;
pub mod clock;

use crate::core::Core;
pub use data::{Message, MsgId, Signature};
use netmod::Endpoint;

/// Primary async ratman router handle
///
/// Make sure you initialise endpoints before calling [`run`], as the
/// set of endpoints gets locked and can't be changed during runtime.
///
/// [`run`]: struct.Router.html#method.run
pub struct Router {
    inner: Core,
    init: bool,
}

impl Router {
    /// Create a new router context
    pub fn new() -> Self {
        Self {
            inner: Core::init(),
            init: false,
        }
    }

    pub fn add_endpoint(&self, ep: impl Endpoint + 'static + Send + Sync) {
        if self.init {
            return;
        }

        self.inner.add_ep(ep);
    }

    /// Dispatch start this router into the background
    pub fn run(&mut self) {
        self.init = true;
        self.inner.run();
    }

    /// Asynchronously send a message through the router
    pub async fn send(&self, msg: Message) {
        self.inner.send(msg).await;
    }

    /// Get the next available message from the router
    pub async fn next(&self) -> Message {
        self.inner.next().await
    }
}
