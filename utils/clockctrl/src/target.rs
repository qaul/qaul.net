use async_std::sync::{Arc, Barrier};
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

/// Represents a single clock target
///
/// Don't construct this object manually, get a mutable builder
/// reference from [`Clockctrl::setup()`]!
///
/// [`Clockctrl::setup()`]: struct.ClockCtrl.html#method.setup
#[derive(Default)]
pub struct Target {
    /// Specify an interval which dictates scheduling of a task
    pub interval: Option<Interval>,
    /// If Interval::Stepped, provide a fence to step the clock with
    pub fence: Option<Box<dyn Fn(Arc<Barrier>) + Send + 'static>>,
}

impl Target {
    /// Set the interval at which this clock will be controlled
    pub fn set(&mut self, iv: Interval) -> &mut Self {
        self.interval = Some(iv);
        self
    }

    /// Setup a fence which will clock control the associated task
    ///
    /// The provided function is called once, on a detached task, and
    /// is expected to block the task with block_on, which can then do
    /// async operation.
    ///
    /// ```
    /// # use async_std::task;
    /// # use clockctrl::{ClockCtrl, Interval};
    /// # let mut clc = ClockCtrl::new();
    /// # #[derive(Hash, Eq, PartialEq, Ord, PartialOrd)] enum MyTasks { TaskA }
    /// clc
    ///     .setup(MyTasks::TaskA)
    ///     .set(Interval::Stepped)
    ///     .fence(|b| {
    ///         task::block_on(async move {
    ///             b.wait().await;
    ///         });
    ///     });
    /// ```
    pub fn fence<F: 'static>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Arc<Barrier>) + Send,
    {
        self.fence = Some(Box::new(f));
        self
    }
}
