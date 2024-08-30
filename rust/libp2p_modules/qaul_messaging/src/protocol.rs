// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Qaul Messaging Protocol

use futures::{
    io::{AsyncRead, AsyncWrite},
    AsyncWriteExt, Future,
};
use libp2p::core::{upgrade, InboundUpgrade, OutboundUpgrade, UpgradeInfo};
use std::{io, iter, pin::Pin};

use crate::types::QaulMessagingData;

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
    type Info = &'static str;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once("/qaul_messaging/1.0.0")
    }
}

impl<TSocket> InboundUpgrade<TSocket> for QaulMessagingProtocol
where
    TSocket: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    type Output = QaulMessagingData;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut substream: TSocket, _info: Self::Info) -> Self::Future {
        Box::pin(async move {
            // receive new QaulMessaging binary message
            let data = upgrade::read_length_prefixed(&mut substream, 65536).await?;

            // hand it directly to the internal message bus
            Ok(QaulMessagingData { data })
        })
    }
}

impl UpgradeInfo for QaulMessagingData {
    type Info = &'static str;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once("/qaul_messaging/1.0.0")
    }
}

impl<TSocket> OutboundUpgrade<TSocket> for QaulMessagingData
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
