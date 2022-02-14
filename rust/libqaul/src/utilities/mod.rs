// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Libqaul Timestamp Module
//! 
//! contains:
//! 
//! * timestamp handling
   
pub mod timestamp;

use timestamp::Timestamp;

/// utilities module structure
pub struct Utilities {

}

impl Utilities {
    /// initialize utilities module
    pub fn init() {
        // initialize timestamp
        Timestamp::init();
    } 
}