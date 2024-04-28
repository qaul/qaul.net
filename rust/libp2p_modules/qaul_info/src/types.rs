// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Routing Info Types
//!
//! Definitions of the network messages sent by the
//! qaul routing info behaviour.

use libp2p::PeerId;

/*
/// message structure that is sent over the network
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulRoutingInfoMessage {
    // TODO: How to do that?
    //       How to find out in the incoming stream what the node_id
    //       of the sender is?
    //pub node_id: PeerId,
    pub data: Vec<u8>,
}
*/

/// a message we sent
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulInfoSend {
    /// node id we need to send the message to
    pub send_to: PeerId,
    /// binary message data
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulInfoReceived {
    /// node id we received this message from
    pub received_from: PeerId,
    /// binary message data
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulInfoData {
    pub data: Vec<u8>,
}
