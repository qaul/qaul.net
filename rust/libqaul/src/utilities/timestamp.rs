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

    pub fn get_timestamp_by(time: &SystemTime) -> u64 {
        //create Duration
        if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
            match u64::try_from(duration.as_millis()) {
                Ok(result) => return result,
                Err(e) => {
                    log::error!("convert timestamp to u64 error: {}", e);
                }
            }    
        }
        else {
            log::error!("Time is before UNIX_EPOCH");
        }
        // one hour after zero == 3600'000 milliseconds
        60 * 60 * 1000
    }    

    /// get a timestamp since UNIX_EPOCH in milliseconds for now
    /// 
    /// The function should never panick.
    /// If it fails it returns the timestamp 1 hour after zero ( == 360'000 milliseconds ).
    pub fn get_timestamp() -> u64 {
        //create SystemTime
        let time = Timestamp::create_time();
        return Self::get_timestamp_by(&time);
    }
}