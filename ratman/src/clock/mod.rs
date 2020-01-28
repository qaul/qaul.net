//! Enables router internal clock manipulation
//!
//! By default, each detached task inside Ratman is run at the speed
//! that the hardware allows, i.e. polling tasks will not wait between
//! poll loops.  This is usually fine, on systems that are not battery
//! or CPU constrained.  However, on systems that are, it can cause
//! huge battery drain.  This is where [`Clockwork`] comes in, a clock
//! receiver which can be configured with various types to manipulate
//! the runtime behaviour of the internal tasks running inside Ratman.
//!
//! ```norun
//! # use ratman::{clock::*, Router};
//! let r = Router::new();
//! let mut clw = Clockwork::new();
//! clw.clock(Target::Journal)
//!     .interval(Interval::Timed(Duration::from_secs(10));
//!
//! clw.clock(Target::SwitchRecv);
//!     .interval(Interval::Stepped)
//!     .fence(move |_| {
//!         // ...
//!     });
//!
//! r.clock(clw);

mod clockwork;
mod error;

pub use clockwork::{Clock, Clockwork};
pub use error::Error;

use std::time::Duration;

/// The type of clocking mechanism to use
#[derive(Clone, Debug)]
pub enum Interval {
    /// Indicates that the parameter should be manually stepped
    Stepped,
    /// Adds a relative delay to the default clock times
    Delay(f32),
    /// Schedules an event in a fixed interval
    Timed(Duration),
}

/// A set of internal components that can be externally clocked
///
/// Each component can be controlled individually, while the
/// [`Clockwork`] data object also allows for global settings to apply
/// for all scheduled tasks.
///
/// [`Clockwork`]: struct.Clockwork.html
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Target {
    /// The dispatch worker task
    Dispatch,
    /// Switch receiver polling task
    SwitchRecv,
    /// Journal re-transmission task
    Journal,
    /// Incoming frame de-sequencing task
    Collector,
}
