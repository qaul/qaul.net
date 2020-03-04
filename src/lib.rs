//! Alexandria storage library

pub(crate) use identity::Identity as Id;

pub(crate) mod wire;
pub(crate) mod meta;
pub(crate) mod crypto;

mod error;
pub use error::{Error, Result};

/// Main alexandria library
pub struct Library {}
