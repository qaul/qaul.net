//! Ratman configuration toolkit
//!
//! Creating networks via Ratman is pretty easy but can involve a fair
//! amount of boilerplate.  To make the network initialisation easier
//! and less repetitive, this library is meant to handle network
//! module state and initialisation, at runtime, either via a
//! configuration language parser, or via the pure code API.

mod parser;
pub use parser::parse_json;

pub mod config;
