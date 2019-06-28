//! RATMAN decentralised routing protocol
//!
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

mod core;
mod data;
mod diag;

use crate::core::RoutingCore;
pub use crate::data::{Message, Payload, Signature};

/// A `RATMAN` router context
pub struct Router {}

impl Router {
    fn new() -> Self {
        unimplemented!()
    }
}
