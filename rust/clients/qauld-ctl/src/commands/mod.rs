// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Trait definitions for commands to be passed to qauld

use crate::proto;

mod node;

pub trait RpcCommand {
    fn encode_request(&self) -> Result<(Vec<u8>, proto::Modules), Box<dyn std::error::Error>>;
    fn decode_response(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
