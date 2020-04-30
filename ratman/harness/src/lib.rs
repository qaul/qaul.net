//! A testing harness for ratman routers
//!
//! This crate contains no actual tests, but makes it easier for
//! application and library tests to create distributed networks,
//! fuzzing input data types and complex service relationships.
//!
//! Pick one of the network creation strategies, chose the required
//! inputs and outputs, and then initialise your application state
//! accordingly.  In case you are writing tests for libqaul, use the
//! iter_mut() to then initialise and store your endpoint state.

use netmod_mem::MemMod;
use ratman::Router;
use std::{sync::Arc, time::Duration};
use tempfile::{tempdir, TempDir};

pub fn temp() -> TempDir {
    tempdir().unwrap()
}

pub fn millis(m: u64) -> Duration {
    Duration::from_millis(m)
}

pub fn sec5() -> Duration {
    Duration::from_secs(5)
}

pub fn sec10() -> Duration {
    Duration::from_secs(10)
}

/// Initialise a network with some application state
pub trait Initialize<T> {
    fn init_with<'a, F: Fn(&'a str, Arc<Router>) -> T>(&'a mut self, cb: F);
}

/// A very simple three-point network
///
/// This notwork consists of three nodes, `A`, `middle`, and `B`, that
/// are topologically layed out as follows: `A` - `middle` - `B`.
/// Only A and B are exposed from this struct, and initialised under
/// the hood.
pub struct ThreePoint<T> {
    pub a: (Arc<Router>, Option<T>),
    pub b: (Arc<Router>, Option<T>),
    _middle: Arc<Router>,
}

impl<T> ThreePoint<T> {
    pub async fn new() -> Self {
        let (mma, ma) = MemMod::make_pair();
        let (mmb, mb) = MemMod::make_pair();

        let a = Router::new();
        a.add_endpoint(mma).await;
        let a = (a, None);

        let _middle = Router::new();
        _middle.add_endpoint(ma).await;
        _middle.add_endpoint(mb).await;

        let b = Router::new();
        b.add_endpoint(mmb).await;
        let b = (b, None);

        Self { a, _middle, b }
    }

    /// Get easy access to the `A` type
    ///
    /// Panics if not initialised
    pub fn a(&self) -> &T {
        self.a.1.as_ref().unwrap()
    }

    /// Get easy access to the `B` type
    ///
    /// Panics if not initialised
    pub fn b(&self) -> &T {
        self.b.1.as_ref().unwrap()
    }
}

impl<T> Initialize<T> for ThreePoint<T> {
    fn init_with<'a, F: Fn(&'a str, Arc<Router>) -> T>(&'a mut self, cb: F) {
        self.a.1 = Some(cb("a", Arc::clone(&self.a.0)));
        self.b.1 = Some(cb("b", Arc::clone(&self.b.0)));
    }
}
