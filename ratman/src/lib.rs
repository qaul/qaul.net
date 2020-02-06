//! # A modular packet router
//!
//! A modular userspace frame router, implementing distance vector
//! routing tables, and delay tolerance.  Its basic design is
//! inspired by BATMAN, and it's written entirely in Rust (hence the
//! name).
//!
//! ## Usage
//!
//! To use Ratman, you need to create a `Router`.  This type exposes
//! an async API to interact with various network abstractions.  A
//! networking endpoint, as well as the basic datagram types are
//! defined in the [`ratman-netmod`] crate, and the routing identities are
//! defined in the [`ratman-identity`] crate.
//!
//! [`ratman-netmod`]: https://crates.io/crate/ratman-netmod
//! [`ratman-identity`]: https://crates.io/crate/ratman-identity
//!
//!
//! ## Netmod architecture
//!
//! Because the router exists entirely in userspace, decoupled from
//! any kernel networking layer.  This means that the router is
//! responsible for also sending any payload to the appropriate
//! driver.  A driver is some binding to send data, via a channel,
//! such as UDP, or in-memory channel interfaces.
//!
//!
//! ## Clocking
//!
//! Generally, Ratman handles scheduling and internal clocking for
//! you.  There's no need to call update functions to make poll's
//! work, or to actually dispatch messages.  During initialisation the
//! constructor spawns several long running tasks, that deal with
//! various tasks in the router stack in a loop.  The downside to this
//! is that the user of the library has no control over how this code
//! is called.
//!
//! This is where the Router API adds clock points, and the `clock`
//! submodule.  The idea is that an external program (i.e. you!)  can
//! use Barriers to set clock points for various tasks, that will
//! internally wait for the external clock intput.  This way it is
//! possible to reduce the data rate in low power mode, without having
//! to teach Ratman what this means.
//!
//! It also means that you can manually clock step the router during
//! tests to check various states and invariants in the tests.
//!
//! Check the documentation for the `Clockwork` type for more details.

pub mod clock;
mod core;
mod data;
mod error;
mod protocol;

pub use crate::{
    data::{Message, MsgId, Signature},
    error::{Error, Result},
    protocol::Protocol,
};

pub use identity::Identity;
pub use netmod;

use crate::core::Core;
use async_std::{sync::Arc, task};
use clock::{ClockCtrl, Tasks};
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
    /// Create a new, empty message router
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Core::init(),
            init: false,
        })
    }

    /// Add a new endpoint to this router
    ///
    /// An endpoint is defined by the [`Endpoint`] trait from the
    /// `ratman-netmod` crate.  Once added, an endpoint can't be
    /// removed while in active operation: the router will have to be
    /// recreated without the endpoint you wish to remove.
    ///
    /// [`Endpoint`]: https://docs.rs/ratman-netmod/0.1.0/ratman_netmod/trait.Endpoint.html
    pub fn add_endpoint(&self, ep: impl Endpoint + 'static + Send + Sync) -> Result<()> {
        if self.init {
            Err(Error::AlreadyInit)
        } else {
            self.inner.add_ep(ep);
            Ok(())
        }
    }

    /// Finalise the routers endpoint map and run the internal tasks
    pub fn finalise(&mut self) {
        self.init = true;
        self.inner.run();
    }

    /// Add an identity to the local set
    ///
    /// Ratman will listen for messages to local identities and offer
    /// them up for polling via the Router API.
    pub fn add_local(&self, id: Identity) -> Result<()> {
        task::block_on(async { self.inner.add_local(id).await });
        Ok(())
    }

    /// Remove a local identity, discarding imcomplete messages
    ///
    /// Ratman will by default remove all cached frames from the
    /// collector.  Optionally these frames can be moved into the
    /// journal with low priority instead.
    pub fn rm_local(&self, id: Identity, keep: bool) -> Result<()> {
        task::block_on(async { self.inner.rm_local(id).await });
        Ok(())
    }

    /// Register a manual clock controller object for internal tasks
    pub fn clock(&self, cc: ClockCtrl<Tasks>) -> Result<()> {
        Ok(())
    }

    /// Dispatch a message into a network
    ///
    /// This operation completes asynchronously, and will yield a
    /// result with information about any error that occured while
    /// sending.
    ///
    /// If you result is an `Error::DispatchFaled`, that just means
    /// that at least one of the packets your Message was sliced into
    /// didn't send properly.  As long as you're not changing the data
    /// layout of your payload, or the `MsgId`, it's safe to simply
    /// retry: the receiving collector/ journals on the way will still
    /// be able to associate the frames, and drop the ones that were
    /// already dispatched, essentially only filling in the missing
    /// gaps.
    pub async fn send(&self, msg: Message) -> Result<()> {
        self.inner.send(msg).await;
        Ok(())
    }

    /// Get the next available message from the router
    ///
    /// **Note**: This function can't ever really fail, because it
    /// only reads from a set of completed Messages that have been
    /// parsed and handled.  When an error occurs on an incoming
    /// Message, the errors are logged in the diagnostics module, and
    /// can be read from there asynchronously.
    pub async fn next(&self) -> Message {
        self.inner.next().await
    }
}
