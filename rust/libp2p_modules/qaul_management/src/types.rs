// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Management Types
//!
//! Definitions of the network messages sent by the qaul network management
//! behaviour (spec §11).

use libp2p::PeerId;

/// a message we send to one neighbour as per: spec 11.4
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulManagementSend {
    /// neighbour to hand the message to
    pub send_to: PeerId,
    /// encoded `ManagementMessage`
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulManagementReceived {
    /// neighbour we received this message from
    pub received_from: PeerId,
    /// encoded `ManagementMessage`
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QaulManagementData {
    pub data: Vec<u8>,
}
