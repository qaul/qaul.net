//! RATMAN decentralised routing protocol
//!
//! A modern approach to fully delay-tolerant mesh routing,
//! implemented network agnostically and entirely in userspace.

mod core;
mod data;
mod protocol;
mod utils;

pub use crate::{
    data::{Message, Payload, Signature},
    protocol::Protocol,
};

use crate::core::Core;

/// A `RATMAN` router context
#[derive(Clone)]
pub struct Router {
    core: Core,
}

impl Router {
    pub fn new() -> Self {
        Self { core: Core }
    }
}
