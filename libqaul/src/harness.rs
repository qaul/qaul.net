//! libqaul stack testing harness (only in test mode)
//!
//! The qaul.net project spans a wide array of functions and
//! abstractions and while most of the time these abstractions make
//! life easier for development, sometimes they are also very
//! cumbersome.  Because realistically a test of libqaul needs to run
//! on a fake network, this requires Ratman initialisation, plus
//! whatever endpoint modules are used to connect the topology in
//! memory.
//!
//! This module is one in a set of test harnesses that can be used
//! outside the crate if the "testing" feature is enabled, meaning
//! that tests can more easily initialise a test network instead of
//! coping the same code over and over.

use crate::Qaul;
use async_std::sync::Arc;
use netmod_mem::MemMod;
use ratman::Router;
use tempfile::{tempdir, TempDir};

fn temp() -> TempDir {
    tempdir().unwrap()
}

/// A very simple three-point network
///
/// This network is meant to test re-transmission of packets via a
/// middle node.  The topology of the network is sketched below.
///
/// ```text
/// ( A ) - ( middle ) - ( B )
/// ```
pub struct ThreePoint {
    pub a: Arc<Qaul>,
    pub middle: Arc<Qaul>,
    pub b: Arc<Qaul>,
}

impl ThreePoint {
    pub async fn new() -> Self {
        let (mma, ma) = MemMod::make_pair();
        let (mmb, mb) = MemMod::make_pair();

        let r1 = Router::new();
        r1.add_endpoint(mma).await;

        let r2 = Router::new();
        r2.add_endpoint(ma).await;
        r2.add_endpoint(mb).await;

        let r3 = Router::new();
        r3.add_endpoint(mmb).await;

        let a = Qaul::new(r1, temp().path());
        let middle = Qaul::new(r2, temp().path());
        let b = Qaul::new(r3, temp().path());

        Self { a, middle, b }
    }
}
