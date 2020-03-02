//! Provides more convenient crypto wrappers

pub(crate) mod aes;
pub(crate) mod asym;

use serde::{Deserialize, Serialize};

/// An encrypted piece of data
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Encrypted {
    /// Number only used once
    nonce: Vec<u8>,
    /// Data buffer
    data: Vec<u8>,
}
