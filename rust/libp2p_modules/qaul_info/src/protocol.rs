// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use futures::{
    io::{AsyncRead, AsyncWrite},
    AsyncWriteExt, Future,
};
use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use std::{io, iter, pin::Pin};

use crate::types::QaulInfoData;

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
    type Info = &'static str;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once("/qaul_info/1.0.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulInfoProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulInfoData;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut substream: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            // receive new QaulInfo binary message
            let data = upgrade::read_length_prefixed(&mut substream, 65536).await?;

            // hand it directly to the internal message bus
            Ok(QaulInfoData { data })
        })
    }
}

impl UpgradeInfo for QaulInfoData {
    type Info = &'static str;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once("/qaul_info/1.0.0")
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulInfoData
where
    TSocket: AsyncWrite + AsyncRead + Send + Unpin + 'static,
{
    type Output = ();
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut substream: TSocket, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            upgrade::write_length_prefixed(&mut substream, self.data).await?;
            substream.close().await?;

            Ok(())
        })
    }
}

impl AsRef<[u8]> for QaulInfoData {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
