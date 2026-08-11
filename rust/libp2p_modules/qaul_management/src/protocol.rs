// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Qaul Management Protocol Definition
//! same package prefix with unsigned_varint just as in routing-info

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

use crate::types::QaulManagementData;

/// Maximum message length in bytes. Packages exceeding this are discarded.
const MAX_MESSAGE_LEN_BYTES: usize = 65535;

/// Protocol name.
pub const PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/qaul_management/v2/1.0.0");

/// Implementation of `ConnectionUpgrade` for the qaul_management protocol.
#[derive(Debug, Clone, Default)]
pub struct QaulManagementProtocol {}

impl QaulManagementProtocol {
    /// Builds a new `QaulManagementProtocol`.
    pub fn new() -> QaulManagementProtocol {
        QaulManagementProtocol {}
    }
}

impl UpgradeInfo for QaulManagementProtocol {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulManagementProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulManagementData;
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

            Ok(QaulManagementData { data })
        })
    }
}

impl UpgradeInfo for QaulManagementData {
    type Info = StreamProtocol;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(PROTOCOL_NAME)
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulManagementData
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

impl AsRef<[u8]> for QaulManagementData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
