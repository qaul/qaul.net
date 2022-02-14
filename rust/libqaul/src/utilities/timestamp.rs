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

    pub fn convert_to_u64(time: SystemTime) -> u64 {
        let duration = time.duration_since(UNIX_EPOCH).expect("Time went backwards");
        u64::try_from(duration.as_millis()).unwrap()
    }
    
}