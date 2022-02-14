// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Timestamp 

use std::convert::TryFrom;
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Timestamp {

}

impl Timestamp {

    pub fn init() {
        info!("Timestamp Initialized!!!");
    }

    fn create_time() -> SystemTime {
        SystemTime::now()
    }

    pub fn convert_to_u64() -> u64 {
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