//! Namespaced API scopes for the main `Library` type
//!
//! To avoid name collisions, or a too crowded namespace, the API
//! modules use a builder-style pattern, where you use an accessor to
//! get an API object.  This object can already be pre-configured with
//! settings, or further tuned before building a transaction.
//!
//! ```rust
//! # use alexandria::{Library, Builder, Result};
//! # fn foo() -> Result<()> {
//! let l = Builder::new().offset("/var/lib/myapp").build()?;
//! l.user(Id::random()).create("car horse battery staple")?;
//! # }
//! ```
//!
//!
//! ## Commiting data
//!
//! All changes made are cached before being written to the
//! active data set.  You need to call `commit()` to queue them from
//! the cache to the active set.  Alternatively you can call
//! `discard()` to clear the entries from the cache.  Dropping the API
//! scope handler is synonymous to calling `commit()`.
//!
//!
//! ## Hot cache & cold cache
//!
//! When you queue an action via the API it will be immediatiely
//! validated and entered into a hot cache.  This ensures that other
//! comsumers get immediate access to the data.  Altenatively you can
//! queue your changes into the cald cache, by first calling `cold()`
//! on the API state handler.
//!
//! Then either `commit()` your changes or release the cold-lock by
//! calling `cache()`.
//!
//! The hot cache will be periodically synced to disk to provide crash
//! resistence; **the cold cache will not!**

mod users;
pub use users::{Update, Users};

mod data;
pub use data::Data;
