use crate::clock::{Error, Interval, Target};
use async_std::sync::{Arc, Barrier};
use std::{collections::BTreeMap, time::Duration};

/// A collection of clocking instructions for Ratman
///
/// Each task can be configured individually via the [`Clock`] type,
/// returned by [`clock()`].
pub struct Clockwork {
    clocks: BTreeMap<Target, Clock>,
}

impl Clockwork {
    pub fn new() -> Self {
        Self {
            clocks: Default::default(),
        }
    }

    /// Override the default clocking scheme for a particular target
    ///
    /// When not providing further interval settings, Ratman will fall
    /// back to a more paced clocking scheme, which is usually
    /// suitable for low power operation modes.  If even this mode is
    /// excessive (say, for embedded platforms or low power devices),
    /// further clock parameters can be set on the `Clock` type.
    pub fn clock(&mut self, trgt: Target) -> &mut Clock {
        self.clocks.entry(trgt).or_insert(Clock::default())
    }
}

/// Represents a single clock target inside the [`Clockwork`] collection
///
/// [`Clockwork`]: struct.Clockwork.html
#[derive(Default)]
pub struct Clock {
    interval: Option<Interval>,
    fence: Option<Box<dyn Fn(Arc<Barrier>)>>,
}

impl Clock {
    
    /// Set the interval at which this clock will be controlled
    pub fn set(&mut self, iv: Interval) -> &mut Self {
        self.interval = Some(iv);
        self
    }

    /// Provide a fence function which will clock the internal speed
    ///
    /// The provided function is called once, on a detached task, and
    /// is expected to block the task with block_on, which can then do
    /// async operation.
    ///
    /// ```
    /// # use ratman::clock::{Clockwork, Target, Interval, Error};
    /// # use async_std::task;
    /// # fn foo() -> Result<(), Error> {
    /// # let mut clw = Clockwork::new();
    /// clw
    ///     .clock(Target::SwitchRecv)
    ///     .set(Interval::Stepped)
    ///     .fence(|b| {
    ///         task::block_on(async move {
    ///             b.wait().await;
    ///         });
    ///     })
    ///     .ok()?;
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    pub fn fence<F: 'static>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Arc<Barrier>),
    {
        self.fence = Some(Box::new(f));
        self
    }

    /// Validate the clock settings
    ///
    /// Calling this function is not mandatory, but recommended
    /// because invalid clock settings will crash the Router during
    /// runtime.
    pub fn ok(&mut self) -> Result<(), Error> {
        match self.interval {
            // Not sure how to make this not a warning
            Some(Interval::Delay(0.0)) => Err(Error::InvalidTime),
            Some(Interval::Timed(dur)) if dur.as_nanos() == 0 => Err(Error::InvalidTime),
            Some(Interval::Stepped) if self.fence.is_none() => Err(Error::NoFence),
            _ => Ok(()),
        }
    }
}
