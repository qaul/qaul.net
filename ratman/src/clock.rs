//! Enables router internal clock manipulation
//!
//! This module largely re-exposes the [`clockctrl`] crate and API.
//! For detailed instructions on how to use this API, check these
//! crate docs instead.
//!
//! [`clockctrl`]: https://docs.rs/clockctrl

pub use clockctrl::{ClockCtrl, Error, Interval, Scheduler, Target};

/// A collection of tasks running inside the Ratman router
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tasks {
    /// Periodically tries to send undeliverable frames
    Journal,
    /// Waits for local addressed frames to desequence them
    Collector,
    /// Main router poll loop checking for new frames
    Switch
}
