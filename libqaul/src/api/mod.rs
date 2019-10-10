//! # `libqaul` service API
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
//!
//! ## Models
//!
//! Models defined in this submodule are different from any other
//! models defined in `libqaul`: they are the public representations,
//! i.e.  only fields that are relevant for service developers to
//! interact with, not including shared service state or secrets.

#[cfg(test)]
mod tests;

mod contacts;
mod files;
mod message_store;
mod messages;
mod models;
mod service;
mod users;

pub use models::*;
