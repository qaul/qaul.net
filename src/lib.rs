//! Encrypted record-oriented database
//!
//! **Experimental:** please note that this database was writted for
//! [qaul.net](git.open-communication.net/qaul/qaul.net), which itself
//! is a very experimental platform.  There will be data retention
//! bugs, and you shouldn't use Alexandria unless you're okay with
//! losing the data you're storing!
//!
//! A multi-payload, zone-encrypting, journaled persistence module,
//! built with low-overhead applications in mind.
//!
//! - Stores data in namespaces and scopes
//! - Key-value stores and lazy blobs
//! - Supports per-scope asymetric encryption key
//! - Uses transaction Deltas for journal and concurrency safety
//! - Integrates into OS persistence layers (storing things on spinning
//!   rust or zappy quantum tunnels)
//!
//! `alexandria` provides an easy to use database interface with
//! transactions, merges and dynamic queries, ensuring that your
//! in-memory representation of data never get's out-of-sync with your
//! on-disk representation. Don't burn your data.
#![allow(dead_code, unused_imports, unused_variables)]

pub(crate) mod core;
pub(crate) mod crypto;
pub(crate) mod delta;
pub(crate) mod dir;
pub(crate) mod meta;
pub(crate) mod notify;
pub(crate) mod store;
pub(crate) mod wire;

pub use crate::core::{Builder, DataApi, Library, UsersApi};

pub mod error;
pub mod record;
pub mod utils;
