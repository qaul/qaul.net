// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Routing Info Protocol Definition
//!
//! The codec used is prefixing all packages with an `unsigned_varint` encoded message length.

use asynchronous_codec::Framed;
use futures::SinkExt;
use futures::StreamExt;
use futures::{
    io::{AsyncRead, AsyncWrite},
    Future,
};
use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use libp2p::swarm::StreamProtocol;
use std::{io, iter, pin::Pin};

use varint_prefixed_codec::VarintPrefixedCodec;

use crate::types::QaulInfoData;

/// define maximal message length in bytes
///
/// This length must not be exceeded. Packages exceeding this size will be discarded.
const MAX_MESSAGE_LEN_BYTES: usize = 65535;

/// create protocol name
pub const PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/qaul_info/1.0.0");

/// Implementation of `ConnectionUpgrade` for the qaul_info protocol.
#[derive(Debug, Clone, Default)]
pub struct QaulInfoProtocol {}

impl QaulInfoProtocol {
    /// Builds a new `QaulInfoProtocol`.
    pub fn new() -> QaulInfoProtocol {
        QaulInfoProtocol {}
    }
}

impl UpgradeInfo for QaulInfoProtocol {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulInfoProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulInfoData;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, socket: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            let mut framed = Framed::new(socket, VarintPrefixedCodec::new(MAX_MESSAGE_LEN_BYTES));

            let incoming_data = framed
                .next()
                .await
                .ok_or_else(|| io::ErrorKind::UnexpectedEof)?;

            let data: Vec<u8> = match incoming_data {
                Ok(data) => data,
                Err(err) => {
                    return Err(err);
                }
            };

            // hand it directly to the internal message bus
            Ok(QaulInfoData { data })
        })
    }
}

impl UpgradeInfo for QaulInfoData {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulInfoData
where
    TSocket: AsyncWrite + AsyncRead + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, socket: TSocket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            let mut framed = Framed::new(socket, VarintPrefixedCodec::new(MAX_MESSAGE_LEN_BYTES));
            framed.send(self.data).await?;
            framed.close().await?;

            Ok(())
        })
    }
}

impl AsRef<[u8]> for QaulInfoData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
