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


pub mod constants;
mod crc;
pub mod flc;
pub mod frame;
pub mod queue;

pub use constants::{QAUL_ID_ADVERT_BYTES, QAUL_ID_BYTES};
pub use flc::{FlcMessage, FlcType, NeighbourUpdate};
pub use frame::{ChunkHeader, FirstChunkHeader, Frame, FrameError};
