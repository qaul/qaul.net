//! Messages API structures

use crate::Change;
use libqaul::{
    messages::{MessageQuery, Recipient},
    users::UserAuth,
    Identity,
};
use serde::{Serialize, Deserialize};

/// Send a raw payload message
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Send {
    auth: UserAuth,
    recipient: Recipient,
    service: String,
    payload: Vec<u8>,
}

/// Send a poll request to the message endpoint
///
/// Polling the API for changes might not be the most performant way
/// of getting new messages.  Instead, consider setting up a push
/// listener for your transport layer.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Poll {
    auth: UserAuth,
    service: String,
}

/// Setup a listener/ push handler for messages
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Subscribe {
    auth: UserAuth,
    service: String,
    listener_id: String,
}

/// Query for a set of messages
///
/// Instead of subscribing to the set of message changes for a
/// service, query the existing set of messages, according to some
/// parameters
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Query {
    auth: UserAuth,
    service: String,
    query: MessageQuery,
}
