//! libqaul (and friends) JVM FFI API
//!
//! This module exposes a set of `extern "C"` functions that are bound
//! to from the Android qaul.net app.  Because the android-support
//! crate handles more functionality than just libqaul, or individual
//! services, the functions here are namespaced to be less cluttered
//! or confusing.  Any shared code is kept in the `utils` module in
//! the root of the crate.

pub mod base;
pub mod chat;
pub mod users;
