// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # qaul ID
//!
//! Conversions of the qaul ID
//!
//! * qaul ID -> small version
//!   * add prefix to small version for data base search
//! * qaul ID -> q8id
//!   * add prefix to q8id for data base search

use libp2p::PeerId;

/// Qaul ID structure
pub struct QaulId {}

impl QaulId {
    const Q8ID_START: usize = 6;
    const Q8ID_END: usize = 14;

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

    /// Convert a qaul user ID to it's 8 Byte ID (q8id)
    ///
    /// The 8 Byte user ID (q8id) is used for the routing protocol.
    /// It only uses byte 7 to 14 of the qaul id.
    /// Bytes 1-6 are always the same, as they declare the hash kind.
    pub fn to_q8id(qaul_id: PeerId) -> Vec<u8> {
        // convert to bytes
        let bytes = qaul_id.to_bytes();

        Self::bytes_to_q8id(bytes)
    }

    /// Borrow the q8id bytes from a qaul user ID byte slice.
    pub fn bytes_as_q8id(bytes: &[u8]) -> &[u8] {
        &bytes[Self::Q8ID_START..Self::Q8ID_END]
    }

    /// Converts the binary form of the qaul user ID to the 8 Byte ID (q8id)
    pub fn bytes_to_q8id(bytes: Vec<u8>) -> Vec<u8> {
        // only use bytes 7 to 14
        let q8id = Self::bytes_as_q8id(&bytes).to_owned();

        // return small id
        q8id
    }

    /// Create a search key prefix for a small vector
    pub fn q8id_to_search_prefix(q8id: Vec<u8>) -> Vec<u8> {
        // add chopped prefix to small_id
        let mut key: Vec<u8> = vec![0x0, 0x24, 0x8, 0x1, 0x12, 0x20];
        key.extend(q8id);

        key
    }

    /// Create the search range for the qaul ID from an q8id
    ///
    /// This function returns a tuple with two qaul ID's for
    /// the range selection.
    /// (lowest_id, highest_id)
    ///
    /// The range can be used to search for the qaul ID in a BTreeMap.
    ///
    /// A qaul ID has 38 bytes.
    /// {prefix}{q8id}{24 bytes}
    pub fn q8id_to_search_range(q8id: Vec<u8>) -> Result<(PeerId, PeerId), String> {
        // add prefix to q8id
        let prefix = Self::q8id_to_search_prefix(q8id);

        // create lower range
        let low_suffix: Vec<u8> = vec![
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];
        let mut lowest = prefix.clone();
        lowest.extend(low_suffix);

        // create higher range
        let high_suffix: Vec<u8> = vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut highest = prefix;
        highest.extend(high_suffix);

        // convert bytes to PeerId
        let lowest_result = PeerId::from_bytes(&lowest);
        let highest_result = PeerId::from_bytes(&lowest);

        if let (Ok(lowest_id), Ok(highest_id)) = (lowest_result, highest_result) {
            return Ok((lowest_id, highest_id));
        }

        Err("q8id range conversion failed".to_string())
    }

    /// qaul ID vector to log string
    ///
    /// Create a meaningful log string without ever returning an error.
    pub fn bytes_to_log_string(bytes: &Vec<u8>) -> String {
        let result: String;

        // check if the vector is a valid qaul ID
        if let Ok(id) = PeerId::from_bytes(bytes) {
            result = id.to_base58();
        } else {
            result = "Not a valid PeerID".to_string();
        }

        // return result string
        result
    }
}
