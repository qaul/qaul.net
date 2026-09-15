// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

// Service and characteristic UUIDs.
//
// Must match android and ios for crossplatform

use bluer::Uuid;

pub fn main_service_uuid() -> Uuid {
    Uuid::parse_str("c7b64399-37d9-4c00-92a0-6d0a87346816").unwrap()
}
// Unused for normal advertisements but can be used in extended advertisements
pub fn msg_service_uuid() -> Uuid {
    Uuid::parse_str("99e91400-80ed-4943-9bcb-39c532a76023").unwrap()
}
pub fn read_char() -> Uuid {
    Uuid::parse_str("c7b64401-37d9-4c00-92a0-6d0a87346816").unwrap()
}
pub fn msg_char() -> Uuid {
    Uuid::parse_str("c7b64402-37d9-4c00-92a0-6d0a87346816").unwrap()
}

pub fn psm_char() -> Uuid {
    Uuid::parse_str("c7b64403-37d9-4c00-92a0-6d0a87346816").unwrap()
}

/// 0xFFFF is the value reserved for testing / internal use.
pub const QAUL_MANUFACTURER_ID: u16 = 0xFFFF;

/// Number of leading qaul ID bytes advertised
pub const QAUL_ID_ADVERT_BYTES: usize = 5;
