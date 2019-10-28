//! # Service API scope providers
//!
//! The idea behind this interface is further documented in the
//! `contribute` book. It goes into detail about using it to write
//! decentralised networking services, using qaul.net as a backend.
//!
//! `qaul.net` itself provides a few primary services for "messaging",
//! "file sharing" and "VoIP", as well as a sort of hidden, management
//! "core" service.  All of them are implemented via this API,
//! allowing external developers to write their own services using
//! qaul.net libraries and networks.

// #[cfg(test)]
// mod tests;
// mod files;

pub use contacts::Contacts;
pub(crate) mod contacts;

pub use messages::Messages;
pub(crate) mod messages;

pub(crate) mod services;
pub use services::Services;

pub use users::Users;
pub(crate) mod users;
