// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Timestamp Utility
//! 
//! This Utility centrally deals with time and
//! timestamp related operations.
//! 
//! All time labels shall use this timestamps.
//! The timestamp is a u64 in milliseconds since UNIX_EPOCH

use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

/// Timestamp Utility
/// 
/// Provides functions to all timestamp related
/// operations.
/// 
/// Timestamps are u64 numbers containing the milliseconds since UNIX_EPOCH
#[derive(Clone, Debug)]
pub struct Timestamp {

}

impl Timestamp {

    pub fn create_time() -> SystemTime {
        SystemTime::now()
    }

    /// get a timestamp from now
    pub fn get_timestamp() -> u64 {
        //create SystemTime
        let time = Timestamp::create_time();
        //create Duration
        let duration = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        
        match u64::try_from(duration.as_millis()) {
            Ok(result) => return result,
            Err(e) => {
                log::error!("convert timestamp to u64 error: {}", e);
                return 0
            }
        }
    }
}