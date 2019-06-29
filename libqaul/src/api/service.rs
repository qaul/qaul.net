//! Defines a basic interface for a Qaul service

use super::models::Message;
use std::error::Error;

/// The interface through which communication with a service occurs
pub trait ServiceConnect<E: Error> {
    /// The ID of the service this ServiceConnect is connected to
    fn service_id(&self) -> String;
    /// Send a Message to the service. Calls to this method must not block.
    fn send_msg(&mut self) -> Result<(), E>;
    /// Check for Messages received from the service. Calls to this message may block,
    /// but not if the last call to poll_messages() returned true (if ServiceConnectAsync
    /// is implemented for this type).
    fn messages(&mut self) -> Result<Vec<Message>, E>;
}

/// Add-on interface for async communication with a service
pub trait ServiceConnectAsync<E: Error>: ServiceConnect<E> {
    /// Check whether or not the next call to messages() will block.
    fn poll_messages(&mut self) -> Result<bool, E>;
}
