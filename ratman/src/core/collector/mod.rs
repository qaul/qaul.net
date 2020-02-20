//! The frame/message collector module
//!
//! The collector module is a bit more complicated than other modules,
//! because of the layers of state and control inversion it has to
//! contend with.
//!
//! It would be possible to do all in one file, but it would quickly
//! become too complicated, and unmaintainable.  Instead, this module
//! splits the code into three sections: the state, the worker, and
//! the manager.  The former two exploit the latter for profit.
//!
//! The manager is exposed from this module as `Collector`, so that
//! the routing core and other modules don't have to care about the
//! inner workings.  The state mostly provides a way to create and
//! yield workers, that are being polled by the manager.  The workers
//! themselves have very little control over their environment, only
//! getting access to the state manager to ask for more work, and then
//! making themselves redundant by handing in their finished messages.

use async_std::sync::{Arc, Mutex};
pub(self) type Locked<T> = Arc<Mutex<T>>;

mod state;
pub(self) use state::State;

mod worker;
pub(self) use worker::Worker;


/// The main collector management structure and API facade
pub(crate) struct Collector {
    
}
