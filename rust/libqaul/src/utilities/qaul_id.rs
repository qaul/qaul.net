// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul ID
//! 
//! Conversations and operations on the qaul ID

use libp2p::PeerId;

/// Qaul ID structure
pub struct QaulId {

}

impl QaulId {
    /// Convert a qaul ID to it's small 16 Byte version
    /// 
    /// The small version is used in the BLE module.
    /// It only uses byte 7 to 24 of the qaul id
    pub fn to_small(qaul_id: PeerId) -> Vec<u8> {
        // convert to bytes
        let bytes = qaul_id.to_bytes();

        // only use bytes 7 to 24
        let small_id = bytes[6..23].to_owned();

        // return small id
        small_id
    }

    /// Create a search key prefix for a small vector
    pub fn small_to_search_prefix(small_id: Vec<u8>) -> Vec<u8> {
        // add chopped prefix to small_id
        let mut key: Vec<u8> = vec![0x0, 0x24, 0x8, 0x1, 0x12, 0x20];
        key.extend(small_id);

        key
    }
}
