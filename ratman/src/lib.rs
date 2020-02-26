//! # Ratman
//!
//! A modular userspace frame router, implementing distance vector
//! routing, and delay tolerance.  Handles topology updates and user
//! discovery via flood heartbeats, and provides a non-namespaced view
//! into a network via ed25519 keyed user IDs.
//!
//! One of the core principles of Ratman is to make network roaming
//! easier, building a general abstraction over a network, leaving it
//! up to drivers to interface with implementation specifics.
//!
//! As such, the Ratman routing tables, and user IDs don't use IPs and
//! one device on the network could potentially be home to many user
//! IDs.  The decoupling of users and devices, making it impossible to
//! track a user back to a specific device, is by design.
//!
//!
//! ## Usage
//!
//! To use Ratman, you need to create a Router.  This type exposes an
//! async API to interact with various network abstractions.  A
//! networking endpoint and basic datagram types are defined in the
//! `ratman-netmod` crate, and the routing identities are defined in
//! the `ratman-identity` crate.
//!
//! [`ratman-netmod`]: https://crates.io/crate/ratman-netmod
//! [`ratman-identity`]: https://crates.io/crate/ratman-identity
//!
//!
//!
//! ## Interface routing
//!
//! The interface that binds the Ratman router to underlying drivers
//! is called `netmod`, which handles sending and receiving frames.  A
//! frame is a piece of data, which a checksum, which may be part of a
//! larger mesage.  In the qaul.net repository, you can find several
//! driver implementations for various platforms.  If you need to
//! write your own, don't hesitate to ask for help.
//!
//! Routing is then done by mapping a user ID to an interface (plus
//! some target data that's left to the driver to interpret).  This
//! way Ratman is able to route across network boundries, and on
//! unpriviledged hardware (such as phones).
//!
//!
//! ## Clocking
//!
//! Generally, Ratman handles scheduling and internal clocking for
//! you.  There's no need to call update functions to make poll's
//! work, or to actually dispatch messages.  During initialisation the
//! constructor spawns several long running tasks, that deal with
//! various tasks in the router stack in a loop.  The downside to this
//! is that the user of the library (your app) has no control over how
//! this code is called.
//!
//! This is where the Router API adds clock points, and the `clock`
//! submodule, enabling you to reduce data rates in low power
//! settings, without having to teach Ratman about your platform
//! specifics.
//!
//!
//! ## License
//!
//! Ratman is part of the qaul.net project, and licensed under the
//! [GNU Affero General Public License version 3 or
//! later](../licenses/agpl-3.0.md).
//!
//! See the main qaul.net repository README for additional permissions
//! granted by the authors for this code.


pub mod clock;
mod core;
mod data;
mod error;
mod protocol;

mod slicer;
pub(crate) use {data::Payload, protocol::Protocol, slicer::Slicer};

pub use crate::{
    data::{Message, MsgId},
    error::{Error, Result},
};

pub use identity::{Identity, ID_LEN};
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
    inner: Arc<Core>,
    proto: Arc<Protocol>,
}

impl Router {
    /// Create a new message router
    pub fn new() -> Arc<Self> {
        let proto = Protocol::new();
        let inner = Arc::new(Core::init());
        inner.run();

        Arc::new(Self { inner, proto })
    }

    /// Add a new endpoint to this router
    ///
    /// An endpoint is defined by the [`Endpoint`] trait from the
    /// `ratman-netmod` crate.  Once added, an endpoint can't be
    /// removed while in active operation: the router will have to be
    /// recreated without the endpoint you wish to remove.
    ///
    /// [`Endpoint`]: https://docs.rs/ratman-netmod/0.1.0/ratman_netmod/trait.Endpoint.html
    pub fn add_endpoint(&self, ep: impl Endpoint + 'static + Send + Sync) -> usize {
        task::block_on(async { self.inner.add_ep(ep).await })
    }

    /// Remove an endpoint from the router by ID
    ///
    /// This function is primarily meant for testing purposes, and
    /// shouldn't be used in heavy operation.  The required ID is
    /// returned by `add_endpoint`.
    pub fn del_endpoint(&self, id: usize) {
        task::block_on(async { self.inner.rm_ep(id).await });
    }
    
    /// Add an identity to the local set
    ///
    /// Ratman will listen for messages to local identities and offer
    /// them up for polling via the Router API.
    pub fn add_local(&self, id: Identity) -> Result<()> {
        task::block_on(async { self.inner.add_local(id).await })
    }

    /// Remove a local identity, discarding imcomplete messages
    ///
    /// Ratman will by default remove all cached frames from the
    /// collector.  Optionally these frames can be moved into the
    /// journal with low priority instead.
    pub fn rm_local(&self, id: Identity, _keep: bool) -> Result<()> {
        task::block_on(async { self.inner.rm_local(id).await })
    }

    /// Set a user ID as online and broadcast announcements
    ///
    /// This function will return an error if the user is already
    /// marked as offline, or if no such user is known to the router
    pub async fn online(&self, id: Identity) -> Result<()> {
        self.inner.known(id, true).await?;
        Arc::clone(&self.proto)
            .online(id, Arc::clone(&self.inner))
            .await
    }

    /// Set a user ID as offline and stop broadcasts
    pub async fn offline(&self, id: Identity) -> Result<()> {
        self.inner.known(id, true).await?;
        self.proto.offline(id).await
    }

    /// Check the local routing table for a user ID
    pub async fn known(&self, id: Identity) -> Result<()> {
        self.inner.known(id, false).await
    }

    /// Check for newly discovered users on the network
    pub async fn discover(&self) -> Identity {
        self.inner.discover().await
    }

    /// Register a manual clock controller object for internal tasks
    pub fn clock(&self, _cc: ClockCtrl<Tasks>) -> Result<()> {
        unimplemented!()
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
