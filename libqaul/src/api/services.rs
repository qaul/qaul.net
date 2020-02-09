//! Service interconnect interface

use crate::{error::Result, Qaul};

/// API scope type to access service management functions
///
/// Used entirely to namespace API endpoints on `Qaul` instance,
/// without having long type identifiers.
pub struct Services<'chain> {
    pub(crate) q: &'chain Qaul,
}

impl<'qaul> Services<'qaul> {
    /// Drop this scope and return back to global `Qaul` scope
    pub fn drop(&'qaul self) -> &'qaul Qaul {
        self.q
    }

    /// Add an external service to the qaul service registry
    ///
    /// Registering a service means that future `Message` listeners
    /// can be allocated for this service, as well as enabling polling.
    ///
    /// Names of services need to be unique, so it's advised to
    /// namespace them on some other key, for example the application
    /// package name (such as `com.example.myapp`)
    pub fn register<S: Into<String>>(&self, name: S) -> Result<()> {
        self.q.services.register(name.into())
    }

    /// Remove an external service from the qaul service registry
    ///
    /// Calling this function will disable the ability to poll for
    /// messages, as well as deleting all already registered message
    /// listeners already existing for this service.
    ///
    /// Will return `Error::NoService` if no such service name could
    /// be found.
    pub fn unregister<S: Into<String>>(&self, name: S) -> Result<()> {
        self.q.services.unregister(name.into())
    }
}
