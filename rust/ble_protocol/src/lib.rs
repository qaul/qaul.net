// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul BLE wire protocol
//!
//! The platform independent parts of the qaul BLE transport: how bytes on the
//! MSG characteristic are framed, and what the flow control messages mean.
//!
//! This crate contains no I/O. 
//!
//! The wire format is a cross platform contract. The reference implementation
//! is the Android module (ReceiveQueue.kt / SendQueue.kt / FlcCreate.kt)


pub mod flc;
pub mod frame;

pub use flc::{FlcMessage, FlcType, NeighbourUpdate};
pub use frame::{ChunkHeader, Frame, FrameError};

/// Number of leading qaul ID bytes used as a peer hint, both in advertisements
/// and in `SEND_NEIGHBOURS` entries. Mirrors `BleConstants.QAUL_ID_ADVERT_BYTES`.
pub const QAUL_ID_ADVERT_BYTES: usize = 5;

/// Full qaul node ID length in bytes ("q8id"), as carried by `SEND_QAUL_ID`.
pub const QAUL_ID_BYTES: usize = 8;
