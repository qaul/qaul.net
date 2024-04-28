// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Messaging Protocol Definition
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

use crate::types::QaulMessagingData;

/// define maximal message length in bytes
///
/// This length must not be exceeded. Packages exceeding this size will be discarded.
const MAX_MESSAGE_LEN_BYTES: usize = 65536;

/// create protocol name
pub const PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/qaul_messaging/1.0.0");

/// Implementation of `ConnectionUpgrade` for the qaul_messaging protocol.
#[derive(Debug, Clone, Default)]
pub struct QaulMessagingProtocol {}

impl QaulMessagingProtocol {
    /// Builds a new `QaulMessagingProtocol`.
    pub fn new() -> QaulMessagingProtocol {
        QaulMessagingProtocol {}
    }
}

impl UpgradeInfo for QaulMessagingProtocol {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulMessagingProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulMessagingData;
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
            Ok(QaulMessagingData { data })
        })
    }
}

impl UpgradeInfo for QaulMessagingData {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulMessagingData
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
