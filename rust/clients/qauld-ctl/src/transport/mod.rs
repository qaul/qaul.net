// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Transport abstraction so qauld-ctl can talk to libqaul either over
//! a Unix socket (default) or in-process (`--features embedded`).
//!
//! Both transports speak the same `QaulRpc` envelope; the trait just
//! sends one envelope and waits for the matching response.

use std::time::Duration;

use async_trait::async_trait;
use prost::Message;

use crate::proto;

mod socket;
pub use socket::SocketTransport;

#[cfg(feature = "embedded")]
mod embedded;
#[cfg(feature = "embedded")]
pub use embedded::EmbeddedTransport;

#[async_trait]
pub trait RpcTransport: Send {
    /// Send a fully-formed `QaulRpc` envelope and wait for the
    /// daemon's response carrying the same `request_id`. Returns
    /// `Ok(None)` when the caller indicated it doesn't expect a
    /// response (fire-and-forget commands).
    async fn request(
        &mut self,
        envelope: proto::QaulRpc,
        timeout: Duration,
        expect_response: bool,
    ) -> Result<Option<proto::QaulRpc>, Box<dyn std::error::Error>>;
}

/// Helper: encode a `QaulRpc` to bytes. Both transports use this.
pub(crate) fn encode_envelope(envelope: &proto::QaulRpc) -> Vec<u8> {
    let mut buf = Vec::with_capacity(envelope.encoded_len());
    envelope
        .encode(&mut buf)
        .expect("Vec<u8> provides capacity as needed");
    buf
}
